//! What a new conversation's start page suggests: up to four things picked
//! by rules from what the app already knows, so they show at once and with
//! no model connected. The page's old example questions only fill the
//! places left ([`MAX`]).
//!
//! The order is the owner's (2026-11-16): what broke, then what was left
//! unfinished, then where the person came from, then what is new for them.
//! A suggestion pressed or hidden stays away while its reason is the same
//! one ([`Suggestion::key`], kept by [`Hidden`]): a new run of failures, a
//! newer meeting, or the next update brings its own.
//!
//! [`pick`] reads only [`Signals`], which the window fills from the
//! History store, the meetings on disk, the setups kept, and the release
//! notes. None of it is a recording's text: a failure's reason is the
//! app's own message, and a meeting's title already shows in the window.

use serde::{Deserialize, Serialize};

/// How many suggestions and example questions a start page shows together.
pub const MAX: usize = 4;

/// How many of the places what broke may take, so one bad day doesn't push
/// everything else off the page.
const BROKE: usize = 2;

/// A run of failed cleanups this long is worth a suggestion: one or two
/// can be a network blip that the next dictation gets past.
pub const FAILED_IN_A_ROW: usize = 3;

/// How far back a failure or a meeting still counts as now.
const RECENT_MS: u64 = 7 * 24 * 60 * 60 * 1000;

/// One dictation that went through AI cleanup, as History keeps it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cleaned {
    pub at_ms: u64,
    /// Why it was pasted without cleanup, in the engine's own words.
    pub fallback: Option<String>,
}

/// One finished meeting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Met {
    pub id: String,
    pub title: String,
    pub start_ms: u64,
    pub duration_ms: u64,
    /// Its notes were tried and not written.
    pub notes_failed: bool,
}

/// The pane the person was on before Assistant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum From {
    Meetings,
    History,
    Labs,
    Output,
}

/// A setup that was begun and whose plan was never applied or put off.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Left {
    pub conversation: u32,
    pub started_ms: u64,
    /// Its plan is written and waits; else rounds are still to answer.
    pub plan: bool,
}

/// Everything [`pick`] reads.
#[derive(Clone, Debug, Default)]
pub struct Signals {
    pub now_ms: u64,
    /// AI cleanup is on, so a failed one is still today's trouble.
    pub cleanup_on: bool,
    /// The dictations that went through cleanup, newest first.
    pub cleaned: Vec<Cleaned>,
    /// Finished meetings, newest first.
    pub meetings: Vec<Met>,
    /// The meeting open in Meetings when the person left it.
    pub open_meeting: Option<String>,
    pub from: Option<From>,
    pub left: Option<Left>,
    /// The version updated to, while "what's new" wasn't asked.
    pub updated: Option<String>,
    /// Labs features added since the last setup that fit the person and
    /// are off, best first: settings key and card title.
    pub new_labs: Vec<(String, String)>,
}

/// Why cleanup didn't run, read from the engine's message.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Why {
    CodexSignedOut,
    CodexLimit,
    CodexMissing,
    OpencodeNotRunning,
    NoOpenRouterKey,
    ModelNotDownloaded,
    /// It didn't answer in time.
    Slow,
    Other,
}

impl Why {
    /// The reason in a fallback as History keeps it. The messages are the
    /// engines' own (`cleanup_server`, `opencode_service`, `generation`); a
    /// test there would not see a reworded one, so [`tests`] holds one of
    /// each as it reads today.
    pub fn of(fallback: &str) -> Self {
        let text = fallback.to_lowercase();
        let has = |part: &str| text.contains(part);
        if has("sign in to codex") {
            Self::CodexSignedOut
        } else if has("codex limit") {
            Self::CodexLimit
        } else if has("codex isn't installed") {
            Self::CodexMissing
        } else if has("opencode isn't running") {
            Self::OpencodeNotRunning
        } else if has("openrouter api key") {
            Self::NoOpenRouterKey
        } else if has("isn't downloaded") {
            Self::ModelNotDownloaded
        } else if has("didn't answer") || has("exceeded") || has("timeout") || has("timed out") {
            Self::Slow
        } else {
            Self::Other
        }
    }
}

