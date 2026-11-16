//! "New for you": the short setup that asks nothing. From the answers a
//! setup would start from now (the last setup's, else what the settings
//! show) the app picks the Labs features that fit the person and are off,
//! at most [`PICKS`], and shows them as one list to tick: each row says
//! what the feature does (its Labs card's line), how things go now, and
//! what changes with it on (the article's `now:` and `with:`). The other
//! features that came since the last setup wait under "More to try". No
//! model is asked; the Labs page's "Fits you" group reads the same picks.
//!
//! A feature fits as a follow-up question's does (`follow::fits`, through
//! the gate: this computer has it, it isn't on, it wasn't answered or
//! declined before). One that came in a release since the last setup
//! counts for more, and so does one the person's own use speaks for
//! (`seen`): there "Now" is a number the app counted in History, only
//! while Assistant may read recordings. It never says what it cannot
//! count.

use serde_json::Value;

use super::since::{self, Release};
use super::{Answer, Spoken, follow, plan};
use crate::app_settings::AppSettings;
use crate::assistant::changes::parse::Proposal;
use crate::assistant::changes::{self, Card, CardKind, Item};
use crate::assistant::facts::State;
use crate::assistant::guide::{self, Article};
use crate::assistant::offer::{Gate, Origin, Verdict};
use crate::assistant::recordings::usage::{LONG_WORDS, Usage};
use crate::desktop::i18n::{tr, tr_fill, tr_named};
use crate::labs::Feature;

/// The most features picked.
pub const PICKS: usize = 5;
/// The most features under "More to try".
pub const MORE: usize = 30;
/// What a feature that came since the last setup adds to its fit, and
/// what the person's own use adds.
const NEW: usize = 2;
const SEEN: usize = 3;
/// The fewest dictations in 30 days, long dictations kept, and the share
/// into a terminal or a code editor, that the use speaks for a feature.
const OFTEN: usize = 20;
const LONG: usize = 5;
const CODE_SHARE: u32 = 30;

/// What a press asks, in English for `tr`: the turn's question.
pub const QUESTION: &str = "What's new for me?";

const NOW_OFTEN: &str =
    "You dictated {} times in the last 30 days. Each message still needs Return from you.";
const NOW_LONG: &str = "{count} of your dictations in History ran over {words} words.";
const NOW_CODE: &str = "{}% of your dictations go into a terminal or a code editor.";

const GAINED_ONE: &str = "Since your setup on {} VibeHex gained 1 Labs feature.";
const GAINED: &str = "Since your setup on {date} VibeHex gained {count} Labs features.";
const FITS_ONE: &str = "One fits how you use it.";
const FITS: &str = "{} fit how you use it.";
const ROWS: &str =
    "Each row says what it does and what changes for you. Untick what you don't want.";
const REST: &str = "None stands out for how you use VibeHex. They are under More to try.";
const NOTHING: &str = "Nothing new fits right now: what suits how you work is already on.";

/// One feature picked for the person.
#[derive(Clone, Debug, PartialEq)]
pub struct Pick {
    /// Its switch, `labs.yolo_mode`.
    pub key: String,
    /// Its Labs card's title and line, in English.
    pub title: &'static str,
    pub does: &'static str,
    /// How things go now and with it on, in Assistant's language.
    pub now: String,
    pub with: String,
    /// It came in a release since the last setup.
    pub new: bool,
    /// "Now" is a number counted from the person's own use.
    pub counted: bool,
}

/// A feature that came since and wasn't picked.
#[derive(Clone, Debug, PartialEq)]
pub struct More {
    pub key: String,
    pub title: &'static str,
    pub does: &'static str,
}

/// What "New for you" shows.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fresh {
    pub picks: Vec<Pick>,
    pub more: Vec<More>,
    /// How many Labs features this computer has came since the last setup.
    pub gained: usize,
}

/// The answers a setup would start from now: round one as answered last
/// time (`since`), the rest as the settings have it.
pub fn answers(since: Option<&since::Since>, settings: &AppSettings, state: &State) -> Vec<Answer> {
    let mut asks = super::questions_for(state.os, Spoken::of(settings, state));
    super::preselect(&mut asks, settings, state);
    if let Some(since) = since {
        since::preselect(&mut asks, since);
        super::preselect(&mut asks, settings, state);
    }
    let shown: Vec<bool> = asks.iter().map(|ask| super::shows(&asks, ask)).collect();
    for (ask, shown) in asks.iter_mut().zip(shown) {
        ask.done = shown;
    }
    super::answers(&asks)
}

