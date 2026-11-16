---
feature: return-to-app
key: labs.return_to_app
title: Return to app
for: [dictation]
ask: Paste into the app you started in, even if you switched?
now: If another app comes to the front while you dictate, the text lands wherever the cursor is when it's ready.
with: The text goes to the app you started in. If that app can't be brought back, nothing is pasted.
group: Output
os: macOS
vibehex: false
---

# Return to app

**What it does.** If another app comes to the front while you dictate, the text normally lands wherever the cursor is when it's ready. With this on, VibeHex switches back to the app you started in and pastes there.

**Turn it on when.** You start a dictation and then click elsewhere, or a notification or another window grabs focus before the text arrives.

**Leave it off when.** You like to start dictating, then move to where the text should go.

**Needs.** macOS. Nothing else.

**Good to know.** If the app has closed or VibeHex can't bring it back within half a second, nothing is pasted: you get the error tone, and Paste last dictation has the text. Such a dictation isn't saved to History. Paste last dictation, meetings and Labs › Copy dictation only never switch apps.

**Where.** Labs › Return to app (`labs.return_to_app`).

**Asked as.** "The text pasted into the wrong window"; "Paste back into the app I started in"; «Текст вставился не в то окно»; «Вставлять в приложение, где начал диктовку».

<!-- sources: src/labs/card.rs:30,146-148; src/labs/catalog.rs:289,455; src/labs/group.rs:97; src/control/settings/registry.rs:366-370; src/paste/return_to_app.rs:17-21; docs/features/labs.md (Return to app section) -->
