//! VibeHex: known key and token shapes cut out of the text around the cursor
//! before it leaves the app it was read from, so a terminal's scrollback or
//! a config file near the cursor doesn't carry them to a cleanup engine.
//!
//! Best effort, by shape only: PEM private key blocks (one with no end is cut
//! to the end of the text, which the read bounds), SSH key material, provider
//! tokens by their prefixes, JSON Web Tokens, `Authorization` and `Bearer`
//! values, a password in a URL, values assigned to a name that says secret,
//! and long opaque base64 or hex runs that mix letters and digits. Each match
//! becomes [`REMOVED`]; anything else visible near the cursor may still be
//! sent. The text is a few KB at most and every pattern is a `regex` (linear
//! time), compiled once.

use std::borrow::Cow;
use std::sync::LazyLock;

use regex::{Captures, Regex};

use crate::dictation::caret_text::CaretText;

/// What stands in place of a secret.
pub(crate) const REMOVED: &str = "[removed]";

/// A PEM private key's armour, BEGIN or END.
const PEM_LABEL: &str = r"[A-Z0-9 ]*PRIVATE KEY(?: BLOCK)?-----";

struct Patterns {
    /// A whole PEM private key, or its start to the end of the text.
    pem: Regex,
    /// The end of a PEM private key whose start was cut off: everything up
    /// to it.
    pem_tail: Regex,
    ssh: Regex,
    url: Regex,
    authorization: Regex,
    bearer: Regex,
    assignment: Regex,
    token: Regex,
    jwt: Regex,
    base64: Regex,
    hex: Regex,
}

static PATTERNS: LazyLock<Patterns> = LazyLock::new(|| {
    let regex = |pattern: &str| Regex::new(pattern).expect("a valid secret pattern");
    Patterns {
        pem: regex(&format!(
            r"(?s)-----BEGIN {PEM_LABEL}.*?(?:-----END {PEM_LABEL}|\z)"
        )),
        pem_tail: regex(&format!(r"(?s)\A.*?-----END {PEM_LABEL}")),
        ssh: regex(
            r"\b((?:ssh-(?:rsa|ed25519|dss)|ecdsa-sha2-nistp[0-9]+|sk-(?:ssh-ed25519|ecdsa-sha2-nistp256)@openssh\.com)\s+)AAAA[A-Za-z0-9+/]+=*",
        ),
        url: regex(r"\b([A-Za-z][A-Za-z0-9+.-]*://[^\s:/@]+:)[^\s/@]+@"),
        authorization: regex(
            r#"(?i)\b(authorization["']?\s*[:=]\s*["']?(?:bearer|basic|token)\s+)[A-Za-z0-9._~+/=-]+"#,
        ),
        bearer: regex(r"(?i)\b(bearer\s+)[A-Za-z0-9._~+/-]{20,}=*"),
        assignment: regex(
            r#"(?i)\b((?:[A-Za-z0-9_.-]*(?:password|passwd|passphrase|secret|token|credentials?|api[_-]?key|access[_-]?key|private[_-]?key)[A-Za-z0-9_.-]*|(?:(?:[A-Za-z0-9_.-]*[_.-])?(?:pass|pwd)|[A-Za-z0-9_.-]*[_-]key)(?:[_.-][A-Za-z0-9_.-]*)?)["']?\s*[:=]\s*)("[^"\n]*"|'[^'\n]*'|[^\s"'`,;(){}\[\]<>=][^\s"'`,;(){}\[\]<>]*)"#,
        ),
        token: regex(concat!(
            r"\b(?:sk-[A-Za-z0-9_-]{16,}",
            r"|(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{20,}",
            r"|github_pat_[A-Za-z0-9_]{20,}",
            r"|glpat-[A-Za-z0-9_-]{20,}",
            r"|xox[abprs]-[A-Za-z0-9-]{10,}",
            r"|(?:AKIA|ASIA)[A-Z0-9]{16}\b",
            r"|AIza[A-Za-z0-9_-]{35}",
            r"|hf_[A-Za-z0-9]{30,}",
            r"|npm_[A-Za-z0-9]{30,}",
            r"|pypi-[A-Za-z0-9_-]{30,}",
            r"|(?:sk|rk|pk)_(?:live|test)_[A-Za-z0-9]{16,})",
        )),
        jwt: regex(r"\beyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}"),
        base64: regex(r"[A-Za-z0-9+/_-]{40,}={0,2}"),
        hex: regex(r"\b[0-9A-Fa-f]{32,}\b"),
    }
});

