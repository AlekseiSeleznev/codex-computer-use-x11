## Why

The REAL LIVE retest `20260601T162050Z` proved the installed standalone X11 plugin mostly works on controlled Cinnamon/X11 windows, but it exposed an evidence-safety bug: `get-app-state --json` serializes screenshot pixels inline as a large `data:image/png;base64,...` value. Machine-readable app-state evidence must be safe by default: screenshots should be written as files or omitted with diagnostics while window, accessibility, and capability layers remain usable.

## What Changes

- Make `get-app-state` screenshot JSON safe by default by replacing inline screenshot `data_url` output with path-oriented screenshot metadata and explicit `screenshot_error` diagnostics.
- Add caller-facing screenshot output control for app state, such as `--screenshot-output <path>` for CLI and an equivalent MCP argument, resolving successful screenshots to non-empty PNG files referenced from JSON.
- Preserve `--no-screenshot` behavior and layer-degraded app-state semantics when screenshot capture is disabled or unavailable.
- If an inline screenshot mode is retained for legacy debugging, require an explicit opt-in such as `--inline-screenshot`, document it as unsafe for durable evidence, and keep default JSON path-only.
- Keep existing `screenshot-crop` behavior path-only and unchanged except for regression coverage proving it remains unaffected.
- Add or rework a reusable real-live controlled fixture runner for manual/industrial retests that launches controlled Tk/GTK fixtures safely, records metadata, avoids project-owned/overlay-looking unsafe target titles, keeps fixtures alive for the retest, and cleans them up reliably.
- Update troubleshooting/e2e/release documentation to explain path-only app-state screenshot evidence, controlled real-live fixture retests, and `NO_AT_BRIDGE=1` diagnostic/remediation behavior.
- No **BREAKING** changes to backend identity, standalone plugin identity, `x11_*` MCP tool names, source-overlay/provider-takeover architecture, or the Cinnamon/X11 v1 scope.

## Capabilities

Modified capabilities:

- `x11-get-app-state-integration` — app-state screenshot capture becomes safe-by-default path-oriented evidence, with explicit opt-in only for inline screenshot data if retained.
- `codex-x11-e2e-test-harness` — reusable controlled real-live fixture runner for manual/industrial Cinnamon/X11 retests, metadata capture, safe target selection, and cleanup.
- `x11-packaging-docs-upstreaming` — docs and release guidance for safe app-state screenshot evidence, real-live controlled fixture retests, and AT-SPI bridge-disabled remediation.

## Impact

- Expected implementation areas include `src/app_state.rs`, `src/cli.rs`, `src/mcp.rs`, `tests/get_app_state_cli.rs`, `tests/mcp_server.rs`, `tests/e2e_harness_scripts.rs`, `scripts/e2e/codex-x11-e2e.py`, and docs under `docs/`.
- The change must preserve ADR 0008 X11 root/global coordinate semantics and path-only screenshot crop evidence, ADR 0009 Cinnamon/X11 v1 safety boundaries and degraded diagnostics, and ADR 0010 standalone/source-overlay/provider-takeover identity boundaries.
- The change is limited to the Cinnamon/X11 baseline. Wayland and portal-required runtime paths are out of scope.
- Verification must include `openspec validate --all --strict`, `make fmt`, `make check`, `make test`, targeted CLI/MCP tests for app-state screenshot JSON safety, and targeted docs/harness tests for the controlled real-live fixture runner.
- No external credentials are required. `.secrets.local.env` must not be read, printed, staged, committed, archived, or copied into artifacts.
