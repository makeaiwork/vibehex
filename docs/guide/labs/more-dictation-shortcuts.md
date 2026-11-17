---
feature: more-dictation-shortcuts
key: labs.more_dictation_shortcuts
title: More dictation shortcuts
for: []
ask: Add up to four more dictation shortcuts, each with its own mode?
then: Add them in General › Shortcut
group: Shortcuts
os: macOS
vibehex: false
---

# More dictation shortcuts

**What it does.** Adds up to four more shortcuts that dictate like the main one, each with its own shortcut mode: Hold or double-tap, Hold, Tap, or Double-tap. For example, Hold on one key and Tap for long hands-free dictation on another.

**Turn it on when.** You have keys to spare, such as on an external keyboard or a macro pad, and want different ways to start a dictation.

**Leave it off when.** One or two shortcuts are enough (Labs › Second dictation shortcut adds one that follows the main shortcut's mode).

**Needs.** macOS. While on, General shows a numbered row for each extra shortcut, with its recorder, its mode and Remove, plus Add shortcut while there are fewer than four. A new row starts empty.

**Good to know.** Each needs keys no other VibeHex shortcut uses. They paste with the same processing as the main shortcut. The shortcut that started a recording owns it. Escape cancels one once it is locked or in Tap mode. Turning the lab off keeps the rows saved but unused.

**Where.** Labs › More dictation shortcuts (`labs.more_dictation_shortcuts`); the rows (`extra_dictation_hotkeys`).

**Asked as.** "Several dictation keys with different modes"; "A tap key and a hold key"; «Несколько клавиш для диктовки»; «Одна клавиша удерживать, другая нажимать».

<!-- sources: src/labs/card.rs:76,275-277; src/labs/catalog.rs:285,509; src/labs/group.rs:130; src/control/settings/registry.rs:199-204,396-401; src/control/settings/requires.rs (extra_dictation_hotkeys); src/rune_ui/general/extra_shortcuts.rs:49-127; src/rune_ui/shortcut_mode.rs:57-63; docs/features/labs.md (More dictation shortcuts section) -->