/// `caret` with every known secret shape cut out of each part.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn cut_from(caret: CaretText) -> CaretText {
    CaretText {
        before: cut(&caret.before).into_owned(),
        selected: cut(&caret.selected).into_owned(),
        after: cut(&caret.after).into_owned(),
    }
}

/// `text` with every known secret shape replaced by [`REMOVED`]; borrowed
/// when there is none.
pub(crate) fn cut(text: &str) -> Cow<'_, str> {
    let p = &*PATTERNS;
    let mut text = Cow::Borrowed(text);
    replace(&mut text, &p.pem, |_| REMOVED.into());
    replace(&mut text, &p.pem_tail, |_| REMOVED.into());
    replace(&mut text, &p.ssh, |found| format!("{}{REMOVED}", &found[1]));
    replace(&mut text, &p.url, |found| {
        format!("{}{REMOVED}@", &found[1])
    });
    replace(&mut text, &p.authorization, |found| {
        format!("{}{REMOVED}", &found[1])
    });
    replace(&mut text, &p.bearer, |found| {
        format!("{}{REMOVED}", &found[1])
    });
    replace(&mut text, &p.assignment, |found| {
        let value = &found[2];
        let quoted = value.starts_with(['"', '\'']);
        if quoted && value.len() > 2 {
            let quote = &value[..1];
            format!("{}{quote}{REMOVED}{quote}", &found[1])
        } else if !quoted && looks_secret(value) {
            format!("{}{REMOVED}", &found[1])
        } else {
            found[0].to_owned()
        }
    });
    replace(&mut text, &p.token, |_| REMOVED.into());
    replace(&mut text, &p.jwt, |_| REMOVED.into());
    replace(&mut text, &p.base64, |found| {
        if opaque(&found[0], 3, true) {
            REMOVED.into()
        } else {
            found[0].to_owned()
        }
    });
    replace(&mut text, &p.hex, |found| {
        if opaque(&found[0], 1, false) {
            REMOVED.into()
        } else {
            found[0].to_owned()
        }
    });
    text
}

