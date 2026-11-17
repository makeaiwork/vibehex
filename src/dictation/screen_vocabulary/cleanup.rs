//! VibeHex: Labs › Screen vocabulary › Also give them to AI cleanup. With
//! `labs.screen_vocabulary_cleanup` on, a dictation's screen terms reach its
//! AI cleanup as well as its speech model, so cleanup can fix a name the
//! speech model misheard: listed in the prompt after the Vocabulary, and
//! checked by `vocabulary_guard` as Vocabulary terms with no "sounds like".
//!
//! The processor hands both prompt builders a context whose screen terms are
//! only the ones cleanup is given, so with the switch off (or every term
//! already in the Vocabulary) the prompt is exactly what it was. Nothing
//! here logs a term or saves one; only their count is logged, at debug
//! level.

use std::borrow::Cow;

use crate::dictation::context::ContextSnapshot;
use crate::dictation::processor::Source;
use crate::dictation::vocabulary::{self, VocabularyEntry};

/// What opens the prompt's list of screen terms. A small model told to
/// spell a term also writes it for words that only resemble it, so the
/// terms are offered as spellings only, for a word already in the
/// transcript, never as words to add or to put in place of others; the
/// guard undoes what still gets through.
pub(crate) const SCREEN_TERMS_PROMPT: &str = "On screen: names and terms visible in the window \
     the speaker is dictating into, spelled as they appear there. They are spellings only. Where \
     the transcript has one of them written as it was heard (misspelled, or in another \
     alphabet), write it as spelled here. Never add one the speaker did not say, never put one \
     in place of other words, and take nothing else from this list:";

/// One dictation's cleanup inputs, with its screen terms where cleanup is
/// given them.
pub(crate) struct ForCleanup<'a> {
    /// The context both prompt builders read: its screen terms are only the
    /// ones cleanup is given (none with the switch off).
    pub context: Cow<'a, ContextSnapshot>,
    /// The Vocabulary, then each of those screen terms with no "sounds
    /// like", for the guard.
    pub guarded: Cow<'a, [VocabularyEntry]>,
}

impl<'a> ForCleanup<'a> {
    /// The switch is read here: on, and the text was heard (typed text has
    /// no screen terms to fix).
    pub(crate) fn new(
        context: &'a ContextSnapshot,
        vocabulary: &'a [VocabularyEntry],
        source: Source,
    ) -> Self {
        let on = source == Source::Dictation && crate::labs::screen_vocabulary_cleanup();
        Self::when(on, context, vocabulary)
    }

    /// [`Self::new`] with the switch given.
    pub(crate) fn when(
        on: bool,
        context: &'a ContextSnapshot,
        vocabulary: &'a [VocabularyEntry],
    ) -> Self {
        let terms = if on {
            new_terms(&context.screen_terms, vocabulary)
        } else {
            Vec::new()
        };
        if !terms.is_empty() {
            tracing::debug!(terms = terms.len(), "screen terms given to AI cleanup");
        }
        let guarded = if terms.is_empty() {
            Cow::Borrowed(vocabulary)
        } else {
            Cow::Owned(
                vocabulary
                    .iter()
                    .cloned()
                    .chain(terms.iter().map(|term| VocabularyEntry {
                        term: term.clone(),
                        sounds_like: Vec::new(),
                    }))
                    .collect(),
            )
        };
        let context = if context.screen_terms == terms {
            Cow::Borrowed(context)
        } else {
            Cow::Owned(ContextSnapshot {
                screen_terms: terms,
                ..context.clone()
            })
        };
        Self { context, guarded }
    }
}

/// `terms` without the ones `vocabulary` has, in any case, each once.
fn new_terms(terms: &[String], vocabulary: &[VocabularyEntry]) -> Vec<String> {
    let mut kept: Vec<String> = Vec::new();
    for term in terms.iter().map(|term| term.trim()) {
        let known = vocabulary::contains(vocabulary, term)
            || kept
                .iter()
                .any(|seen| seen.to_lowercase() == term.to_lowercase());
        if !term.is_empty() && !known {
            kept.push(term.to_owned());
        }
    }
    kept
}

