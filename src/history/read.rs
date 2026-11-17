//! VibeHex: what a dictation read for itself, kept with its History entry.
//!
//! The owner's decision (2026-11-16): History keeps each dictation's text as
//! heard and as pasted, and now also what VibeHex read for it, so it can be
//! looked at and analysed later: the names Labs › Screen vocabulary read from
//! the window, the text Labs › Rewrite reads nearby text read around the
//! cursor (after the secrets pass, `caret_prompt::secrets`), and what of them
//! AI cleanup was given. It rides on the draft like the rest of the entry, so
//! it is written only when the entry is (History Off writes neither), counts
//! toward History's text cap, and leaves with the entry (Delete, retention,
//! the caps, Clear all). Nothing here logs it, and the live log, the local
//! API and Assistant never see it; `hex history --raw --json` prints it.

use serde::{Deserialize, Serialize};

use super::{MAX_LABEL_BYTES, MAX_TEXT_BYTES, truncated};
use crate::dictation::caret_text::CaretText;
use crate::dictation::context::ContextSnapshot;
use crate::dictation::screen_vocabulary::MAX_TERMS;

/// What of the text near the cursor AI cleanup was given.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NearbySent {
    /// None of it: no cleanup ran, or the lab was off by then.
    #[default]
    Nothing,
    /// All of it (a long dictation's pieces share it out: the first gets the
    /// text before the cursor and the selection, the last the text after it).
    All,
    /// A built-in model's part: the end of the text before the cursor and the
    /// selection, none of the text after it (`caret_prompt::cleanup::part`).
    Part,
}

impl NearbySent {
    fn is_nothing(&self) -> bool {
        *self == Self::Nothing
    }
}

/// Which of the two labs' reads a capture started, as its context carries
/// it (`recognition::context_watch`), so a read that found nothing still
/// says so. Off for a lab that was off.
#[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReadTried {
    /// Screen vocabulary read the window.
    pub screen: bool,
    /// Rewrite reads nearby text read around the cursor.
    pub nearby: bool,
}

/// What one dictation's AI cleanup was given of what was read, as the
/// processor knows it (`Processed::read`). Empty when no cleanup ran.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Given {
    /// The screen terms cleanup's prompt listed (`ForCleanup`).
    pub cleanup_terms: Vec<String>,
    pub nearby: NearbySent,
    /// Cleanup repeated the nearby text, so it was asked again without it
    /// (`Nearby::asked`).
    pub asked_again: bool,
}

/// The text around the cursor as the dictation's context held it. Its debug
/// output is lengths only, as [`CaretText`]'s is.
#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadNearby {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub before: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub selected: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub after: String,
}

impl std::fmt::Debug for ReadNearby {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReadNearby")
            .field("before_chars", &self.before.chars().count())
            .field("selected_chars", &self.selected.chars().count())
            .field("after_chars", &self.after.chars().count())
            .finish()
    }
}

impl ReadNearby {
    /// Whether no part holds more than whitespace.
    pub fn is_empty(&self) -> bool {
        self.parts().next().is_none()
    }

    /// The parts that hold more than whitespace, labelled, in reading order.
    pub fn parts(&self) -> impl Iterator<Item = (Part, &str)> {
        [
            (Part::Before, self.before.as_str()),
            (Part::Selected, self.selected.as_str()),
            (Part::After, self.after.as_str()),
        ]
        .into_iter()
        .filter(|(_, text)| !text.trim().is_empty())
    }
}

/// One part of the text near the cursor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Part {
    Before,
    Selected,
    After,
}

impl From<&CaretText> for ReadNearby {
    fn from(caret: &CaretText) -> Self {
        Self {
            before: caret.before.clone(),
            selected: caret.selected.clone(),
            after: caret.after.clone(),
        }
    }
}

/// What VibeHex read for one dictation, and what of it AI cleanup was given.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistoryRead {
    /// The names Screen vocabulary read from the window, in order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub screen_terms: Vec<String>,
    /// The ones of them AI cleanup was given.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cleanup_terms: Vec<String>,
    /// The text around the cursor, secrets already cut out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nearby: Option<ReadNearby>,
    #[serde(default, skip_serializing_if = "NearbySent::is_nothing")]
    pub nearby_sent: NearbySent,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub asked_again: bool,
    /// Screen vocabulary read the window, whether or not it found a name.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub screen_tried: bool,
    /// Rewrite reads nearby text read around the cursor, whether or not
    /// the app gave any text.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub nearby_tried: bool,
}

