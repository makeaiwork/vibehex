---
feature: clean-up-apple-intelligence
key: labs.clean_up_with_apple_intelligence
title: Clean up with Apple Intelligence
for: []
ask: Fix misheard words with Apple Intelligence on this Mac?
group: Output
os: macOS
vibehex: false
---

# Clean up with Apple Intelligence

**What it does.** Gives each dictation one short pass with Apple Intelligence on this Mac. It fixes clearly misheard words from the sentence's context and resolves spoken self-corrections: "Tuesday, no wait, Wednesday" becomes "Wednesday". Otherwise your wording, language, numbers and negations stay.

**Turn it on when.** You want small fixes without sending text anywhere.

**Leave it off when.** You already clean up with AI, or want text exactly as heard.

**Needs.** macOS with Apple Intelligence available and switched on; the card says why when it can't run. It takes the place of the OpenCode rewrite only, so a mode that rewrites with OpenCode keeps its own model, and VibeHex keeps this lab out of stable builds for now.

**Good to know.** Nothing leaves the Mac. The pass has 2 seconds; if Apple Intelligence is unavailable, slow, refuses, or changes too much, the corrected text pastes as it was. A dictation of only numbers is left alone. History's Processing line shows when it ran or why it fell back.

**Where.** Labs › Clean up with Apple Intelligence (`labs.clean_up_with_apple_intelligence`).

**Asked as.** "Fix misheard words on device"; "Remove my 'no wait' corrections"; «Исправлять ошибки распознавания без интернета»; «Убирать оговорки вроде „нет, подожди“».

<!-- sources: src/labs/card.rs:33,155-157; src/labs/catalog.rs:287,458; src/labs/group.rs:94; src/labs/vibehex.rs:553-564; src/control/settings/registry.rs:386-390; src/dictation/apple_cleanup.rs:22,135; src/rune_ui/labs.rs:654-665; docs/features/labs.md:1158-1200 -->
