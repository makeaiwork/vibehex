//! A new conversation's start page adapts to what is going on: one block
//! of up to four rows over "Set VibeHex up for me", picked by the rules in
//! `assistant::suggest` (what broke, what was left unfinished, the pane
//! the person came from, what is new for them), the example questions
//! filling the places left. Each row is one press, with a quiet × that
//! hides it (`suggest::Hidden`, `<support>/assistant/suggestions.json`).
//! What broke is drawn as a warning, and marks the sidebar's Assistant row
//! in the warning colour until Assistant is opened on it.
//!
//! Nothing here reads the disk per frame: the rows are picked again when
//! the pane opens, a conversation starts, a row is pressed or hidden, and
//! every few seconds while the page shows; the meetings are scanned on a
//! thread of their own. A scene's rows come from its fixture.

use std::time::{Duration, Instant};

use rune::{El, Icon, Tx, col, el, icon, row, text};

use super::super::Ui;
use super::super::controls::Paint;
use super::super::icons;
use super::super::sidebar::Pane;
use crate::assistant::suggest::{self, From, Hidden, Signals, Suggestion, Why};
use crate::desktop::i18n::{tr, tr_fill, tr_named, tr_window};

mod signals;
#[cfg(test)]
mod tests;

pub(in crate::rune_ui) mod scenes;

/// How often the rows are picked again while the start page shows.
const TICK: Duration = Duration::from_secs(5);
/// How many Labs features' names a row lists before "and N more".
const NAMED: usize = 3;

/// The start page's suggestions and what is remembered of them.
#[derive(Default)]
pub(in super::super) struct Suggestions {
    /// The rows picked last, most pressing first.
    pub(super) shown: Vec<Suggestion>,
    pub(super) hidden: Hidden,
    hidden_read: bool,
    /// The pane Assistant was opened from, and the meeting open there.
    pub(super) from: Option<From>,
    pub(super) open_meeting: Option<String>,
    /// Pick again before the page next draws.
    stale: bool,
    refreshed_at: Option<Instant>,
    /// Assistant was just opened: what broke lands on the start page.
    arrived: bool,
    /// What broke and wasn't seen yet, by key: the sidebar's mark.
    pub(super) trouble: Vec<String>,
    /// A scene's signals: nothing is read.
    pub(super) scene: Option<Signals>,
    /// The Labs features new for the person, read when the settings or
    /// the setups change.
    labs: Option<signals::NewLabs>,
    watch: signals::Watch,
}

/// Assistant is about to show, switched to from another pane (`labs`:
/// from the Labs page): remembered for the rows that fit where the person
/// was. Called by `Ui::select_pane`.
pub(in super::super) fn arriving(ui: &mut Ui, labs: bool) {
    let from = if labs {
        Some(From::Labs)
    } else {
        match ui.pane {
            Pane::Meetings => Some(From::Meetings),
            Pane::History => Some(From::History),
            Pane::Output => Some(From::Output),
            _ => None,
        }
    };
    let open_meeting = ui
        .meetings
        .selected
        .clone()
        .filter(|_| from == Some(From::Meetings));
    let state = &mut ui.assistant.suggestions;
    state.from = from;
    state.open_meeting = open_meeting;
    state.stale = true;
    state.arrived = true;
}

/// Assistant opened from a link (Back to Assistant, onboarding): it
/// wasn't the person's own way from that pane.
pub(super) fn by_link(ui: &mut Ui) {
    let state = &mut ui.assistant.suggestions;
    state.from = None;
    state.open_meeting = None;
    state.stale = true;
}

/// The sidebar's row pressed: opened, even while Assistant showed.
pub(super) fn opened_from_sidebar(ui: &mut Ui) {
    if ui.pane == Pane::Assistant && !ui.labs_open {
        ui.assistant.suggestions.arrived = true;
    }
}

/// A new conversation: its page picks again.
pub(super) fn again(ui: &mut Ui) {
    ui.assistant.suggestions.stale = true;
}

/// Whether the sidebar marks Assistant for something that stopped working.
pub(in super::super) fn trouble(ui: &Ui) -> bool {
    !ui.assistant.suggestions.trouble.is_empty() && Pane::Assistant.shown(ui)
}

/// Where what was hidden is kept, beside the conversations.
fn path() -> Option<std::path::PathBuf> {
    crate::app_paths::support_dir()
        .ok()
        .map(|dir| dir.join("assistant/suggestions.json"))
}

