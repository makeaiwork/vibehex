//! More dictation shortcuts' first key, set in its own row of "New for
//! you" (owner rule: a setting the feature needs is set in the row, never
//! "then go to General"). The row draws one line as Assistant's
//! shortcuts card draws a row (`keys::line`): the keyboard glyph, "Extra
//! shortcut 1" as General names it, the hint under it, and General's
//! recorder at the right edge, "Not set" until a key is pressed.
//!
//! Pressed, it records through the same path the shortcuts card takes
//! (`keys::record`: the host pauses VibeHex's shortcuts until it ends;
//! the chord comes back through `keys::recorded`), and the key passes
//! General's rules against the settings as the plan would save them. It
//! stays in the row, unsaved, and goes in with the switch on Apply
//! (`changes::extra`); ⌫ alone clears it. Undo puts both back.

use rune::El;
use serde_json::{Map, Value};

use super::super::super::controls::Paint;
use super::super::super::general::{keys_tray, recorder};
use super::super::super::icons::KEYBOARD;
use super::super::super::settings::{self as s, Capture, Face};
use super::super::super::{ShortcutTarget, Ui};
use super::super::changes::{self as act, At};
use super::super::keys;
use crate::app_settings::{AppSettings, HotkeyBinding, HotkeyModifiers};
use crate::assistant::changes::{self, Card, Item, extra};
use crate::control::settings;
use crate::desktop::i18n::{tr, tr_window};

/// The first extra shortcut, the one the row records.
const TARGET: ShortcutTarget = ShortcutTarget::ExtraDictation(0);

/// The row's key line: while the plan waits, the recorder, for a switch
/// that turns on with no extra shortcut set yet; once decided, the key it
/// recorded, if any.
pub(super) fn line(
    ui: &Ui,
    p: Paint,
    at: At,
    (index, item): (usize, &Item),
    open: bool,
) -> Option<El> {
    let shows = match open {
        true => extra::offered(item, &ui.settings, super::super::ask::os(ui)),
        false => item.key == extra::SWITCH && item.shortcut.is_some(),
    };
    if !shows {
        return None;
    }
    let name = tr_window("Extra shortcut {}").replace("{}", "1");
    let none = tr("Not set");
    let control = if open {
        let listening = ui.assistant.recording == Some((at, index))
            && matches!(ui.capture, Capture::Listening { target, .. } if target == TARGET);
        let face = match listening {
            true => s::face(ui, TARGET),
            false => Face::Shortcut { saved: false },
        };
        recorder(
            p,
            "shortcut-keys",
            (face, listening),
            item.shortcut.as_ref(),
            none,
            move |ui: &mut Ui| keys::record(ui, at, index, TARGET),
        )
    } else {
        keys_tray(p, item.shortcut.as_ref(), none)
    };
    let hint = open.then(|| tr("Click the field and press the keys"));
    let words = keys::words(p, &name, hint, Vec::new());
    let c = &p.k.c;
    Some(
        rune::col()
            .key("extra-shortcut")
            .mt(8.0)
            .radius(p.k.radius.md)
            .fill(c.layer_02)
            .border(p.hair, c.border_weak)
            .child(keys::line(p, "line".to_owned(), &KEYBOARD, words, control).label(name)),
    )
}

/// A chord recorded for row `index` of the plan at `at`: into the row,
/// checked, once it passes General's rules against the settings as the
/// plan would save them; else the recorder says why and keeps listening.
pub(in super::super) fn recorded(
    ui: &mut Ui,
    (at, index): (At, usize),
    target: ShortcutTarget,
    binding: Result<HotkeyBinding, &'static str>,
) {
    let Some(card) = act::card(ui, at).cloned() else {
        return s::cancel_capture(ui);
    };
    if binding.as_ref().is_ok_and(HotkeyBinding::is_bare_erase) {
        s::cancel_capture(ui);
        return keep(ui, at, index, None);
    }
    let after = planned(ui, &card, index);
    let rules = super::super::super::shortcut_rules();
    // As `hex settings` reads it back, so it compares with the saved ones.
    let binding = binding.map(|binding| {
        settings::parse_hotkey(&settings::format_hotkey(&binding), rules.key_named)
            .unwrap_or(binding)
    });
    let checked = binding.and_then(|binding| {
        s::validate(&after, target, &binding)?;
        let values = Map::from_iter([(extra::KEY.to_owned(), extra::rows(&after, &binding))]);
        match settings::check(&after, &values, &rules) {
            Ok(_) => Ok(binding),
            Err(_) if binding.modifiers.is_empty() => Err(tr("Add a modifier")),
            Err(_) => Err(tr("Unsupported key")),
        }
    });
    match checked {
        Ok(binding) => {
            s::cancel_capture(ui);
            keep(ui, at, index, Some(binding));
        }
        Err(message) => {
            ui.capture = Capture::Listening {
                target,
                message: Some(message),
                held: HotkeyModifiers::default(),
            };
        }
    }
}

/// Row `index` holds `binding`; a key recorded checks the row.
fn keep(ui: &mut Ui, at: At, index: usize, binding: Option<HotkeyBinding>) {
    act::set(ui, at, |card| {
        if let Some(item) = card.items.get_mut(index) {
            item.included |= binding.is_some();
            item.shortcut = binding;
        }
    });
}

/// The settings as the plan's checked rows would save them, row `index`
/// checked and without its key: what a recorded key is held against.
fn planned(ui: &Ui, card: &Card, index: usize) -> AppSettings {
    let before = ui.drafted();
    let mut card = act::as_saved(ui, card);
    for item in &mut card.items {
        item.shortcut = None;
    }
    if let Some(item) = card.items.get_mut(index) {
        item.included = true;
    }
    let rules = super::super::super::shortcut_rules();
    changes::values(&card, &before)
        .ok()
        .and_then(|values| settings::check(&before, &values, &rules).ok())
        .map(|change| change.settings)
        .unwrap_or_else(|| {
            let mut on = before.clone();
            let switch = Map::from_iter([(extra::SWITCH.to_owned(), Value::Bool(true))]);
            if let Ok(change) = settings::check(&before, &switch, &rules) {
                on = change.settings;
            }
            on
        })
}
