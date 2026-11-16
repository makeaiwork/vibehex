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

use rune::{El, Tx, col, el, icon, row, text};
use rune_kit::{Button, ButtonVariant};

use super::super::Ui;
use super::super::assistant::face::FACE;
use super::super::controls::{DESCRIPTION, Paint, PushSize};
use super::super::list_detail::{FOLDED, UNFOLDED};
use super::super::lists::{Tone, tone_tag};
use super::super::sidebar::Pane;
use super::{Lab, Row};
use crate::app_settings::AppSettings;
use crate::assistant::setup::fresh::Pick;
use crate::assistant::setup::since::Since;
use crate::desktop::i18n::{tr, tr_fill};

/// The group's place among `settings.labs.groups`, where its fold is
/// remembered with the other groups'.
const GROUP: &str = "fits";

/// What a Fits you card says for the person, and whether it is open.
pub(super) struct ForYou<'a> {
    /// The feature's switch, `labs.yolo_mode`.
    pub(super) key: &'a str,
    pub(super) line: &'a str,
    pub(super) open: bool,
}

/// The most labs the group shows.
const SHOWN: usize = 4;

/// The picks, and the settings and number of setups they were read at.
#[derive(Default)]
pub(in crate::rune_ui) struct Fits {
    read: Option<(AppSettings, usize)>,
    picks: Vec<Pick>,
    /// The labs whose "For you" line is open: one line otherwise.
    open: Vec<Lab>,
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
    super::super::assistant::suggestions::load(ui);
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
        open: std::mem::take(&mut ui.lab_store.fits.open),
        scene: ui.lab_store.fits.scene.take(),
    };
}

/// Whether the group's cards show: open until folded by hand.
fn unfolded(ui: &Ui) -> bool {
    ui.settings.labs.groups.get(GROUP).copied().unwrap_or(true)
}

/// "Not for me" on a card: the feature leaves the group for its own
/// place below, and isn't offered on its own again.
fn dismiss(ui: &mut Ui, key: &str) {
    super::super::assistant::suggestions::dismiss(ui, key);
    refresh(ui);
}

/// The quiet × after a card's switch.
pub(super) fn dismiss_button(p: Paint, fit: &ForYou) -> El {
    let c = &p.k.c;
    let key = fit.key.to_owned();
    row()
        .key("fits-dismiss")
        .role("button")
        .label(tr("Not for me"))
        .help(tr("Not for me"))
        .focusable(true)
        .focus_ring(2.0, 0.0, c.border_focus)
        .size(22.0, 22.0)
        .ml(2.0)
        .center()
        .radius(p.k.radius.sm)
        .hover(c.hover)
        .press(c.pressed)
        .child(
            icon(&rune_theme_opencode::icons::CLOSE, 11.0)
                .key("glyph")
                .color(p.faint()),
        )
        .on_press(move |ui: &mut Ui| dismiss(ui, &key))
}

/// A card's line from Assistant: its face under the lab's tile, then what
/// changes for the person, on one line until pressed open.
pub(super) fn line(p: Paint, lab: Lab, fit: &ForYou) -> El {
    let c = &p.k.c;
    let words = text(fit.line.to_owned())
        .key("words")
        .font(DESCRIPTION)
        .color(p.accent())
        .grow(1.0)
        .shrink()
        .min_w(0.0)
        .a11y_hidden();
    let words = match fit.open {
        true => words.wrap(),
        false => words.truncate(),
    };
    row()
        .key("for-you")
        .role("button")
        .label(tr_fill("For you: {}", fit.line))
        .expanded(fit.open)
        .focusable(true)
        .focus_ring(2.0, 0.0, c.border_focus)
        .gap(10.0)
        .py(3.0)
        .items_start()
        .radius(p.k.radius.sm)
        .hover(c.hover)
        .press(c.pressed)
        .child(
            col()
                .key("face")
                .w(super::store::TILE)
                .center()
                .child(icon(&FACE, 16.0).key("glyph").color(p.accent())),
        )
        .child(words)
        .child(
            row().key("fold").pt(2.0).pr(2.0).child(
                icon(if fit.open { &UNFOLDED } else { &FOLDED }, 12.0)
                    .key("chevron")
                    .color(c.icon_faint),
            ),
        )
        .on_press(move |ui: &mut Ui| {
            let open = &mut ui.lab_store.fits.open;
            match open.iter().position(|known| *known == lab) {
                Some(at) => {
                    open.remove(at);
                }
                None => open.push(lab),
            }
        })
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
fn shown<'a>(ui: &'a Ui, rows: &'a [Row]) -> Vec<(&'a Row, ForYou<'a>)> {
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
            let fit = ForYou {
                key: &pick.key,
                line,
                open: ui.lab_store.fits.open.contains(&row.lab),
            };
            Some((row, fit))
        })
        .take(SHOWN)
        .collect()
}

