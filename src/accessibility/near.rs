//! VibeHex: Labs › Screen vocabulary reads what is nearest the focused
//! element first.
//!
//! Upstream's reader walks a window breadth-first from its top and stops at
//! 1,500 elements. An editor's window spends them on its menus, tabs, and
//! side bars: VS Code with a chat panel open is about 6,000 elements, the
//! panel's text starts about 20 levels down, and a dictation into the panel
//! got its names from the chrome around it.
//!
//! [`around`] starts at the focused element instead and works outward: the
//! element itself, then at each level above it the siblings before it,
//! nearest first (the last message of a chat before the first), then the
//! siblings after it. It takes at most [`NODES`] elements, [`BYTES`] of text
//! and [`DEADLINE`] of the read's time; upstream's walk then spends what is
//! left on the rest of the window, skipping [`Near::top`].
//!
//! Running text split over several elements is put back on one line: a
//! paragraph with a bold name in it is two `AXStaticText` pieces and an
//! `AXStrongStyleGroup` holding a third, and alone on a line the name would
//! be taken for a sentence's first word (`screen_vocabulary::extract`).
//! Code (`AXCodeStyleGroup`: a code block, a command, a file name) is
//! marked like a title, so its ordinary words (`String`, `Option`) are not
//! names and its own names (`CfObject`) come after the prose's
//! (`screen_vocabulary::labels`).

use std::ops::Range;
use std::time::{Duration, Instant};

use super::{
    CFEqual, CfObject, MAX_TEXT_DEPTH, MAX_VALUE_CHARS, char_boundary, copy_element,
    copy_elements_up_to, copy_string, copy_string_capped, element_value,
};

/// Most elements read around the focused one, of the 1,500 a read takes.
const NODES: usize = 1_000;
/// Most text collected around it, of the read's 32 KB.
const BYTES: usize = 24 * 1024;
/// How long of the read's 150 ms it may take.
const DEADLINE: Duration = Duration::from_millis(100);
/// Most children of one element looked at.
const CHILDREN: usize = 8_192;
/// Most levels climbed above the focused element.
const CLIMB: usize = 64;

/// What one element holds.
pub(super) enum Held {
    /// A password field: neither it nor anything below it is read.
    Secure,
    /// A piece of running text (`AXStaticText`), which has nothing below it.
    Text(Option<String>),
    /// Bold, italic, or linked text inside a sentence: what is below it
    /// reads on with the text around it.
    Inline,
    /// Any other element: its value, and its title and description.
    Element {
        value: Option<String>,
        labels: [Option<String>; 2],
        /// Whether it and what is below it are code.
        code: bool,
    },
}

/// A window's elements as the walk needs them: the live tree, or a test's.
pub(super) trait Tree {
    type Node;
    fn held(&self, node: &Self::Node) -> Held;
    fn children(&self, node: &Self::Node) -> Vec<Self::Node>;
    /// The element above; none above a window.
    fn parent(&self, node: &Self::Node) -> Option<Self::Node>;
    fn same(&self, left: &Self::Node, right: &Self::Node) -> bool;
    /// Whether the walk's time is up.
    fn late(&self) -> bool;
}

pub(super) struct Limits {
    pub nodes: usize,
    pub bytes: usize,
    /// Levels below the focused element or one of the siblings around it.
    pub depth: usize,
}

/// What was read around the focused element.
pub(super) struct Near<N> {
    pub text: String,
    /// Byte ranges of `text` that are titles and descriptions.
    pub labels: Vec<Range<usize>>,
    pub visited: usize,
    /// The highest element whose subtree the walk went into: the rest of
    /// the window is read without it.
    pub top: Option<N>,
}

