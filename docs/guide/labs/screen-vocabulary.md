---
feature: screen-vocabulary
key: labs.screen_vocabulary
title: Screen vocabulary
for: [code]
ask: Let names visible in the window help spell your dictation?
group: Dictation
os: macOS
vibehex: false
---

# Screen vocabulary

**What it does.** Names and terms on screen, such as a colleague's name in a chat or a function name in your editor, often come out misspelled. With this on, each dictation reads the window you dictate into and uses up to 24 names and technical terms from it as extra vocabulary for that one dictation.

**Turn it on when.** You dictate about what you are looking at: code, tickets, chats full of names and product terms.

**Leave it off when.** Your windows hold nothing like that, or a slightly slower finish bothers you.

**Needs.** macOS and the Accessibility permission VibeHex already has. Parakeet models use the terms only with Labs › Vocabulary biasing on; Whisper and Qwen3-ASR take them as a hint; GigaAM, Apple Speech and models without vocabulary ignore them.

**Good to know.** The window is read on this Mac and nothing is saved; password fields and VibeHex's own window are skipped. A dictation that found terms is transcribed after you let go rather than while you speak, which costs a little time. Meetings, AI cleanup and the local API never see the terms.

**Where.** Labs › Screen vocabulary (`labs.screen_vocabulary`).

**Asked as.** "It misspells names that are right there on screen"; "Use the words in my editor"; «Пусть берёт имена из окна»; «Неправильно пишет названия, которые видны на экране».

<!-- sources: src/labs/card.rs:19,119-121; src/labs/description.rs:25-27; src/labs/catalog.rs:283,444; src/labs/group.rs:72; src/control/settings/registry.rs:321-325; src/transcription/models.rs:355-383,1624; src/parakeet.rs:456-472; docs/features/labs.md (Screen vocabulary section) -->