/// Whether this window keeps what is hidden on disk: never a scene's.
fn keeps(ui: &Ui) -> bool {
    ui.live() && !ui.assistant.fixture
}

/// Reads what was hidden, once a run: as the pane's store loads, or the
/// first time the sidebar's mark is read.
pub(super) fn load(ui: &mut Ui) {
    let keeps = keeps(ui);
    let state = &mut ui.assistant.suggestions;
    if std::mem::replace(&mut state.hidden_read, true) {
        return;
    }
    if keeps && let Some(path) = path() {
        state.hidden = Hidden::read(&path);
    }
}

fn save(ui: &Ui) {
    if !keeps(ui) {
        return;
    }
    let Some(path) = path() else {
        return;
    };
    if let Err(error) = ui.assistant.suggestions.hidden.write(&path) {
        tracing::info!(%error, "couldn't keep the hidden suggestions");
    }
}

/// The keys of what broke and wasn't seen yet.
fn unseen(signals: &Signals, hidden: &Hidden) -> Vec<String> {
    if suggest::unseen(signals, hidden).is_none() {
        return Vec::new();
    }
    suggest::broke(signals)
        .iter()
        .map(Suggestion::key)
        .filter(|key| !hidden.has(key) && !hidden.saw(key))
        .collect()
}

/// What broke is seen: the mark goes, the rows stay. True when it was
/// marked.
fn see(ui: &mut Ui) -> bool {
    let state = &mut ui.assistant.suggestions;
    if state.trouble.is_empty() {
        return false;
    }
    for key in std::mem::take(&mut state.trouble) {
        state.hidden.see(key);
    }
    save(ui);
    true
}

/// Assistant opened while marked for trouble: it lands on a new
/// conversation's start page, where the rows say what broke, and the mark
/// goes. True when it was marked.
pub(super) fn landed(ui: &mut Ui) -> bool {
    ui.assistant.suggestions.arrived = false;
    if !see(ui) {
        return false;
    }
    ui.assistant.open = None;
    ui.assistant.suggestions.stale = true;
    true
}

/// Watches for what stopped working, wherever the window is: History's
/// newest dictations and a scan of the meetings, slowly. True when the
/// mark changed.
pub(super) fn watch(ui: &mut Ui) -> bool {
    if !keeps(ui) || !Pane::Assistant.shown(ui) {
        return false;
    }
    let now_ms = ui.assistant.now_ms();
    let revision = || ui.history.revision_now();
    let mut watch = std::mem::take(&mut ui.assistant.suggestions.watch);
    let looked = watch.look(revision, now_ms);
    ui.assistant.suggestions.watch = watch;
    if !looked.again {
        return false;
    }
    load(ui);
    let signals = signals::for_mark(ui);
    let state = &mut ui.assistant.suggestions;
    let trouble = unseen(&signals, &state.hidden);
    state.stale = true;
    if trouble == state.trouble {
        return false;
    }
    state.trouble = trouble;
    true
}

/// Whether a new conversation's start page shows.
fn start_page(ui: &Ui) -> bool {
    let pane = &ui.assistant;
    ui.pane == Pane::Assistant
        && !ui.labs_open
        && pane.view == super::View::Chat
        && pane.answering_in(pane.open).is_none()
        && pane.consent.is_none()
        && pane
            .conversation()
            .is_none_or(|conversation| conversation.turns.is_empty())
}

/// Lands what broke, and picks the rows again when they are due. True when
/// what shows changed.
pub(super) fn poll(ui: &mut Ui) -> bool {
    if ui.pane != Pane::Assistant || ui.labs_open {
        return false;
    }
    let mut changed = false;
    if ui.assistant.suggestions.arrived {
        changed |= landed(ui);
    }
    if !start_page(ui) {
        return changed;
    }
    let state = &ui.assistant.suggestions;
    let due = state.stale || state.refreshed_at.is_none_or(|at| at.elapsed() >= TICK);
    // Opened, or a new conversation: the meetings are read again unless
    // a scan just did; the slow tick picks from what is known.
    if state.stale && keeps(ui) {
        let now_ms = ui.assistant.now_ms();
        ui.assistant.suggestions.watch.refresh(now_ms);
    }
    if due {
        changed |= refresh(ui);
    }
    // On the page the rows say what broke: the mark has nothing to add.
    changed | see(ui)
}

