//! VibeHex: Labs › Rewrite reads nearby text's card while on: where the text
//! around the cursor goes with the cleanup engine chosen now, and Send it to
//! cloud engines (`labs.cursor_context_cloud`), the consent a cloud engine
//! needs before it gets that text (`dictation::caret_prompt::cleanup`).

use rune::{El, col, el, row, text};
use rune_kit::{ControlSize, Switch};

use super::super::Ui;
use super::super::controls::{DESCRIPTION, Paint};
use crate::app_settings::AppSettings;
use crate::desktop::i18n::{tr, tr_fill};
use crate::dictation::caret_prompt::cleanup::{self, Cloud, Destination};

/// The card's first line: where the nearby text goes now. A warning while a
/// cloud engine cleans up without the consent, so the text is not used.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct Line {
    pub text: String,
    pub warning: bool,
}

fn said(text: impl Into<String>) -> Line {
    Line {
        text: text.into(),
        warning: false,
    }
}

/// [`Line`] for `settings`: nothing reads the text while no mode cleans up;
/// otherwise where the engine chosen now takes it.
pub(super) fn line(settings: &AppSettings) -> Line {
    let processing = &settings.dictation_processing;
    let cleans = processing.default_mode.post_processing.enabled
        || processing
            .modes
            .iter()
            .any(|mode| mode.post_processing.enabled);
    if !cleans {
        return said(tr("Nothing reads it now: AI cleanup is off."));
    }
    let cloud = match cleanup::destination(cleanup::chosen(settings).as_ref()) {
        Destination::ThisComputer => {
            return said(tr(
                "With your cleanup engine now, the nearby text stays on this Mac.",
            ));
        }
        Destination::OpenCode => {
            return said(tr(
                "With your cleanup engine now, the nearby text goes to the mode's OpenCode model, which may be a cloud provider.",
            ));
        }
        Destination::Cloud(Cloud::Server(host)) if host.is_empty() => {
            return said(tr("Nothing reads it now: your server has no address."));
        }
        Destination::Cloud(cloud) => cloud,
    };
    let consent = settings.labs.cursor_context_cloud;
    let text = match (&cloud, consent) {
        (Cloud::Server(host), true) => tr_fill(
            "With your cleanup engine now, the nearby text is sent to your server at {} with each dictation.",
            host,
        ),
        (Cloud::Server(host), false) => tr_fill(
            "Not used now: your cleanup runs on your server at {}. Turn on Send it to cloud engines to use it there.",
            host,
        ),
        (named, true) => tr_fill(
            "With your cleanup engine now, the nearby text is sent to {} with each dictation.",
            name(named),
        ),
        (named, false) => tr_fill(
            "Not used now: your cleanup runs on {}. Turn on Send it to cloud engines to use it there.",
            name(named),
        ),
    };
    Line {
        text,
        warning: !consent,
    }
}

/// A cloud engine's own name.
fn name(cloud: &Cloud) -> &str {
    match cloud {
        Cloud::Codex => "Codex",
        Cloud::OpenRouter => "OpenRouter",
        Cloud::Server(host) => host,
    }
}

