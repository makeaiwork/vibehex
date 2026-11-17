//! VibeHex's own Labs: which features VibeHex added and which of Hex's it
//! reworked (their cards carry a star), which of Hex's labs a stable
//! VibeHex leaves out, the English copy of VibeHex's cards, where feedback
//! goes, and the runtime switches of VibeHex's labs. Kept apart from Hex's
//! registry files so an upstream sync only touches one arm per match there.
//!
//! VibeHex's switches are not bits in Hex's state word ([`super::STATE`]):
//! each has its own atomic here, so a lab Hex adds can never land on one.

use std::sync::atomic::{AtomicBool, Ordering};

use super::Feature;
use crate::app_settings::LabsSettings;

/// Where feedback on VibeHex goes.
pub const FEEDBACK_EMAIL: &str = "zelenpro@gmail.com";

/// A `mailto:` link that opens a new message to [`FEEDBACK_EMAIL`] whose
/// subject names the running version.
pub fn feedback_mailto(version: &str) -> String {
    format!(
        "mailto:{FEEDBACK_EMAIL}?subject={}",
        mailto_encode(&format!("VibeHex feedback (version {version})"))
    )
}

/// Percent-encodes a `mailto:` query value: everything but unreserved
/// characters, so spaces become `%20` (not `+`, which mail apps show as is).
fn mailto_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

impl Feature {
    /// Whether VibeHex added this feature: Hex itself (anomalyco/hex) has
    /// no such lab. Settings › Labs marks these rows with a star.
    pub const fn vibehex(self) -> bool {
        matches!(
            self,
            Self::AiCleanupServer
                | Self::CleanupProgress
                | Self::CollapsibleSidebar
                | Self::Files
                | Self::LowerEveryApp
                | Self::Playback
                | Self::RussianVibeCoding
                | Self::StopShortcut
                | Self::VocabularyPasteList
                | Self::VocabularyQuickAdd
        )
    }

    /// Whether VibeHex reworked this feature: Hex has the lab, and VibeHex
    /// changed how it works. Settings › Labs gives these rows the same star,
    /// said "Reworked in VibeHex" (the owner's request of 2026-11-17).
    ///
    /// - Screen vocabulary (Hex's name): reads from the field being
    ///   dictated into outward, skips interface labels, and can give the
    ///   names to AI cleanup (`dictation::screen_vocabulary::{labels,
    ///   cleanup}`).
    /// - Rewrite reads nearby text (Hex's name): works on every cleanup
    ///   engine, not only OpenCode, and cuts keys and tokens out
    ///   (`dictation::caret_prompt`).
    pub const fn vibehex_reworked(self) -> bool {
        matches!(self, Self::ScreenVocabulary | Self::CursorContext)
    }

