//! The shortcut keys VibeHex's Labs add: Add to vocabulary from a selection's
//! default, and Labs › Stop shortcut's default and its match beside a held
//! dictation shortcut.

use super::*;

impl HotkeyBinding {
    /// Option-Shift-A, Labs › Add to vocabulary from a selection; macOS
    /// only, so elsewhere unbound.
    pub fn vocabulary_add_default() -> Option<Self> {
        #[cfg(target_os = "macos")]
        let binding = Some(Self {
            modifiers: HotkeyModifiers {
                option: Some(ModifierSide::Either),
                shift: Some(ModifierSide::Either),
                ..Default::default()
            },
            key: Some(HotkeyKey {
                code: layout_key_code('a').unwrap_or(0),
                label: "A".into(),
            }),
        });
        #[cfg(not(target_os = "macos"))]
        let binding = None;
        binding
    }

    /// Return, with no modifier: Labs › Stop shortcut's key (macOS).
    pub fn stop_default() -> Self {
        #[cfg(target_os = "macos")]
        let binding = Self {
            modifiers: HotkeyModifiers::default(),
            // Return's virtual key code, the same on every layout.
            key: Some(HotkeyKey {
                code: 36,
                label: "Return".into(),
            }),
        };
        #[cfg(not(target_os = "macos"))]
        let binding = Self::chord(HotkeyModifiers::default(), "return");
        binding
    }
}

impl RuntimeHotkey {
    /// The key press matches while `held`, a shortcut still down, keeps its
    /// own modifiers down: Return with Right ⌥ held, while a Right ⌥
    /// dictation records, reads as Return.
    pub fn matches_key_press_beside(self, code: u16, flags: u64, held: RuntimeHotkey) -> bool {
        self.matches_key_press(code, flags)
            || (!held.modifiers.is_empty()
                && held.required_modifiers_down(flags)
                && self.matches_key_press(code, flags & !modifier_flags(held.modifiers)))
    }
}

/// Every flag that reports one of `modifiers`, on either side.
fn modifier_flags(modifiers: HotkeyModifiers) -> u64 {
    [
        (
            modifiers.control.is_some(),
            CONTROL_KEY_MASK | LEFT_CONTROL_MASK | RIGHT_CONTROL_MASK,
        ),
        (
            modifiers.option.is_some(),
            OPTION_KEY_MASK | LEFT_OPTION_MASK | RIGHT_OPTION_MASK,
        ),
        (
            modifiers.shift.is_some(),
            SHIFT_KEY_MASK | LEFT_SHIFT_MASK | RIGHT_SHIFT_MASK,
        ),
        (
            modifiers.command.is_some(),
            COMMAND_KEY_MASK | LEFT_COMMAND_MASK | RIGHT_COMMAND_MASK,
        ),
        (modifiers.function, FUNCTION_KEY_MASK),
    ]
    .into_iter()
    .filter(|(held, _)| *held)
    .fold(0, |mask, (_, bits)| mask | bits)
}

impl RuntimeHotkeys {
    /// A press of the stop shortcut, counting it beside a recording
    /// shortcut whose modifiers are still held: any dictation shortcut,
    /// Labs › More dictation shortcuts' rows included.
    pub fn stop_pressed(self, code: u16, flags: u64) -> bool {
        self.stop.is_some_and(|stop| {
            [
                Some(self.dictation),
                self.raw_dictation,
                self.second_dictation,
            ]
            .into_iter()
            .chain(self.extra_dictation)
            .flatten()
            .any(|held| stop.matches_key_press_beside(code, flags, held))
        })
    }
}
