//! A transcript with segment and word timings, as every OS's models give it.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegment {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transcript {
    pub text: String,
    pub segments: Vec<TranscriptSegment>,
    /// Word timings, when the model produces them.
    #[serde(default)]
    pub words: Vec<TranscriptSegment>,
}

/// The timing level a caller asked a model for. It is always native: nothing
/// estimates one from text length, chunk boundaries, or the other level.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimestampLevel {
    /// No timings at all, as every request was before this option existed.
    #[default]
    None,
    Segment,
    Word,
}

impl TimestampLevel {
    /// The value a response names in `timestampGranularity`; `None` adds no
    /// timing fields.
    pub const fn granularity(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Segment => Some("segment"),
            Self::Word => Some("word"),
        }
    }

    /// Whether a model whose finest native timings are `capability` answers
    /// this request at the level asked for. Word timings are never derived
    /// from segments, and a word-timed model still answers a segment request.
    pub const fn supported_by(self, capability: Self) -> bool {
        matches!(
            (self, capability),
            (Self::None, _)
                | (Self::Segment, Self::Segment | Self::Word)
                | (Self::Word, Self::Word)
        )
    }
}

/// Words joined from a model's timed tokens, for a model that times its
/// tokens but not its words (GigaAM): a token starting with `▁` or a space
/// starts a word, any other (a word's next piece, punctuation) joins the
/// word before it. Each token is its start, end, and text.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub fn words_from_tokens<'a>(
    tokens: impl IntoIterator<Item = (i64, i64, &'a str)>,
) -> Vec<TranscriptSegment> {
    let mut words: Vec<TranscriptSegment> = Vec::new();
    // A marker alone (GigaAM's space) opens the word after it.
    let mut open = true;
    for (start_ms, end_ms, text) in tokens {
        let piece = text.trim_start_matches(['\u{2581}', ' ']);
        open |= piece.len() != text.len();
        if piece.trim().is_empty() {
            continue;
        }
        match words.last_mut() {
            Some(word) if !open => {
                word.text.push_str(piece);
                word.end_ms = word.end_ms.max(end_ms);
            }
            _ => words.push(TranscriptSegment {
                start_ms,
                end_ms,
                text: piece.to_owned(),
            }),
        }
        open = false;
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_is_answered_only_at_its_own_level_or_coarser() {
        for capability in [
            TimestampLevel::None,
            TimestampLevel::Segment,
            TimestampLevel::Word,
        ] {
            assert!(
                TimestampLevel::None.supported_by(capability),
                "no timings asks nothing of the model"
            );
        }
        assert!(TimestampLevel::Segment.supported_by(TimestampLevel::Segment));
        assert!(TimestampLevel::Segment.supported_by(TimestampLevel::Word));
        assert!(!TimestampLevel::Segment.supported_by(TimestampLevel::None));
        assert!(TimestampLevel::Word.supported_by(TimestampLevel::Word));
        assert!(
            !TimestampLevel::Word.supported_by(TimestampLevel::Segment),
            "word timings are never cut from segments"
        );
        assert!(!TimestampLevel::Word.supported_by(TimestampLevel::None));
    }

    #[test]
    fn only_a_timed_request_names_a_granularity() {
        assert_eq!(TimestampLevel::None.granularity(), None);
        assert_eq!(TimestampLevel::Segment.granularity(), Some("segment"));
        assert_eq!(TimestampLevel::Word.granularity(), Some("word"));
    }

    #[test]
    fn tokens_join_into_words_at_their_markers() {
        let words = super::words_from_tokens([
            (80, 120, "\u{2581}Здравствуйте"),
            (720, 760, "."),
            (1_040, 1_080, "\u{2581}М"),
            (1_080, 1_120, "е"),
            (1_160, 1_200, "ня"),
            (1_240, 1_260, "\u{2581}"),
            (1_280, 1_300, "зовут"),
        ]);
        let read: Vec<(i64, i64, &str)> = words
            .iter()
            .map(|word| (word.start_ms, word.end_ms, word.text.as_str()))
            .collect();
        assert_eq!(
            read,
            [
                (80, 760, "Здравствуйте."),
                (1_040, 1_200, "Меня"),
                (1_280, 1_300, "зовут")
            ]
        );
    }
}
