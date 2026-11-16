# Contributing to VibeHex

VibeHex is a fork of [Hex](https://github.com/anomalyco/hex) and keeps merging
Hex's changes, so the first question is where a report belongs.

## Where to report

- **Something VibeHex added** (the assistant, built-in cleanup, Files,
  anything starred in Labs):
  [open an issue here](https://github.com/makeaiwork/vibehex/issues/new/choose).
- **Something that also happens in Hex itself:** report it to
  [Hex](https://github.com/anomalyco/hex/issues/new/choose). The fix reaches
  VibeHex with the next merge from Hex.
- **Not sure which:** open it here and I'll work out where it belongs.
- **A security problem:** follow [`SECURITY.md`](SECURITY.md), not a public
  issue.

Diagnoses and proposed fixes are welcome in the issue: the code you suspect,
or a patch as a diff snippet. Redact logs first, and don't attach private
dictation, recordings or your settings file.

## Pull requests

Small fixes are welcome as pull requests against `dev`. For anything larger,
such as a new feature or a change in behaviour, open an issue first so we
agree on the approach before you write the code.

- New features land in Labs, off by default: see
  [`docs/features/labs.md`](docs/features/labs.md).
- Keep changes to code VibeHex shares with Hex small. Every difference has to
  be carried through each merge from Hex.
- Run `scripts/check` before you open the pull request, and say which checks
  you ran.

To build VibeHex yourself, see [`AGENTS.md`](AGENTS.md) and
[`docs/`](docs/README.md).

## Licence

VibeHex is MIT-licensed, as Hex is. By contributing you agree that your
contribution is released under the same [`LICENSE`](LICENSE).