/// The Labs feature an article describes.
fn feature(article: &Article) -> Option<Feature> {
    let name = article.id.strip_prefix("labs/")?;
    Feature::available().find(|feature| feature.name() == name)
}

/// What the person's own use says for the feature behind `key`: the
/// sentence for "Now", a number the app counted.
fn seen(key: &str, usage: &Usage) -> Option<String> {
    match key {
        "labs.yolo_mode" if usage.last_30_days >= OFTEN => {
            Some(tr_fill(NOW_OFTEN, &usage.last_30_days.to_string()))
        }
        "labs.more_dictation_shortcuts" if usage.long >= LONG => Some(tr_named(
            NOW_LONG,
            &[
                ("count", &usage.long.to_string()),
                ("words", &LONG_WORDS.to_string()),
            ],
        )),
        "labs.screen_vocabulary" if usage.code_share >= CODE_SHARE => {
            Some(tr_fill(NOW_CODE, &usage.code_share.to_string()))
        }
        _ => None,
    }
}

/// The features that fit `answers` and are off, best first, and the other
/// ones that came in `releases`. `usage` only while Assistant may read
/// recordings.
pub fn fresh(
    answers: &[Answer],
    settings: &AppSettings,
    state: &State,
    usage: Option<&Usage>,
    releases: &[Release],
) -> Fresh {
    let gate = Gate::new(settings, state);
    let came = |key: &str| {
        releases
            .iter()
            .any(|release| release.keys.iter().any(|known| known == key))
    };
    let mut found: Vec<(usize, Pick)> = follow::askable(&gate)
        .filter_map(|(article, key)| {
            let fit = follow::fits(article, answers)?;
            if gate.ask(key, settings, false) != Verdict::Show
                || article.now.is_empty()
                || article.with.is_empty()
            {
                return None;
            }
            let feature = feature(article)?;
            let new = came(key);
            let counted = usage.and_then(|usage| seen(key, usage));
            let score = fit + if new { NEW } else { 0 } + if counted.is_some() { SEEN } else { 0 };
            let pick = Pick {
                key: key.to_owned(),
                title: feature.title(),
                does: feature.summary(),
                counted: counted.is_some(),
                now: counted.unwrap_or_else(|| tr(&article.now).to_owned()),
                with: tr(&article.with).to_owned(),
                new,
            };
            Some((score, pick))
        })
        .collect();
    found.sort_by(|(a, one), (b, other)| b.cmp(a).then(one.title.cmp(other.title)));
    let picks: Vec<Pick> = found
        .into_iter()
        .map(|(_, pick)| pick)
        .take(PICKS)
        .collect();
    let mut gained = 0;
    let mut more = Vec::new();
    for article in guide::articles().iter().filter(|article| article.is_lab()) {
        let Some(key) = article.keys.first().map(String::as_str) else {
            continue;
        };
        if !came(key) || gate.here(key).is_err() || !article.fits(&state.hardware) {
            continue;
        }
        let Some(feature) = feature(article) else {
            continue;
        };
        gained += 1;
        let off = changes::current(settings, key) == Some(Value::Bool(false));
        if off && !picks.iter().any(|pick| pick.key == key) && more.len() < MORE {
            more.push(More {
                key: key.to_owned(),
                title: feature.title(),
                does: feature.summary(),
            });
        }
    }
    more.sort_by_key(|more| more.title);
    Fresh {
        picks,
        more,
        gained,
    }
}

