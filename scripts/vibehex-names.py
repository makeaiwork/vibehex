#!/usr/bin/env python3
"""Say VibeHex in the strings Hex's code brings in.

After an upstream merge, new interface strings name the app "Hex". The tests
in src/desktop/i18n/vibehex/naming.rs fail on each one, one at a time. This
script does the whole pass at once:

  - finds every string literal in src/ (outside the translation tables) that
    a translation table knows and that names Hex, and every literal passed to
    tr / tr_fill / tr_named that names Hex;
  - rewrites the literal in the source to say VibeHex;
  - adds the renamed string's rows to vibehex/naming.rs in every language,
    taken from Hex's own rows with the name replaced;
  - does the same for a literal that already says VibeHex but has no rows,
    when its Hex twin has them.

Names that keep "Hex" (Hex Microphone, Hex Meetings, ...) are left alone, as
in the tests. Run from the repository root:

    python3 scripts/vibehex-names.py          # apply
    python3 scripts/vibehex-names.py --check  # only report

What it cannot derive (a translated string with no rows anywhere, or a
translation that inflects the name) is listed for a person to write.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"
I18N = SRC / "desktop" / "i18n"
NAMING = I18N / "vibehex" / "naming.rs"

KEPT_NAMES = ["Hex Microphone", "Hex Meetings", "Hex Screenshot", "Hex Recording"]
NAME = re.compile(r"(?<![^\W\d_])Hex(?![a-z])(?! Microphone| Meetings| Screenshot| Recording)")
STRING = re.compile(r'"((?:[^"\\]|\\.)*)"', re.S)
SIX_COLUMN = ["desktop/i18n.rs", "desktop/i18n/meetings.rs", "desktop/i18n/labs.rs",
              "desktop/i18n/failures.rs", "desktop/i18n/onboarding.rs", "desktop/i18n/menu.rs"]
TWO_COLUMN = {"FRENCH": "desktop/i18n/french.rs", "PORTUGUESE": "desktop/i18n/portuguese.rs",
              "RUSSIAN": "desktop/i18n/russian.rs"}


def names_hex(text):
    return bool(NAME.search(text))


def rename(text):
    return NAME.sub("VibeHex", text)


def unescape(raw):
    out, i = [], 0
    while i < len(raw):
        if raw[i] == "\\" and i + 1 < len(raw):
            follower = raw[i + 1]
            if follower == "\n":  # a line continuation: skip the break and the indent
                i += 2
                while i < len(raw) and raw[i] in " \t\n":
                    i += 1
                continue
            out.append({"n": "\n", "t": "\t"}.get(follower, follower))
            i += 2
        else:
            out.append(raw[i])
            i += 1
    return "".join(out)


def escape(text):
    return text.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")


def table(text, name, width):
    """The rows of `const <name>` in a table file: key -> list of translations."""
    start = re.search(r"const %s\b[^=]*=\s*&\[" % name, text)
    if not start:
        return {}
    end = text.find("\n];", start.end())
    strings = [unescape(m.group(1)) for m in STRING.finditer(text[start.end():end])]
    rows = {}
    for i in range(0, len(strings) - width, width + 1):
        rows.setdefault(strings[i], strings[i + 1:i + 1 + width])
    return rows


def without_comments(text):
    return "\n".join(line for line in text.splitlines() if not line.lstrip().startswith("//"))


def main():
    check = "--check" in sys.argv[1:]

    six = {}
    for relative in SIX_COLUMN:
        for key, row in table((SRC / relative).read_text(), "TRANSLATIONS", 6).items():
            six.setdefault(key, row)
    # Hex's per-language files each call their table TRANSLATIONS.
    two = {name: table((SRC / path).read_text(), "TRANSLATIONS", 1)
           for name, path in TWO_COLUMN.items()}

    naming = NAMING.read_text()
    # Whole strings where "Hex" is Hex itself, as the tests keep them.
    kept = re.search(r"const KEPT_STRINGS[^=]*=\s*&\[(.*?)\n    \];", naming, re.S)
    kept_strings = {unescape(m.group(1)) for m in STRING.finditer(kept.group(1))} if kept else set()
    ours_six = table(naming, "TRANSLATIONS", 6)
    ours_two = {name: table(naming, name, 1) for name in TWO_COLUMN}
    # Every fork table counts as "has rows", so nothing is added twice.
    fork_keys = set()
    for path in (I18N / "vibehex").glob("*.rs"):
        text = path.read_text()
        for match in re.finditer(r"const (\w+)\b[^=]*=\s*&\[", text):
            fork_keys.update(table(text, match.group(1), 6 if match.group(1) not in TWO_COLUMN else 1))
        fork_keys.update(unescape(m.group(1)) for m in STRING.finditer(text))

    renamed, underivable, inflected = {}, [], []
    edits = 0
    for path in sorted(SRC.rglob("*.rs")):
        relative = path.relative_to(SRC).as_posix()
        if relative.startswith("desktop/i18n/") or relative == "desktop/i18n.rs":
            continue
        text = path.read_text()
        code = without_comments(text)
        called = set()
        for call in ("tr(", "tr_fill(", "tr_named("):
            for match in re.finditer(r"(?<![\w])" + re.escape(call) + r'\s*"((?:[^"\\]|\\.)*)"', code, re.S):
                called.add(unescape(match.group(1)))

        def replace(match):
            nonlocal edits
            value = unescape(match.group(1))
            if value in kept_strings:
                return match.group(0)
            if names_hex(value) and (value in six or value in called):
                new = rename(value)
                if value in six or new in six or new in fork_keys:
                    renamed.setdefault(new, value)
                else:
                    underivable.append((relative, new))
                edits += 1
                return '"' + rename(match.group(1)) + '"'
            return match.group(0)

        new_text = "\n".join(
            line if line.lstrip().startswith("//") else STRING.sub(replace, line)
            for line in text.split("\n")
        ) if any(names_hex(unescape(m.group(1))) for m in STRING.finditer(code)) else text
        called -= kept_strings
        if new_text != text and not check:
            path.write_text(new_text)

        # A literal that already says VibeHex but has no rows yet.
        for value in called:
            if "VibeHex" in value and value not in six and value not in fork_keys:
                twin = value.replace("VibeHex", "Hex")
                if twin in six:
                    renamed.setdefault(value, twin)
                else:
                    underivable.append((relative, value))

    additions = {"TRANSLATIONS": [], **{name: [] for name in TWO_COLUMN}}
    for new, old in sorted(renamed.items()):
        if new in ours_six or new in six:
            continue
        if old not in six or any(old not in two[name] for name in TWO_COLUMN):
            underivable.append(("(tables)", new))
            continue
        row = [rename(cell) for cell in six[old]]
        others = {name: rename(two[name][old][0]) for name in TWO_COLUMN}
        for cell in row + list(others.values()):
            if re.search(r"(?<!Vibe)Hex", cell) and not any(kept in cell for kept in KEPT_NAMES):
                inflected.append((new, cell))
        cells = ", ".join('"%s"' % escape(cell) for cell in row)
        additions["TRANSLATIONS"].append('    ("%s", [%s]),' % (escape(new), cells))
        for name in TWO_COLUMN:
            if new not in ours_two[name]:
                additions[name].append('    ("%s", "%s"),' % (escape(new), escape(others[name])))

    if not check:
        for name, rows in additions.items():
            if not rows:
                continue
            start = re.search(r"const %s\b[^=]*=\s*&\[" % name, naming)
            end = naming.find("\n];", start.end())
            naming = naming[:end] + "\n" + "\n".join(rows) + naming[end:]
        NAMING.write_text(naming)

    verb = "would rename" if check else "renamed"
    print(f"{verb} {edits} literals in sources; {len(additions['TRANSLATIONS'])} strings get rows")
    for new in sorted(renamed):
        print("  ", new)
    if underivable:
        print("\nno rows to derive these from; write them by hand:")
        for where, text in sorted(set(underivable)):
            print(f"   {where}: {text}")
    if inflected:
        print("\ntranslations that still say Hex (an inflected name?); check them:")
        for new, cell in inflected:
            print(f"   {new} -> {cell}")
    return 1 if check and (edits or additions["TRANSLATIONS"] or underivable) else 0


if __name__ == "__main__":
    sys.exit(main())
