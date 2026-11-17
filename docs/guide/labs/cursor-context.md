---
feature: cursor-context
key: labs.cursor_context
title: Rewrite reads nearby text
for: []
ask: Let the AI rewrite read the text around your cursor?
group: Output
os: macOS
vibehex: false
---

# Rewrite reads nearby text

**What it does.** The AI rewrite sees only what you said, so it can misspell names already in your document or fit badly where it pastes. With this on, each dictation reads up to about 1,000 characters before and after the cursor (and the selection) and gives them to the rewrite as context, never as text to copy.

**Turn it on when.** You rewrite with AI and dictate into documents, emails or code full of names and terms.

**Leave it off when.** Your fields hold secrets. A terminal shows its whole scrollback, so tokens printed there can go along.

**Needs.** macOS, the Accessibility permission, and a mode that rewrites with OpenCode: that text goes to the mode's model, which may be a cloud provider. It works only through OpenCode today, so VibeHex keeps this lab out of stable builds for now.

**Good to know.** Password fields are never read. Nothing read is saved, logged or kept in History. A very short dictation or a slow app may send no nearby text. Off, the rewrite gets exactly what it got before.

**Where.** Labs › Rewrite reads nearby text (`labs.cursor_context`).

**Asked as.** "Let the AI see what I'm typing in"; "Rewrite misspells names from my document"; «Пусть ИИ учитывает текст вокруг курсора»; «Переписывание не знает имён из документа».

<!-- sources: src/labs/card.rs:81,290-292; src/labs/description.rs:197-199; src/labs/catalog.rs:284,507; src/labs/group.rs:109; src/labs/vibehex.rs:553-564; src/control/settings/registry.rs:326-330; src/rune_ui/labs.rs:672-680; src/dictation/caret_text.rs:1-25; docs/features/labs.md:488-520 -->