    /// The fork's card title for a Hex lab it reworked, in place of Hex's
    /// ([`Feature::hex_title`]), so the name says it is the improved one
    /// (the owner's request of 2026-11-17). Its `hex labs` name, settings
    /// keys and Hex's website stay Hex's.
    pub(super) const fn vibehex_renamed(self) -> Option<&'static str> {
        match self {
            Self::ScreenVocabulary => Some("Enhanced screen vocabulary"),
            Self::CursorContext => Some("Enhanced cursor context"),
            _ => None,
        }
    }

    /// Hex labs VibeHex leaves out of a stable build, as Hex does its
    /// developer tools: they repeat something VibeHex already has (About),
    /// the owner chose not to ship them, or they run only on OpenCode while
    /// VibeHex cleans up on its own engines (Apple Intelligence cleanup,
    /// Voice prompt), until they run there too. Rewrite reads nearby text
    /// runs on every engine (`dictation::caret_prompt::cleanup`).
    pub const fn vibehex_hides(self) -> bool {
        matches!(
            self,
            Self::MeetingImport
                | Self::ReleaseNotesLink
                | Self::QuitClosesWindow
                | Self::AnnotateReplay
                | Self::CleanUpWithAppleIntelligence
                | Self::VoicePrompt
        )
    }

    /// A VibeHex lab's card title ([`Feature::title`]).
    pub(super) const fn vibehex_title(self) -> &'static str {
        match self {
            Self::AiCleanupServer => "Clean up on your own server",
            Self::VocabularyPasteList => "Vocabulary text view",
            Self::RussianVibeCoding => "Russian vibe coding",
            Self::VocabularyQuickAdd => "Add to vocabulary from a selection",
            Self::CleanupProgress => "Cleanup progress",
            Self::Files => "Files",
            Self::StopShortcut => "Stop shortcut",
            Self::CollapsibleSidebar => "Collapsible sidebar",
            Self::Playback => "Listen to the recording",
            Self::LowerEveryApp => "Lower every app",
            _ => "",
        }
    }

    /// A VibeHex lab's card line ([`Feature::summary`]).
    pub(super) const fn vibehex_summary(self) -> &'static str {
        match self {
            Self::AiCleanupServer => {
                "Rewrite dictations with OpenRouter or a local server such as LM Studio or Ollama, set up in Output"
            }
            Self::VocabularyPasteList => {
                "Edit the whole Vocabulary as text, one term per line as Term | sounds like, to paste, fix, or delete many terms at once"
            }
            Self::RussianVibeCoding => {
                "Adds 90 terms such as Claude Code, GitHub, and MCP, Russian word fixes, and cleanup instructions to your settings, and turns on Clean up with AI and the other starred Labs features except Stop shortcut. Off takes back the terms, fixes, and instructions"
            }
            Self::VocabularyQuickAdd => {
                "Select a misheard word in any app and press a shortcut to add it to Vocabulary with its right spelling, which also replaces the selection where the text can be edited"
            }
            Self::CleanupProgress => {
                "While AI cleanup takes a while, a bar in place of the HUD shows how far it is, with Paste now to paste the part already cleaned and the rest as transcribed"
            }
            Self::Files => {
                "Drop audio and video files to transcribe them, clean them up, and rework the text with your own requests"
            }
            Self::StopShortcut => {
                "Another key, such as Return, finishes a dictation and pastes it, while your shortcut starts it"
            }
            Self::CollapsibleSidebar => {
                "A button beside the logo folds the sidebar into a narrow column of icons, so the pane gets the room"
            }
            Self::Playback => {
                "Play a meeting or a file from its transcript: Play beside the requests opens a player, the line playing lights up, and a click on a line plays from there"
            }
            Self::LowerEveryApp => {
                "Mute and Lower audio turn down every app, Podcasts and browsers too, when macOS can't set your output's volume, such as a monitor's. Needs system audio access"
            }
            _ => "",
        }
    }

    /// A VibeHex lab's one-line description for the CLI
    /// ([`Feature::description`]).
    pub(super) const fn vibehex_description(self) -> &'static str {
        match self {
            Self::AiCleanupServer => {
                "Clean up dictations with OpenRouter or your own OpenAI-compatible server, such as LM Studio or Ollama, instead of OpenCode"
            }
            Self::VocabularyPasteList => {
                "The Vocabulary sheet's Text view shows the whole list as text, one term per line as Term | sounds like, sounds like, to paste, fix, or delete many at once"
            }
            Self::RussianVibeCoding => {
                "Adds the Russian vibe-coding vocabulary, word fixes, and AI cleanup instructions to your settings and turns on Clean up with AI and every other VibeHex feature but Stop shortcut; off takes back what it added and you haven't changed"
            }
            Self::VocabularyQuickAdd => {
                "Select a misheard word in any app and press a shortcut to add it to Vocabulary with its right spelling, which also replaces the selection where the text can be edited"
            }
            Self::CleanupProgress => {
                "While AI cleanup takes a while, a bar beside the HUD shows how far it is, with Paste now to paste the part already cleaned and the rest as transcribed"
            }
            Self::Files => {
                "Drop audio and video files to transcribe them, clean them up, and rework the text with your own requests"
            }
            Self::StopShortcut => {
                "Another key, such as Return, finishes a dictation and pastes it, while your shortcut starts it (stop_hotkey)"
            }
            Self::CollapsibleSidebar => {
                "A button beside the logo folds the sidebar into a narrow column of icons, so the pane gets the room"
            }
            Self::LowerEveryApp => {
                "Mute and Lower audio turn down every other app through a system audio tap when macOS can't set the output's volume, such as a monitor's; needs system audio access"
            }
            Self::Playback => {
                "Play a meeting or a file from its transcript: Play beside the requests opens a player, the line playing lights up, and a click on a line plays from there"
            }
            _ => "",
        }
    }
}

