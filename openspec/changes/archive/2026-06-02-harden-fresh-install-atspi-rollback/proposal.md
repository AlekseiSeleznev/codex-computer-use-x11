## Why

Fresh installs of `codex-computer-use-x11` can currently leave Cinnamon/X11 Computer Use in a partially activated state: the MCP runtime can successfully return screenshot and accessibility-tree app state, while `x11_doctor` still reports a false degraded AT-SPI state because tree availability is hardcoded false. Install and uninstall also need a rollback-first contract that captures every touched plugin, source-overlay, live-asset, gsettings, activation-environment, ownership, mode, and checksum fact so a partial install can be safely reversed without damaging unrelated user settings.

## What Changes

- Replace the doctor hardcoded AT-SPI tree-unavailable fact with a lightweight collector probe that records `tree_available`, `candidate_count`, `match_outcome`, and `controlled_fixture_pass` and only degrades on real bus/tree/bridge failure or unsafe ambiguity.
- Extend fresh install so one install flow can activate the complete Cinnamon/X11 baseline: standalone plugin, optional provider takeover, source overlay, live webview asset patch when available and authorized, and safe AT-SPI/accessibility setup.
- Make AT-SPI activation explicit and non-invasive: enable `org.gnome.desktop.interface toolkit-accessibility=true` when needed, remove or neutralize disabling `NO_AT_BRIDGE=1` from user systemd/dbus activation environments when installer-owned, preserve or set `GTK_MODULES`/`QT_ACCESSIBILITY` only when safe and necessary, and do not enable Orca/screen-reader state as part of this change.
- Introduce a manifest-backed backup/rollback model that records before-state, after-state, ownership, mode, sha256, and whether each state was changed by the installer or was already present.
- Extend uninstall/rollback to restore only installer-owned changes across standalone plugin state, provider takeover source overlay, live webview assets, gsettings, user activation environment, ownership, and file modes; support partial installs, idempotence, dry-run, report-json, drift reporting, and blockers instead of blind rollback.
- Add test and smoke coverage for doctor AT-SPI false-negative prevention, installer/uninstaller env and gsettings backup/rollback, fake fresh install → doctor ok → uninstall restored, and a live-safe verification checklist.

## Capabilities

- Modify `doctor-cli` for AT-SPI collector probe facts and false-negative prevention.
- Modify `x11-atspi-window-correlation` for canonical probe outcomes and controlled fixture pass reporting consumed by doctor.
- Modify `standalone-codex-mcp-plugin` for complete user-local install/uninstall state ownership, activation environment handling, dry-run/report-json, and manifest-backed rollback of standalone plugin paths.
- Modify `codex-source-overlay-extension` for source-overlay, provider-takeover, and live-asset manifest backup/rollback semantics.
- Modify `codex-computer-use-provider-takeover` for rollback-first takeover install behavior that preserves standalone identity and bundled fallback per ADR 0010.
- Modify `codex-x11-e2e-test-harness` for fake and live-safe fresh-install/doctor/uninstall verification evidence.

## Impact

- Affected implementation areas: `src/doctor.rs`, AT-SPI collector/correlation code paths used by `x11_accessibility_tree` and `x11_get_app_state`, `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/install-codex-source-overlay.sh`, `scripts/uninstall-codex-source-overlay.sh`, `scripts/install-x11-provider-takeover.sh`, `scripts/uninstall-x11-provider-takeover.sh`, `scripts/codex-source-overlay.py`, and e2e smoke scripts under `scripts/e2e/`.
- Affected external system state is local-only desktop/user state: `$CODEX_HOME` plugin cache/config/marketplace paths, optional Codex Desktop Linux source target selected by `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented local default, optional root-owned live webview assets when sudo/live patching is requested, gsettings, and user systemd/dbus activation environment. No secret variables are required and `.secrets.local.env` must not be read.
- Architecture constraints: keep standalone plugin identity and `x11_*` tool names; localize provider takeover compatibility to settings/provider resolver payloads; do not globally masquerade as bundled `computer-use`; preserve rollback to bundled mode per ADR 0010; keep Cinnamon/X11 v1 scope from ADR 0009 and root-coordinate/safety constraints from ADR 0008.
- Verification constraints: run OpenSpec validation for changed artifacts before apply, then implementation verification must include `make fmt`, `make check`, `make test`, `scripts/e2e/codex-plugin-smoke.sh --fake`, relevant dry-run install/uninstall checks, and live-safe checklist evidence when live access is available.