impl HistoryRead {
    /// A dictation's record from its context and what its cleanup was given;
    /// `None` when neither lab read, so the entry is what it was before.
    #[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]
    pub fn of(context: &ContextSnapshot, given: &Given) -> Option<Self> {
        Self {
            screen_terms: context.screen_terms.clone(),
            cleanup_terms: given.cleanup_terms.clone(),
            nearby: context.caret_text.as_ref().map(ReadNearby::from),
            nearby_sent: given.nearby,
            asked_again: given.asked_again,
            screen_tried: context.read_tried.screen,
            nearby_tried: context.read_tried.nearby,
        }
        .kept()
    }

    /// Bounded as History bounds its other fields, and `None` when neither
    /// lab read. A part of the nearby text that is only whitespace is none.
    /// What cleanup was given of the nearby text goes with the text.
    pub(super) fn kept(self) -> Option<Self> {
        let terms = |terms: Vec<String>| -> Vec<String> {
            terms
                .iter()
                .filter(|term| !term.is_empty())
                .take(MAX_TERMS)
                .map(|term| truncated(term, MAX_LABEL_BYTES))
                .collect()
        };
        let text = |text: &str| match text.trim().is_empty() {
            true => String::new(),
            false => truncated(text, MAX_TEXT_BYTES),
        };
        let nearby = self
            .nearby
            .map(|nearby| ReadNearby {
                before: text(&nearby.before),
                selected: text(&nearby.selected),
                after: text(&nearby.after),
            })
            .filter(|nearby| !nearby.is_empty());
        let read = Self {
            screen_terms: terms(self.screen_terms),
            cleanup_terms: terms(self.cleanup_terms),
            nearby_sent: nearby
                .as_ref()
                .map_or(NearbySent::Nothing, |_| self.nearby_sent),
            asked_again: nearby.is_some() && self.asked_again,
            nearby,
            screen_tried: self.screen_tried,
            nearby_tried: self.nearby_tried,
        };
        (!read.screen_terms.is_empty()
            || read.nearby.is_some()
            || read.screen_tried
            || read.nearby_tried)
            .then_some(read)
    }

    /// The bytes of text it holds, for History's text cap.
    pub(super) fn text_bytes(&self) -> usize {
        let terms: usize = self
            .screen_terms
            .iter()
            .chain(&self.cleanup_terms)
            .map(String::len)
            .sum();
        terms
            + self.nearby.as_ref().map_or(0, |nearby| {
                nearby.before.len() + nearby.selected.len() + nearby.after.len()
            })
    }

    /// Whether Screen vocabulary read the window: it found names, or says it
    /// found none.
    pub fn read_names(&self) -> bool {
        self.screen_tried || !self.screen_terms.is_empty()
    }

    /// Whether Rewrite reads nearby text read around the cursor (an older
    /// entry holding only whitespace there read too), and the text it found.
    pub fn read_nearby(&self) -> Option<Option<&ReadNearby>> {
        let found = self.nearby.as_ref().filter(|nearby| !nearby.is_empty());
        (self.nearby_tried || self.nearby.is_some()).then_some(found)
    }

