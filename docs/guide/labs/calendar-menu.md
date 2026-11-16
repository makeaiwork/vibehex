---
feature: calendar-menu
key: labs.calendar_menu
title: Upcoming meetings in menu bar
for: [meetings]
ask: Show your next calendar meetings in the menu bar menu?
group: Meetings
os: macOS
vibehex: false
---

# Upcoming meetings in menu bar

**What it does.** Puts an Upcoming section at the top of VibeHex's menu bar menu: up to five of your next meetings, soonest first, each with its time and title ("Now" while it runs, "Tomorrow" or the weekday for later days). Choosing one with a call link opens the call.

**Turn it on when.** You join calls from your calendar and want them one click away.

**Leave it off when.** Your calendar app already does this for you.

**Needs.** macOS, Meetings on, and a calendar connected in Meetings › Settings › Calendar. Reminders don't need to be on.

**Good to know.** It lists the same meetings reminders use, with the calendars you left out still left out. Joining from the menu doesn't start a recording, and that meeting's reminder won't pop up; an app set to Always record may still record once the call starts. A meeting without a link shows but can't be chosen. With no meetings left it reads "No upcoming meetings".

**Where.** Labs › Upcoming meetings in menu bar (`labs.calendar_menu`). Related: `meetings.calendar.connected`.

**Asked as.** "Show my next meetings in the menu bar"; "Join a call from VibeHex"; «Ближайшие встречи в строке меню»; «Подключиться к звонку из меню VibeHex».

<!-- sources: upstream docs/features/labs.md:192-209, 2210-2275; docs/features/meetings.md:712-748; src/labs/card.rs:78, 272-274; src/labs/catalog.rs (MACOS); src/labs/group.rs (Meetings) -->
