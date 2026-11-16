# Upstream sync

How VibeHex takes changes from Hex (`anomalyco/hex`). The `upstream-sync` skill reads this file
before every sync and writes to it after: the profile says how this fork is built and checked, the
standing decisions are answers the owner has already given, and each sync gets one entry.

## Profile

- Fork: `makeaiwork/VibeHex` (`origin`), branch `dev`. Upstream: `anomalyco/hex` (`upstream`), branch `dev`.
- A sync is one merge of `upstream/dev` on a branch `sync/upstream-<date>`, in its own worktree under
  `/Volumes/DropboxSSD/dev/wt/`. The PR goes to `origin` `dev` and is merged with a merge commit, never
  squashed: a squash loses the merge base and the next sync replays every old conflict.
- Build and checks on macOS: `scripts/check` (fmt, code shape, clippy, every test). Cargo is not on
  `PATH`; the toolchain lives on the external disk.
- Checks macOS cannot run: Linux-only code, `rune_ui::tests` and the golden images. GitHub Actions are
  off on the fork, so they run in an arm64 Ubuntu 24.04 container. Golden images take upstream's side
  in the merge and are regenerated there; every changed image is looked at.
- The owner's builds share one data folder. An older build drops settings it does not know, so the
  owner's `settings.json` is copied aside before a merged build first starts, and the build in
  `/Applications` is replaced soon after.

## Standing decisions

| Area | Decision | Since |
|---|---|---|
| Not taking a feature | It is hidden, not deleted: upstream's code stays in the tree and the fork leaves the row out or off. Deleted code conflicts at every later sync | 2026-11-09 |
| Labs hidden from a stable build | Import audio, Release notes link, Quit closes window, Annotate replay | 2026-11-09 |
| Fork Labs | A star on the card and a VibeHex chip after the groups. Each fork lab keeps its own on/off switch, apart from upstream's shared bit word, so a new upstream lab can never land on it | 2026-11-09, switches 2026-11-16 |
| App name | Every string a user sees says VibeHex. An upstream string that names Hex gets a VibeHex row in every language (`src/desktop/i18n/vibehex/`) | 2026-11-09 |
| Middle dots | No "·" between words. Upstream strings that use one are reworded with spacing, a colon or brackets | 2026-11-09 |
| Translations | Fork strings stay apart in `src/desktop/i18n/vibehex/`; upstream's tables are taken as they are | 2026-11-09 |
| README | `README.md` is the fork's. Upstream's edits are read for facts and paths are corrected to VibeHex | 2026-11-16 |
| Sidebar | The fork's order, ending with About and Assistant. Version and updates live in About. Upstream's What's new pane is hidden: Assistant tells what is new | 2026-11-16 |
| First run | The fork's language and model step comes first and nothing downloads before Download is pressed. Upstream's first-run story follows it | 2026-11-16 |
| Meeting recording pill | Upstream's design as it is, with the fork's wording (VibeHex, any notes writer, no middle dots). No Open button is added back | 2026-11-16 |
| AI features that only work through OpenCode | They come in hidden from a stable build until they run on the fork's cleanup engines (Codex, OpenRouter, own server, built-in model). A second "clean up" switch is not shown beside AI cleanup: it becomes one more engine | 2026-11-16 |
| Assistant's guide | Every feature has an article, upstream's new ones included. A lab that graduates is not dropped from the guide: its article moves to where the feature now lives, with its new setting, and Assistant can still change it and offers it in setup as a setting. Setup's questions follow the new features | 2026-11-16 |
| Version | `Cargo.toml` follows upstream's `X.Y.Z`. The release counter continues across upstream versions: after 3.3.0-vibe.7 comes 3.4.0-vibe.8 | 2026-11-16 |
| Updater | VibeHex stages updates in its own folder name, so Hex and VibeHex side by side never delete each other's waiting update | 2026-11-16 |
| Rune | The pin follows upstream's at each sync; `rune-kit` keeps the `markdown` feature | 2026-11-16 |
| CI, site, SDK, iOS, packaging | Taken as they are. Actions are off on the fork, so no workflow runs | 2026-11-16 |
| Windows and Linux | macOS is the shipped platform. Where a fork feature cannot follow an upstream change there (Telemost on Windows), upstream's fallback is accepted | 2026-11-16 |

