---
feature: history-multi-select
key: labs.history_multi_select
title: Select multiple dictations
for: [dictation]
ask: Select several dictations in History to copy or delete together?
group: History
os: every OS
vibehex: false
---

# Select multiple dictations

**What it does.** History normally selects one dictation at a time. With this on, Shift-click selects a range, Cmd-click (Ctrl on Windows and Linux) adds or removes one, and dragging across rows selects them all. With two or more selected, the card reads how many and offers Copy and Delete….

**Turn it on when.** You collect several dictations into one text, or tidy History in batches.

**Leave it off when.** You only ever look at one dictation at a time. Nothing to lose otherwise.

**Needs.** Nothing else.

**Good to know.** Copy joins the texts oldest first, with a blank line between them. Delete… asks once before removing them. A search, new dictations or a delete trims the selection to the rows still shown, and turning the lab off leaves one entry selected.

**Where.** Labs › Select multiple dictations (`labs.history_multi_select`).

**Asked as.** "Copy several dictations at once"; "Delete many history entries"; «Выделить несколько диктовок»; «Удалить сразу несколько записей из истории».

<!-- sources: upstream docs/features/labs.md:128-129, 1885-1916; docs/features/README.md:912-916; src/labs/card.rs:46, 184-186; src/labs/catalog.rs (EVERY); src/labs/group.rs (History) -->
