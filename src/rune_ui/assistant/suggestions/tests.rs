//! The start page's suggestions in the window: where Assistant was opened
//! from, a setup left, pressing and hiding a row, the examples that fill
//! the block, and the sidebar's mark for what broke.

use rune::{Runtime, Tx};

use super::super::super::sidebar::Pane;
use super::super::super::{SCENES, Ui, WINDOW_SIZE};
use super::super::layout::window;
use crate::assistant::suggest::{self, From, Suggestion};

/// Scene `name`, or what `name` sets up after `then`, laid out.
fn scene(name: &str, then: impl FnOnce(&mut Ui)) -> Runtime<Ui> {
    let (_, apply) = SCENES
        .iter()
        .find(|(scene, _)| *scene == name)
        .unwrap_or_else(|| panic!("no scene {name}"));
    window(
        |ui| {
            apply(ui);
            then(ui);
        },
        WINDOW_SIZE,
    )
}

/// Presses `key` and lets the window settle.
fn tap(rt: &mut Runtime<Ui>, key: &str) {
    let (x, y) = rt.find(key).unwrap_or_else(|| panic!("no element {key}"));
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    for _ in 0..60 {
        rt.frame(1.0 / 60.0);
    }
}

/// Presses `key` inside the row `within`.
fn tap_in(rt: &mut Runtime<Ui>, within: &str, key: &str) {
    tap(rt, &format!("{within}/{key}"));
}

fn shown(rt: &Runtime<Ui>) -> Vec<Suggestion> {
    rt.state.assistant.suggestions.shown.clone()
}

/// How many example questions the page shows.
fn examples(rt: &Runtime<Ui>) -> usize {
    (0..4)
        .filter(|index| rt.find(&format!("example{index}")).is_some())
        .count()
}

/// Switching to Assistant remembers the pane it came from (and the
/// meeting open there); the Labs page counts as Labs; other panes and a
/// link back count as none.
#[test]
fn opening_assistant_remembers_where_it_came_from() {
    let mut ui = super::super::super::scenes::mac_fixture();
    let mut tx = Tx::default();
    let from = |ui: &Ui| ui.assistant.suggestions.from;
    ui.pane = Pane::Meetings;
    ui.meetings.selected = Some("standup".into());
    ui.select_pane(Pane::Assistant, &mut tx);
    assert_eq!(from(&ui), Some(From::Meetings));
    assert_eq!(
        ui.assistant.suggestions.open_meeting.as_deref(),
        Some("standup")
    );
    for (pane, expected) in [
        (Pane::History, Some(From::History)),
        (Pane::Output, Some(From::Output)),
        (Pane::General, None),
        (Pane::About, None),
    ] {
        ui.select_pane(pane, &mut tx);
        ui.select_pane(Pane::Assistant, &mut tx);
        assert_eq!(from(&ui), expected, "{pane:?}");
        assert_eq!(ui.assistant.suggestions.open_meeting, None);
    }
    ui.select_pane(Pane::General, &mut tx);
    ui.labs_open = true;
    ui.select_pane(Pane::Assistant, &mut tx);
    assert_eq!(from(&ui), Some(From::Labs));
    // Back to Assistant from a place a suggestion opened: not the person's
    // own way from that pane.
    ui.select_pane(Pane::History, &mut tx);
    super::super::jump::came_for_suggestion(&mut ui);
    super::super::jump::back(&mut ui, &mut tx);
    assert_eq!(ui.pane, Pane::Assistant);
    assert_eq!(from(&ui), None);
}

/// × hides a row for good: it leaves the page, its key is kept, and the
/// next row moves up. An example question takes the place freed.
#[test]
fn hiding_a_row_keeps_its_key_and_frees_its_place() {
    let mut rt = scene("assistant-start-broke", |_| {});
    let rows = shown(&rt);
    assert_eq!(rows.len(), 3, "{rows:?}");
    assert!(matches!(rows[0], Suggestion::CleanupFailed { .. }));
    assert_eq!(examples(&rt), 1);
    let key = rows[0].key();
    tap_in(&mut rt, "suggestion0", "hide");
    let state = &rt.state.assistant.suggestions;
    assert!(state.hidden.has(&key), "kept: {key}");
    assert!(
        !shown(&rt)
            .iter()
            .any(|row| matches!(row, Suggestion::CleanupFailed { .. }))
    );
    assert!(matches!(shown(&rt)[0], Suggestion::NotesFailed(_)));
    assert_eq!(examples(&rt), 2, "the block fills to four");
}

