## TDD Strategy

Use the project-local `tdd` skill with small vertical RED -> GREEN -> REFACTOR slices. Each slice exercises the public script or runner CLI boundary (`scripts/e2e/*.sh` or `scripts/e2e/codex-x11-e2e.py validate-matrix`) rather than private helper functions. Production code for a behavior starts only after a failing Rust integration test, shell command, or script check demonstrates the missing behavior.

The e2e scripts themselves are public interfaces. Rust integration tests may use temp directories, fake `CODEX_HOME`, fake target fixtures, and fake command binaries because those are external system boundaries. Tests must not require a real GUI, sudo, real Codex Desktop app mutation, or `.secrets.local.env`.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Missing plugin install failure logs | `scripts/e2e/codex-plugin-smoke.sh --fake --codex-home <empty>` | Add `tests/e2e_harness_scripts.rs::plugin_smoke_fails_clearly_when_plugin_missing`; run `cargo test plugin_smoke_fails_clearly_when_plugin_missing -- --nocapture`; expect failure because script does not exist or does not write failure evidence/logs | Same command passes: script exits non-zero for missing plugin, stderr/evidence names missing install, and log/evidence files exist | Keep only public script assertions; no private Python helper assertions |
| 2. Fake plugin auto-install metadata | `scripts/e2e/codex-plugin-smoke.sh --fake` with isolated `CODEX_HOME` and test binary | Add `plugin_smoke_fake_auto_install_validates_marketplace_metadata`; run focused cargo test; expect failure because no auto-install/metadata validation exists | Focused test passes: fake smoke auto-installs into temp `CODEX_HOME`, validates marketplace/cache/plugin/MCP metadata, and records install/rollback matrix evidence | Refactor duplicated temp/log helpers in tests only after green |
| 3. MCP startup and tool discovery | Installed plugin `.mcp.json` -> MCP stdio `initialize`/`tools/list` | Add `plugin_smoke_fake_validates_mcp_tool_list`; run focused cargo test; expect missing MCP runner/tool assertion failure | Focused test passes and evidence contains all namespaced `x11_*` tools and no unnamespaced stock names | Keep MCP client minimal JSON-RPC; do not introduce external Python packages |
| 4. Fake X11 doctor/window/focus routing and strict portal check | Plugin smoke fake command fixture through MCP `x11_doctor`, `x11_list_windows`, `x11_focused_window`, `x11_focus_window` | Add `plugin_smoke_fake_exercises_window_routes_without_real_desktop`; run focused cargo test; expect failure because fake command fixture/tool calls are absent | Focused test passes: fake window is listed/focused, focus is verified, RemoteDesktop header-only introspection is unavailable, no real desktop is touched | Refactor fake command writer only while tests remain green |
| 5. Fake app-state, keyboard, and pointer routing | MCP `x11_get_app_state`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, `x11_drag`, `x11_accessibility_tree` | Add `plugin_smoke_fake_records_app_state_and_input_matrix`; run focused cargo test; expect failure because app-state/input smoke evidence is absent | Focused test passes: JSON evidence records pass/degraded entries, fake `xdotool` log contains keyboard/pointer calls, and portal route is not selected when RemoteDesktop methods are absent | Do not loosen focus/bounds assertions to make tests pass; fake data must drive safe route |
| 6. Source-overlay fake reversible smoke | `scripts/e2e/codex-source-overlay-smoke.sh --fake` | Add `source_overlay_smoke_fake_installs_and_uninstalls_fixture`; run focused cargo test; expect missing script/fixture failure | Focused test passes: status clean -> install -> status applied -> uninstall -> final clean, with logs/evidence | Reuse existing source-overlay fixture shapes; do not duplicate target compile logic in fake mode |
| 7. Capability matrix validator | `scripts/e2e/codex-x11-e2e.py validate-matrix --evidence <file>` | Add `matrix_validator_rejects_missing_evidence`; run focused cargo test; expect failure because validator is absent or accepts incomplete matrix | Focused test passes: incomplete evidence fails with `missing evidence`; degraded entries require non-empty reasons; complete fixture passes | Keep group/path constants single-sourced in runner |
| 8. Docs and live/degraded command smoke | `docs/e2e-harness.md`, fake scripts, optional live source-overlay smoke | Add docs grep/check assertions in `e2e_harness_docs_cover_live_manual_steps`; run focused cargo test; expect missing docs failure | Focused test passes and direct fake script runs pass; live/degraded smoke commands are recorded in Evidence Log when available | Docs must not include secrets or hard-code private values beyond documented local default path from constitution/backlog |

## Mocking / Boundary Policy

- Fake `CODEX_HOME` and fake target checkouts are allowed because user-local Codex state and target source trees are external boundaries.
- Fake `wmctrl`, `xprop`, `xdotool`, `busctl`, and optional `gdbus` are allowed through temp `PATH` fixtures because the standalone backend intentionally shells out to those commands.
- Do not mock internal Rust modules or Python helper functions; verify observable script outputs, exit codes, logs, and JSON evidence.
- Live mode must not send real keyboard/pointer input unless a later explicit safe target option exists; degraded live input evidence is acceptable for this change.
- No test may require sudo, a GUI, real `/opt/codex-desktop` mutation, network access, or secret files.

