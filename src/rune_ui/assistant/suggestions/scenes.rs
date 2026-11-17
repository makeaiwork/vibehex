//! The start page's suggestions in scenes: each fills the rules' signals
//! itself (`Suggestions::scene`), so nothing is read from the disk, and
//! the times are the Assistant fixtures' fixed day.

use super::super::super::Ui;
use super::super::fixture::{self, MINUTE};
use crate::assistant::suggest::{Cleaned, From, Met, Signals};

const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;

/// The scene's rows picked from `signals`, as the window picks them.
pub(in super::super) fn show(ui: &mut Ui, signals: Signals) {
    ui.assistant.suggestions.scene = Some(signals);
    super::refresh(ui);
}

fn met(id: &str, title: &str, now: u64, ago: u64, minutes: u64, notes_failed: bool) -> Met {
    Met {
        id: id.into(),
        title: title.into(),
        start_ms: now - ago,
        duration_ms: minutes * MINUTE,
        notes_failed,
    }
}

/// What broke on a new conversation's page: the last five dictations
/// pasted without cleanup while Codex is signed out, a meeting's notes not
/// written, and three Labs features new since the last setup that fit.
/// One example question fills the block.
pub(in crate::rune_ui) fn broke(ui: &mut Ui) {
    fixture::open(ui);
    let now = ui.assistant.now_ms();
    let mut cleaned: Vec<Cleaned> = (1..=5)
        .map(|hours| Cleaned {
            at_ms: now - hours * HOUR,
            fallback: Some("sign in to Codex again".into()),
        })
        .collect();
    cleaned.push(Cleaned {
        at_ms: now - 2 * DAY,
        fallback: None,
    });
    show(
        ui,
        Signals {
            now_ms: now,
            cleanup_on: true,
            cleaned,
            meetings: vec![met("design", "Design review", now, DAY, 45, true)],
            new_labs: vec![
                ("labs.yolo_mode".into(), "YOLO mode".into()),
                (
                    "labs.more_dictation_shortcuts".into(),
                    "More dictation shortcuts".into(),
                ),
                ("labs.return_to_app".into(), "Return to app".into()),
            ],
            ..Signals::default()
        },
    );
}

/// Coming from Meetings with a meeting open: notes not written for
/// another, a question about the open one, and the week's action items.
pub(in crate::rune_ui) fn meetings(ui: &mut Ui) {
    fixture::open(ui);
    let now = ui.assistant.now_ms();
    show(
        ui,
        Signals {
            now_ms: now,
            cleanup_on: true,
            meetings: vec![
                met("arcade", "Arcade launch plan", now, HOUR, 38, false),
                met("design", "Design review", now, DAY, 45, true),
                met("standup", "Standup", now, 2 * DAY, 15, false),
                met("hiring", "Hiring sync", now, 4 * DAY, 30, false),
            ],
            open_meeting: Some("arcade".into()),
            from: Some(From::Meetings),
            ..Signals::default()
        },
    );
}

/// The page's broken rows while General shows: the sidebar's Assistant
/// row carries the warning mark.
#[cfg(test)]
pub(super) fn marked(ui: &mut Ui) {
    broke(ui);
    ui.pane = super::super::super::sidebar::Pane::General;
}