/// The list to tick: a row per pick, checked unless the gate holds it,
/// grouped by area, and the rest under "More to try". `None` with nothing
/// to show.
pub fn card(fresh: &Fresh, settings: &AppSettings, state: &State) -> Option<Card> {
    let gate = Gate::new(settings, state);
    let line = |key: &str, does: &str, optional: bool| Proposal {
        key: key.to_owned(),
        value: Some(Value::Bool(true)),
        why: tr(does).to_owned(),
        plan: true,
        optional,
        line: key.to_owned(),
        ..Proposal::default()
    };
    let rows = |lines: Vec<Proposal>| -> Vec<Item> {
        changes::resolve(&lines, &gate, Origin::Plan)
            .cards
            .into_iter()
            .filter(|card| card.kind == CardKind::Plan)
            .flat_map(|card| card.items)
            .collect()
    };
    let mut items = rows(
        fresh
            .picks
            .iter()
            .map(|pick| line(&pick.key, pick.does, false))
            .collect(),
    );
    for item in &mut items {
        if let Some(pick) = fresh.picks.iter().find(|pick| pick.key == item.key) {
            item.now = pick.now.clone();
            item.with = pick.with.clone();
        }
    }
    items.sort_by_key(|item| plan::area(&item.key));
    let more = rows(
        fresh
            .more
            .iter()
            .map(|more| line(&more.key, more.does, true))
            .collect(),
    );
    (!items.is_empty() || !more.is_empty()).then(|| Card {
        kind: CardKind::Plan,
        items,
        more,
        ..Card::default()
    })
}

/// What the turn says over the list, in Assistant's language. `last_ms`
/// is the last setup's day, 0 without one.
pub fn opening(fresh: &Fresh, last_ms: u64) -> String {
    let mut parts = Vec::new();
    if last_ms > 0 {
        let day = since::date(last_ms);
        match fresh.gained {
            0 => {}
            1 => parts.push(tr_fill(GAINED_ONE, &day)),
            count => parts.push(tr_named(
                GAINED,
                &[("date", &day), ("count", &count.to_string())],
            )),
        }
    }
    match (fresh.picks.len(), fresh.more.is_empty()) {
        (0, true) => return tr(NOTHING).to_owned(),
        (0, false) => parts.push(tr(REST).to_owned()),
        (1, _) => parts.extend([tr(FITS_ONE).to_owned(), tr(ROWS).to_owned()]),
        (count, _) => parts.extend([tr_fill(FITS, &count.to_string()), tr(ROWS).to_owned()]),
    }
    parts.join(" ")
}

