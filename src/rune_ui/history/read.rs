//! VibeHex: "What VibeHex read", one folded section at the end of a History
//! entry's detail (`history::read`): the names Screen vocabulary read from
//! the window, marked where AI cleanup was given them, and the text Rewrite
//! reads nearby text read around the cursor, with what cleanup was given of
//! it. Closed until pressed; an entry without the record has no section.

use rune::text::{RichLayoutOptions, measure, measure_with};
use rune::{El, col, row, text};

use super::super::Ui;
use super::super::controls::{DESCRIPTION, Paint};
use super::super::lists::{Tone, section_label, tone_tag};
use super::HistoryPane;
use crate::desktop::i18n::{tr, tr_fill};
use crate::history::HistoryEntry;
use crate::history::read::{HistoryRead, NearbySent, Part, ReadNearby};

/// The lines a long block of text shows folded, with "Show all" under them.
const LINES: usize = 6;
/// The width a block is measured at to tell whether it is long: about the
/// narrowest the detail's blocks get.
const MEASURE_WIDTH: f32 = 360.0;
const BLOCK_PAD: f32 = 10.0;

/// Which entry's section is open, and which of its blocks show all their
/// text. Choosing another entry's section closes the first.
#[derive(Default)]
pub(in crate::rune_ui) struct Shown {
    entry: Option<u64>,
    all: Vec<Part>,
}

impl Shown {
    fn open(&self, id: u64) -> bool {
        self.entry == Some(id)
    }

    fn toggle(&mut self, id: u64) {
        self.entry = (!self.open(id)).then_some(id);
        self.all.clear();
    }

    fn all(&self, id: u64, part: Part) -> bool {
        self.open(id) && self.all.contains(&part)
    }

    fn toggle_all(&mut self, id: u64, part: Part) {
        if !self.open(id) {
            return;
        }
        match self.all.iter().position(|shown| *shown == part) {
            Some(index) => {
                self.all.remove(index);
            }
            None => self.all.push(part),
        }
    }
}

/// The section for `entry`'s record, if it has one.
pub(super) fn section(ui: &Ui, p: Paint, entry: &HistoryEntry) -> Option<El> {
    let (id, read) = (entry.id, entry.read.as_ref()?);
    let shown = &ui.history.read;
    let open = shown.open(id);
    let mut section = col().key("read").gap(14.0).child(heading(p, id, open));
    if !open {
        return Some(section);
    }
    if !read.screen_terms.is_empty() {
        section = section.child(names(p, read));
    }
    if let Some(nearby) = &read.nearby {
        section = section.child(near_cursor(p, shown, id, read, nearby));
    }
    Some(section)
}

/// "What VibeHex read" and its chevron, which opens and closes the section.
fn heading(p: Paint, id: u64, open: bool) -> El {
    let c = &p.k.c;
    let label = tr("What VibeHex read");
    row()
        .key("read-toggle")
        .role("button")
        .label(label)
        .expanded(open)
        .focusable(true)
        .focus_ring(2.0, 0.0, c.border_focus)
        .self_start()
        .h(24.0)
        .mx(-6.0)
        .px(6.0)
        .gap(4.0)
        .center_y()
        .radius(p.k.radius.md)
        .hover(c.hover)
        .press(c.pressed)
        .on_press(move |ui: &mut Ui| ui.history.read.toggle(id))
        .child(section_label(p, "label", label))
        .child(
            rune::icon(
                if open {
                    p.k.glyph.chevron_down
                } else {
                    &rune_theme_opencode::icons::CHEVRON_RIGHT
                },
                14.0,
            )
            .key("chevron")
            .color(c.icon_muted)
            .a11y_hidden(),
        )
}