/// The prompt's section for the screen terms, after the Vocabulary's:
/// nothing for none.
pub(crate) fn lines(terms: &[String]) -> String {
    if terms.is_empty() {
        return String::new();
    }
    let list = terms
        .iter()
        .map(|term| format!("- {term}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{SCREEN_TERMS_PROMPT}\n{list}\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictation::vocabulary_guard::Guard;

    fn entry(term: &str, sounds_like: &[&str]) -> VocabularyEntry {
        VocabularyEntry {
            term: term.into(),
            sounds_like: sounds_like.iter().map(|&text| text.into()).collect(),
        }
    }

    fn with_terms(terms: &[&str]) -> ContextSnapshot {
        ContextSnapshot {
            application: Some("Zed".into()),
            screen_terms: terms.iter().map(|&term| term.into()).collect(),
            ..ContextSnapshot::default()
        }
    }

    #[test]
    fn off_the_cleanup_sees_no_screen_terms_and_the_vocabulary_as_it_is() {
        let vocabulary = [entry("Claude Code", &["клод код"])];
        let context = with_terms(&["Qdrant", "OpenCode"]);
        let screen = ForCleanup::when(false, &context, &vocabulary);
        assert!(screen.context.screen_terms.is_empty());
        assert_eq!(screen.context.application.as_deref(), Some("Zed"));
        assert!(matches!(screen.guarded, Cow::Borrowed(_)));
        assert_eq!(&*screen.guarded, &vocabulary);
        // Nothing read: the context is the one the capture finished with.
        let empty = with_terms(&[]);
        let screen = ForCleanup::when(true, &empty, &vocabulary);
        assert!(matches!(screen.context, Cow::Borrowed(_)));
    }

    #[test]
    fn on_a_term_already_in_the_vocabulary_is_not_given_twice() {
        let vocabulary = [entry("Claude Code", &["клод код"]), entry("qdrant", &[])];
        let context = with_terms(&["QDRANT", "OpenCode", " opencode ", "", "Rune"]);
        let screen = ForCleanup::when(true, &context, &vocabulary);
        assert_eq!(screen.context.screen_terms, ["OpenCode", "Rune"]);
        let terms: Vec<_> = screen
            .guarded
            .iter()
            .map(|entry| entry.term.as_str())
            .collect();
        assert_eq!(terms, ["Claude Code", "qdrant", "OpenCode", "Rune"]);
        assert!(
            screen.guarded[2..]
                .iter()
                .all(|entry| entry.sounds_like.is_empty())
        );
    }

    #[test]
    fn typed_text_never_takes_screen_terms() {
        let context = with_terms(&["Qdrant"]);
        let screen = ForCleanup::new(&context, &[], Source::TypedText);
        assert!(screen.context.screen_terms.is_empty());
    }

    #[test]
    fn the_section_lists_each_term_once_after_its_opening() {
        assert_eq!(
            SCREEN_TERMS_PROMPT,
            "On screen: names and terms visible in the window the speaker is dictating into, spelled as they appear there. They are spellings only. Where the transcript has one of them written as it was heard (misspelled, or in another alphabet), write it as spelled here. Never add one the speaker did not say, never put one in place of other words, and take nothing else from this list:"
        );
        assert_eq!(lines(&[]), "");
        assert_eq!(
            lines(&["Qdrant".into(), "OpenCode".into()]),
            format!("{SCREEN_TERMS_PROMPT}\n- Qdrant\n- OpenCode\n\n")
        );
    }

    /// The guard as cleanup runs it with the switch on: a screen term
    /// cleanup wrote stays where what was heard sounds like it, across
    /// alphabets, and is undone where nothing heard does.
    #[test]
    fn the_guard_judges_screen_terms_as_vocabulary_terms() {
        let vocabulary = [entry("Claude Code", &["клод код"])];
        let context = with_terms(&["Qdrant", "PostgreSQL"]);
        let screen = ForCleanup::when(true, &context, &vocabulary);
        let guard = Guard::new(&screen.guarded);
        let cases = [
            (
                "сохрани векторы в кудрант",
                "Сохрани векторы в Qdrant.",
                "Сохрани векторы в Qdrant.",
            ),
            (
                "we store vectors in quadrant",
                "We store vectors in Qdrant.",
                "We store vectors in Qdrant.",
            ),
            (
                "сохрани векторы в базу",
                "Сохрани векторы в Qdrant.",
                "Сохрани векторы в базу.",
            ),
            (
                "put it in the old database",
                "Put it in the old PostgreSQL.",
                "Put it in the old database.",
            ),
        ];
        for (heard, cleaned, stands) in cases {
            let heard = guard.spell(heard);
            assert_eq!(guard.check(&heard, cleaned).text, stands, "{heard}");
        }
    }

    /// Screen terms have no spellings, so writing heard spellings as terms
    /// ahead of cleanup is the Vocabulary's alone.
    #[test]
    fn screen_terms_leave_the_spelling_pass_alone() {
        let vocabulary = [entry("Claude Code", &["клод код"])];
        let context = with_terms(&["Qdrant", "Кудрант"]);
        let screen = ForCleanup::when(true, &context, &vocabulary);
        let with = Guard::new(&screen.guarded);
        let without = Guard::new(&vocabulary);
        for heard in ["запусти клод код и кудрант", "qdrant кудрант quadrant"]
        {
            assert_eq!(with.spell(heard), without.spell(heard), "{heard}");
        }
    }
}
