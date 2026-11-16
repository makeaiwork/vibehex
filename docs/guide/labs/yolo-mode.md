---
feature: yolo-mode
key: labs.yolo_mode
title: YOLO mode
for: [dictation, code]
ask: Press Return by itself after a long dictation, sending the message?
now: After a dictation is pasted, sending the message still takes Return from you.
with: Hold the shortcut 2 seconds or more and the message is sent after pasting; on a Mac, Esc right after skips sending.
group: Output
os: every OS
vibehex: false
---

# YOLO mode

**What it does.** Sends dictations automatically. When you held the dictation shortcut at least a chosen time (2 seconds by default), VibeHex pastes the text and then presses Return, so a chat message or an AI agent's prompt goes out on its own. Shorter dictations paste as usual.

**Turn it on when.** You dictate messages or prompts and always press Return right after.

**Leave it off when.** You check text before sending it. A misheard word goes out too.

**Needs.** Nothing else. Its card's Longer than picks 1, 2, 3, 5 or 10 seconds, and Only in limits it to apps you choose (empty means every app).

**Good to know.** On a Mac, pressing Escape just after you let go pastes without sending. A dictation finished with the Stop shortcut is sent by the same rule as one finished with the dictation shortcut. It never sends from Dictate without processing, Copy dictation only, Paste last dictation, `hex dictation` commands, the local API, meetings or Annotate. History marks sent dictations as Send. With apps chosen and the app unknown (some Linux desktops), it never sends.

**Where.** Labs › YOLO mode (`labs.yolo_mode`); Longer than (`labs.yolo_mode_seconds`); Only in (`labs.yolo_mode_apps`).

**Asked as.** "Send the message automatically after dictating"; "Press Enter after paste"; «Автоматически отправлять сообщение после диктовки»; «Нажимать Enter после вставки».

<!-- sources: src/labs/card.rs:84,299; src/labs/catalog.rs:333,506; src/labs/group.rs:108; src/control/settings/registry.rs:604-618; src/app_settings/labs.rs:301,305; src/dictation/yolo_mode.rs:1-25; src/rune_ui/labs/yolo_mode.rs:33-108; docs/features/labs.md (YOLO mode section) -->
