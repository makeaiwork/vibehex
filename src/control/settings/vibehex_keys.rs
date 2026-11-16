//! `hex settings` keys for VibeHex's own Labs and their shortcuts and
//! cleanup server, kept apart from Hex's registry (`registry.rs`) so an
//! upstream sync leaves them alone. AI cleanup on Codex, meeting notes,
//! Files, Assistant, and the sidebar have their own `*_keys.rs`.

use super::{Key, Kind, key, optional};
use crate::app_settings::HotkeySlot;

pub(super) const VIBEHEX_KEYS: &[Key] = &[
    optional(
        "vocabulary_add_hotkey",
        Kind::Shortcut(HotkeySlot::VocabularyAdd),
        "Labs: adds the selected misheard word to Vocabulary (macOS, with \
         labs.vocabulary_quick_add); option+shift+a by default, none turns it off",
    ),
    key(
        "stop_hotkey",
        Kind::Shortcut(HotkeySlot::Stop),
        "Labs: finishes and pastes a recording a shortcut started, in any mode, Return by \
         default (macOS, with labs.stop_shortcut). Hex takes the key only while one runs",
    ),
    key(
        "dictation_processing.server.runs_on",
        Kind::Choice(&["opencode", "openrouter", "custom", "local", "codex"]),
        "Where dictation cleanup runs, with labs.ai_cleanup_server: OpenCode, OpenRouter, your own OpenAI-compatible server, a model Hex downloads (local, macOS), or the Codex CLI on your ChatGPT plan (codex)",
    ),
    key(
        "dictation_processing.server.url",
        Kind::Text,
        "Your own server's OpenAI-compatible base URL, such as http://127.0.0.1:1234/v1 for LM Studio",
    ),
    key(
        "dictation_processing.server.openrouter_model",
        Kind::Text,
        "The OpenRouter model dictation cleanup uses, such as google/gemini-2.5-flash-lite",
    ),
    key(
        "dictation_processing.server.custom_model",
        Kind::Text,
        "The model your own server runs dictation cleanup with, as it names it",
    ),
    key(
        "dictation_processing.server.local_model",
        Kind::Text,
        "The downloaded model local cleanup runs, such as qwen3-4b-instruct-2507; empty for the recommended one",
    ),
    key(
        "labs.ai_cleanup_server",
        Kind::Bool,
        "Labs: dictation cleanup may run on OpenRouter or your own OpenAI-compatible server (dictation_processing.server) instead of OpenCode",
    ),
    key(
        "labs.vocabulary_paste_list",
        Kind::Bool,
        "Labs: the Vocabulary sheet takes many terms at once, one per line as Term | sounds like, sounds like",
    ),
    key(
        "labs.russian_vibe_coding",
        Kind::Bool,
        "Labs: turning it on adds the Russian vibe-coding vocabulary, Global's word fixes and AI cleanup instructions, and turns on Clean up with AI and every other Labs feature VibeHex added that this Hex has except labs.stop_shortcut; off takes back the terms, fixes, and instructions",
    ),
    key(
        "labs.vocabulary_quick_add",
        Kind::Bool,
        "Labs: a shortcut (vocabulary_add_hotkey) adds a selected misheard word to Vocabulary with its right spelling and replaces the selection where it can (macOS)",
    ),
    key(
        "labs.cleanup_progress",
        Kind::Bool,
        "Labs: while AI cleanup takes a while, a bar beside the HUD shows its progress with Paste now, which pastes the part already cleaned and the rest as transcribed (macOS)",
    ),
    key(
        "labs.stop_shortcut",
        Kind::Bool,
        "Labs: stop_hotkey finishes and pastes a recording a shortcut started (macOS)",
    ),
    key(
        "labs.cursor_context_cloud",
        Kind::Bool,
        "Labs: while labs.cursor_context is on, the text around the cursor goes to a cloud \
         cleanup engine too (Codex, OpenRouter, or your own server at an address that is not \
         this computer); off, it is used only by the built-in model, a server on this computer, \
         or OpenCode, and not read for a cloud engine (macOS)",
    ),
    key(
        "labs.lower_every_app",
        Kind::Bool,
        "Labs: with recording_audio_behavior mute or duck and an output macOS can't set the volume of (a DisplayPort or HDMI display), every other app is turned down through a muting system audio tap (macOS 14.2+, needs system audio access)",
    ),
];
