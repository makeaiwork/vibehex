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
  in the merge and are regenerated there. The capture capsule's images are macOS's: regenerate them
  with `HEX_UPDATE_GOLDENS=1 cargo test --locked --bin hex capture_capsule_goldens` on the Mac.
- A Rune update moves the antialiasing of all text, so nearly every image changes. The skill's
  `golden-review.py` sorts them: images whose layout changed are all looked at, beside the fork's
  image from before the merge; the rest differ only in glyph rendering. The comparison lets 0.2% of
  the pixels differ, so an image can be stale: a regenerated one may show an older change.
- The container runs as root, and `updater::legacy::tests::a_legacy_linux_install_moves_into_the_installers_layout`
  runs the installer, which refuses root. It fails there and passes as a normal user (`setpriv`).
- `rune_ui::tests` have never run on the merged code until the container runs them: expect fork
  tests that assumed upstream's old layout or wording, and run each failure alone before fixing it.
- New upstream strings that name Hex: `python3 scripts/vibehex-names.py` rewrites them to VibeHex and
  adds their rows in every language from Hex's own translations (`--check` only lists them).
- A finished meeting has both `transcript.ndjson` and `transcript.md`; test fixtures need both.
- The first push of a sync branch carries upstream's history and the regenerated images and can end
  with HTTP 408: `git -c http.postBuffer=1048576000 -c http.version=HTTP/1.1 push`.
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
| Nearby text and the cloud | The owner's rule of 2026-11-16: a lab that is on works with whatever cleanup engine is chosen; the card and the guide say when that is a cloud engine; no separate consent switch (to keep the text on this Mac, run cleanup here); secrets are cut from the nearby text; terminals are read like any app. Built for Rewrite reads nearby text, shown in a stable build again: every engine gets the text (built-in model, own server, Codex, OpenRouter, OpenCode), the card's one muted line names where it goes with the engine chosen now, and known key and token shapes (PEM keys, SSH keys, provider tokens, JWTs, Authorization values, URL passwords, secret-named assignments, long opaque strings) are cut out in the reader thread, best effort. The section leads the user message, ahead of the transcript; a long dictation's first piece gets the text before the cursor, its last the text after. Assistant may turn the lab on. The fork's code is in `dictation/caret_prompt/cleanup.rs`, `dictation/caret_prompt/secrets.rs`, `rune_ui/labs/cursor_context.rs`; upstream files carry short `// VibeHex:` lines: `dictation/caret_prompt.rs`, `dictation/caret_text.rs`, `dictation/processor.rs`, `labs/description.rs`, `control/settings/registry.rs`, `control/settings.rs` (a test), `rune_ui/labs.rs`, `rune_ui/tests.rs`, `desktop/i18n.rs` | 2026-11-16 |
| Screen vocabulary and AI cleanup | The fork gives the window's terms to AI cleanup too, behind `labs.screen_vocabulary_cleanup` (off by default: a cloud engine then receives them; Assistant may turn it on like any setting; a term shaped like a key or a token is never given to cleanup), as a prompt section after the Vocabulary and as terms the vocabulary guard checks. With it on the window is read whatever the speech model; upstream's live-stream rule in `parakeet.rs` stays as it is. The fork's code is in `dictation/screen_vocabulary/cleanup.rs`, `labs/vibehex.rs` and `rune_ui/labs/screen_vocabulary.rs`; upstream files carry short `// VibeHex:` lines: `dictation/processor.rs`, `dictation/screen_vocabulary.rs`, `app_settings/labs.rs`, `labs/catalog.rs`, `control/settings/registry.rs`, `control/settings/requires.rs`, `control/settings.rs` (a test), `rune_ui/labs.rs`, `desktop/i18n.rs` | 2026-11-16 |
| What a dictation read is kept in History | The owner's decision of 2026-11-16: each dictation's History entry also keeps what VibeHex read for it, so it can be looked at and analysed later: the names Screen vocabulary read (the ones AI cleanup was given marked) and the text Rewrite reads nearby text read (secrets cut out), with what cleanup was given of it and whether it was asked again. Shown folded as "What VibeHex read" at the end of the entry's detail. It is written only when the entry is, follows its retention and leaves with it; it never goes to the logs, the local API or Assistant; `hex history --raw --json` prints it. The fork's code is in `history/read.rs` and `rune_ui/history/read.rs`; upstream files carry short `// VibeHex:` lines: `history.rs`, `parakeet.rs`, `dictation/processor.rs`, `dictation/modes.rs`, `dictation/context.rs` (a comment), `control/transcript_cli.rs`, `rune_ui/history.rs`, `rune_ui/scenes/mod.rs` (the `history` scene's fixture), `labs/card.rs`, `labs/description.rs`, `desktop/i18n.rs`, and every literal of `HistoryEntry`, `HistoryDraft` or `Processed` in tests | 2026-11-16 |
| Screen vocabulary skips interface labels | The owner's report of 2026-11-17: in VS Code the 24 names were all the app's own button, tab and panel labels (Code, Panel, Branch, Changes). Upstream's reader keeps the window's text as it is, and also returns where each `AXTitle` and `AXDescription` sits in it (same bounds and byte budget). The content's terms are ranked first as upstream ranks them; a term found only in labels is kept only when it is no ordinary word (mixed case, digits, or missing from macOS's `/usr/share/dict/words`, its lowercase entries, plurals and `-ed`/`-ing` forms included); function words from `/usr/share/dict/connectives` and the app's own name are never terms. The files are read once, lazily; missing, nothing is filtered; no word list is bundled. History says when a lab read and found nothing ("No names could be read from this window.", "This app gave no text near the cursor."). The fork's code is in `dictation/screen_vocabulary/labels.rs`, `history/read.rs` and `rune_ui/history/read.rs`; upstream files carry short `// VibeHex:` lines: `accessibility.rs` (`focused_window_text`), `dictation/screen_vocabulary.rs` (`extract` split from `uncapped`, `from_focused_window`, `started`), `dictation/caret_text.rs` (`started`), `recognition/context_watch.rs` (`finish`), `dictation/context.rs` (`ContextSnapshot::read_tried` and its literals), and the `ContextSnapshot` literals in `windows/context.rs`, `meeting/detection.rs` and `dictation/processor.rs` (a test) | 2026-11-17 |
| Assistant's setup and new features | Setup asks at most five new questions for the person's profile and lists the rest under "more to try". Settings that left Labs are asked as one step for their area (meetings), only of people who use it. A feature that improves another is offered right after that one is chosen, and again when the person describes the problem it solves | 2026-11-16 |
| Stop key and YOLO mode | One rule: a dictation long enough for YOLO mode is sent however it ended, by the shortcut or by the Stop key. Escape just after pastes without sending | 2026-11-16 |
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
  finish it. Continue without a model asks for each missing permission on its own page and then
  closes the first run; it does not show the Ready page.
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
- Upstream moved the Labs cards' text to `src/labs/card.rs`, and with it the Release notes link card
  said "in the sidebar footer" again; in VibeHex the link is in About.