/// Picks the rows again from what is known now. True when they changed.
pub(super) fn refresh(ui: &mut Ui) -> bool {
    load(ui);
    let signals = signals::gather(ui);
    let state = &mut ui.assistant.suggestions;
    state.stale = false;
    state.refreshed_at = Some(Instant::now());
    let shown = suggest::pick(&signals, &state.hidden);
    // A scene's mark is read from its signals too; the window's from what
    // the watch read.
    if state.scene.is_some() {
        state.trouble = unseen(&signals, &state.hidden);
    }
    if shown == state.shown {
        return false;
    }
    state.shown = shown;
    true
}

/// × on a row: it stays away while its reason is the same.
fn hide(ui: &mut Ui, key: String) {
    let state = &mut ui.assistant.suggestions;
    if state.hidden.hide(key) {
        save(ui);
    }
    refresh(ui);
}

/// A row pressed: what it offers. What broke stays until it works again
/// or is hidden, a pane's question comes back each visit, and new features
/// are hidden once setup starts on them.
fn press(ui: &mut Ui, suggestion: &Suggestion, tx: &mut Tx) {
    if hides_on_press(suggestion) && ui.assistant.suggestions.hidden.hide(suggestion.key()) {
        save(ui);
    }
    let ask = |ui: &mut Ui, question: String| {
        super::composer::ask(ui, question);
    };
    match suggestion {
        Suggestion::CleanupFailed { .. } => {
            super::connect::open_cleanup(ui, tx);
            super::jump::came_for_suggestion(ui);
        }
        Suggestion::NotesFailed(met) => {
            use super::super::meeting_tabs::MeetingTab;
            ui.select_pane(Pane::Meetings, tx);
            ui.meetings.view = super::super::meetings::View::Recordings;
            ui.meetings.select(met.id.clone());
            ui.meetings.tab = Some(MeetingTab::Summary);
            super::jump::came_for_suggestion(ui);
        }
        Suggestion::SetupLeft(left) => {
            ui.open = None;
            ui.assistant.open = Some(left.conversation);
        }
        Suggestion::Updated { .. } => {
            super::whats_new::ask(ui);
        }
        Suggestion::NewLabs { .. } => super::ask::begin(ui),
        Suggestion::MeetingsWeek { .. } => {
            let question = crate::assistant::language::within(&ui.settings, || {
                tr("What are the action items from this week's meetings, and who owns each one?")
                    .to_owned()
            });
            ask(ui, question);
        }
        Suggestion::Meeting(_)
        | Suggestion::WrongWord
        | Suggestion::WhichLabs
        | Suggestion::Cleanup { .. } => {
            let question = crate::assistant::language::within(&ui.settings, || title(suggestion));
            ask(ui, question);
        }
    }
    refresh(ui);
}

/// Whether pressing a row hides it. What broke stays until it works
/// again or is hidden, a pane's question comes back each visit, and
/// Updated to and a setup left go by their own state (asked about,
/// decided); new features go once setup starts on them.
fn hides_on_press(suggestion: &Suggestion) -> bool {
    !suggestion.broke()
        && !suggestion.from_pane()
        && matches!(suggestion, Suggestion::NewLabs { .. })
}

/// A row asks a model: with none connected it waits for the Connect panel.
fn asks(suggestion: &Suggestion) -> bool {
    matches!(
        suggestion,
        Suggestion::Updated { .. }
            | Suggestion::Meeting(_)
            | Suggestion::MeetingsWeek { .. }
            | Suggestion::WrongWord
            | Suggestion::WhichLabs
            | Suggestion::Cleanup { .. }
    )
}

/// The rows the page shows: those that need no model while none can be
/// asked (`asking`).
pub(super) fn shown(ui: &Ui, asking: bool) -> Vec<&Suggestion> {
    ui.assistant
        .suggestions
        .shown
        .iter()
        .filter(|suggestion| asking || !asks(suggestion))
        .collect()
}

/// The example questions that fill the block to [`suggest::MAX`], in
/// order, without one a row already asks.
pub(super) fn examples<'a>(ui: &Ui, asking: bool, all: &[&'a str]) -> Vec<&'a str> {
    let shown = shown(ui, asking);
    let titles: Vec<String> = shown.iter().map(|suggestion| title(suggestion)).collect();
    all.iter()
        .copied()
        .filter(|question| !titles.iter().any(|title| title == tr(question)))
        .take(suggest::MAX.saturating_sub(shown.len()))
        .collect()
}

