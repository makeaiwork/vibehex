---
feature: double-tap-modifiers
key: labs.double_tap_modifiers
title: Double-tap on a lone modifier
for: []
ask: Start hands-free dictation by tapping Option or Globe twice?
then: Choose Double-tap in General › Shortcut mode
group: Shortcuts
os: macOS
vibehex: false
---

# Double-tap on a lone modifier

**What it does.** Double-tap shortcut mode normally needs a shortcut with a key, such as Option+Space. With this on, it also takes a shortcut of modifiers alone, such as Option, Globe or Control+Option: tap it twice quickly to start hands-free dictation, and press it again to finish.

**Turn it on when.** You want hands-free dictation on a single key you can find blind, without holding it.

**Leave it off when.** You use that modifier's own double-press for something else, such as macOS's "press Globe twice" Dictation.

**Needs.** macOS, and General › Shortcut mode set to Double-tap.

**Good to know.** One tap or a hold does nothing; only two complete taps within about a third of a second start it. Any other key between the taps cancels it. The taps still reach the app in front, so macOS's own Globe shortcut may fire too. It also covers the second and extra dictation shortcuts. Off, a modifier-only shortcut falls back from Double-tap as before.

**Where.** Labs › Double-tap on a lone modifier (`labs.double_tap_modifiers`). Related: General › Shortcut mode (`double_tap_only`).

**Asked as.** "Double-tap Option to dictate"; "Hands-free with just the Globe key"; «Двойное нажатие Option для диктовки»; «Диктовка без удержания по двойному Fn».

<!-- sources: upstream docs/features/labs.md:120, 1507-1535; docs/features/README.md:841-843; src/labs/card.rs:41, 166-168; src/labs/catalog.rs (MACOS); src/labs/group.rs (Shortcuts) -->
