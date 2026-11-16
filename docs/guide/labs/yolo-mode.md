---
feature: yolo-mode
key: labs.yolo_mode
title: YOLO mode
for: []
ask: Press Return by itself after a long dictation, sending the message?
group: Output
os: every OS
vibehex: false
---

# YOLO mode

**What it does.** Sends dictations automatically. When you held the dictation shortcut at least a chosen time (2 seconds by default), VibeHex pastes the text and then presses Return, so a chat message or an AI agent's prompt goes out on its own. Shorter dictations paste as usual.

**Turn it on when.** You dictate messages or prompts and always press Return right after.

**Leave it off when.** You check text before sending it. A misheard word goes out too.

**Needs.** Nothing else. Its card's Longer than picks 1, 2, 3, 5 or 10 seconds, and Only in limits it to apps you choose (empty means every app).

**Good to know.** On a Mac, pressing Escape just after you let go pastes without sending. It never sends from Copy dictation only, Paste last dictation, terminal commands, meetings or Annotate. History marks sent dictations as Send. With apps chosen and the app unknown (some Linux desktops), it never sends.

**Where.** Labs › YOLO mode (`labs.yolo_mode`); Longer than (`labs.yolo_mode_seconds`); Only in (`labs.yolo_mode_apps`).

**Asked as.** "Send the message automatically after dictating"; "Press Enter after paste"; «Автоматически отправлять сообщение после диктовки»; «Нажимать Enter после вставки».

<!-- sources: upstream docs/features/labs.md:83, 797-841; src/labs/card.rs:84, 290; src/control/settings/registry.rs:608-616; src/labs/catalog.rs (EVERY); src/labs/group.rs (Output) -->