/// A row's title, in the words `tr` speaks now.
pub(super) fn title(suggestion: &Suggestion) -> String {
    match suggestion {
        Suggestion::CleanupFailed { count, .. } => tr_fill(
            "Your last {} dictations were pasted without AI cleanup",
            &count.to_string(),
        ),
        Suggestion::NotesFailed(met) => tr_fill("Notes for “{}” were not written", &met.title),
        Suggestion::SetupLeft(left) if left.plan => tr("Your setup's plan is waiting").to_owned(),
        Suggestion::SetupLeft(_) => tr("Pick up your setup where you left it").to_owned(),
        Suggestion::Updated { version } => tr_fill("Updated to {}", version),
        Suggestion::Meeting(met) => tr_fill("What did we decide in “{}”?", &met.title),
        Suggestion::MeetingsWeek { count } => tr_fill(
            "Pull the action items from this week's {} meetings",
            &count.to_string(),
        ),
        Suggestion::WrongWord => tr("Some words come out wrong. How do I fix that?").to_owned(),
        Suggestion::WhichLabs => tr("Which Labs features are worth turning on for me?").to_owned(),
        Suggestion::Cleanup { on: true } => tr("How do I tell AI cleanup how to write?").to_owned(),
        Suggestion::Cleanup { on: false } => {
            tr("Can I use my ChatGPT plan to clean up dictation?").to_owned()
        }
        Suggestion::NewLabs { keys, .. } if keys.len() == 1 => {
            tr("1 new feature fits how you use VibeHex").to_owned()
        }
        Suggestion::NewLabs { keys, .. } => tr_fill(
            "{} new features fit how you use VibeHex",
            &keys.len().to_string(),
        ),
    }
}

/// The muted line under a row's title.
fn detail(suggestion: &Suggestion) -> String {
    let line = match suggestion {
        Suggestion::CleanupFailed { why, .. } => match why {
            Why::CodexSignedOut => tr("Codex is signed out. Sign in again or pick another engine"),
            Why::CodexLimit => tr(
                "Your ChatGPT plan's Codex limit is reached. Pick another engine until it resets",
            ),
            Why::CodexMissing => tr("Codex isn't installed. Install it or pick another engine"),
            Why::OpencodeNotRunning => {
                tr("OpenCode isn't running. Start it or pick another engine")
            }
            Why::NoOpenRouterKey => {
                tr("The OpenRouter key is missing. Add it or pick another engine")
            }
            Why::ModelNotDownloaded => {
                tr("The built-in model isn't downloaded yet. Download it in Output")
            }
            Why::Slow => {
                tr("The engine didn't answer in time. Give it longer or pick a faster one")
            }
            Why::Other => tr("See what the engine said, or pick another one"),
        },
        Suggestion::NotesFailed(_) => tr("Open the meeting to write them again"),
        Suggestion::SetupLeft(left) if left.plan => tr("Apply it, change it, or put it off"),
        Suggestion::SetupLeft(_) => tr("A few questions are still to answer"),
        Suggestion::Updated { .. } => tr("Ask me what's new, and what of it suits how you work"),
        Suggestion::Meeting(_) => tr("The meeting you had open"),
        Suggestion::MeetingsWeek { .. } => tr("With who owns each one"),
        Suggestion::WrongWord => {
            tr("Vocabulary, another speech model, or AI cleanup: which one helps here")
        }
        Suggestion::WhichLabs => tr("Picked from your settings and how you use VibeHex"),
        Suggestion::Cleanup { on: true } => {
            tr("Your own instructions, and what it must never change")
        }
        Suggestion::Cleanup { on: false } => {
            tr("Codex, OpenRouter, your own server, or a model built in")
        }
        Suggestion::NewLabs { titles, .. } => return named(titles),
    };
    line.to_owned()
}

/// The first few Labs features by their cards' titles, in the window's
/// words (they name places in it), then how many more.
fn named(titles: &[String]) -> String {
    let first: Vec<&str> = titles
        .iter()
        .take(NAMED)
        .map(|title| tr_window(title))
        .collect();
    let first = first.join(", ");
    match titles.len().checked_sub(NAMED).filter(|more| *more > 0) {
        Some(more) => tr_named(
            "{titles} and {count} more",
            &[("titles", &first), ("count", &more.to_string())],
        ),
        None => first,
    }
}

/// A row's glyph.
fn glyph(suggestion: &Suggestion) -> &'static Icon {
    match suggestion {
        Suggestion::CleanupFailed { .. } | Suggestion::NotesFailed(_) => &WARNING,
        Suggestion::SetupLeft(_) => &super::feed::SLIDERS,
        Suggestion::Updated { .. } => &icons::SPARKLES,
        Suggestion::MeetingsWeek { .. } => &icons::CALENDAR,
        Suggestion::NewLabs { .. } => &rune_theme_opencode::icons::PLUS,
        Suggestion::Meeting(_)
        | Suggestion::WrongWord
        | Suggestion::WhichLabs
        | Suggestion::Cleanup { .. } => &icons::MESSAGE,
    }
}

