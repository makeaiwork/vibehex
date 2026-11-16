---
feature: api-timestamps
key: labs.api_timestamps
title: Timestamps in API transcripts
for: []
ask: Let apps ask the local API for word and segment times?
group: Output
os: every OS
vibehex: false
---

# Timestamps in API transcripts

**What it does.** Apps and scripts that send audio to VibeHex's local API get plain text back. With this on, they can also ask for each segment's or each word's start and end time, as the speech model heard them, for subtitles or for jumping to a spot in the audio.

**Turn it on when.** A tool of yours transcribes through the local API and needs timings.

**Leave it off when.** You don't use the local API. Off, a request for timings is refused and answers stay as before.

**Needs.** An app or script that uses the local API with its token. Word times need a model that makes them; Apple Speech gives none, and a model that only knows segments refuses words rather than guessing.

**Good to know.** It covers sending a whole file for transcription, not live streams or dictations through the API. With processed output on (Labs › Process API transcripts), the times describe the raw transcript. A helper service started without the app reads the switch when it starts.

**Where.** Labs › Timestamps in API transcripts (`labs.api_timestamps`).

**Asked as.** "Word timestamps from the API"; "Get segment times for subtitles"; «Таймкоды слов через API»; «Время начала и конца фраз в расшифровке».

<!-- sources: upstream docs/features/labs.md:139-145, 2915-2942; src/labs/card.rs:73, 257-259; src/labs/catalog.rs (EVERY); src/labs/group.rs (Output) -->