/// One thing the start page offers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Suggestion {
    /// The last `count` dictations were pasted without AI cleanup; the run
    /// began at `since_ms`.
    CleanupFailed {
        count: usize,
        why: Why,
        since_ms: u64,
    },
    /// A meeting's notes were not written.
    NotesFailed(Met),
    /// A setup waits where it was left.
    SetupLeft(Left),
    /// What's new in the version updated to.
    Updated { version: String },
    /// A question about the meeting that was open.
    Meeting(Met),
    /// The action items of the last week's meetings.
    MeetingsWeek { count: usize },
    /// Why a word came out wrong, coming from History.
    WrongWord,
    /// Which Labs features fit, coming from Labs.
    WhichLabs,
    /// How to steer cleanup, or how to get it, coming from Output.
    Cleanup { on: bool },
    /// Labs features new since the last setup that fit and are off.
    NewLabs {
        keys: Vec<String>,
        titles: Vec<String>,
    },
}

impl Suggestion {
    /// What a hidden or pressed suggestion is remembered by: the same
    /// reason keeps the same key, a new one gets its own.
    pub fn key(&self) -> String {
        match self {
            Self::CleanupFailed { since_ms, .. } => format!("cleanup:{since_ms}"),
            Self::NotesFailed(met) => format!("notes:{}", met.id),
            Self::SetupLeft(left) => format!("setup:{}", left.started_ms),
            Self::Updated { version } => format!("updated:{version}"),
            Self::Meeting(met) => format!("meeting:{}", met.id),
            Self::MeetingsWeek { count } => format!("week:{count}"),
            Self::WrongWord => "from:history".to_owned(),
            Self::WhichLabs => "from:labs".to_owned(),
            Self::Cleanup { on } => format!("from:output:{on}"),
            Self::NewLabs { keys, .. } => {
                let mut keys = keys.clone();
                keys.sort();
                format!("new:{}", keys.join(","))
            }
        }
    }

    /// Something stopped working: the row is drawn as a warning, and the
    /// sidebar's Assistant row is marked for it.
    pub fn broke(&self) -> bool {
        matches!(self, Self::CleanupFailed { .. } | Self::NotesFailed(_))
    }

    /// Where the person came from. These come back each visit: hiding one
    /// hides it for that pane, pressing one doesn't.
    pub fn from_pane(&self) -> bool {
        matches!(
            self,
            Self::Meeting(_)
                | Self::MeetingsWeek { .. }
                | Self::WrongWord
                | Self::WhichLabs
                | Self::Cleanup { .. }
        )
    }
}

/// The suggestions for a start page, most pressing first, at most [`MAX`],
/// without the `hidden` ones.
pub fn pick(signals: &Signals, hidden: &Hidden) -> Vec<Suggestion> {
    let mut all = broke(signals);
    all.truncate(BROKE);
    if let Some(left) = signals.left {
        all.push(Suggestion::SetupLeft(left));
    }
    if let Some(version) = &signals.updated {
        all.push(Suggestion::Updated {
            version: version.clone(),
        });
    }
    all.extend(from_pane(signals));
    if !signals.new_labs.is_empty() {
        all.push(Suggestion::NewLabs {
            keys: signals
                .new_labs
                .iter()
                .map(|(key, _)| key.clone())
                .collect(),
            titles: signals
                .new_labs
                .iter()
                .map(|(_, title)| title.clone())
                .collect(),
        });
    }
    all.retain(|suggestion| !hidden.has(&suggestion.key()));
    all.truncate(MAX);
    all
}

/// What stopped working, for the sidebar's mark too: it reads the same
/// rules without the rest of the page.
pub fn broke(signals: &Signals) -> Vec<Suggestion> {
    let mut all = Vec::new();
    if let Some(failed) = cleanup_failed(signals) {
        all.push(failed);
    }
    let recent = |met: &&Met| met.start_ms + RECENT_MS >= signals.now_ms;
    if let Some(met) = signals
        .meetings
        .iter()
        .filter(recent)
        .find(|met| met.notes_failed)
    {
        all.push(Suggestion::NotesFailed(met.clone()));
    }
    all
}

/// The newest dictations all fell back, lately, and cleanup is still on.
fn cleanup_failed(signals: &Signals) -> Option<Suggestion> {
    if !signals.cleanup_on {
        return None;
    }
    let run: Vec<&Cleaned> = signals
        .cleaned
        .iter()
        .take_while(|cleaned| cleaned.fallback.is_some())
        .collect();
    let (newest, oldest) = (run.first()?, run.last()?);
    if run.len() < FAILED_IN_A_ROW || newest.at_ms + RECENT_MS < signals.now_ms {
        return None;
    }
    Some(Suggestion::CleanupFailed {
        count: run.len(),
        why: Why::of(newest.fallback.as_deref().unwrap_or_default()),
        since_ms: oldest.at_ms,
    })
}

