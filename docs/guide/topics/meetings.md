---
topic: meetings
title: Meetings
keys: [meetings.enabled, meetings.summaries, meetings.opencode_model, meetings.providers, meetings.stop_when_call_ends, meetings.calendar.connected, meetings.calendar.remind, meetings.calendar.lead_minutes, meetings.retention, meetings.audio_retention, meetings.output_folder, meetings.export.descriptive_filenames, meetings.export.notes_file, meetings.show_recording_indicator, meetings.recording_indicator_corner, labs.ai_cleanup_server, labs.live_chapters, labs.calendar_menu]
os: macOS, Windows, and Linux (beta; Windows and Linux have a reduced version)
---

# Meetings

Meetings records a call (the other people's audio and your microphone), transcribes it on this computer, and keeps a transcript of "You" and "Them" turns, plus notes. Beta, off by default.

## Turn it on

Switch on **Labs › Meetings (beta)** (macOS, Windows and Linux). Meetings joins the sidebar, with Recordings and Settings. Press **Start recording**. Windows and Linux record as a beta, with no permission prompts and no calendar. On a Mac, **Screen & System Audio Recording** in Meetings › Settings has an Allow button (if it stays grey, **Still grey? Fix it**); recording needs it and the microphone.

## Choose which calls are recorded

When Zoom, Google Meet, Microsoft Teams (its app or its web pages), Slack huddles, FaceTime, WhatsApp, Gather, Discord, Yandex Telemost, Signal, Telegram, WeChat, LINE, Webex, Element or the Phone app uses the microphone, an offer appears: **Record** or **Not now**. Messaging apps, WhatsApp, Gather and Telemost wait about 20 seconds, since voice notes use the microphone too. On a Mac, Google Voice, Messenger and GroupMe pages count as well. Its "…" saves a choice; change it in **Meetings › Settings › Detected calls**: **Ask**, **Always record** or **Never offer**. Teams pages follow the Teams app's choice.

**Stop when the call ends** (top of Detected calls, Mac only, off by default): a recording VibeHex started by itself stops 30 seconds after the call app leaves the microphone and goes quiet, unless you press **Keep recording** in the notice. In a browser Google Meet call, the page naming another call does the same.

## While it records

A small dark pill shows a recording dot, your level and the time; "Microphone is silent" warns you. Point at it for **Stop**, **Copy new** (turns not yet copied), **Copy all** and **Copy agent prompt** (tells a coding agent how to read this meeting with `hex`). Drag or throw it to any corner; offers, reminders and the finished panel open there too. **Show recording indicator** (Settings › Advanced) hides it. An open meeting's transcript follows the newest line; scroll up to stop following.

When it ends, a panel shows the title, length and summary with the same copy buttons, for about 15 seconds or while the pointer is over it; × closes it.

## Connect your calendar

On a Mac, **Settings › Calendar › Connect calendar** asks macOS for access only when pressed. **Remind me before meetings** reminds you at the start, or 1, 2 or 5 minutes before, and names the recording after the event. **Labs › Upcoming meetings in menu bar** lists them in the menu bar.

## What a finished meeting has

Tabs: **Transcript**, **Summary** (chapters come with **Labs › Live chapters**) and **Requests** (questions, with **Labs › Clean up on your own server** and Settings › Questions › Ask about a meeting on; **Questions run on** picks the model). A question reaching past the meeting also reads your other meetings, Files texts and dictations once you allow it, citing them with [n] marks. The "…" menu regenerates the summary or deletes the meeting. Titles and summaries spell names as your Vocabulary does (its first 30 terms).

## Choose who writes the title and summary

**Settings › After a meeting › Summaries** on a Mac: **Off**, **On this Mac** (Apple Intelligence, the default; macOS 26 or later; nothing leaves the Mac) or **OpenCode** (sends the transcript to OpenCode, which must be running). With OpenCode, **Model** appears; left unset, OpenCode's own default model writes. On Windows and Linux Summaries is a switch, off by default, meaning OpenCode.

**Settings › Notes › Written by** can pick another writer; Summaries then hides and is only the fallback. Codex, Built in and Other appear with **Labs › Clean up on your own server** set up in Output › AI cleanup. Apple Intelligence and Built in keep the transcript on the computer; any other writer receives it.

## Where recordings live

Audio and transcripts stay in VibeHex's data folder. **Settings › Export**: **Transcript folder** (a Hex Meetings folder in Documents) for the Markdown copy, **Descriptive meeting file names** (renames it to the meeting's title once notes or a rename give one, keeping date and time) and **Separate notes file** (notes in their own file beside the transcript). **After a meeting › Keep meetings**: 24 hours (the default for new meetings), 7 days, 30 days or Forever, then the Trash. **Keep audio**: Same as meetings, or a shorter time that moves only the audio to the Trash; the text stays, but can't be transcribed again.

Scripts can export subtitles with `hex meetings show <meeting> --srt` or `--vtt`, and an app on the local API can be told when a meeting is ready (it gets only its ID).

**Where.** Labs › Meetings (`meetings.enabled`). Off a Mac, `hex labs on meetings` is refused; `hex settings set meetings.enabled on` works.

**Asked as.** "Can it record my Zoom calls?"; "Where are my meeting notes?"; «Как записывать созвоны?»; «Почему не слышно собеседника в записи?»; «Запись не останавливается после звонка»; «Как удалять аудио встреч, оставляя расшифровку».

<!-- sources: upstream docs/features/meetings.md:7-8, 41-46, 61-65, 75-95, 111-115, 310-330, 403-435, 798-868, 873-914, 1449-1475, 1597-1610, 1672-1675; upstream docs/features/labs.md:2955-2969; upstream src/rune_ui/platform_settings.rs:500-591, 646-690, 955-985; upstream src/control/settings/registry.rs:680-760, 900-918; upstream src/rune_ui/meeting_capsules.rs:125, 989-991; fork: src/rune_ui/meeting_notes.rs:41-210; src/rune_ui/meeting_tabs/requests.rs:56-63; src/app_settings/meetings.rs:93-110 -->
