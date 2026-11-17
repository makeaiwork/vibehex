//! VibeHex: Labs › Screen vocabulary takes names from what the person is
//! working on, not from the app's chrome.
//!
//! The window reader (`accessibility::focused_window_text`) collects each
//! element's text value and, beside it, the `AXTitle` and `AXDescription` of
//! its buttons, tabs and panels. In an editor such as VS Code those labels
//! ("Explorer", "Source Control", "Toggle Primary Side Bar") filled all 24
//! places with ordinary words. [`WindowText`] keeps where each label sits, and
//! [`rank`] then:
//!
//! - keeps every term of the content (text values) as upstream's
//!   [`extract`] ranks it, first;
//! - after them, keeps a term found only in labels only when it is not an
//!   ordinary word: a mixed-case name (`AskUserQuestion`, `GitHub`), letters
//!   with digits, or a word the dictionary lacks (`Qdrant`); a plain
//!   capitalized or all-caps dictionary word (`Panel`, `Changes`, `NOT`)
//!   goes;
//! - anywhere, drops the few function words upstream's rule lets through as
//!   an acronym or a mid-sentence capital (`The`, `AND`), and the words of
//!   the dictation's own app name (`Code`, `Sublime Text`).
//!
//! The dictionary is macOS's own `/usr/share/dict/words` (its lowercase
//! entries only, so a name it lists capitalized is not ordinary) and the
//! function words `/usr/share/dict/connectives`, each read once, lazily, in
//! the reader thread. A missing or unreadable file filters nothing. No word
//! list is bundled.

use std::collections::HashSet;
use std::ops::Range;
#[cfg(target_os = "macos")]
use std::sync::OnceLock;

use super::{MAX_TERMS, extract, uncapped};

/// The dictionary: its entries that start lowercase are ordinary words.
#[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]
const DICTIONARY: &str = "/usr/share/dict/words";
/// The function words dropped anywhere.
#[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]
const CONNECTIVES: &str = "/usr/share/dict/connectives";
/// A word list larger than this is not read: macOS's is 2.4 MB.
#[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]
const MAX_LIST_BYTES: u64 = 8 * 1024 * 1024;
/// The shortest stem an inflection is checked for: `bus` is not `bu`.
const MIN_STEM: usize = 3;

/// A focused window's text as the reader collected it, with where each
/// title and description sits in it. Its debug output is lengths only.
#[derive(Clone, Default, Eq, PartialEq)]
pub(crate) struct WindowText {
    pub text: String,
    /// Byte ranges of `text`, in order and apart, that are labels.
    pub labels: Vec<Range<usize>>,
}

impl std::fmt::Debug for WindowText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WindowText")
            .field("bytes", &self.text.len())
            .field("labels", &self.labels.len())
            .finish()
    }
}

impl WindowText {
    /// The content (every text value, in order) and the labels, each piece
    /// still on its own line as the reader wrote it.
    pub(crate) fn split(&self) -> (String, String) {
        let mut content = String::new();
        let mut labels = String::new();
        let mut at = 0;
        for range in &self.labels {
            let (start, end) = (range.start.max(at), range.end.min(self.text.len()));
            if start >= end
                || !self.text.is_char_boundary(start)
                || !self.text.is_char_boundary(end)
            {
                continue;
            }
            content.push_str(&self.text[at..start]);
            labels.push_str(&self.text[start..end]);
            at = end;
        }
        content.push_str(&self.text[at..]);
        (content, labels)
    }
}

/// A sorted list of words, held as one buffer so macOS's 210,000 entries
/// cost about 3 MB.
pub(crate) struct WordList {
    words: String,
    starts: Vec<u32>,
}

impl WordList {
    /// The lines of `text` that `keep` accepts, trimmed.
    pub(crate) fn new(text: &str, keep: impl Fn(&str) -> bool) -> Self {
        let mut list: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|word| !word.is_empty() && keep(word))
            .collect();
        list.sort_unstable();
        list.dedup();
        let mut words = String::with_capacity(list.iter().map(|word| word.len() + 1).sum());
        let mut starts = Vec::with_capacity(list.len());
        for word in list {
            starts.push(words.len() as u32);
            words.push_str(word);
            words.push('\n');
        }
        Self { words, starts }
    }

    /// The dictionary's ordinary words: its entries that start lowercase.
    pub(crate) fn dictionary(text: &str) -> Self {
        Self::new(text, |word| {
            word.chars().next().is_some_and(char::is_lowercase)
        })
    }

    fn word(&self, start: u32) -> &str {
        let rest = &self.words[start as usize..];
        &rest[..rest.find('\n').unwrap_or(rest.len())]
    }

    pub(crate) fn contains(&self, word: &str) -> bool {
        self.starts
            .binary_search_by(|&start| self.word(start).cmp(word))
            .is_ok()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.starts.len()
    }

    fn iter(&self) -> impl Iterator<Item = &str> {
        self.starts.iter().map(|&start| self.word(start))
    }
}

