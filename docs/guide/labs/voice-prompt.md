---
feature: voice-prompt
key: labs.voice_prompt
title: Voice prompt
for: []
ask: Give one dictation a spoken instruction, like "rewrite this, make it formal"?
group: Output
os: every OS
vibehex: false
---

# Voice prompt

**What it does.** Lets you tell the AI what to do with one dictation, out loud. Say your text, then "rewrite this" and an instruction: "we ship Friday. Rewrite this, make it formal." The text before the phrase is rewritten as asked. With only an instruction, it writes what you asked for; with nothing after the phrase, it rewrites the text clearly.

**Turn it on when.** You often want a quick one-off rewrite, translation or reply without changing your settings.

**Leave it off when.** You never rewrite with AI, or "rewrite this" is something you dictate.

**Needs.** A running OpenCode: the instruction goes to the mode's OpenCode model and replaces its rewrite for that dictation. It works only through OpenCode today, so VibeHex keeps this lab out of stable builds for now.

**Good to know.** The phrase is "rewrite this" by default; its card's Phrase field changes it (at least two words and six letters). Only the last time you say it counts. Saying just the phrase pastes nothing. If the request fails, the text before the phrase pastes. A dictation without the phrase is untouched.

**Where.** Labs › Voice prompt (`labs.voice_prompt`); Phrase (`labs.voice_prompt_phrase`).

**Asked as.** "Tell the AI what to do while dictating"; "Rewrite this dictation formally"; «Голосовая команда для переписывания текста»; «Сказать „перепиши это“ в диктовке».

<!-- sources: upstream docs/features/labs.md:86-96, 914-1003; src/labs/card.rs:80, 278-280; src/control/settings/registry.rs:668-677; src/labs/catalog.rs (EVERY); src/labs/group.rs (Output); hidden from stable builds: owner's decision for this merge -->
