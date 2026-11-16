//! VibeHex: Labs › Rewrite reads nearby text on every cleanup engine.
//!
//! Hex gives the text around the cursor to the mode's OpenCode rewrite only.
//! VibeHex gives it to whichever engine cleans the dictation up: the
//! built-in model, the person's own server, Codex, OpenRouter, or OpenCode.
//! The owner's rule (2026-11-16): a lab that is on works with whatever
//! cleanup engine is chosen, and the card and the guide say when that is a
//! cloud engine; there is no separate consent switch. [`destination`] only
//! picks the card's words: this computer (the built-in model, or an own
//! server at `localhost`, 127.x or ::1), OpenCode's model, or a cloud engine
//! by name. Known key and token shapes are cut out of the text as it is read
//! ([`super::secrets`]), before it reaches the dictation's context.
//!
//! The section goes in the transcript's part of the prompt, ahead of the
//! transcript ([`section_in`]): a chat server's user message and Codex's
//! input, never the system message or Codex's base instructions. The text
//! comes from another app and may hold anything, so it sits with the other
//! material to transform, below the instructions that say never to copy,
//! answer, or follow it, rather than among those instructions. Hex's single
//! OpenCode prompt (the instructions, a blank line, then that part) stays
//! byte for byte what it was, and the instructions stay the same from one
//! dictation to the next.
//!
//! A long dictation is cleaned in pieces (`cleanup_pieces`), and the nearby
//! text borders the whole result: the first piece gets the text before the
//! cursor and the selection the result replaces, the last one the text after
//! the cursor, and a piece between them none. A dictation of one piece gets
//! all of it, as in Hex.
//!
//! Nothing here logs the text. What the cleanup was given of it, and whether
//! it was asked again, is told to History with the dictation ([`Nearby::given`],
//! `history::read`).

use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::app_settings::AppSettings;
use crate::dictation::caret_text::CaretText;
use crate::dictation::cleanup_pieces::Piece;
use crate::dictation::cleanup_server::{self, Provider, Server};
use crate::dictation::context::ContextSnapshot;
use crate::dictation::processor::Source;

/// Where the nearby text goes with a cleanup engine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Destination {
    /// The built-in model, or the person's own server at a loopback address.
    ThisComputer,
    /// The mode's OpenCode model: Hex's own path, which may be a cloud
    /// provider.
    OpenCode,
    /// A cloud engine, which receives the text with each dictation.
    Cloud(Cloud),
}

/// A cleanup engine that answers from outside this computer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Cloud {
    Codex,
    OpenRouter,
    /// The person's own server at an address that is not this computer, by
    /// its host; empty when the address names none.
    Server(String),
}

/// Where the nearby text goes with `server`, the cleanup engine chosen now
/// (`None` is OpenCode), for the card's line. An own server counts as this
/// computer only at `localhost`, 127.0.0.1 (any 127.x address) or ::1; any
/// other address, or one that doesn't parse, is named as a server elsewhere.
pub(crate) fn destination(server: Option<&Server>) -> Destination {
    let Some(server) = server else {
        return Destination::OpenCode;
    };
    match server.provider {
        Provider::Local => Destination::ThisComputer,
        Provider::Codex => Destination::Cloud(Cloud::Codex),
        Provider::OpenRouter => Destination::Cloud(Cloud::OpenRouter),
        Provider::Custom => match url::Url::parse(server.url.trim()) {
            Ok(url) if cleanup_server::is_loopback(&url) => Destination::ThisComputer,
            Ok(url) => Destination::Cloud(Cloud::Server(url.host_str().unwrap_or("").into())),
            Err(_) => Destination::Cloud(Cloud::Server(String::new())),
        },
    }
}

/// The cleanup engine `settings` choose, as the processor finds it: `None`
/// is OpenCode, which is also the answer while Labs › Clean up on your own
/// server is off.
pub(crate) fn chosen(settings: &AppSettings) -> Option<Server> {
    if !settings.labs.ai_cleanup_server {
        return None;
    }
    Server::from_settings(&settings.dictation_processing.server)
}

/// One dictation's nearby text, as its cleanup is given it.
pub(crate) struct Nearby<'a> {
    context: &'a ContextSnapshot,
    /// The text around the cursor this cleanup is given, if any.
    caret: Option<&'a CaretText>,
    /// A built-in model on this computer cleans up: it gets less of the
    /// text ([`part`]).
    small: bool,
    /// For History ([`Self::given`]): a piece was asked, one was asked with
    /// the nearby text, and one was asked again without it.
    asked: AtomicBool,
    gave: AtomicBool,
    again: AtomicBool,
}

