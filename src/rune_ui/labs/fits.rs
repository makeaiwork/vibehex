//! Labs › Discover's "Fits you": while browsing (no chip, no search),
//! above New in Labs, up to four labs that fit the person and are off, as
//! "New for you" picks them (`assistant::setup::fresh`), each drawn with
//! the page's own card, its switch included, and one line more: "For
//! you:", how it goes now when the app counted it, else what changes with
//! it on. "Choose with Assistant" opens the same picks in Assistant as one
//! list to tick. Labs shown here aren't repeated below, and the group
//! goes when nothing fits. Only where Assistant's pane shows.
//!
//! The picks are read when the page opens and again when the settings or
//! the setups change, never per frame; they read what came since the last
//! setup, never the person's use, so "For you" is the "With it" sentence.

use rune::{El, Tx, col, el, row, text};
use rune_kit::{Button, ButtonVariant};

use super::super::Ui;
use super::super::controls::{DESCRIPTION, Paint, PushSize};
use super::super::lists::{Tone, tone_tag};
use super::super::sidebar::Pane;
use super::{Lab, Row};
use crate::app_settings::AppSettings;
use crate::assistant::setup::fresh::Pick;
use crate::assistant::setup::since::Since;
use crate::desktop::i18n::tr;

/// The most labs the group shows.
const SHOWN: usize = 4;

/// The picks, and the settings and number of setups they were read at.
#[derive(Default)]
pub(in crate::rune_ui) struct Fits {
    read: Option<(AppSettings, usize)>,
    picks: Vec<Pick>,
    /// A scene's setup and releases since: nothing else is read in one.
    scene: Option<Since>,
}

/// Reads the picks again when the Labs page shows and the settings or the
/// setups changed since. True when it read them. A scene or a test reads
/// them only from what it set up.
pub(in crate::rune_ui) fn poll(ui: &mut Ui) -> bool {
    if !ui.labs_open || !Pane::Assistant.shown(ui) {
        return false;
    }
    if ui.is_fixture() && ui.lab_store.fits.scene.is_none() {
        return false;
    }
    let setups = ui.assistant.store.setups().len();
    let read = ui.lab_store.fits.read.as_ref();
    if read.is_some_and(|(settings, count)| *settings == ui.settings && *count == setups) {
        return false;
    }
    refresh(ui);
    true
}

/// Reads the picks now.
fn refresh(ui: &mut Ui) {
    let since = match &ui.lab_store.fits.scene {
        Some(since) => Some(since.clone()),
        None => super::super::assistant::fresh::since(ui),
    };
    let picks = super::super::assistant::fresh::found(ui, since.as_ref()).picks;
    ui.lab_store.fits = Fits {
        read: Some((ui.settings.clone(), ui.assistant.store.setups().len())),
        picks,
        scene: ui.lab_store.fits.scene.take(),
    };
}

/// Scenes: the Labs page with Fits you, read from the picker scenes'
/// setup and release, without the person's use, on Assistant's stand-ins
/// for this computer.
pub(in crate::rune_ui) fn scene(ui: &mut Ui) {
    *ui = super::super::scenes::mac_fixture();
    ui.labs_open = true;
    ui.assistant.fixture = true;
    ui.lab_store.fits.scene = Some(Since {
        usage: None,
        ..super::super::assistant::fresh::scenes::since()
    });
    refresh(ui);
}

/// The picks this page has a card for, with each one's "For you" line,
/// at most [`SHOWN`].
fn shown<'a>(ui: &'a Ui, rows: &'a [Row]) -> Vec<(&'a Row, &'a str)> {
    if !Pane::Assistant.shown(ui) {
        return Vec::new();
    }
    ui.lab_store
        .fits
        .picks
        .iter()
        .filter_map(|pick| {
            let row = rows.iter().find(|row| {
                matches!(row.lab, Lab::Feature(feature) if feature.title() == pick.title)
                    && !row.on
                    && !row.locked
            })?;
            let line = if pick.counted { &pick.now } else { &pick.with };
            Some((row, line.as_str()))
        })
        .take(SHOWN)
        .collect()
}

/// The labs the group shows, which the rest of the page leaves out.
pub(super) fn labs(ui: &Ui, rows: &[Row]) -> Vec<Lab> {
    shown(ui, rows)
        .into_iter()
        .map(|(row, _)| row.lab)
        .collect()
}

