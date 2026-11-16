//! VibeHex: an answer that repeats the text around the cursor. A small model
//! given that text beside a short transcript may return the text itself (the
//! sentence after the cursor, in place of the two words that were said)
//! whatever its instructions say. [`echoes`] tells in code, with no model,
//! and the cleanup is then asked again without the nearby text
//! (`cleanup::Nearby::asked`).
//!
//! An answer echoes when it holds a run of [`RUN`] words that stand together
//! near the cursor and were not said that way, and it is also longer than
//! what was said by [`GREW`] words or more; or when such a run is [`LONG`]
//! words, whatever the lengths. A name or a phrase the answer takes from the
//! nearby text to spell what was said (the lab's whole point) is shorter
//! than that, and words the person dictated again are in the transcript too.

use crate::dictation::caret_text::CaretText;

/// A run this long, taken from the nearby text, in an answer that grew.
const RUN: usize = 4;
/// An answer this many words longer than its transcript grew.
const GREW: usize = 3;
/// A run this long from the nearby text is an echo whatever the lengths.
const LONG: usize = 8;

/// The words of `text`, lowercase, without punctuation.
fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn holds(text: &[String], run: &[String]) -> bool {
    text.windows(run.len()).any(|window| window == run)
}

/// Whether `answer`, cleanup's result for the transcript `heard`, repeats
/// `caret`, the text around the cursor it was given.
pub(crate) fn echoes(caret: &CaretText, heard: &str, answer: &str) -> bool {
    let answer = words(answer);
    let heard = words(heard);
    let near = [&caret.before, &caret.selected, &caret.after].map(|part| words(part));
    let taken = |length: usize| {
        answer.len() >= length
            && answer
                .windows(length)
                .any(|run| near.iter().any(|part| holds(part, run)) && !holds(&heard, run))
    };
    (answer.len() >= heard.len() + GREW && taken(RUN)) || taken(LONG)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caret(before: &str, after: &str) -> CaretText {
        CaretText {
            before: before.into(),
            selected: String::new(),
            after: after.into(),
        }
    }

    /// The owner's dictation of 2026-11-16: two words said in the middle of
    /// a paragraph, and the built-in model returned the paragraph's rest.
    #[test]
    fn the_text_after_the_cursor_returned_for_two_words_is_an_echo() {
        let caret = caret(
            "А как проверить вот версию, которую я сейчас использую? Она использует ",
            "? И второе, вот этот вот Cursor контекст используется или нет? Вот я его сейчас включил.",
        );
        let answer = "? И второе, вот этот вот Cursor контекст используется или нет? Вот я его сейчас включил.";
        assert!(echoes(&caret, "Screen vocabulary", answer));
        assert!(!echoes(&caret, "Screen vocabulary", "Screen vocabulary"));
        assert!(!echoes(&caret, "скрин вокабуляри", "Screen Vocabulary"));
    }

    /// A spelling taken from the nearby text, and words the person said
    /// again, are no echo; a long stretch of the nearby text is one even in
    /// an answer no longer than what was said.
    #[test]
    fn a_spelling_from_nearby_is_kept_and_a_long_copy_is_not() {
        let caret = caret(
            "We moved the index to Qdrant last week and it has been stable since then. ",
            " The next step is the migration of the old shards.",
        );
        // The name comes from the nearby text; the sentence was said.
        assert!(!echoes(
            &caret,
            "and kudrant is faster than the old one",
            "And Qdrant is faster than the old one."
        ));
        // Said again, word for word: it is in the transcript too.
        assert!(!echoes(
            &caret,
            "the next step is the migration of the old shards",
            "The next step is the migration of the old shards."
        ));
        // Eight words in a row from before the cursor, never said.
        assert!(echoes(
            &caret,
            "it is faster than the old one by far now",
            "We moved the index to Qdrant last week and it is faster."
        ));
        // Too short to judge.
        assert!(!echoes(&caret, "ok", "The next step"));
    }
}
