---
feature: copy-capture-zip
key: labs.copy_capture_zip
title: Copy captures as a zip
for: []
ask: Copy each finished Annotate capture as one small zip file?
group: Annotate
os: every OS
vibehex: false
---

# Copy captures as a zip

**What it does.** A finished Annotate capture is normally copied as its image and summary. With this on, it goes on the clipboard as one zip file instead, with the summary, the contact sheet and the keyframes, but without the video and audio, so it stays small enough to attach to a chat or an AI agent.

**Turn it on when.** You paste captures into apps that take a single file.

**Leave it off when.** The app you paste into needs the recording itself.

**Needs.** Annotate, in a build of VibeHex that has it.

**Good to know.** The zip is written inside the capture's own folder and deleted with it. The capsule's Paste and Send buttons are unchanged. Pasting into a terminal gives no text, since the clipboard holds only the file. If the zip can't be made, the capture is copied as before.

**Where.** Labs › Copy captures as a zip (`labs.copy_capture_zip`).

**Asked as.** "Copy the capture as one file"; "Attach an Annotate capture to a chat"; «Скопировать запись Annotate одним архивом»; «Захват слишком большой для чата».

<!-- sources: src/labs/card.rs:82,293-295; src/labs/catalog.rs:330,504; src/labs/group.rs:127; src/control/settings/registry.rs:543-547; src/capture_zip.rs:1-35; src/capture_host.rs:3377-3383,3509-3525; src/rune_ui/labs.rs:740-756; docs/features/labs.md:299-313 -->