pub(super) fn body(ui: &Ui, p: Paint) -> El {
    let line = line(&ui.settings);
    let on = ui.settings.labs.cursor_context_cloud;
    let title = tr("Send it to cloud engines");
    col()
        .key("cursor-context")
        .gap(2.0)
        .child(
            text(line.text)
                .key("cursor-context-destination")
                .font(DESCRIPTION)
                .color(if line.warning { p.warning() } else { p.muted() })
                .wrap(),
        )
        .child(
            row()
                .key("cursor-context-cloud-row")
                .mt(6.0)
                .gap(8.0)
                .center_y()
                .child(
                    text(title)
                        .key("label")
                        .font(DESCRIPTION)
                        .color(p.text())
                        .wrap()
                        .shrink()
                        .min_w(0.0),
                )
                .child(el().key("spacer").grow(1.0).min_w(8.0))
                .child(
                    Switch::new(on)
                        .control_size(ControlSize::Small)
                        .label(title)
                        .hide_label()
                        .on_toggle(|ui: &mut Ui| {
                            ui.update(|settings| {
                                settings.labs.cursor_context_cloud =
                                    !settings.labs.cursor_context_cloud;
                            });
                        })
                        .build(p.k)
                        .key("cursor-context-cloud-switch"),
                ),
        )
        .child(
            text(tr(
                "Codex, OpenRouter, or a server that is not this Mac gets the text around your cursor with each dictation.",
            ))
            .key("note")
            .font(DESCRIPTION)
            .color(p.muted())
            .wrap(),
        )
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

    fn quiet(text: &str) -> Line {
        said(text)
    }

    fn warned(text: &str) -> Line {
        Line {
            text: text.into(),
            warning: true,
        }
    }

    /// The first line names where the text goes with the engine chosen now:
    /// on this Mac, to OpenCode's model, or to a cloud engine by name, plainly
    /// in the warning colour while that engine may not get it.
    #[test]
    fn the_card_names_where_the_nearby_text_goes() {
        let here = "With your cleanup engine now, the nearby text stays on this Mac.";
        assert_eq!(line(&settings(CleanupRunsOn::Local, "")), quiet(here));
        assert_eq!(
            line(&settings(CleanupRunsOn::Custom, "http://127.0.0.1:1234/v1")),
            quiet(here)
        );
        assert_eq!(
            line(&settings(CleanupRunsOn::Opencode, "")),
            quiet(
                "With your cleanup engine now, the nearby text goes to the mode's OpenCode model, which may be a cloud provider."
            )
        );
        // Clean up on your own server off: OpenCode, whatever Runs on says.
        let mut off = settings(CleanupRunsOn::Codex, "");
        off.labs.ai_cleanup_server = false;
        assert_eq!(line(&off), line(&settings(CleanupRunsOn::Opencode, "")));

        assert_eq!(
            line(&settings(CleanupRunsOn::Codex, "")),
            warned(
                "Not used now: your cleanup runs on Codex. Turn on Send it to cloud engines to use it there."
            )
        );
        assert_eq!(
            line(&settings(CleanupRunsOn::Openrouter, "")),
            warned(
                "Not used now: your cleanup runs on OpenRouter. Turn on Send it to cloud engines to use it there."
            )
        );
        assert_eq!(
            line(&settings(
                CleanupRunsOn::Custom,
                "http://192.168.1.20:1234/v1"
            )),
            warned(
                "Not used now: your cleanup runs on your server at 192.168.1.20. Turn on Send it to cloud engines to use it there."
            )
        );
        let consented = |runs_on, url: &str| {
            let mut settings = settings(runs_on, url);
            settings.labs.cursor_context_cloud = true;
            line(&settings)
        };
        assert_eq!(
            consented(CleanupRunsOn::Codex, ""),
            quiet(
                "With your cleanup engine now, the nearby text is sent to Codex with each dictation."
            )
        );
        assert_eq!(
            consented(CleanupRunsOn::Openrouter, ""),
            quiet(
                "With your cleanup engine now, the nearby text is sent to OpenRouter with each dictation."
            )
        );
        assert_eq!(
            consented(CleanupRunsOn::Custom, "https://api.groq.com/openai/v1"),
            quiet(
                "With your cleanup engine now, the nearby text is sent to your server at api.groq.com with each dictation."
            )
        );
        assert_eq!(consented(CleanupRunsOn::Local, ""), quiet(here));
        assert_eq!(
            line(&settings(CleanupRunsOn::Custom, "")),
            quiet("Nothing reads it now: your server has no address.")
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
            assert_eq!(line(&settings), quiet(off));
            let mut mode = settings.dictation_processing.default_mode.clone();
            mode.post_processing.enabled = true;
            settings.dictation_processing.modes.push(mode);
            assert_ne!(line(&settings), quiet(off));
        }
    }
}