/// The most of the text before the cursor a built-in model is given, in
/// characters: enough for the sentence being continued and the names in it.
const SMALL_BEFORE: usize = 300;

impl<'a> Nearby<'a> {
    /// The lab is read here: on, whatever engine runs the cleanup
    /// (`server`; `None` is OpenCode).
    pub(crate) fn new(
        context: &'a ContextSnapshot,
        source: Source,
        server: Option<&Server>,
    ) -> Self {
        Self {
            small: server.is_some_and(|server| server.provider == Provider::Local),
            ..Self::when(crate::labs::cursor_context(), context, source)
        }
    }

    /// Cleanup's answer for `piece`, asked with the nearby text the piece
    /// borders. An answer that repeats that text instead of what was said
    /// (`echo`) is asked again without it, so a model that can't keep the
    /// two apart still cleans the dictation up.
    pub(crate) fn asked<E>(
        &self,
        piece: Piece<'_>,
        ask: impl Fn(&ContextSnapshot, &str) -> Result<String, E>,
    ) -> Result<String, E> {
        let context = self.context(piece);
        self.asked.store(true, Ordering::Relaxed);
        self.gave
            .fetch_or(context.caret_text.is_some(), Ordering::Relaxed);
        let answer = ask(&context, piece.text)?;
        let echoed = context
            .caret_text
            .as_ref()
            .is_some_and(|caret| super::echo::echoes(caret, piece.text, &answer));
        if !echoed {
            return Ok(answer);
        }
        tracing::info!("cleanup repeated the nearby text; asked again without it");
        self.again.store(true, Ordering::Relaxed);
        let bare = ContextSnapshot {
            caret_text: None,
            ..self.context.clone()
        };
        ask(&bare, piece.text)
    }

    /// [`Self::new`] with the lab given. Typed text never takes it.
    pub(crate) fn when(used: bool, context: &'a ContextSnapshot, source: Source) -> Self {
        let caret = super::for_source(context, source).filter(|_| used);
        Self {
            context,
            caret,
            small: false,
            asked: AtomicBool::new(false),
            gave: AtomicBool::new(false),
            again: AtomicBool::new(false),
        }
    }

    /// What this dictation's cleanup was given, for its History entry: the
    /// screen terms its prompt listed (the context's, `ForCleanup`) once a
    /// piece was asked, and how much of the nearby text: all of it, a
    /// built-in model's part (unless that part is all there was), or none.
    pub(crate) fn given(&self) -> crate::history::read::Given {
        use crate::history::read::NearbySent;
        let asked = self.asked.load(Ordering::Relaxed);
        let nearby = match self.caret {
            Some(caret) if self.gave.load(Ordering::Relaxed) => {
                let whole = Piece {
                    text: "",
                    first: true,
                    last: true,
                };
                if self.small && part(caret, whole, true) != *caret {
                    NearbySent::Part
                } else {
                    NearbySent::All
                }
            }
            _ => NearbySent::Nothing,
        };
        crate::history::read::Given {
            cleanup_terms: match asked {
                true => self.context.screen_terms.clone(),
                false => Vec::new(),
            },
            nearby,
            asked_again: self.again.load(Ordering::Relaxed),
        }
    }

    /// The context `piece`'s prompt reads: its caret text is the part that
    /// borders `piece`, or none. The dictation's own context while that is
    /// all of it, so a one-piece prompt is exactly Hex's.
    pub(crate) fn context(&self, piece: Piece<'_>) -> Cow<'a, ContextSnapshot> {
        let part = self.caret.map(|caret| part(caret, piece, self.small));
        if part.as_ref() == self.context.caret_text.as_ref() {
            return Cow::Borrowed(self.context);
        }
        Cow::Owned(ContextSnapshot {
            caret_text: part.filter(|part| !part.is_empty()),
            ..self.context.clone()
        })
    }
}

/// What of `caret` borders `piece`: before the cursor and the selection for
/// the first piece, after the cursor for the last. A built-in model
/// (`small`) gets the end of the text before the cursor and none of the text
/// after it: beside a short transcript it returned the text after the cursor
/// in place of what was said, and that text only helps fit the last
/// punctuation mark.
fn part(caret: &CaretText, piece: Piece<'_>, small: bool) -> CaretText {
    if small {
        return CaretText {
            before: match piece.first {
                true => tail(&caret.before, SMALL_BEFORE).to_owned(),
                false => String::new(),
            },
            selected: match piece.first {
                true => caret.selected.clone(),
                false => String::new(),
            },
            after: String::new(),
        };
    }
    let keep = |borders: bool, text: &str| {
        if borders {
            text.to_owned()
        } else {
            String::new()
        }
    };
    CaretText {
        before: keep(piece.first, &caret.before),
        selected: keep(piece.first, &caret.selected),
        after: keep(piece.last, &caret.after),
    }
}

/// The last `most` characters of `text`, from the start of a word.
fn tail(text: &str, most: usize) -> &str {
    let Some((start, _)) = text.char_indices().rev().nth(most) else {
        return text;
    };
    let cut = &text[start..];
    cut.find(char::is_whitespace)
        .map_or(cut, |space| cut[space..].trim_start())
}

/// `context` as a prompt for `source` reads it: typed text has no caret
/// text, a dictation all of its own.
pub(crate) fn for_source(context: &ContextSnapshot, source: Source) -> Cow<'_, ContextSnapshot> {
    let whole = Piece {
        text: "",
        first: true,
        last: true,
    };
    Nearby::when(true, context, source).context(whole)
}

