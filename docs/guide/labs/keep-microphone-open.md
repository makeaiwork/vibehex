---
feature: keep-microphone-open
key: labs.keep_microphone_open
title: Keep mic open after dictating
for: []
ask: Keep the microphone open a while after each dictation?
group: Dictation
os: every OS
vibehex: false
---

# Keep mic open after dictating

**What it does.** With Turn off the mic when not dictating on, the microphone closes the moment a dictation ends, so the next one starts cold and can lose its first words. With this lab on, the microphone stays open for a while after each dictation, so a dictation soon after starts at once and keeps the moment before your press.

**Turn it on when.** You keep the microphone off between dictations but often dictate several times in a row.

**Leave it off when.** Turn off the mic when not dictating is off: the microphone is then always open and this does nothing.

**Needs.** General's Turn off the mic when not dictating on. Its card's Keep open for picks 30 seconds, 1, 3, 5 or 30 minutes (1 minute by default).

**Good to know.** While it stays open, your system shows the microphone in use. Turning the lab off closes it at the next idle moment. Annotate captures and local API dictations count as dictations too.

**Where.** Labs › Keep mic open after dictating (`labs.keep_microphone_open`); Keep open for (`labs.keep_microphone_open_seconds`). Related: `release_microphone_while_idle`.

**Asked as.** "The first word is cut when I dictate again"; "Keep the mic warm between dictations"; «Микрофон каждый раз включается заново»; «Обрезается начало следующей диктовки».

<!-- sources: src/labs/card.rs:27,139-141; src/labs/catalog.rs:308,452; src/labs/group.rs:74; src/control/settings/registry.rs:102,346-355; src/app_settings/labs.rs:298,308; src/app_settings.rs:461; src/rune_ui/labs.rs:1002-1015,1121-1150; src/rune_ui/general.rs:254-258; docs/features/labs.md (Keep mic open section) -->
