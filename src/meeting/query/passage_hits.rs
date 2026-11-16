//! Assistant's search over meetings: the passages that match a question,
//! with the transcript entries each one joins.

use std::sync::Arc;

use super::{Meeting, ParsedQuery, PassageKind, VocabularyEntry, rank};

/// One passage [`search_passages`] found: a turn's text or a meeting's notes.
pub struct PassageHit {
    pub meeting: Arc<Meeting>,
    /// Into the meeting; 0 for its notes.
    pub start_ms: u64,
    pub notes: bool,
    /// `Me`, `Them`, or `Notes`.
    pub speaker: &'static str,
    /// The meeting's transcript entries it joins; none for its notes.
    pub entries: std::ops::Range<usize>,
}

/// The passages of `meetings` matching `query` or `vocabulary`'s spellings, best first (for Assistant).
pub fn search_passages(
    meetings: &[Arc<Meeting>],
    query: &str,
    vocabulary: &[VocabularyEntry],
) -> Vec<PassageHit> {
    let parsed = ParsedQuery::parse(query);
    if parsed.is_empty() {
        return Vec::new();
    }
    rank(meetings, &parsed, true, vocabulary)
        .into_iter()
        .map(|hit| {
            let passage = &hit.meeting.passages[hit.passage];
            PassageHit {
                start_ms: passage.start_ms,
                notes: passage.kind == PassageKind::Notes,
                speaker: passage.speaker(),
                entries: passage.entries.clone(),
                meeting: hit.meeting.clone(),
            }
        })
        .collect()
}
