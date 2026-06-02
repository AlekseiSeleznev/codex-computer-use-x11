## Why

The installed `codex-computer-use-x11` plugin can reach the AT-SPI bus on the supported Linux Mint Cinnamon/X11 desktop, but current readiness evidence still collapses `tree_available=false` into a generic environment limitation and misses the actionable clue that Codex is inheriting `NO_AT_BRIDGE=1`. Operators need diagnostics and fixture evidence that distinguish a reachable AT-SPI bus from a disabled GTK/ATK bridge without weakening the X11-only baseline or using real user windows as fallback targets.

## What Changes

- Detect `NO_AT_BRIDGE=1` explicitly in `doctor --json` and classify it as `atspi_gtk_bridge_disabled_by_environment` when the AT-SPI bus is reachable but tree extraction is unavailable.
- Make AT-SPI recommendations precise: remove or avoid inheriting `NO_AT_BRIDGE`, restart the Cinnamon/Codex session or the affected fixture process, and verify with a controlled GTK fixture before claiming semantic accessibility readiness.
- Correct the controlled GTK fixture launch contract so fixture subprocesses remove `NO_AT_BRIDGE` from their environment rather than setting it to `0`; keep `GTK_MODULES=gail:atk-bridge` only where needed and record sanitized bridge-env metadata.
- Extend fake tests and validation evidence for `NO_AT_BRIDGE=1`, bridge-disabled diagnosis, and fixture bridge environment recording.
- Update troubleshooting/retest documentation with an “AT-SPI bus reachable but tree extraction unavailable” path covering packages, gsettings, processes, `NO_AT_BRIDGE`, controlled GTK fixture verification, and expected degraded semantics for the Cinnamon/X11 baseline.

## Capabilities

Modified capabilities:

- `doctor-cli` — AT-SPI bridge-disabled diagnosis, env fact redaction, recommendation text, and machine-readable readiness/degraded fields.
- `x11-atspi-window-correlation` — controlled GTK fixture bridge environment contract and canonical bridge-disabled outcome mapping.
- `codex-x11-e2e-test-harness` — fake/live fixture self-test evidence, bridge-env recording, and validator treatment of bridge-disabled environment limitations.
- `x11-packaging-docs-upstreaming` — troubleshooting and safe retest documentation for bus-reachable/tree-unavailable AT-SPI diagnostics.

## Impact

- Affected implementation will likely include `src/doctor.rs`, AT-SPI/accessibility diagnostic models, e2e fixture orchestration in `scripts/e2e/codex-x11-e2e.py`, GTK fixture metadata, Rust integration tests, docs, and validation tests.
- No external systems or credentials are required. `.secrets.local.env` must remain unread, unprinted, unstaged, uncommitted, and absent from evidence.
- The change stays inside ADR 0009’s Cinnamon/X11 `x11-ewmh` baseline. Wayland and portal-required runtime paths remain out of scope.
- ADR 0008 remains in force: any fixture screenshot/app-state evidence uses X11 root coordinates and controlled targets only.
- ADR 0010 remains in force: no bundled `computer-use` patching, provider takeover, global plugin-id masquerade, or rollback behavior changes are part of this change.
- Live input, pointer, overlay, screenshot, app-state, and AT-SPI checks may only use controlled fixtures; absence of fixture code or a bridge-disabled fixture is degraded evidence, not permission to inspect or act on real user windows.
- Future apply must use TDD slices and pass `make fmt`, `make check`, `make test`, `doctor --json` JSON validation, fake smoke/validator checks, and OpenSpec strict validation, or record exact blockers.
