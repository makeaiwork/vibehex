---
feature: cursor-context
key: labs.cursor_context
keys: [labs.cursor_context, labs.cursor_context_cloud]
title: Rewrite reads nearby text
for: [dictation, cleanup]
ask: Let AI cleanup read the text around your cursor?
now: AI cleanup sees only what you said, so it can misspell names already in your document.
with: AI cleanup also reads the text around your cursor to spell names right; a cloud engine gets it only if you allow.
group: Output
os: macOS
vibehex: false
---

# Rewrite reads nearby text

**What it does.** AI cleanup sees only what you said, so it can misspell names already in your document or fit badly where it pastes. With this on, each dictation reads up to about 1,000 characters before and after the cursor (and the selection) and gives them to AI cleanup as context, never as text to copy.

**Where the text goes.** It works with every cleanup engine. With the built-in model, or your own server at this Mac's address (localhost, 127.0.0.1 or ::1), the text stays on this Mac and nothing more is asked. Codex, OpenRouter, or your own server at any other address get it only with **Send it to cloud engines** on, a switch on the card (off by default); while it is off, the text is not even read for them, and the card says so. With OpenCode it goes to the mode's OpenCode model, which may be a cloud provider. The card's first line names where it goes with the engine you chose in Output › AI cleanup.

**Turn it on when.** You clean up with AI and dictate into documents, emails or code full of names and terms.

**Leave it off when.** Your fields hold secrets. Terminals are read like any other app: a terminal shows its whole scrollback, so tokens printed there can go along.

**Needs.** macOS, the Accessibility permission, and Clean up with AI on (or a mode that rewrites).

**Good to know.** Password fields are never read. Nothing read is saved, logged or kept in History. A very short dictation or a slow app may send no nearby text. A long dictation cleaned in pieces gives the text before the cursor to its first piece and the text after it to its last. Off, cleanup gets exactly what it got before. Assistant never turns on Send it to cloud engines for you.

**Where.** Labs › Rewrite reads nearby text (`labs.cursor_context`); its switch Send it to cloud engines shows on the card while it is on (`labs.cursor_context_cloud`).

**Asked as.** "Let the AI see what I'm typing in"; "Cleanup misspells names from my document"; "Use the text around my cursor"; «Пусть ИИ учитывает текст вокруг курсора»; «Очистка не знает имён из документа».

<!-- sources: src/labs/card.rs:81,290-292; src/labs/description.rs:197-200; src/labs/catalog.rs (labs.cursor_context_cloud); src/labs/group.rs:109; src/labs/vibehex.rs (vibehex_hides, CURSOR_CONTEXT_CLOUD, cursor_context_cloud); src/control/settings/registry.rs (labs.cursor_context); src/control/settings/vibehex_keys.rs (labs.cursor_context_cloud); src/control/settings/requires.rs; src/rune_ui/labs.rs (CursorContext row); src/rune_ui/labs/cursor_context.rs; src/dictation/caret_text.rs (CaretTextRead::start); src/dictation/caret_prompt/cleanup.rs; src/dictation/processor.rs (prompt, prompt_parts, process_cancellable_with); src/dictation/cleanup_pieces.rs (Piece); src/assistant/changes/catalog.rs (NEVER); docs/features/labs.md (Rewrite reads nearby text section) -->