/// VibeHex's labs a worker reads at run time, each with its own switch in
/// [`SWITCHES`], in this order. The rest (Vocabulary text view, Russian vibe
/// coding, Stop shortcut, Collapsible sidebar, Listen to the recording) are
/// read from the settings where they act.
const SWITCHED: [Feature; 5] = [
    Feature::AiCleanupServer,
    Feature::VocabularyQuickAdd,
    Feature::CleanupProgress,
    Feature::Files,
    Feature::LowerEveryApp,
];

/// This process's switch of each [`SWITCHED`] lab, written by [`store`]
/// beside Hex's state word.
static SWITCHES: [AtomicBool; SWITCHED.len()] = [const { AtomicBool::new(false) }; SWITCHED.len()];

/// Labs › Screen vocabulary's terms go to AI cleanup too: VibeHex's switch
/// on Hex's lab, written by [`store`] beside [`SWITCHES`].
static SCREEN_VOCABULARY_CLEANUP: AtomicBool = AtomicBool::new(false);

/// Whether `settings` give Screen vocabulary's terms to AI cleanup: the
/// switch is on and so is the lab, in this build and unlocked.
fn screen_vocabulary_cleanup_enabled(settings: &LabsSettings) -> bool {
    settings.screen_vocabulary_cleanup && super::screen_vocabulary_enabled(settings)
}

/// Whether `settings` hold `feature`'s switch on.
fn saved(settings: &LabsSettings, feature: Feature) -> bool {
    match feature {
        Feature::AiCleanupServer => settings.ai_cleanup_server,
        Feature::VocabularyQuickAdd => settings.vocabulary_quick_add,
        Feature::CleanupProgress => settings.cleanup_progress,
        Feature::Files => settings.files,
        Feature::LowerEveryApp => settings.lower_every_app,
        _ => false,
    }
}

/// The switches `settings` hold for the [`SWITCHED`] labs `reaches` admits,
/// in [`SWITCHED`]'s order.
pub(super) fn switches_where(
    settings: &LabsSettings,
    reaches: impl Fn(Feature) -> bool,
) -> [bool; SWITCHED.len()] {
    SWITCHED.map(|feature| saved(settings, feature) && reaches(feature))
}

/// Records the switches the loaded or saved settings hold, as
/// [`super::apply`] records Hex's: this build ships the lab and no
/// organization locks it.
pub(super) fn store(settings: &LabsSettings) {
    let policy = crate::policy::Policy::current();
    let on = switches_where(settings, |feature| {
        feature.shipped() && policy.labs_lock(feature).is_none()
    });
    for (switch, on) in SWITCHES.iter().zip(on) {
        switch.store(on, Ordering::Release);
    }
    SCREEN_VOCABULARY_CLEANUP.store(
        screen_vocabulary_cleanup_enabled(settings),
        Ordering::Release,
    );
}

/// `feature`'s switch in this process, loading the saved settings first in
/// a process that never applied them.
fn switch(feature: Feature) -> bool {
    let _ = super::state();
    SWITCHED
        .iter()
        .position(|known| *known == feature)
        .is_some_and(|index| SWITCHES[index].load(Ordering::Acquire))
}