/// Pressing what broke opens where it is fixed and leaves the row: it
/// stays until it works again or is hidden. Back to Assistant returns to
/// the start page.
#[test]
fn pressing_what_broke_does_not_hide_it() {
    let mut rt = scene("assistant-start-broke", |_| {});
    let key = shown(&rt)[0].key();
    tap_in(&mut rt, "suggestion0", "go");
    assert_ne!(rt.state.pane, Pane::Assistant, "AI cleanup's settings");
    assert!(rt.find("back-to-assistant").is_some(), "Back to Assistant");
    assert!(
        !rt.state.assistant.suggestions.hidden.has(&key),
        "not hidden"
    );
    tap(&mut rt, "back-to-assistant");
    assert_eq!(rt.state.pane, Pane::Assistant);
    assert_eq!(rt.state.assistant.open, None, "the start page");
    assert!(matches!(shown(&rt)[0], Suggestion::CleanupFailed { .. }));
    assert!(rt.find("suggestion0").is_some());

    // A meeting's notes open its Summary, with the way back.
    tap_in(&mut rt, "suggestion1", "go");
    assert_eq!(rt.state.pane, Pane::Meetings);
    assert_eq!(rt.state.meetings.selected.as_deref(), Some("design"));
    assert_eq!(
        rt.state.meetings.tab,
        Some(super::super::super::meeting_tabs::MeetingTab::Summary)
    );
    assert!(rt.find("back-to-assistant").is_some());
}

/// The examples fill the block to four, without one a row already asks.
#[test]
fn examples_fill_the_block_to_four() {
    assert_eq!(examples(&scene("assistant-start", |_| {})), 4);
    assert_eq!(examples(&scene("assistant-updated", |_| {})), 3);
    assert_eq!(examples(&scene("assistant-start-meetings", |_| {})), 1);
    // From History: the row asks the second example, which steps aside.
    let rt = scene("assistant-start", |ui| {
        super::scenes::show(
            ui,
            suggest::Signals {
                from: Some(From::History),
                ..suggest::Signals::default()
            },
        );
    });
    assert_eq!(shown(&rt), [Suggestion::WrongWord]);
    let texts: Vec<String> = rt.elements().into_iter().filter_map(|e| e.text).collect();
    let wrong = "Some words come out wrong. How do I fix that?";
    assert_eq!(texts.iter().filter(|text| *text == wrong).count(), 1);
    assert_eq!(examples(&rt), 3);
}

/// Whether Assistant's sidebar row carries a mark, and its colour's why.
fn marked(rt: &Runtime<Ui>) -> bool {
    rt.elements()
        .into_iter()
        .any(|e| e.key.as_deref() == Some("mark") && e.path.contains("nav-assistant"))
}

/// What broke marks the sidebar's Assistant row while another pane shows,
/// over an update's mark. Opening Assistant lands on the start page, where
/// the rows stay, and the mark goes: what broke is seen.
#[test]
fn the_trouble_mark_shows_until_assistant_opens() {
    let mut rt = scene("assistant-start-broke", |ui| {
        super::scenes::marked(ui);
        ui.assistant.mark = Some("3.3.0-vibe.7".into());
        let id = ui.assistant.store.start("Earlier", None, None);
        ui.assistant.store.push(
            id,
            crate::assistant::store::Turn {
                question: "Earlier".into(),
                answer: "An answer.".into(),
                ..Default::default()
            },
        );
        ui.assistant.open = Some(id);
    });
    assert!(marked(&rt), "the row is marked");
    assert!(super::trouble(&rt.state));
    let tree = rt.accessibility_json().to_string();
    assert!(
        tree.contains("Something stopped working"),
        "its description"
    );
    let keys: Vec<String> = suggest::broke(rt.state.assistant.suggestions.scene.as_ref().unwrap())
        .iter()
        .map(Suggestion::key)
        .collect();
    tap(&mut rt, "nav-assistant");
    assert_eq!(rt.state.pane, Pane::Assistant);
    assert_eq!(rt.state.assistant.open, None, "a new conversation");
    assert!(!super::trouble(&rt.state));
    let state = &rt.state.assistant.suggestions;
    assert!(keys.iter().all(|key| state.hidden.saw(key)), "seen");
    assert!(keys.iter().all(|key| !state.hidden.has(key)), "not hidden");
    assert!(rt.find("suggestion0").is_some(), "the rows stay");
    tap(&mut rt, "nav-general");
    assert!(!super::trouble(&rt.state));
}

/// A setup begun in another conversation and left is offered, the newest
/// one; the conversation open is not.
#[test]
fn a_setup_left_elsewhere_is_offered() {
    let mut ui = super::super::super::scenes::mac_fixture();
    ui.meetings.meetings.clear();
    ui.history.entries.clear();
    super::super::ask::begin(&mut ui);
    let id = ui.assistant.open.expect("a setup");
    let store = &ui.assistant.store;
    assert_eq!(super::signals::left(store, Some(id)), None, "it is open");
    let left = super::signals::left(store, None).expect("left");
    assert_eq!(left.conversation, id);
    assert!(!left.plan, "round one still to answer");
}
