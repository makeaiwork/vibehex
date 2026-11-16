---
feature: pause-menu
key: labs.pause_menu
title: Pause from menu bar
for: []
ask: Add Pause VibeHex to the menu bar, freeing its shortcut keys?
group: Shortcuts
os: macOS
vibehex: false
---

# Pause from menu bar

**What it does.** Adds Pause VibeHex to the menu bar menu. While paused, VibeHex ignores all its shortcuts, so another app, such as a game or a remote desktop, gets those keys. The item then reads Resume VibeHex.

**Turn it on when.** VibeHex's keys get in the way in another app now and then, and quitting VibeHex is too much.

**Leave it off when.** You never need the keys elsewhere.

**Needs.** macOS and the menu bar icon: with Labs › Hide menu bar icon on there is no Resume item, so turning this lab off is what resumes.

**Good to know.** Pausing cancels a dictation being recorded, but not a meeting recording. After Resume, shortcuts come back only once no key is held. `hex dictation` commands and the menu's own items still work while paused. A restart, or turning the lab off, always resumes.

**Where.** Labs › Pause from menu bar (`labs.pause_menu`).

**Asked as.** "Pause VibeHex temporarily"; "My game needs the Option key"; «Временно отключить горячие клавиши VibeHex»; «Поставить VibeHex на паузу».

<!-- sources: upstream docs/features/labs.md:191, 2290-2350; docs/features/README.md:1637-1640; src/labs/card.rs:77, 269-271; src/labs/catalog.rs (MACOS); src/labs/group.rs (Shortcuts) -->
