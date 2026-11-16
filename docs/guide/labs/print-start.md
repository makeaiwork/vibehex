---
feature: print-start
key: labs.print_start
title: Print-only CLI dictation
for: []
ask: Let a terminal command start a dictation and get its text back?
group: Output
os: macOS
vibehex: false
---

# Print-only CLI dictation

**What it does.** Lets a script or AI agent run `hex dictation start --print`: VibeHex starts recording, and when the recording ends, however it ends, the transcript goes back to that command instead of being pasted. You can finish with your usual shortcut.

**Turn it on when.** A tool of yours should ask you something by voice and read your answer from the terminal.

**Leave it off when.** You don't script VibeHex. Off, the command answers that the lab is off.

**Needs.** macOS and VibeHex running. Linux and Windows answer that they don't support it.

**Good to know.** Nothing is pasted or copied; History keeps the dictation as Printed and Paste last dictation skips it. A start while anything else records is refused as busy. Cancelling answers cancelled, and silence answers that no speech was heard. The command waits at most 31 minutes.

**Where.** Labs › Print-only CLI dictation (`labs.print_start`).

**Asked as.** "Get dictated text in a script"; "hex dictation start --print says disabled"; «Получить надиктованный текст в терминале»; «Диктовка для скрипта без вставки».

<!-- sources: src/labs/card.rs:79,284-286; src/labs/catalog.rs:282,502; src/labs/group.rs:106; src/control/settings/registry.rs:663-668; src/control.rs:414; src/rune_ui/labs.rs:576-581; docs/features/labs.md (Print-only CLI dictation section) -->
