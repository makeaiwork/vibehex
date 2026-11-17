---
feature: screen-vocabulary
key: labs.screen_vocabulary
keys: [labs.screen_vocabulary, labs.screen_vocabulary_cleanup]
title: Screen vocabulary
for: [code]
ask: Let names visible in the window help spell your dictation?
now: VibeHex does not look at the window you dictate into, so names on screen can come out misspelled.
with: Names and terms from the window you dictate into help spell that dictation; History keeps them and password fields are skipped.
group: Dictation
os: macOS
vibehex: false
---

# Screen vocabulary

**What it does.** Names and terms on screen, such as a colleague's name in a chat or a function name in your editor, often come out misspelled. With this on, each dictation reads the window you dictate into and uses up to 24 names and technical terms from it as extra vocabulary for that one dictation.

**Turn it on when.** You dictate about what you are looking at: code, tickets, chats full of names and product terms.

**Leave it off when.** Your windows hold nothing like that, or a slightly slower finish bothers you.

**Needs.** macOS and the Accessibility permission VibeHex already has. Parakeet models use the terms only with Labs › Vocabulary biasing on; Whisper and Qwen3-ASR take them as a hint; GigaAM, Apple Speech and models without vocabulary ignore them. With **Also give them to AI cleanup** on, AI cleanup gets the names too, so the lab also helps with GigaAM, Apple Speech and Parakeet without biasing (AI cleanup has to be on).

**Good to know.** The window is read on this Mac; password fields and VibeHex's own window are skipped. The names read for a dictation are kept with it in History on this Mac, under **What VibeHex read** (the ones AI cleanup was given marked), and are removed with the entry; keeping them sends them nowhere. A dictation that found terms for a speech model that uses them is transcribed after you let go rather than while you speak, which costs a little time. Meetings and the local API never see the terms. AI cleanup sees them only with that switch on (off by default): with a cloud engine (Codex, OpenRouter, or an online OpenCode model) the names are sent to it with the dictation. Cleanup only corrects a name you said; one it writes where you said nothing like it is undone. With that switch on and a cloud cleanup engine (Codex, OpenRouter, a server that is not this Mac), the screen's names are sent to it with each dictation; keeping cleanup on this Mac keeps them here. A name shaped like a key or a token is never given to cleanup.

**Where.** Labs › Screen vocabulary (`labs.screen_vocabulary`); its switch Also give them to AI cleanup shows on the card while it is on (`labs.screen_vocabulary_cleanup`).

**Asked as.** "It misspells names that are right there on screen"; "Use the words in my editor"; «Пусть берёт имена из окна»; «Неправильно пишет названия, которые видны на экране».

<!-- sources: src/labs/card.rs:19,119-122; src/labs/description.rs:25-28; src/labs/catalog.rs:210-213,287,448; src/labs/group.rs:72; src/labs/vibehex.rs:188-196,271-277; src/control/settings/registry.rs:321-332; src/control/settings/requires.rs (labs.screen_vocabulary_cleanup); src/transcription/models.rs:355-383,1624; src/parakeet.rs:456-472; src/dictation/screen_vocabulary.rs:127-139,209-214; src/dictation/screen_vocabulary/cleanup.rs; src/dictation/processor.rs:513-525,540-557,1310; src/rune_ui/labs/screen_vocabulary.rs; src/history/read.rs; src/rune_ui/history/read.rs; docs/features/labs.md (Screen vocabulary section); docs/features/README.md (History) -->