## Required Checks

Before marking apply complete:

- Focused RED/GREEN cargo tests per slice.
- `cargo test e2e_harness -- --nocapture` or the relevant integration-test filter covering all e2e harness tests.
- Direct fake script smoke:
  - `scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/manual-plugin-fake`
  - `scripts/e2e/codex-source-overlay-smoke.sh --fake --log-dir target/e2e-logs/manual-source-overlay-fake`
- `make fmt`
- `make check`
- `make test`
- `openspec validate add-codex-x11-e2e-test-harness --type change --strict`
- Live/degraded source-overlay smoke when target checkout is available and clean:
  - `scripts/e2e/codex-source-overlay-smoke.sh --live --target /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full --log-dir target/e2e-logs/live-source-overlay`
  - final `git -C /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full status --short` must be clean.

## Evidence Log

- Slice 1 RED: `cargo test plugin_smoke_fails_clearly_when_plugin_missing -- --nocapture` failed because `scripts/e2e/codex-plugin-smoke.sh` did not exist.
- Slice 1 GREEN: same command passed after adding thin wrapper, runner skeleton, missing metadata failure, and failure `evidence.json` retention.
- Slice 2 RED: `cargo test plugin_smoke_fake_auto_install_validates_marketplace_metadata -- --nocapture` failed with `fake auto-install is not implemented yet`.
- Slice 2 GREEN: same command passed after fake mode auto-installed into isolated `CODEX_HOME` and validated marketplace/cache/plugin/MCP metadata.
- Slice 3 RED: `cargo test plugin_smoke_fake_validates_mcp_tool_list -- --nocapture` failed because `mcp_tools_list` evidence was absent.
- Slice 3 GREEN: same command passed after MCP stdio `initialize`/`tools/list` validation from installed `.mcp.json`.
- Slice 4 RED: `cargo test plugin_smoke_fake_exercises_window_routes_without_real_desktop -- --nocapture` failed because `fake_window_routes` evidence was absent.
- Slice 4 GREEN: same command passed after fake `wmctrl`/`xprop`/`busctl` fixtures validated doctor/list/focused/focus and strict RemoteDesktop false-positive handling.
- Slice 5 RED: `cargo test plugin_smoke_fake_records_app_state_and_input_matrix -- --nocapture` failed because app-state/input matrix evidence was absent.
- Slice 5 GREEN: same command passed after `x11_get_app_state`, keyboard, pointer, AT-SPI, and fake `xdotool` log evidence were added.
- Slice 6 RED: `cargo test source_overlay_smoke_fake_installs_and_uninstalls_fixture -- --nocapture` failed with `source-overlay smoke is not implemented yet`.
- Slice 6 GREEN: same command passed after fake source-overlay status/install/status/uninstall/final-clean smoke was implemented.
- Slice 7 GREEN: `cargo test matrix_validator_rejects_missing_evidence -- --nocapture` passed, proving incomplete matrix fixtures fail with `missing evidence` and complete fixtures pass. The validator skeleton was introduced with the runner skeleton and then covered by this public-interface test.
- Slice 8 RED: `cargo test e2e_harness_docs_cover_live_manual_steps -- --nocapture` failed because `docs/e2e-harness.md` was missing.
- Slice 8 GREEN: same command passed after adding `docs/e2e-harness.md` and README documentation link.
- Focused all-e2e GREEN: `cargo test --test e2e_harness_scripts -- --nocapture` passed (8 tests).
- Direct fake plugin smoke GREEN: `scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/manual-plugin-fake-latest` passed; evidence: `target/e2e-logs/manual-plugin-fake-latest/standalone_plugin-fake-20260531T100417Z-2243801/evidence.json`.
- Direct fake source-overlay smoke GREEN: `scripts/e2e/codex-source-overlay-smoke.sh --fake --log-dir target/e2e-logs/manual-source-overlay-fake-latest` passed; evidence: `target/e2e-logs/manual-source-overlay-fake-latest/source_overlay-fake-20260531T100417Z-2243805/evidence.json`.
- Live source-overlay smoke GREEN: `scripts/e2e/codex-source-overlay-smoke.sh --live --target /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full --log-dir target/e2e-logs/live-source-overlay-latest` passed; evidence: `target/e2e-logs/live-source-overlay-latest/source_overlay-live-20260531T100429Z-2244189/evidence.json`; final target `git status --short` was clean.
- Final checks GREEN: `make fmt`, `make check`, `make test`, `openspec validate add-codex-x11-e2e-test-harness --type change --strict`, and `scripts/e2e/codex-x11-e2e.py validate-matrix --evidence <plugin/source/live evidence>` all passed.

## TDD Exceptions

None.
