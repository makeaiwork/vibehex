---
feature: api-browser-access
key: labs.api_browser_access
title: Browser access to the local API
for: []
ask: Let web pages you list call VibeHex's local API?
group: Output
os: every OS
vibehex: false
---

# Browser access to the local API

**What it does.** A browser normally blocks web pages from talking to VibeHex's local API. With this on, its card takes a list of origins, such as `http://localhost:5173`, and pages served from exactly those may call the API from the browser.

**Turn it on when.** You build a web page or tool that should dictate or transcribe through VibeHex.

**Leave it off when.** You don't develop such a page. Off, or for an origin not listed, nothing changes.

**Needs.** The page still needs the API token for every request; this only lets the browser through. Each entry is a scheme, host and optional port; wildcards and paths are ignored.

**Good to know.** List only pages you trust: a listed page that has the token can do whatever the API allows. The API still listens only on this computer.

**Where.** Labs › Browser access to the local API (`labs.api_browser_access`); its origins (`labs.api_browser_origins`).

**Asked as.** "Call VibeHex from my web app"; "CORS error from the local API"; «Доступ к API VibeHex из браузера»; «Ошибка CORS при запросе к VibeHex».

<!-- sources: upstream docs/features/labs.md:146-167; src/labs/card.rs:74, 260-262; src/control/settings/registry.rs:637-640; src/labs/catalog.rs (EVERY); src/labs/group.rs (Output) -->