impl<N> Near<N> {
    fn nothing() -> Self {
        Self {
            text: String::new(),
            labels: Vec::new(),
            visited: 0,
            top: None,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Order {
    /// As the document reads: after the focused element.
    Forward,
    /// Last first: before the focused element, nearest first.
    Backward,
}

/// The text around `focus`, nearest first, up to `window`.
pub(super) fn around<T: Tree>(
    tree: &T,
    focus: T::Node,
    window: &T::Node,
    limits: Limits,
) -> Near<T::Node> {
    let mut walk = Walk {
        tree,
        limits,
        text: String::new(),
        labels: Vec::new(),
        visited: 0,
        pending: 0,
    };
    walk.run(std::iter::once(&focus), Order::Forward, 0, false);
    let mut top = focus;
    for _ in 0..CLIMB {
        if walk.full() || tree.same(&top, window) {
            break;
        }
        let Some(parent) = tree.parent(&top) else {
            break;
        };
        let siblings = tree.children(&parent);
        // An element its parent does not list: what is around it can't be
        // told from what is in it, so the rest is left to the window's walk.
        let Some(at) = siblings.iter().position(|sibling| tree.same(sibling, &top)) else {
            break;
        };
        walk.own(&parent);
        walk.run(siblings[..at].iter().rev(), Order::Backward, 0, false);
        walk.run(siblings[at + 1..].iter(), Order::Forward, 0, false);
        top = parent;
    }
    Near {
        text: walk.text,
        labels: walk.labels,
        visited: walk.visited,
        top: Some(top),
    }
}

struct Walk<'a, T: Tree> {
    tree: &'a T,
    limits: Limits,
    text: String,
    labels: Vec<Range<usize>>,
    visited: usize,
    /// Bytes of running text waiting to be put on one line.
    pending: usize,
}

impl<T: Tree> Walk<'_, T> {
    fn full(&self) -> bool {
        self.visited >= self.limits.nodes
            || self.text.len() + self.pending >= self.limits.bytes
            || self.tree.late()
    }

    /// The elements of one parent, in `order`, each with what is below it.
    /// Pieces of running text next to each other become one line; `code`
    /// is whether they are inside code.
    fn run<'n>(
        &mut self,
        nodes: impl Iterator<Item = &'n T::Node>,
        order: Order,
        depth: usize,
        code: bool,
    ) where
        T::Node: 'n,
    {
        let mut pieces = Vec::new();
        self.each(nodes, order, depth, code, &mut pieces);
        self.line(&mut pieces, order, code);
    }

    fn each<'n>(
        &mut self,
        nodes: impl Iterator<Item = &'n T::Node>,
        order: Order,
        depth: usize,
        code: bool,
        pieces: &mut Vec<String>,
    ) where
        T::Node: 'n,
    {
        for node in nodes {
            if self.full() {
                break;
            }
            self.visited += 1;
            let held = self.tree.held(node);
            let below = depth < self.limits.depth && !self.full();
            match held {
                Held::Secure => self.line(pieces, order, code),
                Held::Text(value) => {
                    if let Some(value) = value {
                        self.pending += value.len() + 1;
                        pieces.push(value);
                    }
                }
                Held::Inline => {
                    if below {
                        let children = self.tree.children(node);
                        match order {
                            Order::Forward => {
                                self.each(children.iter(), order, depth + 1, code, pieces)
                            }
                            Order::Backward => {
                                self.each(children.iter().rev(), order, depth + 1, code, pieces)
                            }
                        }
                    }
                }
                Held::Element {
                    value,
                    labels,
                    code: its,
                } => {
                    self.line(pieces, order, code);
                    self.element(value, labels, code || its);
                    if !below {
                        continue;
                    }
                    let children = self.tree.children(node);
                    match order {
                        Order::Forward => self.run(children.iter(), order, depth + 1, code || its),
                        Order::Backward => {
                            self.run(children.iter().rev(), order, depth + 1, code || its)
                        }
                    }
                }
            }
        }
    }

    /// An element above the focused one: what it holds itself, not what is
    /// below it.
    fn own(&mut self, node: &T::Node) {
        self.visited += 1;
        match self.tree.held(node) {
            Held::Secure | Held::Inline => {}
            Held::Text(value) => self.element(value, [None, None], false),
            Held::Element { value, labels, .. } => self.element(value, labels, false),
        }
    }

    fn element(&mut self, value: Option<String>, labels: [Option<String>; 2], code: bool) {
        if let Some(value) = value {
            self.push(&value, code);
        }
        for label in labels.into_iter().flatten() {
            self.push(&label, true);
        }
    }

    /// Writes the waiting pieces of running text as one line, in the order
    /// they read; code is marked as a label.
    fn line(&mut self, pieces: &mut Vec<String>, order: Order, code: bool) {
        self.pending = 0;
        if pieces.is_empty() {
            return;
        }
        if order == Order::Backward {
            pieces.reverse();
        }
        let line = pieces.join(" ");
        pieces.clear();
        self.push(&line, code);
    }

    fn push(&mut self, value: &str, label: bool) {
        let room = self.limits.bytes.saturating_sub(self.text.len() + 1);
        if room == 0 || value.is_empty() {
            return;
        }
        let end = char_boundary(value, value.len().min(room));
        let at = self.text.len();
        self.text.push_str(&value[..end]);
        self.text.push('\n');
        if label {
            self.labels.push(at..self.text.len());
        }
    }
}

/// The subrole of code: a block, or a few words inside a sentence.
const CODE: &str = "AXCodeStyleGroup";

/// Whether an element only styles part of a sentence: a link, or a style
/// group (`AXStrongStyleGroup`, `AXEmphasisStyleGroup`) other than code.
fn inline(role: Option<&str>, subrole: Option<&str>) -> bool {
    role == Some("AXLink")
        || subrole.is_some_and(|subrole| subrole.ends_with("StyleGroup") && subrole != CODE)
}

