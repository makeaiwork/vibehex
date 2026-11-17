---
feature: microphone-key
key: labs.microphone_key
title: Microphone key alone
for: [dictation]
ask: Dictate with the microphone key on your Mac laptop?
then: Record the mic key in General › Shortcut
now: VibeHex refuses the microphone key on a Mac laptop as an unsupported key, so it cannot start dictation.
with: The microphone key can be your dictation key; while a shortcut uses it, VibeHex takes it from other apps.
group: Shortcuts
os: macOS
vibehex: false
---

# Microphone key alone

**What it does.** Many Mac laptops have a microphone key in F5's spot. VibeHex normally refuses it as "Unsupported key". With this on, you can record it as a shortcut on its own, for example as the dictation key.

**Turn it on when.** You want the key labelled with a microphone to start VibeHex dictation.

**Leave it off when.** You use macOS's own Dictation on that key, or your keyboard has no such key.

**Needs.** macOS and a keyboard with the microphone key. After turning it on, record the key in General › Shortcut (or another shortcut row).

**Good to know.** Once a VibeHex shortcut uses the key, VibeHex takes it from other apps; while it isn't bound, or while VibeHex is paused, macOS's Dictation keeps it. If macOS's Dictation still opens, change its shortcut in System Settings › Keyboard › Dictation. It can't be a Screenshots shortcut. Turning the lab off puts any shortcut on that key back to its default.

**Where.** Labs › Microphone key alone (`labs.microphone_key`). Related: General › Shortcut (`dictation_hotkey`).

**Asked as.** "Use the F5 microphone key"; "Dictate with the mic key on my MacBook"; «Диктовать клавишей с микрофоном»; «Назначить клавишу F5 с микрофоном».

<!-- sources: src/labs/card.rs:55,220-222; src/labs/description.rs:125-127; src/labs/catalog.rs:261,480; src/labs/group.rs:137; src/control/settings/registry.rs:170,462-467; src/rune_ui/labs.rs:629-634; src/rune_ui/general.rs:50-60; docs/features/labs.md (Microphone key alone section) -->
