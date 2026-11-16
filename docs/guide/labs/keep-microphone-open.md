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

<!-- sources: upstream docs/features/labs.md:78-80, 584-640; src/labs/card.rs:27, 128-130; src/control/settings/registry.rs:351-354 (30/60/180/300/1800); src/app_settings.rs:441 (release_microphone_while_idle false); src/labs/catalog.rs (EVERY) -->