/// The word lists [`rank`] filters with. Either may be missing, and then it
/// filters nothing.
#[derive(Default)]
pub(crate) struct Words {
    pub dictionary: Option<WordList>,
    pub connectives: Option<WordList>,
}

impl Words {
    /// Reads both lists from their files; a missing, unreadable or oversized
    /// one is `None`.
    #[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]
    pub(crate) fn load(dictionary: &str, connectives: &str) -> Self {
        let read = |path: &str| -> Option<String> {
            let size = std::fs::metadata(path).ok()?.len();
            (size <= MAX_LIST_BYTES)
                .then(|| std::fs::read_to_string(path).ok())
                .flatten()
        };
        Self {
            dictionary: read(dictionary).map(|text| WordList::dictionary(&text)),
            connectives: read(connectives)
                .map(|text| WordList::new(&text.to_lowercase(), |_| true)),
        }
    }
}

/// macOS's word lists, read on first use, in the reader thread.
#[cfg(target_os = "macos")]
fn words() -> &'static Words {
    static WORDS: OnceLock<Words> = OnceLock::new();
    WORDS.get_or_init(|| Words::load(DICTIONARY, CONNECTIVES))
}

/// The terms of one read window, for the frontmost app: [`rank`] with
/// macOS's word lists and the app's name.
#[cfg(target_os = "macos")]
pub(crate) fn terms(window: &WindowText, known: &[String]) -> Vec<String> {
    let (content, labels) = window.split();
    let application = crate::dictation::context::frontmost_other_application();
    rank(&content, &labels, known, application.as_deref(), words())
}

/// The window's terms: the content's as [`extract`] ranks them, then the
/// labels' that are not ordinary words, never a function word, a word of
/// `application`'s name, or one in `known`, at most [`MAX_TERMS`].
pub(crate) fn rank(
    content: &str,
    labels: &str,
    known: &[String],
    application: Option<&str>,
    words: &Words,
) -> Vec<String> {
    let mut known = known.to_vec();
    if let Some(connectives) = &words.connectives {
        known.extend(connectives.iter().map(str::to_owned));
    }
    known.extend(
        application
            .unwrap_or_default()
            .split(|character: char| !character.is_alphanumeric())
            .filter(|word| !word.is_empty())
            .map(str::to_owned),
    );
    // The content's own list is whole unless it is already full.
    let mut terms = extract(content, &known);
    let seen: HashSet<String> = terms.iter().map(|term| term.to_lowercase()).collect();
    for term in uncapped(labels, &known) {
        if terms.len() >= MAX_TERMS {
            break;
        }
        if seen.contains(&term.to_lowercase()) {
            continue;
        }
        if words
            .dictionary
            .as_ref()
            .is_some_and(|dictionary| ordinary(&term, dictionary))
        {
            continue;
        }
        terms.push(term);
    }
    terms
}

/// Whether a label's term is just a word: no digit, no capital right after
/// a lowercase letter (`GitHub`, `iOS`, `AskUserQuestion`), and every part
/// between `.`, `-` and `_` an ordinary word or a simple inflection of one.
fn ordinary(term: &str, dictionary: &WordList) -> bool {
    if term.chars().any(char::is_numeric) {
        return false;
    }
    let characters: Vec<char> = term.chars().collect();
    if characters
        .windows(2)
        .any(|pair| pair[0].is_lowercase() && pair[1].is_uppercase())
    {
        return false;
    }
    term.to_lowercase()
        .split(['.', '-', '_'])
        .all(|part| !part.is_empty() && a_word(part, dictionary))
}

