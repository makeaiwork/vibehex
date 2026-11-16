//! "New for you" in the window: its three ways in, Add, and a save of the
//! switch with what its row sets.

use rune::Runtime;

use super::super::super::{SCENES, Ui, WINDOW_SIZE};
use super::super::changes::{self as act, At};
use super::super::layout::window;
use crate::assistant::changes::{Card, CardKind, Decision};
use crate::assistant::setup::flow::{self, Step};
use crate::assistant::suggest::{From, Signals, Suggestion};

/// Scene `name`, then what `then` sets up, laid out in a window tall
/// enough to show the whole conversation.
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
        (WINDOW_SIZE.0, 4000.0),
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

/// The open conversation's "New for you" list, where the setup stands at
/// its plan, asked of no model.
fn picker(ui: &Ui) -> (At, Card) {
    assert!(
        ui.assistant
            .answering
            .as_ref()
            .is_none_or(|answering| Some(answering.conversation) != ui.assistant.open),
        "no model was asked here"
    );
    let open = ui.assistant.conversation().expect("a conversation");
    let Step::Plan { turn } = flow::step(&open.turns, ui.assistant.store.setups()) else {
        panic!("not at a plan: {:?}", open.turns);
    };
    assert_eq!(
        open.turns[turn].question,
        crate::assistant::setup::fresh::QUESTION
    );
    let card = open.turns[turn].cards[0].clone();
    assert_eq!(card.kind, CardKind::Plan);
    assert!(!card.items.is_empty());
    assert!(
        card.items
            .iter()
            .all(|item| !item.now.is_empty() && !item.with.is_empty())
    );
    let at = At {
        conversation: open.id,
        turn,
        card: 0,
    };
    (at, card)
}

/// The start page's new-features row and its "Which Labs features…" (the
/// row from Labs and the example question) open the picker, with no
/// model asked; the row from Labs shows with no model connected too.
#[test]
fn the_start_page_opens_the_picker() {
    // The new-features row.
    let mut rt = scene("assistant-start-broke", |_| {});
    let at = rt
        .state
        .assistant
        .suggestions
        .shown
        .iter()
        .position(|row| matches!(row, Suggestion::NewLabs { .. }))
        .expect("the new-features row");
    tap(&mut rt, &format!("suggestion{at}/go"));
    picker(&rt.state);
    // From Labs, with no model connected: it asks none.
    let from_labs = |ui: &mut Ui| {
        super::super::suggestions::scenes::show(
            ui,
            Signals {
                from: Some(From::Labs),
                ..Signals::default()
            },
        )
    };
    let mut rt = scene("assistant-connect", from_labs);
    assert_eq!(
        rt.state.assistant.suggestions.shown,
        [Suggestion::WhichLabs]
    );
    assert!(super::super::connect::needed(&rt.state));
    tap(&mut rt, "suggestion0/go");
    picker(&rt.state);
    // The example question.
    let mut rt = scene("assistant-start", |_| {});
    tap(&mut rt, "example0");
    picker(&rt.state);
}

/// Opened while a model answers in another conversation, the picker
/// comes at once in a new one, and the answer goes on.
#[test]
fn the_picker_opens_while_a_model_answers_elsewhere() {
    let mut ui = super::super::super::scenes::mac_fixture();
    super::super::fixture::answering(&mut ui);
    let answering = ui.assistant.open;
    super::begin(&mut ui);
    assert_ne!(ui.assistant.open, answering);
    picker(&ui);
    assert!(ui.assistant.answering.is_some(), "the other answer goes on");
}

/// Labs' "Choose with Assistant" opens Assistant on the picker.
#[test]
fn labs_choose_with_assistant_opens_the_picker() {
    let mut rt = scene("mac-labs-fits", |_| {});
    assert!(rt.find("fits-group").is_some());
    tap(&mut rt, "fits-choose");
    assert_eq!(rt.state.pane, super::super::super::sidebar::Pane::Assistant);
    assert!(!rt.state.labs_open);
    picker(&rt.state);
}

/// Add moves a feature from More to try into the list, checked; it counts
/// only then. Decided, More to try adds nothing.
#[test]
fn add_moves_a_feature_into_the_list() {
    let mut rt = scene("assistant-new-for-you-more", |_| {});
    let (at, before) = picker(&rt.state);
    let key = before.more[0].key.clone();
    tap(&mut rt, "more0/add");
    let card = act::card(&rt.state, at).unwrap().clone();
    let added = card.items.iter().find(|item| item.key == key).unwrap();
    assert!(added.included);
    assert!(card.more.iter().all(|item| item.key != key));
    assert_eq!(card.items.len(), before.items.len() + 1);
    let (all, selected) = crate::assistant::changes::counts::held(&card);
    assert_eq!(
        (all.changes, selected.changes),
        (card.items.len(), card.items.len())
    );
    rt.update(|ui, _| act::decline(ui, at));
    for _ in 0..10 {
        rt.frame(1.0 / 60.0);
    }
    assert!(rt.find("more0").is_some(), "the fold stays");
    assert!(rt.find("more0/add").is_none(), "with nothing to add");
}

