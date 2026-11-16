//! VibeHex: Labs › Rewrite reads nearby text's card while on: one line that
//! names where the text around the cursor goes with the cleanup engine
//! chosen now (`dictation::caret_prompt::cleanup`). The lab works with any
//! engine; a cloud one receives the text with each dictation, and the line
//! says so.

use rune::{El, text};

use super::super::Ui;
use super::super::controls::{DESCRIPTION, Paint};
use crate::app_settings::AppSettings;
use crate::desktop::i18n::{tr, tr_fill};
use crate::dictation::caret_prompt::cleanup::{self, Cloud, Destination};
use crate::dictation::cleanup_server;

/// The card's line for `settings`: nothing reads the text while no mode
/// cleans up; otherwise where the engine chosen now takes it.
pub(super) fn line(settings: &AppSettings) -> String {
    if !cleanup_server::cleans_up(&settings.dictation_processing) {
        return tr("Nothing reads it now: AI cleanup is off.").into();
    }
    match cleanup::destination(cleanup::chosen(settings).as_ref()) {
        Destination::ThisComputer => {
            tr("With your cleanup engine now, the nearby text stays on this Mac.").into()
        }
        Destination::OpenCode => tr(
            "With your cleanup engine now, the nearby text goes to the mode's OpenCode model, which may be a cloud provider.",
        )
        .into(),
        Destination::Cloud(Cloud::Server(host)) if host.is_empty() => {
            tr("Nothing reads it now: your server has no address.").into()
        }
        Destination::Cloud(Cloud::Server(host)) => tr_fill(
            "With your cleanup engine now, the nearby text is sent to your server at {} with each dictation.",
            &host,
        ),
        Destination::Cloud(Cloud::Codex) => tr_fill(
            "With your cleanup engine now, the nearby text is sent to {} with each dictation.",
            "Codex",
        ),
        Destination::Cloud(Cloud::OpenRouter) => tr_fill(
            "With your cleanup engine now, the nearby text is sent to {} with each dictation.",
            "OpenRouter",
        ),
    }
}

pub(super) fn body(ui: &Ui, p: Paint) -> El {
    text(line(&ui.settings))
        .key("cursor-context-destination")
        .font(DESCRIPTION)
        .color(p.muted())
        .wrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_settings::CleanupRunsOn;

    fn settings(runs_on: CleanupRunsOn, url: &str) -> AppSettings {
        let mut settings = AppSettings::default();
        settings.labs.cursor_context = true;
        settings.labs.ai_cleanup_server = true;
        settings
            .dictation_processing
            .default_mode
            .post_processing
            .enabled = true;
        settings.dictation_processing.server.runs_on = runs_on;
        settings.dictation_processing.server.url = url.into();
        settings
    }

    /// The line names where the text goes with the engine chosen now: on
    /// this Mac, to OpenCode's model, or to a cloud engine by name.
    #[test]
    fn the_card_names_where_the_nearby_text_goes() {
        let here = "With your cleanup engine now, the nearby text stays on this Mac.";
        assert_eq!(line(&settings(CleanupRunsOn::Local, "")), here);
        assert_eq!(
            line(&settings(CleanupRunsOn::Custom, "http://127.0.0.1:1234/v1")),
            here
        );
        let opencode = "With your cleanup engine now, the nearby text goes to the mode's OpenCode model, which may be a cloud provider.";
        assert_eq!(line(&settings(CleanupRunsOn::Opencode, "")), opencode);
        // Clean up on your own server off: OpenCode, whatever Runs on says.
        let mut off = settings(CleanupRunsOn::Codex, "");
        off.labs.ai_cleanup_server = false;
        assert_eq!(line(&off), opencode);
        assert_eq!(
            line(&settings(CleanupRunsOn::Codex, "")),
            "With your cleanup engine now, the nearby text is sent to Codex with each dictation."
        );
        assert_eq!(
            line(&settings(CleanupRunsOn::Openrouter, "")),
            "With your cleanup engine now, the nearby text is sent to OpenRouter with each dictation."
        );
        assert_eq!(
            line(&settings(
                CleanupRunsOn::Custom,
                "https://api.groq.com/openai/v1"
            )),
            "With your cleanup engine now, the nearby text is sent to your server at api.groq.com with each dictation."
        );
        assert_eq!(
            line(&settings(CleanupRunsOn::Custom, "")),
            "Nothing reads it now: your server has no address."
        );
    }

    /// While no mode cleans up with AI, nothing reads the text, whatever
    /// the engine; a mode of its own that cleans up is enough.
    #[test]
    fn with_ai_cleanup_off_nothing_reads_it() {
        let off = "Nothing reads it now: AI cleanup is off.";
        for runs_on in [CleanupRunsOn::Codex, CleanupRunsOn::Local] {
            let mut settings = settings(runs_on, "");
            settings
                .dictation_processing
                .default_mode
                .post_processing
                .enabled = false;
            assert_eq!(line(&settings), off);
            let mut mode = settings.dictation_processing.default_mode.clone();
            mode.post_processing.enabled = true;
            settings.dictation_processing.modes.push(mode);
            assert_ne!(line(&settings), off);
        }
    }
}
