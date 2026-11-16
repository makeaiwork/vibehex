---
feature: app-model-switching
key: labs.app_model_switching
title: Switch model with app
for: []
ask: Switch the speech model by itself when you change apps?
group: Models
os: [macOS, Windows]
vibehex: false
---

# Switch model with app

**What it does.** Lets each dictation mode, Global included, name a model and a language. When you bring another app to the front, VibeHex switches to the model of the mode that app uses, about 0.3 seconds later.

**Turn it on when.** You dictate in different languages or need a different model in different apps, for example one language in a messenger and another in your editor.

**Leave it off when.** One model serves you everywhere.

**Needs.** macOS or Windows, and models already downloaded: a switch never downloads one. In Hex the choice is a Speech model row in the Modes pane, which VibeHex doesn't show, so for now set it through Output › Edit with agent or the `hex settings` command.

**Good to know.** Apps are matched by application only; website rules are ignored. VibeHex's own window is ignored. A switch due while you dictate waits until the dictation finishes. Choosing a model by hand cancels a pending switch. With Switch model with keyboard language also on, whichever switch comes later wins. Linux doesn't have it.

**Where.** Labs › Switch model with app (`labs.app_model_switching`).

**Asked as.** "Different model for each app"; "Change dictation language in Telegram automatically"; «Своя модель распознавания для каждого приложения»; «Автоматически менять язык диктовки по приложению».

<!-- sources: upstream docs/features/labs.md:68-72, 503-540; docs/features/README.md:722-795; src/rune_ui/modes.rs:879-950 (speech_row in the Modes pane); VibeHex hides Modes (docs/guide/topics/getting-started.md); src/labs/catalog.rs (MACOS_WINDOWS); src/labs/group.rs (Models) -->