/// Every English phrase this file shows, for the translation tables'
/// test.
#[cfg(test)]
pub fn words() -> Vec<&'static str> {
    vec![
        QUESTION, NOW_OFTEN, NOW_LONG, NOW_CODE, GAINED_ONE, GAINED, FITS_ONE, FITS, ROWS, REST,
        NOTHING,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> State {
        State {
            os: "macos",
            ..State::default()
        }
    }

    /// Someone who dictates, code too, with nothing set up before.
    fn coder() -> Vec<Answer> {
        let answer = |id: &str, chosen: &[&str]| Answer {
            id: id.into(),
            chosen: chosen.iter().map(|id| (*id).to_owned()).collect(),
            changes: Vec::new(),
        };
        vec![
            answer("use", &["dictation_any", "dictation_code"]),
            answer("language", &["en"]),
            answer("cleanup", &["none"]),
        ]
    }

    fn release(keys: &[&str]) -> Release {
        Release {
            version: "3.4.0".into(),
            keys: keys.iter().map(|key| (*key).to_owned()).collect(),
            ..Release::default()
        }
    }

    /// At most five, each off, each with what it does, how things go now
    /// and what changes, and none twice.
    #[test]
    fn picks_are_few_off_and_say_what_changes() {
        let settings = AppSettings::default();
        let found = fresh(&coder(), &settings, &state(), None, &[]);
        assert!(!found.picks.is_empty() && found.picks.len() <= PICKS);
        for pick in &found.picks {
            assert_eq!(
                changes::current(&settings, &pick.key),
                Some(Value::Bool(false)),
                "{}",
                pick.key
            );
            assert!(!pick.does.is_empty() && !pick.now.is_empty() && !pick.with.is_empty());
            assert!(!pick.new && !pick.counted);
            let twice = found.picks.iter().filter(|other| other.key == pick.key);
            assert_eq!(twice.count(), 1);
        }
        // With no release since, nothing came and nothing is under More.
        assert_eq!(found.gained, 0);
        assert!(found.more.is_empty());
    }

    /// A feature already on is never picked, and one that came since the
    /// last setup comes before an older one that fits as well.
    #[test]
    fn what_is_on_is_left_out_and_what_came_since_comes_first() {
        let mut settings = AppSettings::default();
        let all = fresh(&coder(), &settings, &state(), None, &[]);
        let first = all.picks[0].key.clone();
        let on = serde_json::Map::from_iter([(first.clone(), Value::Bool(true))]);
        let rules = crate::rune_ui::shortcut_rules();
        settings = crate::control::settings::check(&settings, &on, &rules)
            .map(|change| change.settings)
            .unwrap_or(settings);
        if changes::current(&settings, &first) == Some(Value::Bool(true)) {
            let rest = fresh(&coder(), &settings, &state(), None, &[]);
            assert!(rest.picks.iter().all(|pick| pick.key != first));
        }
        let settings = AppSettings::default();
        let last = all.picks.last().unwrap().key.clone();
        let since = fresh(&coder(), &settings, &state(), None, &[release(&[&last])]);
        let at = since
            .picks
            .iter()
            .position(|pick| pick.key == last)
            .unwrap();
        assert!(since.picks[at].new);
        assert!(at < all.picks.len() - 1 || all.picks.len() == 1);
        assert_eq!(since.gained, 1);
        // Picked, it isn't listed again under More to try.
        assert!(since.more.iter().all(|more| more.key != last));
    }

    /// The person's use speaks only where the app counted something, and
    /// only for the feature it is about.
    #[test]
    fn a_number_from_use_names_only_what_was_counted() {
        let usage = Usage {
            dictations: 400,
            last_30_days: 212,
            long: 38,
            ..Usage::default()
        };
        let often = seen("labs.yolo_mode", &usage).unwrap();
        assert!(often.contains("212"), "{often}");
        let long = seen("labs.more_dictation_shortcuts", &usage).unwrap();
        assert!(long.contains("38") && long.contains("60"), "{long}");
        assert_eq!(seen("labs.screen_vocabulary", &usage), None);
        assert_eq!(seen("labs.return_to_app", &usage), None);
        let quiet = Usage {
            last_30_days: 3,
            long: 1,
            ..Usage::default()
        };
        assert_eq!(seen("labs.yolo_mode", &quiet), None);
        assert_eq!(seen("labs.more_dictation_shortcuts", &quiet), None);
    }

    /// The list: a checked row per pick with its two sentences, the rest
    /// that came since under More to try, unchecked.
    #[test]
    fn the_list_has_a_row_per_pick_and_the_rest_under_more() {
        let settings = AppSettings::default();
        let state = state();
        let labs: Vec<String> = guide::articles()
            .iter()
            .filter(|article| article.is_lab())
            .filter_map(|article| article.keys.first().cloned())
            .collect();
        let keys: Vec<&str> = labs.iter().map(String::as_str).collect();
        let found = fresh(&coder(), &settings, &state, None, &[release(&keys)]);
        assert!(found.gained > found.picks.len());
        assert!(!found.more.is_empty());
        let card = card(&found, &settings, &state).unwrap();
        assert_eq!(card.kind, CardKind::Plan);
        assert!(!card.items.is_empty() && card.items.len() <= found.picks.len());
        for item in &card.items {
            assert_eq!(item.to, Value::Bool(true));
            assert!(!item.why.is_empty() && !item.now.is_empty() && !item.with.is_empty());
        }
        assert!(card.more.iter().all(|item| !item.included));
        assert!(
            card.more
                .iter()
                .all(|item| card.items.iter().all(|row| row.key != item.key))
        );
        // The rows stand by area, as a setup's plan has them.
        let areas: Vec<plan::Area> = card
            .items
            .iter()
            .map(|item| plan::area(&item.key))
            .collect();
        assert!(areas.windows(2).all(|pair| pair[0] <= pair[1]));
    }

    /// The opening counts what came and what fits, and says so plainly
    /// when nothing does.
    #[test]
    fn the_opening_counts_what_came_and_what_fits() {
        let pick = |key: &str| Pick {
            key: key.into(),
            title: "YOLO mode",
            does: "does",
            now: "now".into(),
            with: "with".into(),
            new: true,
            counted: false,
        };
        let four = Fresh {
            picks: vec![pick("a"), pick("b"), pick("c"), pick("d")],
            more: Vec::new(),
            gained: 23,
        };
        // 2026-11-02.
        let words = opening(&four, 1_793_577_600_000);
        assert!(
            words.contains("2026-11-02") && words.contains("23"),
            "{words}"
        );
        assert!(words.contains("4 fit"), "{words}");
        // Without a setup before, no day is named.
        assert!(!opening(&four, 0).contains("Since"));
        assert_eq!(opening(&Fresh::default(), 0), NOTHING);
    }
}