/// The block of rows, or none to show.
pub(super) fn block(ui: &Ui, p: Paint, asking: bool) -> Option<El> {
    let shown = shown(ui, asking);
    if shown.is_empty() {
        return None;
    }
    let mut block = col().key("suggestions").gap(8.0);
    for (index, suggestion) in shown.into_iter().enumerate() {
        block = block.child(suggestion_row(p, index, suggestion));
    }
    Some(block)
}

/// One row: a tile, the title over its line, a chevron, all one press;
/// then the × that hides it. What broke is tinted as a warning, new
/// features take the accent on their tile, the rest are neutral.
fn suggestion_row(p: Paint, index: usize, suggestion: &Suggestion) -> El {
    let c = &p.k.c;
    let warning = suggestion.broke();
    let (tile, ink) = if warning {
        (p.k.alpha(p.warning(), 0.16), p.warning())
    } else if matches!(suggestion, Suggestion::NewLabs { .. }) {
        (p.k.alpha(p.accent(), 0.14), p.accent())
    } else {
        (c.layer_02, c.icon_base)
    };
    let (fill, border) = if warning {
        (p.k.alpha(p.warning(), 0.08), p.k.alpha(p.warning(), 0.35))
    } else {
        (c.layer_01, c.border_base)
    };
    // The Updated to row keeps the key it had before suggestions came.
    let key = match suggestion {
        Suggestion::Updated { .. } => "whats-new".to_owned(),
        _ => format!("suggestion{index}"),
    };
    let title = title(suggestion);
    let detail = detail(suggestion);
    let pressed = suggestion.clone();
    let go = row()
        .key("go")
        .role("button")
        .label(title.as_str())
        .description(detail.as_str())
        .focusable(true)
        .focus_ring(2.0, 0.0, c.border_focus)
        .w(0.0)
        .grow(1.0)
        .min_w(0.0)
        .gap(12.0)
        .pl(12.0)
        .pr(6.0)
        .py(10.0)
        .center_y()
        .radius(p.k.radius.md)
        .hover(c.hover)
        .press(c.pressed)
        .child(
            el().key("tile")
                .size(30.0, 30.0)
                .center()
                .radius(p.k.radius.sm)
                .fill(tile)
                .child(icon(glyph(suggestion), 15.0).key("glyph").color(ink)),
        )
        .child(
            col()
                .key("words")
                .grow(1.0)
                .shrink()
                .min_w(0.0)
                .gap(2.0)
                .child(
                    text(title.clone())
                        .key("title")
                        .font(p.k.ty.body_medium)
                        .color(p.text())
                        .wrap_anywhere(),
                )
                .child(
                    text(detail)
                        .key("lead")
                        .font(p.k.ty.small)
                        .color(p.muted())
                        .wrap_anywhere(),
                ),
        )
        .child(
            icon(&rune_theme_opencode::icons::CHEVRON_RIGHT, 14.0)
                .key("chevron")
                .color(c.icon_muted),
        )
        .on_press_tx(move |ui: &mut Ui, tx: &mut Tx| press(ui, &pressed, tx));
    let hidden = suggestion.key();
    let close = row()
        .key("hide")
        .role("button")
        .label(tr("Hide this suggestion"))
        .focusable(true)
        .focus_ring(2.0, 0.0, c.border_focus)
        .size(26.0, 26.0)
        .center()
        .radius(p.k.radius.sm)
        .hover(c.hover)
        .press(c.pressed)
        .child(
            icon(&rune_theme_opencode::icons::CLOSE, 12.0)
                .key("glyph")
                .color(p.faint()),
        )
        .on_press(move |ui: &mut Ui| hide(ui, hidden.clone()));
    row()
        .key(key)
        .gap(4.0)
        .pr(6.0)
        .center_y()
        .radius(p.k.radius.md)
        .fill(fill)
        .border(p.hair, border)
        .child(go)
        .child(close)
}

static WARNING: Icon = Icon {
    name: "hex-assistant-warning",
    view: 16.0,
    body: r#"<path d="M8 2.2 14.2 13H1.8Z" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/><path d="M8 6.5v3M8 11.3v.2" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>"#,
};