/// Another process's window, read through Accessibility.
struct Live {
    started: Instant,
}

impl Tree for Live {
    type Node = CfObject;

    fn held(&self, node: &CfObject) -> Held {
        let role = copy_string(node, c"AXRole");
        if role.as_deref() == Some("AXStaticText") {
            return Held::Text(element_value(node));
        }
        let subrole = copy_string(node, c"AXSubrole");
        if role.as_deref() == Some("AXSecureTextField")
            || subrole.as_deref() == Some("AXSecureTextField")
        {
            return Held::Secure;
        }
        let code = subrole.as_deref() == Some(CODE);
        if inline(role.as_deref(), subrole.as_deref()) {
            return Held::Inline;
        }
        Held::Element {
            value: element_value(node),
            labels: [c"AXTitle", c"AXDescription"]
                .map(|name| copy_string_capped(node, name, MAX_VALUE_CHARS)),
            code,
        }
    }

    fn children(&self, node: &CfObject) -> Vec<CfObject> {
        copy_elements_up_to(node, c"AXChildren", CHILDREN)
    }

    /// The element above, and none above a window: the application's other
    /// windows are not around the focused element.
    fn parent(&self, node: &CfObject) -> Option<CfObject> {
        let parent = copy_element(node, c"AXParent")?;
        (copy_string(&parent, c"AXRole").as_deref() != Some("AXApplication")).then_some(parent)
    }

    fn same(&self, left: &CfObject, right: &CfObject) -> bool {
        // SAFETY: Two live Core Foundation objects.
        unsafe { CFEqual(left.0, right.0) != 0 }
    }

    fn late(&self) -> bool {
        self.started.elapsed() >= DEADLINE
    }
}

