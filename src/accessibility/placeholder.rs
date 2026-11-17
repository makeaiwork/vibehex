//! VibeHex: a web field's placeholder is not text the person wrote.
//!
//! A browser or Electron app can draw an empty field's hint (or a suggested
//! message, as Claude Code's panel in VS Code does) as an element inside the
//! field. Accessibility then reports the hint as the field's value, with the
//! cursor at its end, and Labs › Rewrite reads nearby text took it for the
//! text before the cursor and gave it to AI cleanup.
//!
//! The hint can be told from typed text: Chromium lists `AXEditableAncestor`
//! among the attributes of every element the person can edit, and not among
//! those of a hint drawn in the field (asked for anyway, it answers with the
//! field for both, so the list is what tells). [`only`] is true when a field
//! has text and none of it can be edited, or when its value is its own
//! `AXPlaceholderValue`.

use std::ffi::CStr;
use std::ptr;

use super::{
    AXUIElementCopyAttributeNames, CFArrayGetCount, CFArrayGetValueAtIndex, CFEqual, CfObject,
    MAX_VALUE_CHARS, cf_string_literal, copy_elements_up_to, copy_string, copy_string_capped,
};

/// What an element lists when the person can edit it.
const EDITABLE: &CStr = c"AXEditableAncestor";

/// Most elements of a field looked at, and most children of one of them.
const NODES: usize = 64;
const CHILDREN: usize = 32;

/// Whether the focused field shows only a placeholder.
pub(super) fn only(field: &CfObject) -> bool {
    let value = copy_string_capped(field, c"AXValue", MAX_VALUE_CHARS);
    if let (Some(value), Some(placeholder)) = (&value, copy_string(field, c"AXPlaceholderValue"))
        && is_placeholder(value, &placeholder)
    {
        return true;
    }
    // Only web content names editable ancestors; a native field's text is
    // never a placeholder.
    if !lists(field, EDITABLE) {
        return false;
    }
    let mut pieces = Vec::new();
    let mut queue = copy_elements_up_to(field, c"AXChildren", CHILDREN);
    let mut visited = 0;
    while let Some(element) = queue.pop() {
        visited += 1;
        if visited > NODES {
            // Too much to tell: read it as text.
            return false;
        }
        if copy_string(&element, c"AXRole").as_deref() == Some("AXStaticText") {
            if copy_string(&element, c"AXValue").is_some_and(|text| !text.trim().is_empty()) {
                pieces.push(lists(&element, EDITABLE));
            }
            continue;
        }
        queue.extend(copy_elements_up_to(&element, c"AXChildren", CHILDREN));
    }
    none_editable(&pieces)
}

/// Whether `element` lists the attribute `name`.
fn lists(element: &CfObject, name: &CStr) -> bool {
    let mut names = ptr::null();
    // SAFETY: Copy rule: a retained CFArray of CFStrings on success.
    let status = unsafe { AXUIElementCopyAttributeNames(element.0, &mut names) };
    if status != 0 || names.is_null() {
        return false;
    }
    let names = CfObject(names);
    let Some(wanted) = cf_string_literal(name).map(CfObject) else {
        return false;
    };
    // SAFETY: Indexes within the array, whose items it owns.
    let count = unsafe { CFArrayGetCount(names.0) };
    (0..count).any(|index| {
        let item = unsafe { CFArrayGetValueAtIndex(names.0, index) };
        !item.is_null() && unsafe { CFEqual(item, wanted.0) } != 0
    })
}

/// Whether a field's value is its placeholder and not text of the same
/// words typed in.
fn is_placeholder(value: &str, placeholder: &str) -> bool {
    !placeholder.trim().is_empty() && value.trim() == placeholder.trim()
}

/// Whether a field has pieces of text and the person can edit none of them
/// (`editable` is one flag per piece). A field with no pieces is a plain
/// one whose text is its value.
fn none_editable(editable: &[bool]) -> bool {
    !editable.is_empty() && editable.iter().all(|editable| !editable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_field_whose_text_cannot_be_edited_shows_a_placeholder() {
        assert!(none_editable(&[false]));
        assert!(none_editable(&[false, false]));
    }

    #[test]
    fn typed_text_is_never_a_placeholder() {
        assert!(!none_editable(&[true]));
        assert!(
            !none_editable(&[true, false]),
            "typed text beside a fixed piece such as a mention"
        );
        assert!(!none_editable(&[]), "a plain field has no pieces");
    }

    #[test]
    fn a_value_equal_to_the_placeholder_is_the_placeholder() {
        assert!(is_placeholder(
            "Queue another message…",
            "Queue another message…"
        ));
        assert!(!is_placeholder("Queue", "Queue another message…"));
        assert!(!is_placeholder("", ""));
    }
}
