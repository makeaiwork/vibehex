//! Word times in a meeting's transcript: an entry keeps when each of its
//! words starts once the model timed them, so Listen to the recording can
//! light the word playing (Labs › Listen to the recording).

use super::{MeetingSource, TRANSCRIPT_TURN_GAP_MS, TranscriptEntry};

/// `words` when it times each of `text`'s words, else none.
pub fn word_starts<'a>(text: &str, words: &'a [u64]) -> Option<&'a [u64]> {
    (!words.is_empty() && words.len() == text.split_whitespace().count()).then_some(words)
}

/// One sentence of timed words as an entry: its words joined by spaces,
/// each word's start kept (a word the model spaced inside, once per piece).
pub(super) fn sentence_entry(
    source: MeetingSource,
    group: &[crate::transcription::transcript::TranscriptSegment],
) -> TranscriptEntry {
    let start =
        |word: &crate::transcription::transcript::TranscriptSegment| word.start_ms.max(0) as u64;
    TranscriptEntry {
        source,
        start_ms: group.first().map_or(0, start),
        end_ms: group
            .iter()
            .map(|word| word.end_ms)
            .max()
            .unwrap_or_default() as u64,
        text: group
            .iter()
            .map(|word| word.text.as_str())
            .collect::<Vec<_>>()
            .join(" "),
        words: group
            .iter()
            .flat_map(|word| std::iter::repeat_n(start(word), word.text.split_whitespace().count()))
            .collect(),
    }
}

/// A transcript's entries joined into turns: one speaker's entries closer
/// than [`TRANSCRIPT_TURN_GAP_MS`] make one turn, keeping their word times
/// while every part has them.
pub fn coalesce_transcript(
    entries: impl IntoIterator<Item = TranscriptEntry>,
) -> Vec<TranscriptEntry> {
    coalesce(entries).0
}

/// Where each entry's words (trimmed) land in the transcript as the window
/// shows it: [`coalesce_transcript`]'s turns joined by a line break.
pub fn coalesced_ranges(entries: &[TranscriptEntry]) -> Vec<std::ops::Range<usize>> {
    coalesce(entries.iter().cloned()).1
}

fn coalesce(
    entries: impl IntoIterator<Item = TranscriptEntry>,
) -> (Vec<TranscriptEntry>, Vec<std::ops::Range<usize>>) {
    let mut turns: Vec<TranscriptEntry> = Vec::new();
    let mut ranges = Vec::new();
    // Where the last turn starts in the joined text.
    let mut base = 0;
    for entry in entries {
        let lead = entry.text.len() - entry.text.trim_start().len();
        let words = entry.text.trim().len();
        if let Some(turn) = turns.last_mut()
            && turn.source == entry.source
            && entry.start_ms.saturating_sub(turn.end_ms) <= TRANSCRIPT_TURN_GAP_MS
        {
            if !turn.text.ends_with(char::is_whitespace) {
                turn.text.push(' ');
            }
            let at = base + turn.text.len();
            ranges.push(at..at + words);
            let timed = word_starts(&turn.text, &turn.words).is_some()
                && word_starts(&entry.text, &entry.words).is_some();
            if timed {
                turn.words.extend(&entry.words);
            } else {
                turn.words.clear();
            }
            turn.text.push_str(entry.text.trim());
            turn.end_ms = turn.end_ms.max(entry.end_ms);
        } else {
            if let Some(last) = turns.last() {
                base += last.text.len() + 1;
            }
            ranges.push(base + lead..base + lead + words);
            turns.push(entry);
        }
    }
    (turns, ranges)
}
