## 1. CLI app-state composition

- [x] 1.1 RED: Add `tests/get_app_state_cli.rs` coverage proving `get-app-state --window-id 0x2 --no-screenshot --json` resolves `window_context` and leaves `window_error` null.
- [x] 1.2 GREEN: Add minimal `src/app_state.rs`, CLI parser/help wiring, and target-resolution composition to pass the window-id no-screenshot test.
- [x] 1.3 RED: Add CLI coverage proving ambiguous title targets produce `window_error`, no arbitrary `window_context`, and exit 0 with valid JSON.
- [x] 1.4 GREEN: Map `input::ResolveError` into app-state layer-degraded diagnostics and candidate evidence.

## 2. Screenshot layer

- [x] 2.1 RED: Add fake `gdbus` CLI test proving missing window target still returns a screenshot data URL and a window error.
- [x] 2.2 GREEN: Implement app-state screenshot capture through GNOME Shell-compatible `gdbus Screenshot`, PNG dimension parsing, base64 data URL encoding, and best-effort temp cleanup.
- [x] 2.3 RED: Add focused CLI tests for `--no-screenshot` and screenshot-provider failure semantics.
- [x] 2.4 GREEN: Implement `--no-screenshot` and screenshot failure handling without failing the whole app-state report.

## 3. Accessibility layer

- [x] 3.1 RED: Add fake `python3` collector CLI test proving high-confidence AT-SPI correlation populates `accessibility_tree` and null `accessibility_error`.
- [x] 3.2 GREEN: Reuse `accessibility_tree_report_from_system` in app-state and copy tree/correlation diagnostics into the composed report.
- [x] 3.3 RED: Add fake collector tests for ambiguous/unavailable AT-SPI preserving `window_context` and screenshot data while setting `accessibility_error`.
- [x] 3.4 GREEN: Implement accessibility layer-degraded mapping for ambiguous, unavailable, and low-confidence states.

## 4. Doctor live diagnostics

- [x] 4.1 RED: Add doctor fake-command test proving live RemoteDesktop, portal Screenshot, GNOME Shell-compatible screenshot, and AT-SPI bus probe outputs are gathered into facts.
- [x] 4.2 GREEN: Extend `doctor::gather_system_facts()` with safe read-only DBus/AT-SPI probes and preserve strict RemoteDesktop parsing.
- [x] 4.3 RED/GREEN: Add or update tests proving DBus/session details and secret-like values are not serialized in doctor/app-state diagnostics.

## 5. MCP app-state tool

- [x] 5.1 RED: Update MCP `tools/list` test to expect `x11_get_app_state` in deterministic order.
- [x] 5.2 GREEN: Add `x11_get_app_state` tool definition and input schema.
- [x] 5.3 RED: Add MCP call tests for `x11_get_app_state` with `include_screenshot=false` and malformed `window_id`.
- [x] 5.4 GREEN: Implement MCP argument parsing, app-state report wrapping, malformed argument tool errors, and layer-degraded `isError=false` behavior.

## 6. Docs and evidence

- [x] 6.1 Update README command list and standalone MCP tool list for `get-app-state` / `x11_get_app_state`.
- [x] 6.2 Update `docs/integration-contract.md` with future source-overlay guidance for stock target `get_app_state` reuse.
- [x] 6.3 Record RED/GREEN evidence for every implemented TDD slice in `test-plan.md`.

## 7. Verification and archive readiness

- [x] 7.1 Run focused tests: `cargo test --test get_app_state_cli`, relevant doctor unit tests, and `cargo test --test mcp_server`.
- [x] 7.2 Run `openspec validate add-x11-get-app-state-integration --strict --no-interactive`.
- [x] 7.3 Run `make fmt`.
- [x] 7.4 Run `make check`.
- [x] 7.5 Run `make test`.
- [x] 7.6 Run live/degraded Cinnamon/X11 smoke for no-target app-state, a safe listed window target or exact degraded reason, and missing-target-with-screenshot/provider-degraded behavior.
- [x] 7.7 Confirm the project has no staged/tracked secret values, `git status --short` is expected, and the target checkout remains clean/read-only before archive.