/// The section `context`'s caret text makes, ahead of the transcript block;
/// empty without any, so the prompt is what it was.
pub(crate) fn section_in(context: &ContextSnapshot) -> String {
    context
        .caret_text
        .as_ref()
        .map(super::section)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_settings::CleanupRunsOn;

    fn server(provider: Provider, url: &str) -> Server {
        Server {
            provider,
            url: url.into(),
            model: String::new(),
            effort: String::new(),
        }
    }

    fn caret(before: &str, selected: &str, after: &str) -> CaretText {
        CaretText {
            before: before.into(),
            selected: selected.into(),
            after: after.into(),
        }
    }

    fn with_caret() -> ContextSnapshot {
        ContextSnapshot {
            application: Some("Notes".into()),
            caret_text: Some(caret("We ship on ", "Friday", " morning.")),
            ..ContextSnapshot::default()
        }
    }

    fn piece(first: bool, last: bool) -> Piece<'static> {
        Piece {
            text: "",
            first,
            last,
        }
    }

    /// For the card's line: the built-in model and an own server at this
    /// computer's address keep the text here; Codex, OpenRouter and an own
    /// server anywhere else are cloud engines, named by their host; OpenCode
    /// is Hex's own path.
    #[test]
    fn each_engine_names_where_the_text_goes() {
        use Destination::{Cloud as Away, OpenCode, ThisComputer};
        assert_eq!(destination(None), OpenCode);
        let cases = [
            (server(Provider::Local, ""), ThisComputer),
            (server(Provider::Codex, ""), Away(Cloud::Codex)),
            (
                server(Provider::OpenRouter, cleanup_server::OPENROUTER_URL),
                Away(Cloud::OpenRouter),
            ),
            (
                server(Provider::Custom, cleanup_server::LM_STUDIO_URL),
                ThisComputer,
            ),
            (
                server(Provider::Custom, cleanup_server::OLLAMA_URL),
                ThisComputer,
            ),
            (
                server(Provider::Custom, "http://localhost:8080/v1"),
                ThisComputer,
            ),
            (
                server(Provider::Custom, " http://LOCALHOST:8080/v1 "),
                ThisComputer,
            ),
            (
                server(Provider::Custom, "http://[::1]:8080/v1"),
                ThisComputer,
            ),
            (
                server(Provider::Custom, "http://192.168.1.20:1234/v1"),
                Away(Cloud::Server("192.168.1.20".into())),
            ),
            (
                server(Provider::Custom, "https://api.groq.com/openai/v1"),
                Away(Cloud::Server("api.groq.com".into())),
            ),
            (
                server(Provider::Custom, "http://0.0.0.0:1234/v1"),
                Away(Cloud::Server("0.0.0.0".into())),
            ),
            (
                server(Provider::Custom, "http://localhost.example.com/v1"),
                Away(Cloud::Server("localhost.example.com".into())),
            ),
            (
                server(Provider::Custom, ""),
                Away(Cloud::Server(String::new())),
            ),
        ];
        for (server, expected) in cases {
            assert_eq!(destination(Some(&server)), expected, "{}", server.url);
        }
    }

    /// The chosen engine is OpenCode while Clean up on your own server is
    /// off, whatever Runs on says.
    #[test]
    fn the_engine_chosen_follows_the_cleanup_lab() {
        let mut settings = AppSettings::default();
        settings.dictation_processing.server.runs_on = CleanupRunsOn::Codex;
        assert_eq!(chosen(&settings), None);
        settings.labs.ai_cleanup_server = true;
        assert_eq!(
            chosen(&settings).map(|server| server.provider),
            Some(Provider::Codex)
        );
    }

    /// The lab on gives the text to whatever engine runs; off (or for
    /// typed text) nothing.
    #[test]
    fn the_lab_alone_decides_whether_cleanup_gets_the_text() {
        let context = with_caret();
        let whole = piece(true, true);
        let refused = Nearby::when(false, &context, Source::Dictation);
        assert_eq!(refused.context(whole).caret_text, None);
        assert_eq!(refused.context(whole).application.as_deref(), Some("Notes"));
        let given = Nearby::when(true, &context, Source::Dictation);
        assert!(matches!(given.context(whole), Cow::Borrowed(_)));
        crate::labs::apply(&AppSettings::default().labs);
        let off = Nearby::new(&context, Source::Dictation, None);
        assert_eq!(off.context(whole).caret_text, None, "the lab is off");
        let mut settings = AppSettings::default();
        settings.labs.cursor_context = true;
        crate::labs::apply(&settings.labs);
        let on = Nearby::new(&context, Source::Dictation, None);
        assert_eq!(
            on.context(whole).caret_text.is_some(),
            crate::labs::Feature::CursorContext.on_this_os()
        );
        crate::labs::apply(&AppSettings::default().labs);
    }

    /// A long dictation's first piece gets what comes before the cursor and
    /// the selection, its last what comes after, a piece between them
    /// nothing; one piece gets it all, as the dictation's own context.
    #[test]
    fn each_piece_gets_the_part_it_borders() {
        let context = with_caret();
        let nearby = Nearby::when(true, &context, Source::Dictation);
        let first = nearby.context(piece(true, false));
        assert_eq!(first.caret_text, Some(caret("We ship on ", "Friday", "")));
        let middle = nearby.context(piece(false, false));
        assert_eq!(middle.caret_text, None);
        assert_eq!(middle.application.as_deref(), Some("Notes"));
        let last = nearby.context(piece(false, true));
        assert_eq!(last.caret_text, Some(caret("", "", " morning.")));
        assert!(matches!(
            nearby.context(piece(true, true)),
            Cow::Borrowed(_)
        ));
        // A cursor at the end of the field has nothing after it.
        let at_end = ContextSnapshot {
            caret_text: Some(caret("Dear Anna,", "", "")),
            ..ContextSnapshot::default()
        };
        let nearby = Nearby::when(true, &at_end, Source::Dictation);
        assert_eq!(nearby.context(piece(false, true)).caret_text, None);
    }

    /// Typed text never takes the nearby text, and a context without any is
    /// left as it is.
    #[test]
    fn typed_text_and_a_context_without_text_take_none() {
        let context = with_caret();
        let typed = Nearby::when(true, &context, Source::TypedText);
        assert_eq!(typed.context(piece(true, true)).caret_text, None);
        assert_eq!(for_source(&context, Source::TypedText).caret_text, None);
        assert!(matches!(
            for_source(&context, Source::Dictation),
            Cow::Borrowed(_)
        ));
        let plain = ContextSnapshot::default();
        assert!(matches!(
            Nearby::when(true, &plain, Source::Dictation).context(piece(true, false)),
            Cow::Borrowed(_)
        ));
        assert_eq!(section_in(&plain), "");
        assert_eq!(
            section_in(&context),
            super::super::section(context.caret_text.as_ref().unwrap())
        );
    }
    /// An answer that repeats the nearby text is asked again without it,
    /// once; an honest answer is asked once.
    #[test]
    fn an_answer_that_repeats_the_nearby_text_is_asked_again_without_it() {
        use std::cell::RefCell;
        let context = ContextSnapshot {
            caret_text: Some(caret(
                "Она использует ",
                "",
                "? И второе, вот этот вот Cursor контекст используется или нет?",
            )),
            ..ContextSnapshot::default()
        };
        let nearby = Nearby::when(true, &context, Source::Dictation);
        let whole = Piece {
            text: "Screen vocabulary",
            first: true,
            last: true,
        };
        let asked = RefCell::new(Vec::new());
        // A model that returns the text after the cursor whenever it has it.
        let echoing = |context: &ContextSnapshot, heard: &str| -> Result<String, ()> {
            asked.borrow_mut().push(context.caret_text.is_some());
            Ok(match &context.caret_text {
                Some(caret) => caret.after.clone(),
                None => heard.to_owned(),
            })
        };
        assert_eq!(nearby.asked(whole, echoing), Ok("Screen vocabulary".into()));
        assert_eq!(*asked.borrow(), [true, false]);
        asked.borrow_mut().clear();
        let honest = |context: &ContextSnapshot, heard: &str| -> Result<String, ()> {
            asked.borrow_mut().push(context.caret_text.is_some());
            Ok(heard.to_owned())
        };
        assert_eq!(nearby.asked(whole, honest), Ok("Screen vocabulary".into()));
        assert_eq!(*asked.borrow(), [true]);
    }

    /// A built-in model gets the end of the text before the cursor, from a
    /// word's start, the selection, and none of the text after the cursor.
    #[test]
    fn a_built_in_model_gets_less_of_the_nearby_text() {
        let long = "слово ".repeat(120);
        let text = caret(&long, "выделено", " после курсора.");
        let small = part(&text, piece(true, true), true);
        assert!(small.after.is_empty());
        assert_eq!(small.selected, "выделено");
        assert!(small.before.chars().count() <= SMALL_BEFORE);
        assert!(small.before.starts_with("слово") && long.ends_with(&small.before));
        // Short text is given whole; another engine gets all of it.
        assert_eq!(
            part(&caret("Привет, ", "", " мир"), piece(true, true), true).before,
            "Привет, "
        );
        assert_eq!(part(&text, piece(true, true), false), text);
        let local = server(Provider::Local, "");
        let context = ContextSnapshot::default();
        assert!(Nearby::new(&context, Source::Dictation, Some(&local)).small);
        let codex = server(Provider::Codex, "");
        assert!(!Nearby::new(&context, Source::Dictation, Some(&codex)).small);
        assert!(!Nearby::new(&context, Source::Dictation, None).small);
    }

    /// VibeHex History: what a dictation's cleanup was given of what it
    /// read, told once its pieces were asked.
    #[test]
    fn what_cleanup_was_given_is_told_to_history() {
        use crate::history::read::{Given, NearbySent};
        let whole = Piece {
            text: "Screen vocabulary",
            first: true,
            last: true,
        };
        let honest = |_: &ContextSnapshot, heard: &str| -> Result<String, ()> { Ok(heard.into()) };
        let context = ContextSnapshot {
            screen_terms: vec!["Qdrant".into()],
            ..with_caret()
        };
        let nearby = Nearby::when(true, &context, Source::Dictation);
        assert_eq!(nearby.given(), Given::default(), "nothing was asked yet");
        nearby.asked(whole, honest).unwrap();
        assert_eq!(
            nearby.given(),
            Given {
                cleanup_terms: vec!["Qdrant".into()],
                nearby: NearbySent::All,
                asked_again: false,
            }
        );

        // The lab off, or typed text: the text was not given.
        for nearby in [
            Nearby::when(false, &context, Source::Dictation),
            Nearby::when(true, &context, Source::TypedText),
        ] {
            nearby.asked(whole, honest).unwrap();
            assert_eq!(nearby.given().nearby, NearbySent::Nothing);
            assert!(!nearby.given().asked_again);
        }

        // A built-in model's part, unless that part is all there was.
        let long = ContextSnapshot {
            caret_text: Some(caret(&"слово ".repeat(120), "", "")),
            ..ContextSnapshot::default()
        };
        for (context, sent) in [(&long, NearbySent::Part), (&context, NearbySent::Part)] {
            let small = Nearby {
                small: true,
                ..Nearby::when(true, context, Source::Dictation)
            };
            small.asked(whole, honest).unwrap();
            assert_eq!(small.given().nearby, sent);
        }
        let short = ContextSnapshot {
            caret_text: Some(caret("We ship on ", "Friday", "")),
            ..ContextSnapshot::default()
        };
        let small = Nearby {
            small: true,
            ..Nearby::when(true, &short, Source::Dictation)
        };
        small.asked(whole, honest).unwrap();
        assert_eq!(small.given().nearby, NearbySent::All);

        // An answer that repeats the text is told as asked again.
        let echoed = ContextSnapshot {
            caret_text: Some(caret(
                "Она использует ",
                "",
                "? И второе, вот этот вот Cursor контекст используется или нет?",
            )),
            ..ContextSnapshot::default()
        };
        let echoing = |context: &ContextSnapshot, heard: &str| -> Result<String, ()> {
            Ok(match &context.caret_text {
                Some(caret) => caret.after.clone(),
                None => heard.to_owned(),
            })
        };
        let nearby = Nearby::when(true, &echoed, Source::Dictation);
        assert_eq!(nearby.asked(whole, echoing), Ok("Screen vocabulary".into()));
        assert!(nearby.given().asked_again);
        assert_eq!(nearby.given().nearby, NearbySent::All);
    }
}
