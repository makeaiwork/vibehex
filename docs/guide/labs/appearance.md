---
feature: appearance
key: labs.appearance
title: Appearance
for: []
ask: Keep VibeHex's window light or dark, whatever the system uses?
group: Dictation
os: every OS
vibehex: false
---

# Appearance

**What it does.** VibeHex's window follows your system's light or dark look. With this on, its card shows Theme: System, Light mode or Dark mode. Light and Dark keep the main window that way whatever the system does.

**Turn it on when.** You want a dark VibeHex on a light system, or the other way round.

**Leave it off when.** Following the system suits you. With Theme on System, nothing changes.

**Needs.** Nothing else.

**Good to know.** Only the main window changes. The dictation indicator and the meeting recording pill always stay dark, and other floating windows (capsules, live text, screenshots) follow the system. On Windows the title bar follows the window too; Windows 10 keeps its own caption background. The choice saves at once.

**Where.** Labs › Appearance (`labs.appearance`); Theme (`labs.appearance_theme`).

**Asked as.** "Dark mode for VibeHex only"; "The window ignores my theme"; «Тёмная тема только для VibeHex»; «Как сделать окно светлым».

<!-- sources: upstream docs/features/labs.md:60, 700-730; docs/features/README.md:42-45; src/labs/card.rs:58, 216; src/control/settings/registry.rs:517-520; src/labs/catalog.rs (EVERY); src/labs/group.rs (Dictation) -->
