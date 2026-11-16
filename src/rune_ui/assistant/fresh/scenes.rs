//! "New for you" in scenes: the picker as the setup machine writes it, on
//! the Assistant fixture's Mac, after a setup two weeks before the
//! fixture's day, a release since that brought every Labs feature, and
//! the person's use as History counts it. Everything else that fits is on
//! already, so four features are left to pick. Nothing is read from the
//! disk.

use serde_json::{Map, Value};

use super::super::super::Ui;
use super::super::changes::At;
use super::super::fixture::{self, AT, MINUTE};
use crate::assistant::recordings::usage::Usage;
use crate::assistant::setup::Answer;
use crate::assistant::setup::past::Setup;
use crate::assistant::setup::since::{Release, Since};

/// The four features the picker scenes leave off: YOLO mode, whose "Now"
/// the app counted, and three more.
pub(in crate::rune_ui) const PICKED: [&str; 4] = [
    "labs.yolo_mode",
    "labs.more_dictation_shortcuts",
    "labs.vocabulary_quick_add",
    "labs.recent_dictations",
];

const DAY: u64 = 24 * 60 * MINUTE;

/// Someone who dictates, code too, in English, with no AI cleanup, set up
/// two weeks ago; a release since brought every Labs feature; they
/// dictated 212 times in 30 days.
pub(in crate::rune_ui) fn since() -> Since {
    let answer = |id: &str, chosen: &[&str]| Answer {
        id: id.into(),
        chosen: chosen.iter().map(|id| (*id).to_owned()).collect(),
        changes: Vec::new(),
    };
    let keys: Vec<String> = crate::assistant::guide::articles()
        .iter()
        .filter(|article| article.is_lab())
        .filter_map(|article| article.keys.first().cloned())
        .collect();
    Since {
        last: Some(Setup {
            at_ms: AT - 14 * DAY,
            answers: vec![
                answer("use", &["dictation_any", "dictation_code"]),
                answer("language", &["en"]),
                answer("cleanup", &["none"]),
            ],
            ..Setup::default()
        }),
        releases: vec![Release {
            version: "3.4.0".into(),
            date_ms: AT - 7 * DAY,
            title: "Labs features".into(),
            keys,
        }],
        from: None,
        usage: Some(Usage {
            dictations: 640,
            last_30_days: 212,
            ..Usage::default()
        }),
    }
}

/// The picker: four features that fit, one whose "Now" the app counted,
/// More to try folded.
pub(in crate::rune_ui) fn new_for_you(ui: &mut Ui) {
    fixture::open(ui);
    settled(ui, &since());
    super::open(ui, Some(since()));
}

/// Turns on every other feature `since` would pick, so only [`PICKED`]
/// are left: what else suits the person is on already.
pub(in crate::rune_ui) fn settled(ui: &mut Ui, since: &Since) {
    let rules = super::super::super::shortcut_rules();
    for _ in 0..64 {
        let found = super::found(ui, Some(since));
        let Some(other) = found
            .picks
            .iter()
            .find(|pick| !PICKED.contains(&pick.key.as_str()))
        else {
            return;
        };
        let on = Map::from_iter([(other.key.clone(), Value::Bool(true))]);
        match crate::control::settings::check(&ui.settings, &on, &rules) {
            Ok(change) => ui.settings = change.settings,
            Err(_) => return,
        }
    }
}

/// The same with More to try unfolded, each feature with Add.
pub(in crate::rune_ui) fn new_for_you_more(ui: &mut Ui) {
    new_for_you(ui);
    if let Some(conversation) = ui.assistant.open {
        ui.assistant.more_open.push(At {
            conversation,
            turn: 0,
            card: 0,
        });
    }
}