- The capture capsule's permission images said "Hex needs Screen Recording".
- Two tests walk every scene and outgrew the shared time limit once the fork's scenes joined
  upstream's new ones; they have their own limit in `.config/nextest.toml`.
- Fork tests that met upstream's changes: the granted permissions now float off before the page
  moves on (1.3 s), Skip shows on the new sketch scene, the summary model's "none" reads "OpenCode's
  default model", and a meeting's retention note has its own line, which moved the cited passage.

**Follow-up after this sync** (separate PRs; the owner's answers of 2026-11-16)

1. The three hidden AI features on the fork's cleanup engines, shown again when they work.
   - Apple Intelligence is one more engine in Output › AI cleanup › Runs on, and only that: it is
     not a fallback for an engine that did not answer. It makes its light fixes (misheard words,
     spoken self-corrections) and the row says the person's instructions are not applied. Its Labs
     switch stays hidden.
   - Voice prompt runs on the chosen engine. It works with Clean up with AI off, as long as an
     engine is set up: the phrase sends that one dictation. The phrase follows the dictation
     language («перепиши это» for Russian, "rewrite this" for English) and changes with it. My
     reading, to confirm in the PR: a phrase typed in the card wins over the language's, and with
     the language on automatic the interface language picks it.
   - Cursor context works on every engine. Done in `feat/cursor-context-engines` under the
     owner's later rule (Standing decisions, "Nearby text and the cloud"), which replaces "Codex
     and OpenRouter ask once" and "terminals are not read by default": no consent switch, the card
     names where the text goes, secrets are cut out, terminals are read like any app. Voice prompt
     and Apple Intelligence stay open.
2. Assistant's setup.
   - One step for the meeting settings that left Labs (how long audio is kept, stop when the call
     ends, descriptive file names, notes in their own file), asked only of people who record
     meetings.
   - At most five new questions for the person's profile, the rest of the 23 new labs in one "more
     to try" list at the end. The PR carries the table of which lab is asked of whom.
   - Cursor context and Voice prompt are offered right after AI cleanup is chosen, with where the
     nearby text goes for the engine chosen, and suggested in conversation when the person
     describes what they fix ("it misspells names from my document").
3. Stop key and YOLO mode: decided, stays as it is (the table above). Done in this branch: the two
   guide articles say it.

**Checks**

- macOS, `scripts/check`: fmt, code shape, clippy, 3,827 of 3,827 tests.
- Linux (arm64 Ubuntu 24.04 container): clippy clean; 3,313 of 3,314 tests. The one failure is the
  installer test that refuses root (see Profile); it passes there as a normal user.
- Golden images: 293 regenerated on Linux and 4 on macOS (the capture capsule's permission bars).
  90 had a layout change (new scenes included) and were looked at one by one beside the fork's image
  from before the merge, as were the 4; the other 203 differ only in glyph rendering after the Rune
  update. Of the capsule images that came from upstream unchanged, 12 were looked at on one sheet.
- Seen in the images: no "Hex" as the app's name, no middle dot, no What's new row in the sidebar;
  the fork's model step, Back, Skip and Set up with Assistant in the first run; Keep audio and Export
  in Meetings settings.
- `python3 scripts/vibehex-names.py --check`: nothing left to rename. `git diff --check`: clean.
- The owner's `settings.json` (a copy) through this build's launch-time load: no existing value
  changes. New settings arrive with their defaults; the three retired meeting switches leave `labs`
  and their values move to `meetings.export`, `meetings.stop_when_call_ends` and the audio retention
  already under `meetings`; `migrations` gains its mark.
- The fork's `dev` moved during the sync (#42, documents only) and was merged into the branch; the
  macOS check was run again after it.
- `rune_ui::assistant::flow_pins::a_first_setup_then_a_returning_one` fails on the fork's `dev` at
  6c218b7c without this sync: the release notes for 3.3.0-vibe.7 moved two labs into a dated release
  after the pin was last written. This branch updates the pin.
- Not run: Windows (no machine), the first-run walkthrough that downloads a model
  (`scripts/check-onboarding.sh`), a real meeting and a real dictation in the merged build. The owner
  tries the build before this PR is merged.
- Flaky, upstream's own: `meeting_capsules::tests::a_light_host_draws_the_dark_recording_from_the_first_frame`
  compares 17 frames and fails when the machine is busy enough for the clock to cross a second
  before its first frame. It passed in both full runs and failed twice beside heavier tests.

**Documentation after the merge** (checked against the code by separate agents, 2026-11-16)

- The 23 new Labs articles: seven had a wrong or unclear sentence; every key, group and OS matches
  the code, and each article's sources now name the lines it was checked against.
- The topic articles: each of the eleven graduated features is described where it lives now, with
  its setting; sixteen plain settings the guide never mentioned were added (sounds and volume, the
  interface language, Files' prompt, the Summary prompt, Assistant's own engine).
- `docs/releases/next.md` said "Nothing new yet". It now lists what this sync brings, without the
  hidden features, so Assistant can answer "what's new" after the update.
- README, `docs/features` and AGENTS.md: only the fork's lines were touched. Upstream's text that is
  wrong for VibeHex has a fork line beside it instead of an edit.
- Guide search: four Russian phrasings joined "Asked as", after longer topics pushed their own
  questions out of the top three.

**Left as upstream has it** (seen in the review, not changed)

- A shortcut row has no trash button: Reset appears when the shortcut differs from its default, and
  Backspace in the recorder turns it off (#1809).
- The first-run story has its own "Skip →" under the keyboard beside the corner Skip.
- Assistant's Russian setup card says "Затем:" before an English sentence. It did before this sync.
