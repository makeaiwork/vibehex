//! "New for you" in the window (`assistant::setup::fresh`): the Labs
//! features that fit the person and are off, as one list to tick, written
//! by the app with no question and no model. One picker, three ways in:
//! the start page's "N new features fit how you use VibeHex" row, its
//! "Which Labs features are worth turning on for me?" (the row from Labs
//! and the example question), and Labs' "Choose with Assistant". It runs
//! with no model connected and while a model answers elsewhere.
//!
//! Each row says what the feature does and, under it, "Now" and "With it"
//! side by side, each on its own backing ([`rows::pair`]); a setting the
//! feature needs is set in the row itself (YOLO mode's "longer than",
//! a companion; More dictation shortcuts' first key, [`shortcut`]). The
//! other features that came since wait under "More to try", each with
//! Add ([`rows::more`]). The start page's row and Labs' "Fits you" read
//! the same picks ([`found`]).

mod rows;
pub(in crate::rune_ui) mod scenes;
#[cfg(target_os = "macos")]
mod shortcut;
#[cfg(test)]
mod tests;

use rune::{El, Tx};

pub(super) use rows::{more, pair};

use super::super::Ui;
use super::super::controls::Paint;
use super::super::sidebar::Pane;
use super::changes::At;
use crate::assistant::changes::Item;
use crate::assistant::setup::flow::Event;
use crate::assistant::setup::fresh::{self, Fresh};
use crate::assistant::setup::since::Since;
use crate::desktop::i18n::tr;

/// Opens the picker in a new conversation (or the empty one open), with
/// what came since the last setup and, while Assistant may read
/// recordings, the person's use, as a returning setup reads them.
pub(in crate::rune_ui) fn begin(ui: &mut Ui) {
    let since = super::returning::since(ui);
    open(ui, since);
}

/// The picker for `since`, in a conversation of its own: the empty one
/// open, unless a question waits there for leave to read recordings.
pub(super) fn open(ui: &mut Ui, since: Option<Since>) {
    let _speaking = crate::assistant::language::speak(&ui.settings);
    let pane = &mut ui.assistant;
    let empty = pane
        .conversation()
        .filter(|open| open.turns.is_empty())
        .map(|open| open.id)
        .filter(|id| {
            pane.answering_in(Some(*id)).is_none()
                && pane
                    .consent
                    .as_ref()
                    .is_none_or(|pending| pending.conversation != *id)
        });
    let id = match empty {
        Some(id) => id,
        None => pane
            .store
            .start(tr(fresh::QUESTION), pane.engine.take(), pane.effort.take()),
    };
    pane.open = Some(id);
    pane.error = None;
    super::ask::send(ui, id, Event::Fresh { since });
}

/// Labs' "Choose with Assistant": Assistant on the picker.
pub(in crate::rune_ui) fn from_labs(ui: &mut Ui, tx: &mut Tx) {
    if !Pane::Assistant.shown(ui) {
        return;
    }
    ui.select_pane(Pane::Assistant, tx);
    ui.assistant.view = super::View::Chat;
    super::load(ui);
    super::whats_new::opened(ui);
    super::suggestions::landed(ui);
    begin(ui);
}

/// What the picker would find now for `since`, as the setup machine finds
/// it: for the start page's row and Labs' "Fits you", in the words `tr`
/// speaks now.
pub(in crate::rune_ui) fn found(ui: &Ui, since: Option<&Since>) -> Fresh {
    let context = super::changes::context(ui);
    fresh::find(since, &context.settings, &context.state).1
}

/// What came since the last setup, without reading the person's use:
/// what the start page and Labs read, never the recordings.
pub(in crate::rune_ui) fn since(ui: &Ui) -> Option<Since> {
    super::returning::since_reading(ui, false)
}

/// "Add" on a row of More to try: into the list, checked.
fn add(ui: &mut Ui, at: At, key: &str) {
    super::changes::set(ui, at, |card| fresh::add(card, key));
}

/// More dictation shortcuts' key in its row, on a Mac; nothing elsewhere.
pub(super) fn shortcut_line(
    ui: &Ui,
    p: Paint,
    at: At,
    (index, item): (usize, &Item),
    open: bool,
) -> Option<El> {
    #[cfg(target_os = "macos")]
    {
        shortcut::line(ui, p, at, (index, item), open)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (ui, p, at, index, item, open);
        None
    }
}

#[cfg(target_os = "macos")]
pub(super) use shortcut::recorded;