/// Apply saves each checked switch with what its row sets: YOLO mode with
/// the seconds picked in its row, More dictation shortcuts with the key
/// recorded there (a Mac's), checked against General's rules first. Undo
/// puts it all back.
#[test]
fn apply_saves_the_switch_and_what_its_row_sets() {
    let mut ui = super::super::super::scenes::mac_fixture();
    super::scenes::new_for_you(&mut ui);
    let (at, card) = picker(&ui);
    let before = ui.settings.clone();
    let row = |key: &str| card.items.iter().position(|item| item.key == key).unwrap();
    let yolo = row("labs.yolo_mode");
    let three = card.items[yolo]
        .companion
        .as_ref()
        .expect("YOLO mode's seconds in its row")
        .options
        .iter()
        .position(|option| *option == serde_json::json!("3"))
        .unwrap();
    act::pick_companion(&mut ui, at, yolo, false, three);
    #[cfg(target_os = "macos")]
    let key = {
        use super::super::super::ShortcutTarget;
        use super::super::super::settings::Capture;
        let rules = super::super::super::shortcut_rules();
        let chord =
            |text: &str| crate::control::settings::parse_hotkey(text, rules.key_named).unwrap();
        let extra = row("labs.more_dictation_shortcuts");
        let target = ShortcutTarget::ExtraDictation(0);
        super::super::keys::record(&mut ui, at, extra, target);
        assert_eq!(ui.assistant.recording, Some((at, extra)));
        // The dictation shortcut's own keys are refused, as General does.
        let taken = ui.settings.dictation_hotkey.clone();
        super::super::keys::recorded(&mut ui, target, Ok(taken));
        assert!(matches!(
            ui.capture,
            Capture::Listening {
                message: Some(_),
                ..
            }
        ));
        let f5 = chord("control+option+f5");
        super::super::keys::recorded(&mut ui, target, Ok(f5.clone()));
        assert!(matches!(ui.capture, Capture::Idle));
        assert_eq!(ui.assistant.recording, None);
        let item = &act::card(&ui, at).unwrap().items[extra];
        assert_eq!(item.shortcut.as_ref(), Some(&f5));
        assert_eq!(ui.settings, before, "nothing saved before Apply");
        f5
    };
    act::apply(&mut ui, at);
    let applied = act::card(&ui, at).unwrap().clone();
    assert!(
        matches!(applied.decision, Decision::Applied { .. }),
        "{:?}",
        applied.decision
    );
    assert!(ui.settings.labs.yolo_mode);
    assert_eq!(ui.settings.labs.yolo_mode_seconds, 3);
    #[cfg(target_os = "macos")]
    {
        assert!(ui.settings.labs.more_dictation_shortcuts);
        assert_eq!(ui.settings.extra_dictation_hotkeys.len(), 1);
        assert_eq!(ui.settings.extra_dictation_hotkeys[0].binding, Some(key));
        assert_eq!(
            ui.settings.extra_dictation_hotkeys[0].mode,
            crate::app_settings::ExtraDictationMode::default()
        );
    }
    act::undo(&mut ui, at);
    assert_eq!(act::card(&ui, at).unwrap().decision, Decision::Undone);
    assert!(!ui.settings.labs.yolo_mode);
    assert_eq!(
        ui.settings.labs.yolo_mode_seconds,
        before.labs.yolo_mode_seconds
    );
    assert!(!ui.settings.labs.more_dictation_shortcuts);
    assert_eq!(
        ui.settings.extra_dictation_hotkeys,
        before.extra_dictation_hotkeys
    );
}

/// With no key recorded, only the switch is saved.
#[cfg(target_os = "macos")]
#[test]
fn without_a_key_only_the_switch_is_saved() {
    let mut ui = super::super::super::scenes::mac_fixture();
    super::scenes::new_for_you(&mut ui);
    let (at, _) = picker(&ui);
    act::apply(&mut ui, at);
    assert!(ui.settings.labs.more_dictation_shortcuts);
    assert!(
        ui.settings
            .extra_dictation_hotkeys
            .iter()
            .all(|row| row.binding.is_none())
    );
}