/// What fits the pane the person came from.
fn from_pane(signals: &Signals) -> Vec<Suggestion> {
    match signals.from {
        Some(From::Meetings) => {
            let mut all = Vec::new();
            let open = signals.open_meeting.as_deref();
            // A meeting whose notes failed has its own row above.
            if let Some(met) = signals
                .meetings
                .iter()
                .find(|met| Some(met.id.as_str()) == open && !met.notes_failed)
            {
                all.push(Suggestion::Meeting(met.clone()));
            }
            let week = signals
                .meetings
                .iter()
                .filter(|met| met.start_ms + RECENT_MS >= signals.now_ms)
                .count();
            if week >= 2 {
                all.push(Suggestion::MeetingsWeek { count: week });
            }
            all
        }
        Some(From::History) => vec![Suggestion::WrongWord],
        Some(From::Labs) if signals.new_labs.is_empty() => vec![Suggestion::WhichLabs],
        Some(From::Labs) => Vec::new(),
        Some(From::Output) => vec![Suggestion::Cleanup {
            on: signals.cleanup_on,
        }],
        None => Vec::new(),
    }
}

/// The suggestions hidden or acted on, by key, newest last. Kept beside the
/// conversations (`<support>/assistant/suggestions.json`); it holds keys
/// only, which name a time, a version, a meeting's id, or settings keys.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Hidden {
    #[serde(default)]
    keys: Vec<String>,
    /// What broke that Assistant was already opened on: the sidebar's mark
    /// goes then, though the row stays on the page until it is dealt with.
    #[serde(default)]
    seen: Vec<String>,
}

impl Hidden {
    /// As many keys as are kept: old ones can't come back, their reasons
    /// are long gone.
    const MOST: usize = 200;

    pub fn has(&self, key: &str) -> bool {
        self.keys.iter().any(|kept| kept == key)
    }

    /// Hides `key`. True when it wasn't hidden yet.
    pub fn hide(&mut self, key: String) -> bool {
        Self::add(&mut self.keys, key)
    }

    pub fn saw(&self, key: &str) -> bool {
        self.seen.iter().any(|kept| kept == key)
    }

    /// Marks what broke as seen. True when it wasn't yet.
    pub fn see(&mut self, key: String) -> bool {
        Self::add(&mut self.seen, key)
    }

    fn add(keys: &mut Vec<String>, key: String) -> bool {
        if keys.contains(&key) {
            return false;
        }
        keys.push(key);
        if keys.len() > Self::MOST {
            keys.drain(..keys.len() - Self::MOST);
        }
        true
    }

