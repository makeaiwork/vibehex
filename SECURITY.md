# Security

Report vulnerabilities in VibeHex through
[GitHub private vulnerability reporting](https://github.com/makeaiwork/vibehex/security/advisories/new).
Do not include secrets, private transcripts, browsing history, or unredacted
diagnostic logs in a public issue.

VibeHex is a fork of [Hex](https://github.com/anomalyco/hex). If the problem
is also in Hex itself, please report it to Hex too, through
[its own private reporting](https://github.com/anomalyco/hex/security/advisories/new),
so both apps get the fix.

Custom command handlers and npm dependencies execute locally with the user's
normal permissions. They are supervised for lifecycle and bounded IPC, but they
are not sandboxed.