/// What is around the application's focused element, when it has one that
/// is not the window itself. `started` is when the whole read began.
pub(super) fn read(application: &CfObject, window: &CfObject, started: Instant) -> Near<CfObject> {
    let tree = Live { started };
    let Some(focus) =
        copy_element(application, c"AXFocusedUIElement").filter(|focus| !tree.same(focus, window))
    else {
        return Near::nothing();
    };
    around(
        &tree,
        focus,
        window,
        Limits {
            nodes: NODES,
            bytes: BYTES,
            depth: MAX_TEXT_DEPTH,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tree written as nested items; a node is the path to it.
    enum Item {
        Text(&'static str),
        Secure(Vec<Item>),
        Group(&'static str, Vec<Item>),
        /// Bold text inside a sentence.
        Strong(Vec<Item>),
        Code(Vec<Item>),
    }

    struct Fake {
        root: Item,
    }

    impl Fake {
        fn item(&self, path: &[usize]) -> &Item {
            let mut item = &self.root;
            for &index in path {
                item = match item {
                    Item::Group(_, children)
                    | Item::Secure(children)
                    | Item::Strong(children)
                    | Item::Code(children) => &children[index],
                    Item::Text(_) => panic!("text has nothing below it"),
                };
            }
            item
        }
    }

    impl Tree for Fake {
        type Node = Vec<usize>;

        fn held(&self, node: &Vec<usize>) -> Held {
            match self.item(node) {
                Item::Text(text) => Held::Text(Some((*text).into())),
                Item::Secure(_) => Held::Secure,
                Item::Strong(_) => Held::Inline,
                Item::Code(_) => Held::Element {
                    value: None,
                    labels: [None, None],
                    code: true,
                },
                Item::Group(title, _) => Held::Element {
                    value: None,
                    labels: [(!title.is_empty()).then(|| (*title).into()), None],
                    code: false,
                },
            }
        }

        fn children(&self, node: &Vec<usize>) -> Vec<Vec<usize>> {
            let count = match self.item(node) {
                Item::Group(_, children)
                | Item::Secure(children)
                | Item::Strong(children)
                | Item::Code(children) => children.len(),
                Item::Text(_) => 0,
            };
            (0..count)
                .map(|index| {
                    let mut path = node.clone();
                    path.push(index);
                    path
                })
                .collect()
        }

        fn parent(&self, node: &Vec<usize>) -> Option<Vec<usize>> {
            let mut path = node.clone();
            path.pop().map(|_| path)
        }

        fn same(&self, left: &Vec<usize>, right: &Vec<usize>) -> bool {
            left == right
        }

        fn late(&self) -> bool {
            false
        }
    }

    fn group(children: Vec<Item>) -> Item {
        Item::Group("", children)
    }

    /// A window: a side bar, then a panel with three messages and the field
    /// being typed in, then a status bar.
    fn window() -> Fake {
        Fake {
            root: Item::Group(
                "Window",
                vec![
                    Item::Group("Explorer", vec![Item::Text("main.rs")]),
                    group(vec![
                        group(vec![
                            group(vec![Item::Text("First message.")]),
                            group(vec![Item::Text("Second message.")]),
                            group(vec![
                                Item::Text("We store vectors in "),
                                Item::Strong(vec![Item::Text("Qdrant")]),
                                Item::Text(", not Postgres."),
                            ]),
                        ]),
                        group(vec![Item::Text("typed so far")]),
                        Item::Group("Send", vec![]),
                    ]),
                    Item::Group("Status", vec![Item::Text("Ln 1")]),
                ],
            ),
        }
    }

    const FIELD: [usize; 2] = [1, 1];

    fn read(fake: &Fake, focus: &[usize], nodes: usize) -> Near<Vec<usize>> {
        around(
            fake,
            focus.to_vec(),
            &Vec::new(),
            Limits {
                nodes,
                bytes: 4096,
                depth: 40,
            },
        )
    }

    fn lines(near: &Near<Vec<usize>>) -> Vec<&str> {
        near.text.lines().collect()
    }

    #[test]
    fn the_field_comes_first_then_the_newest_text_before_it() {
        let fake = window();
        let near = read(&fake, &FIELD, 100);
        assert_eq!(
            lines(&near),
            [
                "typed so far",
                "We store vectors in  Qdrant , not Postgres.",
                "Second message.",
                "First message.",
                "Send",
                "Window",
                "Explorer",
                "main.rs",
                "Status",
                "Ln 1",
            ]
        );
        assert_eq!(near.top, Some(Vec::new()), "the whole window was read");
    }

    /// A bold name is its own element: back on its sentence's line it is a
    /// name, alone on a line it would be a sentence's first word.
    #[test]
    fn a_name_in_its_own_element_stays_inside_its_sentence() {
        let fake = window();
        let near = read(&fake, &FIELD, 100);
        let terms = crate::dictation::screen_vocabulary::extract(&near.text, &[]);
        assert!(terms.contains(&"Qdrant".to_string()), "{terms:?}");
        assert!(terms.contains(&"Postgres".to_string()), "{terms:?}");
    }

    #[test]
    fn titles_are_marked_as_labels_and_text_is_not() {
        let fake = window();
        let near = read(&fake, &FIELD, 100);
        let labels: Vec<&str> = near
            .labels
            .iter()
            .map(|range| near.text[range.clone()].trim_end())
            .collect();
        assert_eq!(labels, ["Send", "Window", "Explorer", "Status"]);
    }

    /// With room for a few elements only, they are the ones nearest the
    /// field, and the rest of the window is told what not to read again.
    #[test]
    fn a_small_budget_is_spent_nearest_the_field() {
        let fake = window();
        // The field and its text, the panel, the list, the last message,
        // its two pieces of text, and the bold name with its own.
        let near = read(&fake, &FIELD, 9);
        assert_eq!(
            lines(&near),
            [
                "typed so far",
                "We store vectors in  Qdrant , not Postgres."
            ]
        );
        assert_eq!(near.visited, 9);
        assert_eq!(near.top, Some(vec![1]), "the panel is not read twice");
    }

    /// Code is marked like a title: its ordinary words are not names, and
    /// the sentence around it reads on after it.
    #[test]
    fn code_is_marked_apart_from_the_sentence_it_is_in() {
        let fake = Fake {
            root: group(vec![
                group(vec![
                    Item::Text("The read returns "),
                    Item::Code(vec![Item::Text("Option<String>")]),
                    Item::Text(" from Qdrant."),
                ]),
                group(vec![Item::Text("typed")]),
            ]),
        };
        let near = read(&fake, &[1], 100);
        assert_eq!(
            lines(&near),
            [
                "typed",
                " from Qdrant.",
                "Option<String>",
                "The read returns "
            ]
        );
        let labels: Vec<&str> = (near.labels.iter())
            .map(|range| near.text[range.clone()].trim_end())
            .collect();
        assert_eq!(labels, ["Option<String>"]);
    }

    #[test]
    fn a_password_field_and_what_is_below_it_are_never_read() {
        let fake = Fake {
            root: group(vec![
                Item::Secure(vec![Item::Text("hunter2")]),
                group(vec![Item::Text("typed")]),
            ]),
        };
        let near = read(&fake, &[1], 100);
        assert_eq!(lines(&near), ["typed"]);
        let near = read(&fake, &[0], 100);
        assert_eq!(lines(&near), ["typed"], "focused in the password field");
    }

    #[test]
    fn the_text_stops_at_its_byte_budget() {
        let fake = window();
        let near = around(
            &fake,
            FIELD.to_vec(),
            &Vec::new(),
            Limits {
                nodes: 100,
                bytes: 24,
                depth: 40,
            },
        );
        assert!(near.text.len() <= 24, "{}", near.text.len());
        assert!(near.text.starts_with("typed so far\n"));
    }
}
