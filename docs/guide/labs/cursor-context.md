---
feature: cursor-context
key: labs.cursor_context
title: Rewrite reads nearby text
for: [dictation, cleanup]
ask: Let AI cleanup read the text around your cursor?
now: AI cleanup sees only what you said, so it can misspell names already in your document.
with: AI cleanup also reads the text around your cursor to spell names right, and a cloud engine receives that text.
group: Output
os: macOS
vibehex: reworked
---

# Rewrite reads nearby text

**What it does.** AI cleanup sees only what you said, so it can misspell names already in your document or fit badly where it pastes. With this on, each dictation reads up to about 1,000 characters before and after the cursor (and the selection) and gives them to AI cleanup as context, never as text to copy. VibeHex reworked Hex's version of this lab, which worked only with OpenCode: it works with every cleanup engine and cuts keys and tokens out of the text.

**Where the text goes.** It works with every cleanup engine, and goes wherever your cleanup runs: with the built-in model, or your own server at this Mac's address (localhost, 127.0.0.1 or ::1), it stays on this Mac; with Codex, OpenRouter, or a server that is not this Mac, it is sent to that engine with each dictation; with OpenCode it goes to the mode's OpenCode model, which may be a cloud provider. There is no separate switch for the cloud: if the text should not leave this Mac, run cleanup on the built-in model or a local server. The card's line names where it goes with the engine you chose in Output › AI cleanup.

**Turn it on when.** You clean up with AI and dictate into documents, emails or code full of names and terms.

**Leave it off when.** Your fields hold secrets in shapes VibeHex can't recognise, and your cleanup runs in the cloud. Terminals are read like any other app.

**Needs.** macOS, the Accessibility permission, and Clean up with AI on (or a mode that rewrites).

**Good to know.** Known key and token shapes are cut out before the text leaves the app it was read from: private key blocks, SSH keys, provider tokens (OpenAI, Anthropic, GitHub, GitLab, Slack, AWS, Google, Hugging Face, npm, PyPI, Stripe, OpenRouter), web tokens, Authorization headers, passwords in addresses, values of names like password, DB_PASS or api_key, and long random strings. That is best effort: anything else near the cursor may be sent. Password fields are never read, and a selection over about 2,000 characters skips the read. Nothing read is logged. The text read for a dictation (secrets already cut out) is kept with it in History on this Mac, under **What VibeHex read**, with what cleanup was given of it; it is removed with the entry, and keeping it sends it nowhere. Grey hint text in an empty field, such as a suggested message in an AI coding panel, is not text you wrote and is not read as text near the cursor. A very short dictation or a slow app may send no nearby text, and some apps, such as Sublime Text, give VibeHex no text near the cursor at all: History's **What VibeHex read** then says so. A long dictation cleaned in pieces gives the text before the cursor to its first piece and the text after it to its last. A model built into VibeHex gets only about the last 300 characters before the cursor (and the selection), nothing after it. If a model answers with the nearby text instead of what you said, VibeHex notices and asks it again without that text. Off, cleanup gets exactly what it got before.

**Where.** Labs › Rewrite reads nearby text (`labs.cursor_context`).

**Asked as.** "Let the AI see what I'm typing in"; "Cleanup misspells names from my document"; "Use the text around my cursor"; «Пусть ИИ учитывает текст вокруг курсора»; «Очистка не знает имён из документа».

<!-- sources: src/labs/card.rs:81,291-293; src/labs/description.rs:198-201; src/labs/group.rs:109; src/labs/vibehex.rs (vibehex_hides); src/control/settings/registry.rs (labs.cursor_context); src/rune_ui/labs.rs (CursorContext row); src/rune_ui/labs/cursor_context.rs; src/dictation/caret_text.rs (CaretTextRead::start, started); src/recognition/context_watch.rs (finish); src/dictation/caret_prompt/cleanup.rs; src/dictation/caret_prompt/secrets.rs; src/dictation/processor.rs (prompt, prompt_parts, process_cancellable_with); src/dictation/cleanup_pieces.rs (Piece); src/history/read.rs; src/rune_ui/history/read.rs; docs/features/labs.md (Rewrite reads nearby text section); docs/features/README.md (History) -->
