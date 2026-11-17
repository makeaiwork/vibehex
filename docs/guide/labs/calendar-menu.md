---
feature: calendar-menu
key: labs.calendar_menu
title: Upcoming meetings in menu bar
for: [meetings]
ask: Show your next calendar meetings in the menu bar menu?
now: VibeHex's menu bar menu does not list your upcoming meetings.
with: Up to five next meetings show in the menu, and picking one with a call link opens the call.
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

<!-- sources: src/labs/card.rs:78,281-283; src/labs/catalog.rs:286,508; src/labs/group.rs:122; src/control/settings/registry.rs:650-656,741-745; src/calendar_menu.rs:16,137-145,186-199; src/macos_rune/status_item.rs:321-324; src/rune_ui/platform_settings.rs:765-896; docs/features/labs.md:2305-2341 -->