/// Replaces every match of `pattern` in `text` with what `with` makes of it.
fn replace(text: &mut Cow<'_, str>, pattern: &Regex, with: impl Fn(&Captures<'_>) -> String) {
    if pattern.is_match(text) {
        let replaced = pattern.replace_all(text, |found: &Captures<'_>| with(found));
        // A match may be kept as it is (a word, not a secret).
        if replaced != *text {
            *text = Cow::Owned(replaced.into_owned());
        }
    }
}

/// Whether an unquoted value assigned to a secret's name looks like one: it
/// has a digit or a symbol, so `password: String` or `secret: the plan`
/// stay.
fn looks_secret(value: &str) -> bool {
    value
        .chars()
        .any(|c| c.is_ascii_digit() || "!@#$%^+/=~-".contains(c))
}

/// Whether a long run looks random rather than like a word, a name or a
/// path: letters and at least `digits` digits, and (where `separators`
/// count) few `-`, `_` or `/`, which a path or a slug has many of.
fn opaque(run: &str, digits: usize, separators: bool) -> bool {
    let letters = run.chars().filter(char::is_ascii_alphabetic).count();
    let numbers = run.chars().filter(char::is_ascii_digit).count();
    let breaks = run.chars().filter(|c| matches!(c, '-' | '_' | '/')).count();
    letters > 0 && numbers >= digits && (!separators || breaks * 8 < run.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn removed(text: &str) -> String {
        cut(text).into_owned()
    }

    #[test]
    fn a_pem_private_key_is_cut_whole_or_to_the_end() {
        let key = "config:\n-----BEGIN OPENSSH PRIVATE KEY-----\nb3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQ\n-----END OPENSSH PRIVATE KEY-----\ndone";
        assert_eq!(removed(key), "config:\n[removed]\ndone");
        for label in [
            "RSA PRIVATE KEY",
            "EC PRIVATE KEY",
            "DSA PRIVATE KEY",
            "PRIVATE KEY",
            "ENCRYPTED PRIVATE KEY",
            "PGP PRIVATE KEY BLOCK",
        ] {
            let block = format!("a -----BEGIN {label}-----\nMIIEow\n-----END {label}----- b");
            assert_eq!(removed(&block), "a [removed] b", "{label}");
        }
        // The read is bounded, so the block may be cut off at either end.
        let start = "see below\n-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA";
        assert_eq!(removed(start), "see below\n[removed]");
        let tail = "Proc-Type: 4\nMIIEowIBAAKCAQEA\n-----END RSA PRIVATE KEY-----\nnext";
        assert_eq!(removed(tail), "[removed]\nnext");
        // A public key's armour is not a secret.
        let public = "-----BEGIN PUBLIC KEY-----";
        assert_eq!(removed(public), public);
    }

    #[test]
    fn ssh_key_material_is_cut_and_its_type_kept() {
        for kind in ["ssh-ed25519", "ssh-rsa", "ssh-dss", "ecdsa-sha2-nistp256"] {
            let line = format!("{kind} AAAAC3NzaC1lZDI1NTE5AAAAIGq0 denis@mac");
            assert_eq!(removed(&line), format!("{kind} [removed] denis@mac"));
        }
    }

    #[test]
    fn provider_tokens_are_cut_by_their_prefix() {
        // Each example is put together here, so no line of this file reads as a
        // real key to a scanner.
        for (prefix, rest) in [
            ("sk-", "proj-Ab3dEf6hIj9kLm2nOp5q"),
            ("sk-", "ant-api03-Ab3dEf6hIj9kLm2nOp5qRs8t"),
            ("sk-", "or-v1-0123456789abcdef0123"),
            ("ghp_", "Ab3dEf6hIj9kLm2nOp5qRs8tUv1w"),
            ("gho_", "Ab3dEf6hIj9kLm2nOp5qRs8tUv1w"),
            ("ghu_", "Ab3dEf6hIj9kLm2nOp5qRs8tUv1w"),
            ("ghs_", "Ab3dEf6hIj9kLm2nOp5qRs8tUv1w"),
            ("ghr_", "Ab3dEf6hIj9kLm2nOp5qRs8tUv1w"),
            ("github_pat_", "11ABCDEFG0123456789_abcdefghij"),
            ("glpat-", "Ab3dEf6hIj9kLm2nOp5q"),
            ("xoxb-", "1234567890-abcdefghij"),
            ("xoxp-", "1234567890-abcdefghij"),
            ("AKIA", "IOSFODNN7EXAMPLE"),
            ("ASIA", "IOSFODNN7EXAMPLE"),
            ("AIza", "SyAb3dEf6hIj9kLm2nOp5qRs8tUv1wXy4zQ"),
            ("hf_", "Ab3dEf6hIj9kLm2nOp5qRs8tUv1wXy4z"),
            ("npm_", "Ab3dEf6hIj9kLm2nOp5qRs8tUv1wXy4z"),
            ("pypi-", "AgEIcHlwaS5vcmcCJDEyMzQ1Njc4LTk0ZjUtNDU"),
            ("sk_live_", "Ab3dEf6hIj9kLm2nOp5q"),
            ("rk_live_", "Ab3dEf6hIj9kLm2nOp5q"),
            ("pk_live_", "Ab3dEf6hIj9kLm2nOp5q"),
        ] {
            let token = format!("{prefix}{rest}");
            assert_eq!(
                removed(&format!("use {token} now")),
                "use [removed] now",
                "{token}"
            );
        }
    }

    #[test]
    fn a_json_web_token_is_cut() {
        let jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";
        assert_eq!(removed(&format!("jwt {jwt}.")), "jwt [removed].");
    }

    #[test]
    fn authorization_values_are_cut_and_their_scheme_kept() {
        assert_eq!(
            removed("Authorization: Bearer abc.def-123"),
            "Authorization: Bearer [removed]"
        );
        assert_eq!(
            removed("-H 'authorization: Basic dXNlcjpwYXNz'"),
            "-H 'authorization: Basic [removed]'"
        );
        assert_eq!(
            removed("send Bearer 0123456789abcdefghijklmnop with it"),
            "send Bearer [removed] with it"
        );
        // A short word after "bearer" in prose stays.
        assert_eq!(removed("the bearer of bad news"), "the bearer of bad news");
    }

    #[test]
    fn a_password_in_a_url_is_cut_and_the_rest_kept() {
        assert_eq!(
            removed("postgres://admin:hunter2@db.local:5432/app"),
            "postgres://admin:[removed]@db.local:5432/app"
        );
        assert_eq!(
            removed("https://example.com:8080/path"),
            "https://example.com:8080/path"
        );
    }

    #[test]
    fn values_assigned_to_a_secrets_name_are_cut() {
        for (line, expected) in [
            ("password=hunter2", "password=[removed]"),
            // Short names an .env file uses: a whole word of the name.
            ("DB_PASS=hunter2", "DB_PASS=[removed]"),
            ("ADMIN_PWD=s3cret!", "ADMIN_PWD=[removed]"),
            ("STRIPE_KEY=abc123def", "STRIPE_KEY=[removed]"),
            ("PGPASSWORD=pg-pass-1", "PGPASSWORD=[removed]"),
            (
                "ssh_passphrase: \"correct horse\"",
                "ssh_passphrase: \"[removed]\"",
            ),
            ("DB_PASSWD: s3cret!", "DB_PASSWD: [removed]"),
            (
                "export GITHUB_TOKEN=ghp_x1",
                "export GITHUB_TOKEN=[removed]",
            ),
            (r#""api_key": "abc""#, r#""api_key": "[removed]""#),
            ("apiKey = 'k-42'", "apiKey = '[removed]'"),
            ("api-key: z9", "api-key: [removed]"),
            ("AWS_ACCESS_KEY=x-1", "AWS_ACCESS_KEY=[removed]"),
            ("private_key = \"x\"", "private_key = \"[removed]\""),
            ("client_secret: 0a", "client_secret: [removed]"),
            ("secret = ''", "secret = ''"),
        ] {
            assert_eq!(removed(line), expected, "{line}");
        }
    }

    #[test]
    fn long_opaque_runs_are_cut() {
        let base64 = "dGhpcyBpcyBhIHNlY3JldCB2YWx1ZSAxMjM0NTY3ODkw";
        assert_eq!(removed(&format!("key {base64} end")), "key [removed] end");
        let hex = "9f86d081884c7d659a2feaa0c55ad015";
        assert_eq!(removed(&format!("hash {hex}")), "hash [removed]");
    }

    /// Prose, code and paths come through unchanged, and borrowed.
    #[test]
    fn ordinary_text_is_left_alone() {
        for text in [
            "We ship the new onboarding on Friday; Anna reviews the copy.",
            "Не забудь пароль от стенда: спроси у Дениса.",
            "let token = next_token(&mut lexer);",
            "if token == other { return; }",
            "struct Login { password: String, secret: Option<Secret> }",
            "the secret: keep it short",
            "/Volumes/DropboxSSD/Dropbox/135/58/VibeHex/src/dictation/processor.rs",
            "https://github.com/makeaiwork/vibehex/pull/46/files#diff-3a",
            "cargo test --locked --bin hex rune_ui::tests::goldens",
            "ThisIsAVeryLongCamelCaseIdentifierWithoutAnyDigitsInside",
            "test_parses_42_lines_of_input_in_under_100_ms_on_a_laptop",
            "the bearer token goes in the header",
            "tests passed: 12, skipped: 3",
            "compass = north-east",
            "bypass=1",
            "ssh-ed25519 is the key type we use",
            "",
        ] {
            assert!(
                matches!(cut(text), Cow::Borrowed(_)),
                "{text} -> {}",
                cut(text)
            );
        }
    }

    #[test]
    fn every_part_of_the_text_around_the_cursor_is_cut() {
        let caret = CaretText {
            before: "export OPENAI_API_KEY=sk-proj-Ab3dEf6hIj9kLm2nOp5q\n$ ".into(),
            selected: "AKIAIOSFODNN7EXAMPLE".into(),
            after: " # ok".into(),
        };
        let cut = cut_from(caret);
        assert_eq!(cut.before, "export OPENAI_API_KEY=[removed]\n$ ");
        assert_eq!(cut.selected, "[removed]");
        assert_eq!(cut.after, " # ok");
    }
}