/// The group: its heading with the count and "Choose with Assistant", the
/// line under it, then the cards two by two. `None` when nothing fits.
pub(super) fn group(ui: &Ui, p: Paint, rows: &[Row], layer: &mut Vec<El>) -> Option<El> {
    let shown = shown(ui, rows);
    if shown.is_empty() {
        return None;
    }
    let choose = Button::new(tr("Choose with Assistant"))
        .variant(ButtonVariant::Neutral)
        .on_press_tx(|ui: &mut Ui, tx: &mut Tx| super::super::assistant::fresh::from_labs(ui, tx))
        .build_small_push(p.k)
        .key("fits-choose");
    let heading = row()
        .key("fits-heading")
        .gap(6.0)
        .center_y()
        .child(p.section_label(tr("Fits you")).key("label"))
        .child(tone_tag(
            p,
            "fits-count",
            &shown.len().to_string(),
            Tone::Neutral,
        ))
        .child(el().key("space").grow(1.0))
        .child(choose);
    let mut cards = col().key("fits").gap(super::store::GAP);
    for (index, two) in shown.chunks(2).enumerate() {
        cards = cards.child(super::store::pair(
            &format!("fits-row{index}"),
            two.iter()
                .map(|(entry, line)| {
                    super::store::card(ui, p, entry, (false, true), Some(*line), layer)
                })
                .collect(),
        ));
    }
    Some(
        col()
            .key("fits-group")
            .role("group")
            .label(tr("Fits you"))
            .gap(8.0)
            .child(heading)
            .child(
                text(tr(
                    "Picked from your settings and how you use VibeHex. Each card says what changes for you. Nothing is turned on until you say so.",
                ))
                .key("fits-lead")
                .font(DESCRIPTION)
                .color(p.muted())
                .wrap(),
            )
            .child(cards.mt(4.0)),
    )
}

#[cfg(test)]
mod tests {
    use rune::Runtime;

    use super::super::super::{SCENES, Ui, WINDOW_SIZE};
    use super::super::{Lab, row_key};
    use crate::labs::Feature;

    fn scene(name: &str) -> Runtime<Ui> {
        let (_, apply) = SCENES.iter().find(|(scene, _)| *scene == name).unwrap();
        super::super::super::assistant::layout::window(*apply, WINDOW_SIZE)
    }

    /// The labs the group shows, by id.
    fn shown(rt: &Runtime<Ui>) -> Vec<String> {
        rt.elements()
            .into_iter()
            .filter_map(|element| element.key)
            .filter_map(|key| key.strip_prefix("lab-fits-").map(str::to_owned))
            .collect()
    }

    /// Presses the switch of the lab `id` shows in the group.
    fn flip(rt: &mut Runtime<Ui>, id: &str) {
        let feature = Feature::ALL
            .into_iter()
            .find(|feature| feature.name() == id)
            .unwrap();
        let switch = row_key(Lab::Feature(feature));
        let card = format!("lab-fits-{id}");
        let rect = rt
            .elements()
            .into_iter()
            .find(|element| {
                element.path.contains(&format!("/{card}/"))
                    && element.key.as_deref() == Some(switch)
            })
            .unwrap_or_else(|| panic!("no switch on {card}"))
            .rect;
        let (x, y) = (rect[0] + rect[2] / 2.0, rect[1] + rect[3] / 2.0);
        rt.pointer_down(x, y);
        rt.pointer_up(x, y);
        for _ in 0..30 {
            rt.frame(1.0 / 60.0);
        }
    }

    /// Fits you shows the picks, each with "For you", not again below; a
    /// pick turned on from its card leaves it, and with nothing left to
    /// fit the group goes.
    #[test]
    fn the_group_shows_the_picks_and_loses_one_turned_on() {
        let mut rt = scene("mac-labs-fits");
        let first = shown(&rt);
        assert!(
            !first.is_empty() && first.len() <= super::SHOWN,
            "{first:?}"
        );
        for id in &first {
            assert!(rt.find(&format!("lab-card-{id}")).is_none(), "{id} twice");
            assert!(
                rt.find(&format!("lab-featured-{id}")).is_none(),
                "{id} twice"
            );
            let for_you = rt.elements().into_iter().any(|element| {
                element.path.contains(&format!("/lab-fits-{id}/"))
                    && element.key.as_deref() == Some("for-you")
            });
            assert!(for_you, "{id} says what it changes");
        }
        flip(&mut rt, &first[0]);
        let after = shown(&rt);
        assert!(!after.contains(&first[0]), "{after:?}");
        assert!(
            rt.find(&format!("lab-card-{}", first[0])).is_some(),
            "below now"
        );
        for _ in 0..40 {
            let Some(id) = shown(&rt).into_iter().next() else {
                break;
            };
            flip(&mut rt, &id);
        }
        assert!(shown(&rt).is_empty());
        assert!(rt.find("fits-group").is_none(), "nothing fits: no group");
        // Nor where Assistant's pane doesn't show.
        let mut rt = scene("mac-labs-fits");
        rt.update(|ui, _| ui.settings.assistant.enabled = false);
        for _ in 0..10 {
            rt.frame(1.0 / 60.0);
        }
        assert!(rt.find("fits-group").is_none());
    }
}