    /// Reads the file; a missing or broken one hides nothing.
    pub fn read(path: &std::path::Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn write(&self, path: &std::path::Path) -> std::io::Result<()> {
        if let Some(folder) = path.parent() {
            std::fs::create_dir_all(folder)?;
        }
        std::fs::write(path, serde_json::to_vec_pretty(self)?)
    }
}

/// What broke and wasn't seen yet: what the sidebar marks Assistant for.
pub fn unseen(signals: &Signals, hidden: &Hidden) -> Option<Suggestion> {
    broke(signals).into_iter().find(|suggestion| {
        let key = suggestion.key();
        !hidden.has(&key) && !hidden.saw(&key)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: u64 = 24 * 60 * 60 * 1000;
    const NOW: u64 = 100 * DAY;

    fn failed(at_ms: u64, why: &str) -> Cleaned {
        Cleaned {
            at_ms,
            fallback: Some(why.to_owned()),
        }
    }

    fn fine(at_ms: u64) -> Cleaned {
        Cleaned {
            at_ms,
            fallback: None,
        }
    }

    fn met(id: &str, days_ago: u64, notes_failed: bool) -> Met {
        Met {
            id: id.to_owned(),
            title: format!("Meeting {id}"),
            start_ms: NOW - days_ago * DAY,
            duration_ms: 30 * 60 * 1000,
            notes_failed,
        }
    }

    fn signals() -> Signals {
        Signals {
            now_ms: NOW,
            cleanup_on: true,
            ..Signals::default()
        }
    }

    #[test]
    fn a_reason_is_read_from_each_engines_own_message() {
        // One of each, as the engines word them today.
        let rows = [
            ("sign in to Codex again", Why::CodexSignedOut),
            (
                "your ChatGPT plan's Codex limit is reached",
                Why::CodexLimit,
            ),
            (
                crate::dictation::cleanup_server::codex::NOT_INSTALLED,
                Why::CodexMissing,
            ),
            ("OpenCode isn't running", Why::OpencodeNotRunning),
            ("add your OpenRouter API key", Why::NoOpenRouterKey),
            (
                "Gemma 4 E2B isn't downloaded yet; download it in Output",
                Why::ModelNotDownloaded,
            ),
            ("Codex didn't answer within 20 seconds", Why::Slow),
            ("OpenCode generation exceeded 30 seconds", Why::Slow),
            ("timeout", Why::Slow),
            (
                "3 of 4 pieces kept as transcribed: sign in to Codex again",
                Why::CodexSignedOut,
            ),
            ("the cleaned text changed too much", Why::Other),
        ];
        for (message, why) in rows {
            assert_eq!(Why::of(message), why, "{message}");
        }
    }

    #[test]
    fn a_run_of_failed_cleanups_is_offered_and_a_blip_is_not() {
        let mut s = signals();
        s.cleaned = vec![
            failed(NOW - 10, "sign in to Codex again"),
            failed(NOW - 20, "sign in to Codex again"),
            fine(NOW - 30),
        ];
        assert_eq!(pick(&s, &Hidden::default()), [], "two can be a blip");

        s.cleaned.insert(1, failed(NOW - 15, "timeout"));
        assert_eq!(
            pick(&s, &Hidden::default()),
            [Suggestion::CleanupFailed {
                count: 3,
                why: Why::CodexSignedOut,
                since_ms: NOW - 20
            }],
            "the newest failure's reason, the run's start"
        );

        // One that worked since ends the run.
        s.cleaned.insert(0, fine(NOW - 5));
        assert_eq!(pick(&s, &Hidden::default()), []);
    }

    #[test]
    fn old_failures_and_cleanup_turned_off_are_not_todays_trouble() {
        let mut s = signals();
        s.cleaned = (0..5)
            .map(|i| failed(NOW - 8 * DAY - i, "timeout"))
            .collect();
        assert_eq!(pick(&s, &Hidden::default()), [], "over a week old");

        s.cleaned = (0..5).map(|i| failed(NOW - i, "timeout")).collect();
        s.cleanup_on = false;
        assert_eq!(pick(&s, &Hidden::default()), [], "cleanup is off now");
    }

    #[test]
    fn a_hidden_run_stays_hidden_while_it_goes_on_and_a_new_one_shows() {
        let mut s = signals();
        s.cleaned = (0..3)
            .map(|i| failed(NOW - 100 + i * 10, "timeout"))
            .rev()
            .collect();
        let shown = pick(&s, &Hidden::default());
        let mut hidden = Hidden::default();
        assert!(hidden.hide(shown[0].key()));
        assert!(!hidden.hide(shown[0].key()), "once");

        // A fourth failure of the same run.
        s.cleaned.insert(0, failed(NOW - 1, "timeout"));
        assert_eq!(pick(&s, &hidden), [], "the same run");

        // It worked, then failed three times again: a run of its own.
        s.cleaned.insert(0, fine(NOW + 10));
        for i in 0..3 {
            s.cleaned.insert(0, failed(NOW + 20 + i, "timeout"));
        }
        s.now_ms = NOW + 100;
        assert_eq!(pick(&s, &hidden).len(), 1, "a new run");
    }

    #[test]
    fn the_newest_meeting_without_notes_is_offered_within_a_week() {
        let mut s = signals();
        s.meetings = vec![
            met("a", 1, false),
            met("b", 2, true),
            met("c", 3, true),
            met("d", 9, true),
        ];
        assert_eq!(
            pick(&s, &Hidden::default()),
            [Suggestion::NotesFailed(met("b", 2, true))]
        );
        s.meetings = vec![met("d", 9, true)];
        assert_eq!(pick(&s, &Hidden::default()), []);
    }

    #[test]
    fn what_broke_comes_first_then_unfinished_then_the_pane_then_new() {
        let mut s = signals();
        s.cleaned = (0..3).map(|i| failed(NOW - i, "timeout")).collect();
        s.meetings = vec![met("a", 0, false), met("b", 1, true)];
        s.open_meeting = Some("a".into());
        s.from = Some(From::Meetings);
        s.left = Some(Left {
            conversation: 4,
            started_ms: NOW - DAY,
            plan: true,
        });
        s.updated = Some("3.4.0-vibe.8".into());
        s.new_labs = vec![("labs.yolo_mode".into(), "YOLO mode".into())];
        let shown = pick(&s, &Hidden::default());
        assert_eq!(shown.len(), MAX);
        assert!(matches!(shown[0], Suggestion::CleanupFailed { .. }));
        assert!(matches!(shown[1], Suggestion::NotesFailed(_)));
        assert!(matches!(shown[2], Suggestion::SetupLeft(_)));
        assert!(matches!(shown[3], Suggestion::Updated { .. }));

        // Hiding makes room for what came after.
        let mut hidden = Hidden::default();
        hidden.hide(shown[0].key());
        hidden.hide(shown[2].key());
        let shown = pick(&s, &hidden);
        assert!(matches!(shown[0], Suggestion::NotesFailed(_)));
        assert!(matches!(shown[1], Suggestion::Updated { .. }));
        assert_eq!(shown[2], Suggestion::Meeting(met("a", 0, false)));
        assert_eq!(shown[3], Suggestion::MeetingsWeek { count: 2 });
    }

    #[test]
    fn each_pane_brings_its_own_question() {
        let mut s = signals();
        s.from = Some(From::History);
        assert_eq!(pick(&s, &Hidden::default()), [Suggestion::WrongWord]);

        s.from = Some(From::Output);
        assert_eq!(
            pick(&s, &Hidden::default()),
            [Suggestion::Cleanup { on: true }]
        );

        s.from = Some(From::Labs);
        assert_eq!(pick(&s, &Hidden::default()), [Suggestion::WhichLabs]);
        // With new features that fit, their row says it better.
        s.new_labs = vec![("labs.yolo_mode".into(), "YOLO mode".into())];
        assert!(matches!(
            pick(&s, &Hidden::default())[..],
            [Suggestion::NewLabs { .. }]
        ));

        // From Meetings with nothing open and one meeting: nothing to ask.
        let mut s = signals();
        s.from = Some(From::Meetings);
        s.meetings = vec![met("a", 0, false)];
        assert_eq!(pick(&s, &Hidden::default()), []);
        // The open meeting's notes failed: one row, not two.
        s.meetings = vec![met("a", 0, true)];
        s.open_meeting = Some("a".into());
        assert_eq!(
            pick(&s, &Hidden::default()),
            [Suggestion::NotesFailed(met("a", 0, true))]
        );
    }

    #[test]
    fn new_features_keep_one_key_whatever_their_order() {
        let a = Suggestion::NewLabs {
            keys: vec!["labs.b".into(), "labs.a".into()],
            titles: Vec::new(),
        };
        let b = Suggestion::NewLabs {
            keys: vec!["labs.a".into(), "labs.b".into()],
            titles: Vec::new(),
        };
        assert_eq!(a.key(), b.key());
        assert_eq!(a.key(), "new:labs.a,labs.b");
    }

    #[test]
    fn the_mark_is_for_what_broke_until_it_is_seen_or_hidden() {
        let mut s = signals();
        s.updated = Some("3.4.0-vibe.8".into());
        let mut hidden = Hidden::default();
        assert_eq!(unseen(&s, &hidden), None, "an update is not trouble");

        s.cleaned = (0..3).map(|i| failed(NOW - i, "timeout")).collect();
        let trouble = unseen(&s, &hidden).expect("marked");
        assert!(trouble.broke());
        assert!(hidden.see(trouble.key()));
        assert_eq!(unseen(&s, &hidden), None, "seen");
        assert_eq!(pick(&s, &hidden).len(), 2, "still on the page");
    }

    #[test]
    fn hidden_keys_round_trip_and_stay_few() {
        let folder = std::env::temp_dir().join(format!("hex-suggest-{}", std::process::id()));
        let path = folder.join("assistant").join("suggestions.json");
        assert_eq!(
            Hidden::read(&path),
            Hidden::default(),
            "no file hides nothing"
        );
        let mut hidden = Hidden::default();
        for i in 0..Hidden::MOST + 5 {
            hidden.hide(format!("notes:{i}"));
        }
        hidden.see("cleanup:1".into());
        hidden.write(&path).unwrap();
        let read = Hidden::read(&path);
        assert_eq!(read, hidden);
        assert!(!read.has("notes:0") && read.has(&format!("notes:{}", Hidden::MOST + 4)));
        assert!(read.saw("cleanup:1"));
        std::fs::write(&path, "not json").unwrap();
        assert_eq!(Hidden::read(&path), Hidden::default());
        let _ = std::fs::remove_dir_all(&folder);
    }
}
