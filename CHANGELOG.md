# Changelog

All notable changes for `codex-computer-use-x11` are recorded here.

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