/// The window's names as quiet tags, the ones cleanup was given on the
/// accent tint, and what the marking means.
fn names(p: Paint, read: &HistoryRead) -> El {
    let mut tags = row().key("tags").gap(4.0).flex_wrap();
    for (index, term) in read.screen_terms.iter().enumerate() {
        let key = format!("name{index}");
        tags = tags.child(if read.given(term) {
            tone_tag(p, &key, term, Tone::Accent).label(tr_fill("{}: given to AI cleanup", term))
        } else {
            tone_tag(p, &key, term, Tone::Neutral)
        });
    }
    let legend = if read.cleanup_terms.is_empty() {
        tr("AI cleanup was not given these names.")
    } else {
        tr("Marked names were also given to AI cleanup.")
    };
    col()
        .key("names")
        .gap(8.0)
        .child(
            row()
                .key("names-head")
                .gap(8.0)
                .center_y()
                .child(section_label(p, "label", tr("Names from the window")))
                .child(
                    text(read.screen_terms.len().to_string())
                        .key("count")
                        .font(p.k.ty.small)
                        .color(p.faint()),
                ),
        )
        .child(tags)
        .child(note(p, "names-legend", legend))
}

/// The text before, inside and after the cursor, each in its own block,
/// then what cleanup was given of it and whether it was asked again.
fn near_cursor(p: Paint, shown: &Shown, id: u64, read: &HistoryRead, nearby: &ReadNearby) -> El {
    let mut section =
        col()
            .key("nearby")
            .gap(8.0)
            .child(section_label(p, "label", tr("Text near the cursor")));
    for (part, words) in nearby.parts() {
        section = section.child(block(p, id, part, words, shown.all(id, part)));
    }
    let sent = match read.nearby_sent {
        NearbySent::All => tr("AI cleanup was given all of it."),
        NearbySent::Part => tr(
            "The built-in model was given only the end of the text before the cursor and the selection.",
        ),
        NearbySent::Nothing => tr("AI cleanup was not given this text."),
    };
    section = section.child(note(p, "nearby-sent", sent));
    if read.asked_again {
        section = section.child(
            text(tr(
                "AI cleanup repeated this text instead of the dictation, so it was asked again without it.",
            ))
            .key("asked-again")
            .font(DESCRIPTION)
            .color(p.warning())
            .wrap(),
        );
    }
    section
}

/// One part's label and its text on its own backing, capped at [`LINES`]
/// lines with "Show all" when it is longer.
fn block(p: Paint, id: u64, part: Part, words: &str, all: bool) -> El {
    let c = &p.k.c;
    let (key, label) = match part {
        Part::Before => ("before", tr("Before the cursor")),
        Part::Selected => ("selected", tr("Selected")),
        Part::After => ("after", tr("After the cursor")),
    };
    let font = p.k.ty.mono;
    let long = lines(words, font) > LINES;
    let mut body = text(words.to_owned())
        .key("text")
        .font(font)
        .color(p.muted())
        .wrap_anywhere()
        .w_full();
    if long && !all {
        body = body.max_lines(LINES);
    }
    let mut backing = col()
        .key("backing")
        .w_full()
        .px(BLOCK_PAD)
        .py(8.0)
        .gap(6.0)
        .radius(p.k.radius.sm)
        .fill(c.layer_02)
        .border(p.hair, c.border_weak)
        .child(body);
    if long {
        let says = if all { tr("Show less") } else { tr("Show all") };
        backing = backing.child(
            row()
                .key("all")
                .role("button")
                .label(says)
                .expanded(all)
                .focusable(true)
                .focus_ring(2.0, 0.0, c.border_focus)
                .self_start()
                .px(4.0)
                .mx(-4.0)
                .radius(p.k.radius.xs)
                .hover(c.hover)
                .child(text(says).key("says").font(p.k.ty.small).color(p.text()))
                .on_press(move |ui: &mut Ui| ui.history.read.toggle_all(id, part)),
        );
    }
    col()
        .key(key)
        .gap(4.0)
        .child(text(label).key("label").font(DESCRIPTION).color(p.muted()))
        .child(backing)
}

