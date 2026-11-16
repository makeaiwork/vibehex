//! A plan row that turns on More dictation shortcuts (macOS) carries the
//! key for the first extra shortcut, recorded in the row itself rather
//! than "then go to General" (owner rule, 2026-11-16). It starts unset;
//! a key recorded there is saved with the switch, in the same save, as
//! the first row of `extra_dictation_hotkeys` in the mode General gives
//! a new row. With no key, only the switch is saved. The settings key
//! stays one no card names (`catalog::never`, a JSON list): only this
//! row's own key reaches it, and Undo puts the list back with the switch
//! (`Decision::Applied` names every key the save changed).

use serde_json::Value;

use super::Item;
use crate::app_settings::{AppSettings, HotkeyBinding};

/// The switch whose row carries the key.
pub const SWITCH: &str = "labs.more_dictation_shortcuts";
/// Where the key is saved.
pub const KEY: &str = "extra_dictation_hotkeys";

/// Whether `item` offers a key in its row: it turns the switch on, on a
/// Mac (`os`), while no extra shortcut has a key yet (one set before in
/// General is the person's own, and stays).
pub fn offered(item: &Item, settings: &AppSettings, os: &str) -> bool {
    item.key == SWITCH
        && item.to == Value::Bool(true)
        && os == "macos"
        && settings
            .extra_dictation_hotkeys
            .iter()
            .all(|row| row.binding.is_none())
}

/// The extra shortcuts `settings` would hold with `binding` as the first:
/// in the first row, unset until now, or a new row as General adds one.
pub fn rows(settings: &AppSettings, binding: &HotkeyBinding) -> Value {
    let mut with = settings.clone();
    if with.extra_dictation_hotkeys.is_empty() {
        with.add_extra_dictation_hotkey();
    }
    if let Some(first) = with.extra_dictation_hotkeys.first_mut() {
        first.binding = Some(binding.clone());
    }
    serde_json::to_value(&with.extra_dictation_hotkeys).unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::super::{Card, CardKind, Decision, applied, values};
    use super::*;
    use crate::app_settings::{ExtraDictationMode, HotkeyKey, HotkeyModifiers};
    use crate::control::settings;

    fn f5() -> HotkeyBinding {
        HotkeyBinding {
            modifiers: HotkeyModifiers::control_option(),
            key: Some(HotkeyKey::named("f5")),
        }
    }

    fn rules() -> settings::ShortcutRules {
        settings::ShortcutRules {
            key_named: |name| Ok(HotkeyKey::named(name)),
            rule: |_, _| Ok(()),
        }
    }

    fn row(shortcut: Option<HotkeyBinding>) -> Card {
        Card {
            kind: CardKind::Plan,
            items: vec![Item {
                key: SWITCH.into(),
                from: json!(false),
                to: json!(true),
                included: true,
                shortcut,
                ..Item::default()
            }],
            ..Card::default()
        }
    }

    /// The row offers a key only for the switch going on, on a Mac, while
    /// no extra shortcut has one.
    #[test]
    fn a_key_is_offered_only_while_none_is_set() {
        let mut settings = AppSettings::default();
        let item = &row(None).items[0];
        assert!(offered(item, &settings, "macos"));
        assert!(!offered(item, &settings, "windows"));
        let off = Item {
            to: json!(false),
            ..item.clone()
        };
        assert!(!offered(&off, &settings, "macos"));
        // An unset row General added leaves room for it.
        settings.add_extra_dictation_hotkey();
        assert!(offered(item, &settings, "macos"));
        settings.extra_dictation_hotkeys[0].binding = Some(f5());
        assert!(!offered(item, &settings, "macos"));
    }

    /// A key goes in the first row in General's default mode, a new one
    /// when there is none; the rows already there follow it.
    #[test]
    fn the_key_is_the_first_row() {
        let mut settings = AppSettings::default();
        let rows_then = rows(&settings, &f5());
        let saved: Vec<crate::app_settings::ExtraDictationHotkey> =
            serde_json::from_value(rows_then).unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].binding, Some(f5()));
        assert_eq!(saved[0].mode, ExtraDictationMode::default());
        settings.add_extra_dictation_hotkey();
        settings.add_extra_dictation_hotkey();
        settings.extra_dictation_hotkeys[1].mode = ExtraDictationMode::Tap;
        let saved: Vec<crate::app_settings::ExtraDictationHotkey> =
            serde_json::from_value(rows(&settings, &f5())).unwrap();
        assert_eq!(saved.len(), 2);
        assert_eq!(saved[0].binding, Some(f5()));
        assert_eq!(saved[1].mode, ExtraDictationMode::Tap);
    }

    /// With a key, the save carries the switch and the key together, and
    /// what it records to undo puts both back; without one, only the
    /// switch.
    #[test]
    fn the_switch_and_its_key_save_and_undo_together() {
        let before = AppSettings::default();
        let plain = values(&row(None), &before).unwrap();
        assert_eq!(plain.keys().collect::<Vec<_>>(), [SWITCH]);
        let with = values(&row(Some(f5())), &before).unwrap();
        assert!(with.contains_key(SWITCH) && with.contains_key(KEY));
        // The extra shortcuts are a Mac's; elsewhere a save refuses them.
        if !cfg!(target_os = "macos") {
            return;
        }
        let saved = settings::check(&before, &with, &rules()).unwrap().settings;
        assert!(saved.labs.more_dictation_shortcuts);
        assert_eq!(saved.extra_dictation_hotkeys.len(), 1);
        assert_eq!(saved.extra_dictation_hotkeys[0].binding, Some(f5()));
        let Decision::Applied { before: back, .. } = applied(settings::diff(&before, &saved), 0)
        else {
            unreachable!()
        };
        assert!(back.contains_key(SWITCH) && back.contains_key(KEY));
        let undone = settings::check(&saved, &back, &rules()).unwrap().settings;
        assert!(!undone.labs.more_dictation_shortcuts);
        assert!(undone.extra_dictation_hotkeys.is_empty());
    }
}