    /// Whether AI cleanup was given `term`.
    pub fn given(&self, term: &str) -> bool {
        self.cleanup_terms.iter().any(|given| given == term)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::history::{HistoryDraft, HistoryEntry, HistoryKind, HistoryRetention, HistoryStore};

    fn caret(before: &str, selected: &str, after: &str) -> CaretText {
        CaretText {
            before: before.into(),
            selected: selected.into(),
            after: after.into(),
        }
    }

    fn context(terms: &[&str], caret: Option<CaretText>) -> ContextSnapshot {
        ContextSnapshot {
            application: Some("Zed".into()),
            screen_terms: terms.iter().map(|&term| term.into()).collect(),
            caret_text: caret,
            ..Default::default()
        }
    }

    fn sample() -> HistoryRead {
        HistoryRead::of(
            &context(
                &["Qdrant", "OpenCode", "Rune"],
                Some(caret("We ship on ", "Friday", " morning.")),
            ),
            &Given {
                cleanup_terms: vec!["Qdrant".into()],
                nearby: NearbySent::Part,
                asked_again: true,
            },
        )
        .unwrap()
    }

    fn temp_path(name: &str) -> std::path::PathBuf {
        let directory =
            std::env::temp_dir().join(format!("hex-history-read-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).unwrap();
        directory.join("history.json")
    }

    fn draft(read: Option<HistoryRead>) -> HistoryDraft {
        HistoryDraft {
            kind: HistoryKind::Dictation,
            raw_text: "so the qdrant index".into(),
            final_text: "So the Qdrant index.".into(),
            application: Some("Zed".into()),
            processing: None,
            audio_ms: 900,
            inference_ms: 80,
            total_ms: 1_100,
            recording: None,
            model: None,
            read,
        }
    }

    #[test]
    fn a_dictation_that_read_nothing_keeps_nothing() {
        assert_eq!(
            HistoryRead::of(&context(&[], None), &Given::default()),
            None
        );
        // What cleanup was given means nothing without what was read.
        let given = Given {
            cleanup_terms: Vec::new(),
            nearby: NearbySent::All,
            asked_again: true,
        };
        assert_eq!(HistoryRead::of(&context(&[], None), &given), None);
        let empty = Some(caret("", "", ""));
        assert_eq!(HistoryRead::of(&context(&[""], empty), &given), None);
    }

    #[test]
    fn the_record_holds_the_context_and_what_cleanup_was_given() {
        let read = sample();
        assert_eq!(read.screen_terms, ["Qdrant", "OpenCode", "Rune"]);
        assert_eq!(read.cleanup_terms, ["Qdrant"]);
        assert!(read.given("Qdrant") && !read.given("Rune"));
        let nearby = read.nearby.as_ref().unwrap();
        assert_eq!(
            (&*nearby.before, &*nearby.selected, &*nearby.after),
            ("We ship on ", "Friday", " morning.")
        );
        assert_eq!(read.nearby_sent, NearbySent::Part);
        assert!(read.asked_again);
        // Names alone: what cleanup got of a text there was none of is nothing.
        let names = HistoryRead::of(
            &context(&["Qdrant"], None),
            &Given {
                cleanup_terms: Vec::new(),
                nearby: NearbySent::All,
                asked_again: true,
            },
        )
        .unwrap();
        assert_eq!(names.nearby, None);
        assert_eq!(names.nearby_sent, NearbySent::Nothing);
        assert!(!names.asked_again);
    }

    #[test]
    fn the_record_is_bounded_like_the_entry() {
        let many: Vec<String> = (0..40).map(|index| format!("Term{index}")).collect();
        let long = "x".repeat(MAX_TEXT_BYTES + 10);
        let read = HistoryRead {
            screen_terms: many.clone(),
            cleanup_terms: vec!["y".repeat(MAX_LABEL_BYTES + 5)],
            nearby: Some(ReadNearby {
                before: long.clone(),
                selected: String::new(),
                after: long,
            }),
            nearby_sent: NearbySent::All,
            asked_again: false,
            ..Default::default()
        }
        .kept()
        .unwrap();
        assert_eq!(read.screen_terms.len(), MAX_TERMS);
        assert_eq!(read.cleanup_terms[0].len(), MAX_LABEL_BYTES);
        let nearby = read.nearby.as_ref().unwrap();
        assert_eq!(nearby.before.len(), MAX_TEXT_BYTES);
        assert_eq!(nearby.after.len(), MAX_TEXT_BYTES);
        assert_eq!(
            read.text_bytes(),
            many[..MAX_TERMS].iter().map(String::len).sum::<usize>()
                + MAX_LABEL_BYTES
                + 2 * MAX_TEXT_BYTES
        );
    }

    #[test]
    fn an_entry_with_the_record_reads_back_and_one_without_is_as_before() {
        let path = temp_path("round-trip");
        let mut store = HistoryStore::open(path.clone(), HistoryRetention::Week, 0);
        let plain = store.record(draft(None), 1_000).unwrap().unwrap();
        let read = store.record(draft(Some(sample())), 2_000).unwrap().unwrap();
        let reopened = HistoryStore::open(path.clone(), HistoryRetention::Week, 3_000);
        assert_eq!(reopened.entry(plain).unwrap().read, None);
        assert_eq!(reopened.entry(read).unwrap().read, Some(sample()));

        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(
            saved["entries"][1]["read"],
            serde_json::json!({
                "screen_terms": ["Qdrant", "OpenCode", "Rune"],
                "cleanup_terms": ["Qdrant"],
                "nearby": {"before": "We ship on ", "selected": "Friday", "after": " morning."},
                "nearby_sent": "part",
                "asked_again": true,
            })
        );
        // An entry without it is written byte for byte as it was before the
        // record existed, and the old format reads back without it.
        let entry = reopened.entry(plain).unwrap().clone();
        let today = serde_json::to_string(&entry).unwrap();
        let before = format!(
            "{{\"id\":{plain},\"timestamp_ms\":1000,\"kind\":\"dictation\",\
             \"raw_text\":\"so the qdrant index\",\"final_text\":\"So the Qdrant index.\",\
             \"application\":\"Zed\",\"audio_ms\":900,\"inference_ms\":80,\"total_ms\":1100}}"
        );
        assert_eq!(today, before);
        let old: HistoryEntry = serde_json::from_str(&before).unwrap();
        assert_eq!(old, entry);
    }

    #[test]
    fn the_record_leaves_with_its_entry_and_is_never_written_with_history_off() {
        let path = temp_path("retention");
        let day_ms = HistoryRetention::Day.max_age_ms().unwrap();
        let mut store = HistoryStore::open(path.clone(), HistoryRetention::Day, 0);
        let old = store.record(draft(Some(sample())), 1_000).unwrap().unwrap();
        let deleted = store
            .record(draft(Some(sample())), day_ms)
            .unwrap()
            .unwrap();
        let kept = store
            .record(draft(Some(sample())), day_ms)
            .unwrap()
            .unwrap();
        let held = || {
            std::fs::read_to_string(&path)
                .unwrap()
                .matches("Qdrant")
                .count()
        };
        assert_eq!(
            held(),
            3 * 3,
            "each entry names Qdrant in its text and twice in its record"
        );

        store.expire(day_ms + 1_001);
        assert!(store.entry(old).is_none());
        store.delete(deleted).unwrap();
        assert_eq!(held(), 3, "retention and Delete take theirs with them");
        assert!(store.entry(kept).unwrap().read.is_some());
        store.clear().unwrap();
        assert_eq!(held(), 0);

        store
            .set_retention(HistoryRetention::Off, day_ms + 2_000)
            .unwrap();
        assert_eq!(
            store.record(draft(Some(sample())), day_ms + 2_000).unwrap(),
            None
        );
        assert_eq!(held(), 0, "History off writes no entry, and so no record");
    }

    fn tried(
        terms: &[&str],
        caret: Option<CaretText>,
        screen: bool,
        nearby: bool,
    ) -> ContextSnapshot {
        ContextSnapshot {
            read_tried: ReadTried { screen, nearby },
            ..context(terms, caret)
        }
    }

    #[test]
    fn a_lab_that_read_and_found_nothing_still_says_so() {
        let names = HistoryRead::of(&tried(&[], None, true, false), &Given::default()).unwrap();
        assert!(names.read_names() && names.screen_terms.is_empty());
        assert_eq!(names.read_nearby(), None, "the other lab was off");
        let nearby = HistoryRead::of(&tried(&[], None, false, true), &Given::default()).unwrap();
        assert!(!nearby.read_names());
        assert_eq!(nearby.read_nearby(), Some(None));
        assert_eq!(
            serde_json::to_value(&nearby).unwrap(),
            serde_json::json!({"nearby_tried": true})
        );
        // Both labs off: no record, as before.
        assert_eq!(
            HistoryRead::of(&tried(&[], None, false, false), &Given::default()),
            None
        );
    }

    #[test]
    fn whitespace_near_the_cursor_is_no_text() {
        let blank = Some(caret(" ", "\t", "\n\n\n"));
        assert_eq!(
            HistoryRead::of(&context(&[], blank.clone()), &Given::default()),
            None
        );
        let read = HistoryRead::of(&tried(&[], blank, false, true), &Given::default()).unwrap();
        assert_eq!(read.nearby, None);
        assert_eq!(read.read_nearby(), Some(None));
        let read = HistoryRead::of(
            &tried(&[], Some(caret("We ship on ", "", "\n\n\n")), false, true),
            &Given::default(),
        )
        .unwrap();
        let nearby = read.nearby.as_ref().unwrap();
        assert_eq!((&*nearby.before, &*nearby.after), ("We ship on ", ""));
        // An older entry that kept only whitespace read too, and found none.
        let old: HistoryRead = serde_json::from_str(r#"{"nearby": {"after": "\n\n\n"}}"#).unwrap();
        assert!(!old.nearby_tried);
        assert_eq!(old.read_nearby(), Some(None));
        assert_eq!(old.nearby.as_ref().unwrap().parts().count(), 0);
    }

    #[test]
    fn an_older_record_reads_as_not_tried() {
        let old: HistoryRead = serde_json::from_str(r#"{"screen_terms": ["Qdrant"]}"#).unwrap();
        assert!(!old.screen_tried && !old.nearby_tried);
        assert!(old.read_names());
        assert_eq!(old.read_nearby(), None);
    }

    #[test]
    fn the_record_counts_toward_the_text_cap() {
        let path = temp_path("cap");
        let mut store = HistoryStore::open(path, HistoryRetention::Forever, 0);
        let id = store.record(draft(Some(sample())), 1_000).unwrap().unwrap();
        let entry = store.entry(id).unwrap();
        assert_eq!(
            entry.text_bytes(),
            entry.raw_text.len() + entry.final_text.len() + sample().text_bytes()
        );
    }
}