## Syncs

### 2026-11-16: upstream 5f01839e (173 commits, Hex 3.4.0)

- Base: a65a5e37 (sync of 2026-11-09). Fork at 6c218b7c. Branch `sync/upstream-2026-11-16`.
- Size: 218 conflict hunks in 78 text files, 133 golden images. 103 of upstream's commits meet the
  fork outside the shared lists, so the sync is one merge.

**Taken as it is**

- Dictation: keep an interrupted dictation, keep it for Paste Last when a paste fails, key shortcuts
  under secure input, dead-key shortcuts, the microphone key, double-tap on a lone modifier, more
  dictation shortcuts each with its own mode, Return and Escape while holding, YOLO mode, return to
  the starting app, speakers-only for While dictating, quiet-speech and long-clip fixes.
- Models: speech model per front app, custom model fixes, smaller model files, real download stages,
  waiting for another process's download.
- Meetings: ten labs graduate into normal behaviour and Meetings settings, notes in their own file,
  Teams web calls, a browser recording ends when its page names another call, upcoming meetings in
  the menu bar, quiet microphone chunks raised, OpenCode's default model when none is chosen.
- History and captures: select several dictations, ask before deleting, play and copy kept audio,
  annotations and replays in Captures, captures as a zip, choose the save folder.
- Settings: saves keep other writers' changes and unknown keys, refuse to save after an unreadable
  load, sorted keys, sound volume slider, Reset on shortcut rows, appearance setting.
- CLI and API: `hex transcribe` for any media file and long files, shell completions, browser access
  to the local API, word timestamps, `dictation start --print`.
- Logs capped at 10 MB. Windows and Linux fixes. CI, site, SDK, iOS.

**Decisions** (the owner's answers; the lasting ones are in the table above)

- First run: the fork's model step, then upstream's story. A run counts as finished by upstream's
  rule (the last button, or Continue without a model); the fork's Skip and Set up with Assistant still
  finish it. Continue without a model goes to the first missing permission, then Ready.
- What's new pane: hidden.
- Meeting pill: upstream's, no Open button.
- Apple Intelligence cleanup, Voice prompt, Cursor context: hidden from a stable build in this sync;
  connected to the fork's engines in the follow-up below.
- Assistant: articles for all 23 new labs; the 11 graduated or retired labs move into topic articles
  with their new settings; presets and setup follow the new keys.
- Next release is 3.4.0-vibe.8.

**Fixed because of the merge** (no conflict marker shows these)

- Fork Labs sat on bits 63..57 and upstream's reached bit 60: four pairs of labs would have switched
  each other on. Fork labs get their own switches.
- Fork code that used what upstream removed or changed: the settings `save()`, `ld::item`, private
  `ComboboxState` fields, `labs::page`'s new argument, `known_meeting_providers`, the retired meeting
  labs in presets and Assistant, `jump_to_permissions`, the shortcut targets moved to `request.rs`,
  the Stop key beside upstream's new cancel rules.
- About 60 new upstream strings that name Hex or use a middle dot.
- Upstream's finished-meeting panel assumed Apple Intelligence wrote the notes.

**Follow-up after this sync** (separate PRs)

1. Voice prompt and Cursor context run on the chosen cleanup engine; Apple Intelligence becomes an
   engine in AI cleanup; the three are shown again.
2. Assistant's setup asks about the new features and offers them where they help (cursor context
   for more accurate cleanup, and the like).
3. Whether the Stop key ends a dictation that YOLO mode would send.

**Checks**: filled in when the branch is ready.