/// The labs the group shows, which the rest of the page leaves out.
pub(super) fn labs(ui: &Ui, rows: &[Row]) -> Vec<Lab> {
    // Folded, the group shows none: they keep their places below.
    if !unfolded(ui) {
        return Vec::new();
    }
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
    let open = unfolded(ui);
    let count = shown.len();
    let choose = Button::new(tr("Choose with Assistant"))
        .variant(ButtonVariant::Neutral)
        .icon(&FACE)
        .on_press_tx(|ui: &mut Ui, tx: &mut Tx| super::super::assistant::fresh::from_labs(ui, tx))
        .build_small_push(p.k)
        .key("fits-choose");
    let c = &p.k.c;
    // The name, the count and a chevron fold the group, as the groups
    // below fold.
    let fold = row()
        .key("fits-fold")
        .role("button")
        .label(tr("Fits you"))
        .expanded(open)
        .focusable(true)
        .focus_ring(2.0, 0.0, c.border_focus)
        .gap(6.0)
        .px(4.0)
        .ml(-4.0)
        .center_y()
        .radius(p.k.radius.sm)
        .hover(c.hover)
        .press(c.pressed)
        .child(p.section_label(tr("Fits you")).key("label").a11y_hidden())
        .child(tone_tag(p, "fits-count", &count.to_string(), Tone::Neutral))
        .child(
            icon(if open { &UNFOLDED } else { &FOLDED }, 14.0)
                .key("chevron")
                .color(c.icon_faint)
                .a11y_hidden(),
        )
        .on_press(move |ui: &mut Ui| {
            ui.update(|settings| {
                settings.labs.groups.insert(GROUP.to_owned(), !open);
            });
        });
    let heading = row()
        .key("fits-heading")
        .gap(6.0)
        .center_y()
        .child(fold)
        .child(el().key("space").grow(1.0))
        .child(choose);
    let group = col()
        .key("fits-group")
        .role("group")
        .label(tr("Fits you"))
        .gap(8.0)
        .child(heading);
    if !open {
        return Some(group);
    }
    let mut cards = col().key("fits").gap(super::store::GAP);
    let mut shown = shown.into_iter();
    let mut index = 0;
    loop {
        let two: Vec<El> = shown
            .by_ref()
            .take(2)
            .map(|(entry, fit)| super::store::card(ui, p, entry, (false, true), Some(fit), layer))
            .collect();
        if two.is_empty() {
            break;
        }
        cards = cards.child(super::store::pair(&format!("fits-row{index}"), two));
        index += 1;
    }
    Some(
        group
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
    /// Presses the element keyed `key` on the card of lab `id`.
    fn press(rt: &mut Runtime<Ui>, id: &str, key: &str) {
        let card = format!("lab-fits-{id}");
        let rect = rt
            .elements()
            .into_iter()
            .find(|element| {
                element.path.contains(&format!("/{card}/")) && element.key.as_deref() == Some(key)
            })
            .unwrap_or_else(|| panic!("no {key} on {card}"))
            .rect;
        let (x, y) = (rect[0] + rect[2] / 2.0, rect[1] + rect[3] / 2.0);
        rt.pointer_down(x, y);
        rt.pointer_up(x, y);
        for _ in 0..30 {
            rt.frame(1.0 / 60.0);
        }
    }

    /// A card's line from Assistant is one line that opens and closes. Its
    /// × takes the lab out of the group to its own place below, and it is
    /// remembered as dismissed, so Assistant doesn't offer it either.
    #[test]
    fn a_card_s_line_opens_and_its_cross_dismisses_the_lab() {
        let mut rt = scene("mac-labs-fits");
        let first = shown(&rt);
        let id = first[0].clone();
        let mut open = 0;
        rt.update(|ui, _| open = ui.lab_store.fits.open.len());
        assert_eq!(open, 0, "one line to begin with");
        press(&mut rt, &id, "for-you");
        rt.update(|ui, _| open = ui.lab_store.fits.open.len());
        assert_eq!(open, 1);
        press(&mut rt, &id, "for-you");
        rt.update(|ui, _| open = ui.lab_store.fits.open.len());
        assert_eq!(open, 0);
        press(&mut rt, &id, "fits-dismiss");
        let after = shown(&rt);
        assert!(!after.contains(&id), "{after:?}");
        assert!(rt.find(&format!("lab-card-{id}")).is_some(), "below now");
        let mut dismissed = Vec::new();
        rt.update(|ui, _| {
            dismissed = super::super::super::assistant::suggestions::dismissed(ui);
        });
        assert_eq!(dismissed.len(), 1, "{dismissed:?}");
        let feature = Feature::ALL
            .into_iter()
            .find(|feature| feature.name() == id)
            .unwrap();
        let mut picked = Vec::new();
        rt.update(|ui, _| {
            let since = ui.lab_store.fits.scene.clone();
            let found = super::super::super::assistant::fresh::found(ui, since.as_ref());
            picked = found.picks.iter().map(|pick| pick.title).collect();
        });
        assert!(!picked.contains(&feature.title()), "{picked:?}");
    }

    /// Folded by its heading, the group keeps only the heading and the
    /// button, and its labs stand in their own places below.
    #[test]
    fn the_group_folds_and_its_labs_keep_their_places() {
        let mut rt = scene("mac-labs-fits");
        let first = shown(&rt);
        let (x, y) = rt.find("fits-fold").expect("the heading folds");
        rt.pointer_down(x, y);
        rt.pointer_up(x, y);
        for _ in 0..30 {
            rt.frame(1.0 / 60.0);
        }
        assert!(shown(&rt).is_empty());
        assert!(rt.find("fits-group").is_some());
        assert!(rt.find("fits-choose").is_some());
        assert!(rt.find("fits-lead").is_none());
        // Nothing is held back from the groups below (a lab's own group
        // may be folded there).
        let mut held = usize::MAX;
        rt.update(|ui, _| {
            let rows = super::super::shown(ui);
            held = super::labs(ui, &rows).len();
        });
        assert_eq!(held, 0, "{first:?} keep their places");
    }
}
