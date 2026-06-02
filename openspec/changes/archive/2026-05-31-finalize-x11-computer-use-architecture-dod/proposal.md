## Why

The backlog has delivered X11/EWMH capabilities in separate stages, but v1 still needs an explicit architecture and Definition-of-Done gate that answers whether `codex-computer-use-x11` is a complete Cinnamon/X11 Computer Use baseline and where any degraded boundaries remain.

## What Changes

- Add a final architecture/DoD capability that records the v1 decision set, evidence expectations, and a fine-grained Computer Use capability matrix.
- Add a machine-checkable DoD validator that fails when required v1 capability rows, architecture decisions, or evidence/degraded reasons are missing.
- Update release/e2e documentation so final handoff uses the DoD validator in addition to existing fake/live smoke evidence and OpenSpec validation.
- Record fresh research for the current local project state, current target checkout, and current external references/license posture before making final v1 claims.

## Capabilities

- New capability: `x11-computer-use-architecture-dod` — defines final architecture decision coverage, DoD evidence rows, checker behavior, and the precise v1 readiness answer for Cinnamon/X11.

## Impact

- Affected code/scripts: final DoD validation script under `scripts/`, tests under `tests/`, and any existing e2e evidence fixtures/validation helpers needed to feed the checker.
- Affected docs: `README.md`, `docs/release-checklist.md`, e2e/release/architecture DoD documentation, and OpenSpec canonical specs after archive.
- Affected architecture: consolidates in-force ADR/design decisions including `x11-ewmh`, `WindowInfo` primary model, sidecar diagnostics, shell-out thresholds, strict portal diagnostics, verified-focus input safety, root-coordinate screenshot/crop semantics, get-app-state composition, standalone-vs-source-overlay strategy, licensing boundaries, and Cinnamon Wayland/extension v2 scope.
- Verification constraints: run `make fmt`, `make check`, `make test`, DoD checker tests, fake standalone/source-overlay e2e matrix validation, `openspec validate finalize-x11-computer-use-architecture-dod --type change --strict`, and `openspec validate --all --strict` before archive.
- Secret handling: no `.secrets.local.env` access is needed; evidence and docs must use variable names only, such as `CODEX_DESKTOP_LINUX_FULL_PATH`, and must not record credentials or secret values.

## Research refresh

- Date: 2026-05-31.
- Local project state: `main` branch was clean before scaffold; active backlog next item is `backlog/13-final-architecture-dod.md`; previous OpenSpec changes through packaging/docs/upstreaming are archived.
- Target checkout state: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is on `main` and clean. Current `computer-use-linux/src/server.rs` exposes stock `activate_window`, `get_app_state`, `type_text`, `press_key`, `click`, `scroll`, and `drag`; there is no requirement to invent stock `focus_window` or `mousemove` for v1.
- Target files reviewed: `computer-use-linux/src/windowing/`, `server.rs`, `diagnostics.rs`, `atspi_tree.rs`, `screenshot.rs`, and `remote_desktop.rs`.
- External references refreshed: freedesktop EWMH spec for `_NET_CLIENT_LIST`/`_NET_ACTIVE_WINDOW`; xdg-desktop-portal Screenshot/RemoteDesktop documentation; GNOME Shell `Shell.Screenshot`; AT-SPI D-Bus documentation; GitHub metadata/license checks for `psychon/x11rb`, `jordansissel/xdotool`, and `ReimuNotMoe/ydotool`.
- Ideas kept: continue treating `x11rb` as a future native X11 threshold option; treat portal Screenshot method presence separately from RemoteDesktop input readiness; keep runtime command invocation distinct from copying/vendoring external command source.
- Ideas rejected for v1: Cinnamon Wayland, an unstable Cinnamon/Muffin extension, unverified direct X11/`xdotool --window` injection as a safety boundary, and a parallel source-overlay `x11_get_app_state` stock tool.