/// Whether `word` (lowercase) or a stem of a plural, `-ed` or `-ing` form of
/// it is in the dictionary: changes, branches, libraries, copied, stopped,
/// changing, running.
fn a_word(word: &str, dictionary: &WordList) -> bool {
    if dictionary.contains(word) {
        return true;
    }
    let mut stems: Vec<String> = Vec::new();
    for (suffix, endings) in [
        ("ies", &["y"][..]),
        ("ied", &["y"][..]),
        ("es", &[""][..]),
        ("s", &[""][..]),
        ("ed", &["", "e"][..]),
        ("ing", &["", "e"][..]),
    ] {
        let Some(stem) = word.strip_suffix(suffix) else {
            continue;
        };
        for ending in endings {
            stems.push(format!("{stem}{ending}"));
        }
        // stopped, running: one of a doubled last consonant.
        let mut characters = stem.chars().rev();
        if matches!(suffix, "ed" | "ing")
            && let (Some(last), Some(before)) = (characters.next(), characters.next())
            && last == before
        {
            stems.push(stem[..stem.len() - last.len_utf8()].to_owned());
        }
    }
    stems
        .iter()
        .any(|stem| stem.chars().count() >= MIN_STEM && dictionary.contains(stem))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small dictionary in macOS's shape: names capitalized, words not.
    const DICTIONARY_TEXT: &str = "a\nagent\nartifact\nbar\nbash\nbranch\nbuild\nchannel\ncode\n\
        change\ncheckout\nconsole\ncontrol\ncursor\nDenis\ndoc\nexplorer\ngit\nhere\nlibrary\nlive\n\
        not\nonly\npanel\nproblem\npull\nread\nrequest\nrun\nsecondary\nsection\nside\nsimulator\n\
        slack\nsplit\nstop\nsubagent\nswitcher\nsynchronize\ntag\nterminal\nthe\naction\ncopy\n";
    const CONNECTIVES_TEXT: &str = "the\nof\nand\nnot\nfor\n";

    fn words() -> Words {
        Words {
            dictionary: Some(WordList::dictionary(DICTIONARY_TEXT)),
            connectives: Some(WordList::new(CONNECTIVES_TEXT, |_| true)),
        }
    }

    /// The owner's VS Code window of 2026-11-17: labels, one per line as the
    /// reader writes them.
    fn vs_code_labels() -> String {
        [
            "Bash",
            "Code",
            "Read",
            "Git",
            "Terminal",
            "Agent",
            "Side",
            "Bar",
            "Actions",
            "Section",
            "Live",
            "Panel",
            "Secondary",
            "Switcher",
            "Explorer",
            "Console",
            "Split",
            "Checkout",
            "Branch",
            "Tag",
            "Synchronize",
            "Changes",
            "Problems",
            "The",
            "NOT",
            "Control",
            "Artifact",
            "Subagent",
        ]
        .iter()
        .map(|label| format!("Toggle {label} View\n"))
        .collect()
    }

    #[test]
    fn interface_labels_go_and_the_contents_names_stay_ahead() {
        let content = "we index the vectors in Qdrant before Orukeet reads them\n";
        let labels = format!("{}Run AskUserQuestion\n", vs_code_labels());
        let terms = rank(content, &labels, &[], Some("Code"), &words());
        assert_eq!(terms, ["Qdrant", "Orukeet", "AskUserQuestion"]);
    }

    #[test]
    fn without_the_filter_the_labels_fill_the_list() {
        let labels = vs_code_labels();
        let terms = rank("", &labels, &[], None, &Words::default());
        assert_eq!(
            terms.len(),
            MAX_TERMS,
            "the missing dictionary filters nothing"
        );
        assert!(terms.iter().any(|term| term == "Panel"));
    }

    #[test]
    fn a_label_only_name_that_is_no_plain_word_is_kept() {
        let labels = "Open GitHub Pull Requests\nShow iOS Simulator\nRun Build v2.4 П24-1\n\
            Ask Denis\nAsk Qdrant Docs\n";
        assert_eq!(
            rank("", labels, &[], None, &words()),
            ["GitHub", "iOS", "П24-1", "Denis", "Qdrant"]
        );
    }

    #[test]
    fn a_dictionary_word_counts_in_the_content_but_not_in_a_label() {
        let labels = "Open Slack Channel\nMove Cursor Here\n";
        assert!(rank("", labels, &[], None, &words()).is_empty());
        let content = "I asked in Slack and then in Cursor about it\n";
        assert_eq!(
            rank(content, labels, &[], None, &words()),
            ["Slack", "Cursor"]
        );
    }

    #[test]
    fn function_words_and_the_apps_name_are_never_terms() {
        let content = "I said THE plan FOR now uses Rune in Sublime Text with Zed\n";
        let terms = rank(
            content,
            "Show The Panel\n",
            &[],
            Some("Sublime Text"),
            &words(),
        );
        assert_eq!(terms, ["Rune", "Zed"]);
        // Without the function words' list, upstream's rule lets them in.
        let bare = Words {
            connectives: None,
            ..words()
        };
        assert_eq!(
            rank(content, "", &[], Some("Sublime Text"), &bare),
            ["THE", "FOR", "Rune", "Zed"]
        );
    }

    #[test]
    fn known_terms_stay_out_of_labels_too() {
        let known = vec!["askuserquestion".into()];
        assert!(rank("", "Run AskUserQuestion\n", &known, None, &words()).is_empty());
    }

    #[test]
    fn inflections_of_ordinary_words_are_ordinary() {
        let dictionary = WordList::dictionary(DICTIONARY_TEXT);
        for word in [
            "Changes",
            "Branches",
            "Libraries",
            "Copied",
            "Stopped",
            "Changing",
            "Running",
            "Actions",
            "Problems",
            "PANEL",
            "Side-Bar",
            "Read_Only",
        ] {
            assert!(ordinary(word, &dictionary), "{word}");
        }
        for word in [
            "Qdrant",
            "Denis",
            "GitHub",
            "AskUserQuestion",
            "Bu",
            "Node.js",
        ] {
            assert!(!ordinary(word, &dictionary), "{word}");
        }
    }

    #[test]
    fn the_window_splits_into_content_and_labels() {
        let text = "hello there\nExplorer\nmore text\nSide Bar\n";
        let window = WindowText {
            text: text.into(),
            labels: vec![12..21, 31..40],
        };
        let (content, labels) = window.split();
        assert_eq!(content, "hello there\nmore text\n");
        assert_eq!(labels, "Explorer\nSide Bar\n");
        // A range out of place is ignored rather than trusted.
        let odd = WindowText {
            text: "abc\n".into(),
            labels: vec![2..99, 0..1],
        };
        assert_eq!(odd.split(), ("ab".into(), "c\n".into()));
        assert_eq!(format!("{window:?}"), "WindowText { bytes: 40, labels: 2 }");
    }

    #[test]
    fn the_word_list_is_sorted_and_answers_exactly() {
        let list = WordList::dictionary("zebra\nApple\napple\n\nmango\nzebra\n");
        assert_eq!(list.len(), 3);
        assert!(list.contains("apple") && list.contains("zebra") && list.contains("mango"));
        assert!(!list.contains("Apple") && !list.contains("app") && !list.contains(""));
    }

    #[test]
    fn a_missing_list_filters_nothing() {
        let words = Words::load("/nonexistent/hex-words", "/nonexistent/hex-connectives");
        assert!(words.dictionary.is_none() && words.connectives.is_none());
        assert_eq!(
            rank("", "Show The Panel\n", &[], None, &words),
            ["The", "Panel"]
        );
    }

    /// The real reader against the app in front, printing counts only,
    /// never the text: `cargo test real_reader -- --ignored --nocapture`.
    #[cfg(target_os = "macos")]
    #[test]
    #[ignore = "reads the frontmost app's window, which needs Accessibility"]
    fn the_real_reader_tells_labels_from_content() {
        let pid = crate::dictation::context::frontmost_pid().expect("an app is in front");
        let Some(window) = crate::accessibility::focused_window_text(pid) else {
            eprintln!("the window gave nothing to read");
            return;
        };
        let (content, labels) = window.split();
        let before = extract(&window.text, &[]).len();
        let started = std::time::Instant::now();
        let kept = terms(&window, &[]);
        let elapsed = started.elapsed().as_millis();
        let from_content = extract(&content, &[]).len().min(kept.len());
        let label_only = uncapped(&labels, &[]).len();
        let label_kept = match &super::words().dictionary {
            Some(dictionary) => uncapped(&labels, &[])
                .iter()
                .filter(|term| !ordinary(term, dictionary))
                .count(),
            None => label_only,
        };
        eprintln!(
            "bytes {}: content {}, labels {} in {} pieces; upstream's terms {before}; \
             kept {} ({from_content} from the content) in {elapsed} ms with the lists loaded; \
             labels alone: {label_only} candidates, {label_kept} not ordinary words",
            window.text.len(),
            content.len(),
            labels.len(),
            window.labels.len(),
            kept.len(),
        );
        assert_eq!(content.len() + labels.len(), window.text.len());
    }

    /// macOS's own lists load when the files are there (they are on every
    /// Mac; a Linux runner may have none).
    #[test]
    fn the_systems_word_lists_load_when_present() {
        let words = Words::load(DICTIONARY, CONNECTIVES);
        if std::path::Path::new(DICTIONARY).exists() {
            let dictionary = words.dictionary.as_ref().expect("the dictionary loads");
            assert!(dictionary.len() > 10_000);
            assert!(dictionary.contains("panel") && !dictionary.contains("Panel"));
            assert!(ordinary("Problems", dictionary) && !ordinary("AskUserQuestion", dictionary));
        }
        if std::path::Path::new(CONNECTIVES).exists() {
            let connectives = words.connectives.as_ref().expect("the connectives load");
            assert!(connectives.contains("the"));
        }
    }
}
