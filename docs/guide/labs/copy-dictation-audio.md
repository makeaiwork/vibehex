---
feature: copy-dictation-audio
key: labs.copy_dictation_audio
title: Copy dictation audio
for: []
ask: Copy a dictation's kept recording as a file to attach?
group: History
os: every OS
vibehex: false
---

# Copy dictation audio

**What it does.** Puts a dictation's recording on the clipboard as one WAV file, ready to attach in a chat, an email or an AI agent. History entries with a kept recording get Copy audio beside Play, and the menu bar (or tray) gets Copy last dictation audio.

**Turn it on when.** You want to send what you actually said, for example to show a misheard word.

**Leave it off when.** You don't keep recordings, or never share them.

**Needs.** Labs › Keep recordings (Keep audio too in History) on as well; this lab keeps no audio of its own. Only dictations recorded since then have a file.

**Good to know.** The menu item copies the newest entry that still has its audio, and is dimmed when there is none. In History the button shows Copying…, then Copied, or why it couldn't. Turning either lab off removes the buttons.

**Where.** Labs › Copy dictation audio (`labs.copy_dictation_audio`). Related: Labs › Keep recordings (`labs.keep_recordings`).

**Asked as.** "Copy the audio of my dictation"; "Attach the recording to a message"; «Скопировать аудио диктовки»; «Отправить запись диктовки файлом».

<!-- sources: src/labs/card.rs:85,300-302; src/labs/catalog.rs:334,510; src/labs/group.rs:117; src/control/settings/registry.rs:468-477; src/rune_ui/history.rs:898-913,1185-1205; src/macos_rune/status_item.rs:332; docs/features/labs.md (Copy dictation audio section) -->
