//! "New for you"'s rows in the plan card: "Now" and "With it" under a
//! row's line, and "More to try" under the rows.

use rune::{El, col, icon, row, text};
use rune_theme_opencode::icons as theme;

use super::super::super::Ui;
use super::super::super::controls::{DESCRIPTION, Paint};
use super::super::card::{Look, push};
use super::super::changes::At;
use crate::assistant::changes::{Card, Decision, Item, names};
use crate::desktop::i18n::{tr, tr_fill, tr_window};

/// How wide a cell is before the two share a line: narrower, the second
/// goes under the first.
const CELL: f32 = 150.0;

/// "Now" and "With it" under a row's line: two equal cells side by side,
/// each on its own rounded backing with a small label over its sentence,
/// the second on a soft accent tint. Too narrow for both, they stack.
pub(in super::super) fn pair(p: Paint, item: &Item) -> Option<El> {
    let (now, with) = (item.now.trim(), item.with.trim());
    if now.is_empty() || with.is_empty() {
        return None;
    }
    let c = &p.k.c;
    let cell =
        |key: &str, label: &str, words: &str, look: (rune::Color, rune::Color, rune::Color)| {
            let (fill, border, ink) = look;
            col()
                .key(key.to_owned())
                .role("group")
                .label(label.to_owned())
                .w(CELL)
                .grow(1.0)
                .min_w(0.0)
                .gap(2.0)
                .px(10.0)
                .pt(7.0)
                .pb(8.0)
                .radius(p.k.radius.md)
                .fill(fill)
                .border(p.hair, border)
                .child(
                    text(label.to_owned())
                        .key("label")
                        .font(p.k.ty.small)
                        .color(ink)
                        .a11y_hidden(),
                )
                .child(
                    text(words.to_owned())
                        .key("words")
                        .font(DESCRIPTION)
                        .color(p.text())
                        .wrap_anywhere(),
                )
        };
    Some(
        row()
            .key("pair")
            .mt(6.0)
            .gap(6.0)
            .flex_wrap()
            .items_stretch()
            .child(cell(
                "now",
                tr("Now"),
                now,
                (c.layer_02, c.border_weak, p.faint()),
            ))
            .child(cell(
                "with",
                tr("With it"),
                with,
                (
                    p.k.alpha(p.accent(), 0.08),
                    p.k.alpha(p.accent(), 0.3),
                    p.accent(),
                ),
            )),
    )
}

/// "More to try: 19 new features ›": the other features that came since,
/// folded; unfolded, a compact row each with what it does and Add, which
/// moves it into the list, checked. A decided plan adds nothing.
pub(in super::super) fn more(ui: &Ui, p: Paint, at: At, card: &Card) -> Option<El> {
    if card.more.is_empty() {
        return None;
    }
    let open = ui.assistant.more_open.contains(&at);
    let words = match card.more.len() {
        1 => tr("More to try: 1 new feature").to_owned(),
        count => tr_fill("More to try: {} new features", &count.to_string()),
    };
    let c = &p.k.c;
    let toggle = row()
        .key("more-toggle")
        .role("button")
        .label(words.clone())
        .checked(open)
        .focusable(true)
        .focus_ring(2.0, 0.0, c.border_focus)
        .gap(6.0)
        .px(6.0)
        .h(24.0)
        .center_y()
        .radius(p.k.radius.sm)
        .hover(c.hover)
        .child(text(words).key("words").font(p.k.ty.small).color(p.muted()))
        .child(
            icon(
                if open {
                    &theme::CHEVRON_DOWN
                } else {
                    &theme::CHEVRON_RIGHT
                },
                12.0,
            )
            .key("chevron")
            .color(c.icon_faint),
        )
        .on_press(move |ui: &mut Ui| {
            let shown = &mut ui.assistant.more_open;
            match shown.iter().position(|known| *known == at) {
                Some(index) => {
                    shown.remove(index);
                }
                None => shown.push(at),
            }
        });
    let mut fold = col()
        .key("more")
        .px(8.0)
        .py(6.0)
        .gap(2.0)
        .child(row().key("head").child(toggle));
    if open {
        let adds = matches!(card.decision, Decision::Proposed | Decision::Failed { .. });
        for (index, item) in card.more.iter().enumerate() {
            fold = fold.child(feature(ui, p, at, index, item, adds));
        }
    }
    Some(fold)
}

/// One feature under More to try: its name, where it lives, what it does,
/// and Add while the plan waits.
fn feature(ui: &Ui, p: Paint, at: At, index: usize, item: &Item, adds: bool) -> El {
    let (name, path) = match names::place(&item.key, &ui.settings) {
        Some(place) => (
            tr_window(&place.name).to_owned(),
            tr_window(place.path).to_owned(),
        ),
        None => (item.key.clone(), String::new()),
    };
    let mut words = col()
        .key("words")
        .grow(1.0)
        .shrink()
        .min_w(0.0)
        .gap(1.0)
        .child(
            row()
                .key("names")
                .gap(8.0)
                .flex_wrap()
                .child(
                    text(name.clone())
                        .key("name")
                        .font(rune::TextStyle {
                            weight: 520.0,
                            ..p.k.ty.small
                        })
                        .color(p.text()),
                )
                .children(
                    (!path.is_empty())
                        .then(|| text(path).key("path").font(p.k.ty.small).color(p.faint())),
                ),
        );
    if !item.why.trim().is_empty() {
        words = words.child(
            text(item.why.trim().to_owned())
                .key("why")
                .font(p.k.ty.small)
                .color(p.muted())
                .wrap_anywhere(),
        );
    }
    let key = item.key.clone();
    row()
        .key(format!("more{index}"))
        .role("group")
        .label(name)
        .px(6.0)
        .py(5.0)
        .gap(10.0)
        .items_start()
        .child(words)
        .children(adds.then(|| {
            push(p, "add", tr("Add"), Look::Secondary, move |ui, _| {
                super::add(ui, at, &key)
            })
        }))
}
