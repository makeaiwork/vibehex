//! The first run's scenes: the tour state a scene starts from, then the
//! first run played to a page.

use super::super::{Ui, onboarding};
use super::live::Page;

/// The tour state a Linux first-run scene starts from.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::rune_ui) enum FirstRun {
    /// Nothing installed and nothing chosen: nothing downloads.
    Choosing,
    /// The person pressed Download; it is a third of the way.
    Downloading,
    /// The download they started stopped.
    Failed,
    /// The model is installed.
    Ready,
    /// The model is installed, and the desktop asks before VibeHex's first
    /// paste (GNOME and KDE on Wayland).
    PasteAccess,
}

pub(in crate::rune_ui) fn first_run(ui: &mut Ui, state: FirstRun, page: Page) {
    *ui = super::super::portal_fixture();
    onboarding::fixture(
        ui,
        crate::desktop::onboarding::Step::Setup,
        state != FirstRun::Choosing,
    );
    match state {
        FirstRun::Choosing => {}
        FirstRun::Downloading => onboarding::set_downloaded(ui, 31),
        FirstRun::Failed => {
            if let Some(view) = &mut ui.view {
                view.transcription.preparing = None;
                view.transcription.error = Some("The connection was reset.".into());
            }
        }
        FirstRun::Ready | FirstRun::PasteAccess => {
            ui.picker.installed = Some(vec![ui.settings.transcription.model]);
            if let Some(view) = &mut ui.view {
                view.transcription.preparing = None;
            }
        }
    }
    if state == FirstRun::PasteAccess {
        ui.platform.paste = Some(super::super::paste_access::PasteAccess::Ask);
    }
    onboarding::poll(ui);
    super::live::fixture(ui, page);
}

/// The model step as it opens, with the interface in `language`.
pub(in crate::rune_ui) fn first_run_in(ui: &mut Ui, language: &str) {
    first_run(ui, FirstRun::Choosing, Page::Download);
    ui.settings.ui_language = Some(language.into());
    crate::desktop::i18n::apply(Some(language));
    super::live::refresh_choice(ui);
}

pub(in crate::rune_ui) fn mac_first_run(ui: &mut Ui, step: onboarding::MacStep, page: Page) {
    *ui = super::super::mac_fixture();
    onboarding::mac_fixture(ui, step);
    super::live::fixture(ui, page);
}
