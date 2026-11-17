---
feature: speakers-only
key: labs.speakers_only
title: Only on speakers
for: []
ask: Mute or lower other sound only when it plays through speakers?
group: Dictation
os: every OS
vibehex: false
---

# Only on speakers

**What it does.** General › While dictating can mute, lower or pause other sound so it doesn't get into your dictation. Through headphones that sound can't reach the microphone anyway. With this on, While dictating applies only when sound plays through speakers; with wired or Bluetooth headphones, your music and calls are left alone.

**Turn it on when.** You set While dictating to Mute, Lower audio or Pause media and often wear headphones.

**Leave it off when.** While dictating is Do nothing, or you want sound lowered on headphones too.

**Needs.** While dictating set to something other than Do nothing.

**Good to know.** It decides as each dictation or screen capture starts. Anything VibeHex can't tell, such as USB or HDMI outputs, counts as speakers. Bluetooth speakers may read as headphones (on a Mac always), so nothing is lowered for them.

**Where.** Labs › Only on speakers (`labs.speakers_only`). Related: General › While dictating (`recording_audio_behavior`).

**Asked as.** "Don't mute my music in headphones"; "Lower audio only on speakers"; «Не глушить музыку в наушниках»; «Приглушать звук только через колонки».

<!-- sources: src/labs/card.rs:75,272-274; src/labs/catalog.rs:331,500; src/labs/group.rs:83; src/control/settings/registry.rs:140-143,644-649; src/app_settings.rs:233-235; src/rune_ui/general.rs:890; docs/features/labs.md (Only on speakers section) -->
