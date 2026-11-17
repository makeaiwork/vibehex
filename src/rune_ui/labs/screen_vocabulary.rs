//! VibeHex: Labs › Screen vocabulary's card while on: whether AI cleanup is
//! given the window's terms too (`labs.screen_vocabulary_cleanup`), with
//! what that means for a cloud engine under it.

use rune::{El, col, el, row, text};
use rune_kit::{ControlSize, Switch};

use super::super::Ui;
use super::super::controls::{DESCRIPTION, Paint};
use crate::desktop::i18n::tr;

pub(super) fn body(ui: &Ui, p: Paint) -> El {
    let on = ui.settings.labs.screen_vocabulary_cleanup;
    let title = tr("Also give them to AI cleanup");
    col()
        .key("screen-vocabulary-cleanup")
        .gap(2.0)
        .child(
            row()
                .key("screen-vocabulary-cleanup-row")
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
                                settings.labs.screen_vocabulary_cleanup =
                                    !settings.labs.screen_vocabulary_cleanup;
                            });
                        })
                        .build(p.k)
                        .key("screen-vocabulary-cleanup-switch"),
                ),
        )
        .child(
            text(tr(
                "Cleanup can fix a name the speech model misheard. With a cloud engine these names are sent to it with the dictation.",
            ))
                .key("note")
                .font(DESCRIPTION)
                .color(p.muted())
                .wrap(),
        )
}
