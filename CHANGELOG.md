# Changelog

All notable changes for `codex-computer-use-x11` are recorded here.

## v0.1.2 — 2026-06-02

Documentation refresh release for the fresh public baseline.

### Documentation

- Added the Codex plugin page screenshot to the README as tracked release evidence.
- Moved the plugin screenshot below the introductory `codex-computer-use-x11` paragraph so the README opens with the hero image, then the product summary, then the installed-plugin screenshot.
- Updated release identity to `v0.1.2`.

### Verification

Validated before release with:

```bash
make fmt
make check
make test
openspec validate --all --strict
git diff --check
```

## v0.1.1 — 2026-06-02

Maintenance release for the Cinnamon/X11 baseline after full OpenSpec archive and live verification.

### Fixed

- Fixed `doctor --json` AT-SPI false-negative where a large valid collector response could fill stdout/stderr pipes, block the bounded probe, and be reported as `collector_unavailable` even while `accessibility-tree --window-id ... --json` succeeded.
- Preserved bounded doctor probe timeout behavior while draining collector output concurrently.
- Removed RemoteDesktop portal and Wayland runtime noise from X11-only doctor readiness: absent RemoteDesktop portal and `WAYLAND_DISPLAY` no longer add blockers, degraded reasons, optional enrichments, unsupported-out-of-scope entries, or recommended next steps.
- Kept RemoteDesktop/Wayland facts as compatibility/debug context only; X11 development input readiness now depends on local X11-supported backends (`/dev/uinput` or ydotool).

### Documentation and specifications

- Archived the AT-SPI doctor fix and X11-only doctor readiness changes, then synced canonical `doctor-cli`, `x11-atspi-window-correlation`, and `x11-packaging-docs-upstreaming` specs.
- Updated README architecture diagram to show X11-only readiness and debug-only RemoteDesktop/Wayland compatibility facts.
- Updated release identity to `v0.1.1`.

### Verification

Validated before release with:

```bash
openspec validate <all canonical specs> --type spec --strict
make fmt
make check
make test
```

Live-safe local CLI comparison confirmed `doctor --json` reports `tree_available=true` and `match_outcome=tree_available` when `accessibility-tree --window-id ... --json` succeeds on the focused X11 window.

## v0.1.0 — 2026-06-01

First fresh public baseline for safe Cinnamon/X11 Computer Use in Codex.

### Added

- Standalone Rust CLI with `doctor`, window listing/focus, verified keyboard input, pointer actions, accessibility tree, screenshot crop, app-state, target-window context, and MCP server mode.
- User-local Codex MCP plugin installer and uninstaller that write only owned `codex-computer-use-x11` paths under `CODEX_HOME`.
- Namespaced `x11_*` MCP tool surface for direct Codex use without replacing bundled `computer-use` globally.
- Reversible source overlay for local Codex Desktop Linux validation, including status, install, uninstall, drift detection, and provider-takeover support.
- Controlled fake and live-fixture e2e harness with capability-matrix validation.
- GitHub README hero image and release installation notes.

### Safety

- Targeted keyboard and pointer actions require exact target resolution and verified X11 focus before injection.
- X11 root/global pixels are the shared coordinate model for bounds, pointer points, screenshot crops, and app-state composition.
- `get-app-state --json` returns screenshot metadata and paths by default; inline screenshot data requires explicit `--inline-screenshot` opt-in.
- AT-SPI tree extraction returns a subtree only for confident correlation; missing, ambiguous, or environment-disabled accessibility is reported as degraded diagnostics.
- Live input/pointer/screenshot/app-state checks are valid only against controlled fixtures.

### Documentation and specifications

- Canonical OpenSpec specs are current and have non-placeholder Purpose sections.
- `ARCHITECTURE.md` is the current architecture snapshot and includes the runtime architecture overview.
- `docs/final-architecture-dod.md` records the v1 readiness answer and evidence boundaries.
- The retired planning backlog has been removed; canonical behavior now lives in `openspec/specs/`, archived change evidence, ADRs, and docs.

### Verification

Validated before release with:

```bash
make fmt
make check
make test
openspec validate --all --strict
git diff --check
```