/// A muted line under a part of the section.
fn note(p: Paint, key: &str, words: &str) -> El {
    text(words)
        .key(key)
        .font(DESCRIPTION)
        .color(p.muted())
        .wrap()
}

/// How many lines `words` take in a block [`MEASURE_WIDTH`] wide.
fn lines(words: &str, font: rune::TextStyle) -> usize {
    let room = MEASURE_WIDTH - 2.0 * BLOCK_PAD;
    let (_, line) = measure("X", &[], font, None);
    let wrap = RichLayoutOptions::new().wrap_anywhere();
    let (_, height) = measure_with(words, &[], font, Some(room), wrap);
    (height / line.max(1.0)).round() as usize
}

impl HistoryPane {
    /// The History scene's entries, the rewritten one with what VibeHex
    /// read for it (the section folded, as it opens).
    pub(in crate::rune_ui) fn fixture_read() -> Self {
        let mut pane = Self::fixture();
        if let Some(entry) = pane.entries.iter_mut().find(|entry| entry.id == 2) {
            entry.read = Some(fixture());
        }
        pane
    }
}

/// A Slack window's names and the message being written, as the built-in
/// model was given them and asked again without the text.
fn fixture() -> HistoryRead {
    HistoryRead {
        screen_terms: [
            "Dax",
            "CacheKey",
            "deploy-api",
            "GitHub Actions",
            "Kit",
            "v3.4",
        ]
        .map(str::to_owned)
        .to_vec(),
        cleanup_terms: vec!["CacheKey".into(), "Dax".into()],
        nearby: Some(ReadNearby {
            before: "Dax: did the deploy go out last night?\nMe: not yet, ".into(),
            selected: String::new(),
            after: " I'll retry after lunch.".into(),
        }),
        nearby_sent: NearbySent::Part,
        asked_again: true,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use rune::text::TextSystem;
    use rune::{Env, Runtime};

    use super::super::super::{Pane, Service, Ui, offscreen, on_key, poll, view};
    use super::*;
    use crate::app_settings::AppSettings;

    const SIZE: (f32, f32) = (980.0, 1400.0);

    fn runtime(read: Option<HistoryRead>) -> Runtime<Ui> {
        let mut state = Ui::with(AppSettings::default(), Service::Fixture(Vec::new()));
        state.history = HistoryPane::fixture();
        state.history.entries[1].read = read;
        state.pane = Pane::History;
        let env = Env {
            system_dark: true,
            scale: 2.0,
        };
        let text = TextSystem::new(rune_theme_opencode::FONTS, rune_theme_opencode::FAMILIES);
        let mut rt = offscreen::runtime(state, view, SIZE, env, text);
        rt.poll = Some(poll);
        rt.on_key = Some(on_key);
        rt.settle_all();
        rt.frame(0.0);
        rt
    }

    fn settle(rt: &mut Runtime<Ui>) {
        for _ in 0..60 {
            rt.frame(1.0 / 60.0);
        }
        rt.settle_all();
        rt.frame(0.0);
    }

    fn tap(rt: &mut Runtime<Ui>, key: &str) {
        let (x, y) = rt.find(key).unwrap_or_else(|| panic!("no element {key}"));
        rt.pointer_down(x, y);
        rt.pointer_up(x, y);
        settle(rt);
    }

    fn texts(rt: &Runtime<Ui>) -> Vec<String> {
        rt.elements().into_iter().filter_map(|e| e.text).collect()
    }

    fn shows(rt: &Runtime<Ui>, words: &str) -> bool {
        texts(rt).iter().any(|text| text == words)
    }

    fn read(sent: NearbySent, given: &[&str], again: bool, before: &str) -> HistoryRead {
        HistoryRead {
            cleanup_terms: given.iter().map(|&term| term.into()).collect(),
            nearby: Some(ReadNearby {
                before: before.into(),
                selected: "Friday".into(),
                after: String::new(),
            }),
            nearby_sent: sent,
            asked_again: again,
            ..fixture()
        }
    }

    #[test]
    fn an_entry_without_the_record_has_no_section() {
        let rt = runtime(None);
        assert!(rt.find("history-detail").is_some());
        assert!(rt.find("read-toggle").is_none());
    }

    #[test]
    fn the_section_is_folded_until_pressed_and_shows_what_was_read() {
        let mut rt = runtime(Some(fixture()));
        assert!(
            rt.find("read-toggle").is_some(),
            "the entry shows the section"
        );
        assert!(shows(&rt, "What VibeHex read"));
        assert!(!shows(&rt, "Names from the window"), "folded by default");
        assert!(rt.find("tags").is_none() && rt.find("nearby").is_none());

        tap(&mut rt, "read-toggle");
        for words in [
            "Names from the window",
            "6",
            "Dax",
            "GitHub Actions",
            "Marked names were also given to AI cleanup.",
            "Text near the cursor",
            "Before the cursor",
            "After the cursor",
            "Dax: did the deploy go out last night?\nMe: not yet, ",
            " I'll retry after lunch.",
            "The built-in model was given only the end of the text before the cursor and the selection.",
            "AI cleanup repeated this text instead of the dictation, so it was asked again without it.",
        ] {
            assert!(shows(&rt, words), "{words:?} in {:?}", texts(&rt));
        }
        assert!(!shows(&rt, "Selected"), "an empty part has no block");
        let named = rt.accessibility_json().to_string();
        assert!(named.contains("\"What VibeHex read\""), "the fold is named");
        assert!(
            named.contains("CacheKey: given to AI cleanup"),
            "a marked name says so"
        );
        assert!(!named.contains("Kit: given"), "an unmarked one does not");

        tap(&mut rt, "read-toggle");
        assert!(rt.find("tags").is_none(), "pressed again, it folds");
    }

    #[test]
    fn each_thing_cleanup_got_has_its_line() {
        let cases = [
            (
                NearbySent::All,
                &["Dax"][..],
                "AI cleanup was given all of it.",
            ),
            (
                NearbySent::Nothing,
                &[][..],
                "AI cleanup was not given this text.",
            ),
        ];
        for (sent, given, line) in cases {
            let mut rt = runtime(Some(read(sent, given, false, "We ship on ")));
            tap(&mut rt, "read-toggle");
            assert!(shows(&rt, line), "{line}");
            assert!(shows(&rt, "Selected") && shows(&rt, "Friday"));
            assert!(rt.find("asked-again").is_none());
            let legend = match given.is_empty() {
                true => "AI cleanup was not given these names.",
                false => "Marked names were also given to AI cleanup.",
            };
            assert!(shows(&rt, legend), "{legend}");
        }
    }

    #[test]
    fn a_long_block_is_capped_until_show_all() {
        let long = "the cache key changed and the deploy failed again ".repeat(30);
        let mut rt = runtime(Some(read(NearbySent::All, &[], false, &long)));
        tap(&mut rt, "read-toggle");
        let height = |rt: &Runtime<Ui>| {
            rt.elements()
                .into_iter()
                .find(|e| e.path.ends_with("before/backing/text"))
                .unwrap()
                .rect[3]
        };
        let folded = height(&rt);
        assert!(shows(&rt, "Show all"));
        tap(&mut rt, "all");
        assert!(shows(&rt, "Show less"));
        assert!(height(&rt) > folded * 2.0, "{} then {folded}", height(&rt));
        assert!(
            rt.accessibility_json()
                .to_string()
                .contains("\"Show less\"")
        );
        // A short block has no control.
        let mut rt = runtime(Some(read(NearbySent::All, &[], false, "We ship on ")));
        tap(&mut rt, "read-toggle");
        assert!(rt.find("all").is_none());
    }
}