/// Labs › Add to vocabulary from a selection: the shortcut that adds a
/// selected misheard word to Vocabulary (macOS).
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub fn vocabulary_quick_add() -> bool {
    switch(Feature::VocabularyQuickAdd)
}

/// Labs › Cleanup progress: AI cleanup reports how far it is, which a plate
/// beside the HUD shows with Paste now (macOS).
pub fn cleanup_progress() -> bool {
    switch(Feature::CleanupProgress)
}

/// Labs › Files: the Files pane, its queue, and its worker (macOS).
pub fn files() -> bool {
    switch(Feature::Files)
}

/// Labs › Lower every app: Mute and Lower audio turn every other app down
/// through a system audio tap when the output has no volume (macOS).
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub fn lower_every_app() -> bool {
    switch(Feature::LowerEveryApp)
}

/// Labs › Screen vocabulary › Also give them to AI cleanup: a dictation's
/// screen terms reach its cleanup too, and the window is read for them
/// whatever the speech model (macOS; read as the capture becomes real and as
/// its cleanup starts). Only while the lab itself is on.
pub fn screen_vocabulary_cleanup() -> bool {
    super::screen_vocabulary() && SCREEN_VOCABULARY_CLEANUP.load(Ordering::Acquire)
}

/// Dictation cleanup may run on OpenRouter or the person's own
/// OpenAI-compatible server instead of OpenCode (read as each dictation's
/// rewrite starts).
pub fn ai_cleanup_server() -> bool {
    switch(Feature::AiCleanupServer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_settings::AppSettings;

    /// VibeHex hides only Hex's labs, never its own, and a stable build
    /// leaves the hidden ones out (a dev build keeps them, as tests do).
    #[test]
    fn vibehex_hides_only_hex_labs_and_only_from_a_stable_build() {
        for feature in Feature::ALL {
            assert!(
                !(feature.vibehex() && feature.vibehex_hides()),
                "{feature:?}"
            );
        }
        assert!(Feature::ReleaseNotesLink.shipped());
        #[cfg(target_os = "linux")]
        {
            let _stable = crate::dev_build::pretend_stable();
            for feature in Feature::ALL.into_iter().filter(|f| f.vibehex_hides()) {
                assert!(!feature.shipped(), "{feature:?}");
            }
            assert!(Feature::Files.shipped());
        }
    }

    /// Giving the screen's terms to AI cleanup is off by default and does
    /// nothing without Screen vocabulary itself.
    #[test]
    fn screen_terms_reach_cleanup_only_with_the_lab_on() {
        let mut labs = LabsSettings::default();
        assert!(!labs.screen_vocabulary_cleanup);
        assert!(!screen_vocabulary_cleanup_enabled(&labs));
        labs.screen_vocabulary_cleanup = true;
        assert!(!screen_vocabulary_cleanup_enabled(&labs), "the lab is off");
        labs.screen_vocabulary = true;
        assert!(screen_vocabulary_cleanup_enabled(&labs));
        labs.screen_vocabulary_cleanup = false;
        assert!(!screen_vocabulary_cleanup_enabled(&labs));

        labs.screen_vocabulary_cleanup = true;
        super::super::apply(&labs);
        assert_eq!(
            screen_vocabulary_cleanup(),
            Feature::ScreenVocabulary.on_this_os()
        );
        labs.screen_vocabulary = false;
        super::super::apply(&labs);
        assert!(!screen_vocabulary_cleanup());
        super::super::apply(&LabsSettings::default());
        assert!(!screen_vocabulary_cleanup());
        assert_eq!(
            Feature::for_settings_key("labs.screen_vocabulary_cleanup"),
            Some(Feature::ScreenVocabulary)
        );
    }

    /// A lab is VibeHex's own or one of Hex's it reworked, never both, and
    /// a reworked one stays in a stable build.
    #[test]
    fn no_lab_is_both_added_and_reworked() {
        for feature in Feature::ALL {
            assert!(
                !(feature.vibehex() && feature.vibehex_reworked()),
                "{feature:?}"
            );
            if feature.vibehex_reworked() {
                assert!(!feature.vibehex_hides(), "{feature:?}");
            }
        }
        let reworked: Vec<&str> = Feature::ALL
            .into_iter()
            .filter(|feature| feature.vibehex_reworked())
            .map(Feature::name)
            .collect();
        assert_eq!(reworked, ["screen-vocabulary", "cursor-context"]);
    }

    /// Only a reworked lab takes the fork's name, and it says so; Hex's
    /// name stays for Hex's website.
    #[test]
    fn a_reworked_lab_has_the_forks_name() {
        for feature in Feature::ALL {
            match feature.vibehex_renamed() {
                Some(title) => {
                    assert!(feature.vibehex_reworked(), "{feature:?}");
                    assert!(title.starts_with("Enhanced "), "{title}");
                    assert_eq!(feature.title(), title);
                }
                None => assert_eq!(feature.title(), feature.hex_title()),
            }
        }
        assert_eq!(Feature::ScreenVocabulary.hex_title(), "Screen vocabulary");
        assert_eq!(
            Feature::CursorContext.hex_title(),
            "Rewrite reads nearby text"
        );
    }

    /// Rewrite reads nearby text shows in a stable build now that it runs
    /// on every cleanup engine.
    #[test]
    fn the_nearby_text_lab_is_not_hidden() {
        assert!(!Feature::CursorContext.vibehex_hides());
    }

    #[test]
    fn feedback_mailto_names_the_version_in_the_subject() {
        assert_eq!(
            feedback_mailto("0.0.0-dev-12"),
            "mailto:zelenpro@gmail.com?subject=VibeHex%20feedback%20%28version%200.0.0-dev-12%29"
        );
    }

    /// Every VibeHex lab has its card's words, and only VibeHex's labs take
    /// them from here.
    #[test]
    fn vibehex_labs_carry_their_own_copy() {
        for feature in Feature::ALL {
            let own = !feature.vibehex_title().is_empty()
                && !feature.vibehex_summary().is_empty()
                && !feature.vibehex_description().is_empty();
            assert_eq!(own, feature.vibehex(), "{feature:?}");
            if feature.vibehex() {
                assert_eq!(feature.title(), feature.vibehex_title());
                assert_eq!(feature.summary(), feature.vibehex_summary());
                assert_eq!(feature.description(), feature.vibehex_description());
            }
        }
    }

    /// No two labs share a runtime switch: turned on alone, each lab sets
    /// bits of Hex's state word or VibeHex's own switches that no other lab
    /// sets. Caps Lock dictation's light choice is part of its own lab.
    #[test]
    fn no_two_labs_share_a_switch() {
        let minutes = super::super::MINUTES_MASK << super::super::MINUTES_SHIFT;
        let ignored = super::super::KNOWN | minutes;
        let mut seen: Vec<(Feature, u64, [bool; SWITCHED.len()])> = Vec::new();
        for feature in Feature::ALL {
            let mut settings = AppSettings::default();
            // Russian vibe coding turns other labs on through its preset;
            // its own switch is the one under test.
            if feature == Feature::RussianVibeCoding {
                settings.labs.russian_vibe_coding = true;
            } else {
                feature.set(&mut settings, true);
            }
            let word = super::super::encode_where(&settings.labs, |_| true) & !ignored;
            let own = switches_where(&settings.labs, |_| true);
            for (other, other_word, other_own) in &seen {
                assert_eq!(
                    word & other_word,
                    0,
                    "{feature:?} and {other:?} share a bit of the state word"
                );
                assert!(
                    !own.iter().zip(other_own).any(|(a, b)| *a && *b),
                    "{feature:?} and {other:?} share a VibeHex switch"
                );
            }
            seen.push((feature, word, own));
        }
        for feature in SWITCHED {
            let (_, _, own) = seen.iter().find(|(known, ..)| *known == feature).unwrap();
            assert_eq!(own.iter().filter(|on| **on).count(), 1, "{feature:?}");
        }
    }
}
