---
feature: shell-completions
key: labs.shell_completions
title: Shell completions
for: []
ask: Add Tab completion for the hex command in your terminal?
group: Output
os: every OS
vibehex: false
---

# Shell completions

**What it does.** Adds `hex completions <shell>`, which prints a completion script so your terminal can complete `hex` commands and options with Tab. It knows bash, zsh, fish, elvish and PowerShell, and always matches the commands this VibeHex has.

**Turn it on when.** You use the `hex` command often.

**Leave it off when.** You don't use the terminal. Off, the command answers that the lab is off and stays out of `hex --help`.

**Needs.** Nothing else; VibeHex doesn't need to be running. Save the script where your shell looks, for example `hex completions zsh > ~/.zfunc/_hex` with `~/.zfunc` on zsh's `fpath`, or `hex completions fish > ~/.config/fish/completions/hex.fish`.

**Good to know.** Turn it on with `hex labs on shell-completions` from the terminal too. After an update, print the script again to pick up new commands.

**Where.** Labs › Shell completions (`labs.shell_completions`).

**Asked as.** "Tab completion for hex"; "zsh completions"; «Автодополнение команды hex»; «Как включить дополнение по Tab для hex».

<!-- sources: upstream docs/cli.md:86, 202-223; docs/features/labs.md:179-186; src/labs/card.rs:83, 287-289; src/labs/catalog.rs (EVERY); src/labs/group.rs (Output) -->
