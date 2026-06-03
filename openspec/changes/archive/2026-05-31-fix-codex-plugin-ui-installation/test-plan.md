## TDD Strategy

Use the project-local `tdd` skill with vertical public-interface slices. Each production change is preceded by one failing test or smoke check that exercises installer output, MCP protocol output, or e2e smoke evidence through existing public commands/scripts. Refactor only after GREEN.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | `scripts/install-codex-plugin.sh` generates Codex UI metadata with `AlekseiSeleznev`, GitHub website, no privacy/terms links, marketplace display name, and installed logo asset. | `cargo test --test plugin_installer plugin_installer_creates_owned_bundle_and_config` fails on missing metadata/icon assertions. | Same command passes after installer and asset update. | Manifest generation remains centralized in installer; no unrelated config sections changed. |
| 2 | Installed MCP binary and fake plugin smoke validate all fourteen current `x11_*` tools and reject stale six-tool installs. | `cargo test --test e2e_harness_scripts` or a focused plugin smoke metadata/tool-surface test fails for stale/missing tool validation. | Focused e2e harness test and `scripts/e2e/codex-plugin-smoke.sh --fake` pass. | Keep fake mode hermetic; no real desktop mutation. |
| 3 | `codex-computer-use-x11 mcp` hydrates missing desktop env from deterministic local fixture sources before `x11_doctor`, preserving explicit env. | Focused `cargo test --test mcp_server mcp_server_hydrates_desktop_env_for_doctor` fails because `DISPLAY` remains missing. | Focused MCP hydration tests pass with fixture env and preserve explicit `DISPLAY`. | Hydration allowlist only; no stdout noise; no secret values serialized. |
| 4 | Full installer/MCP/e2e regression remains valid after integration. | `make test` may fail while slices are incomplete. | `make fmt`, `make check`, `make test`, `openspec validate fix-codex-plugin-ui-installation --strict`, and fake plugin smoke pass or exact live-environment blocker is reported. | Remove duplication and keep helpers small after all checks are GREEN. |

## Mocking / Boundary Policy

Mock only external desktop/session boundaries through fake commands, temporary `CODEX_HOME`, fixture files, and deterministic environment variables. Do not mock internal Rust report builders or JSON-RPC handlers. Do not read `.secrets.local.env`.

## Required Checks

- Focused RED/GREEN commands from each slice.
- `make fmt`.
- `make check`.
- `make test`.
- `openspec validate fix-codex-plugin-ui-installation --strict`.
- `scripts/e2e/codex-plugin-smoke.sh --fake`.
- `scripts/e2e/codex-plugin-smoke.sh --live` when the live environment can be safely exercised; otherwise report the exact blocker.
- `git status --short` before checkpoint and final report.

## Evidence Log

- Slice 1 RED: `cargo test --test plugin_installer plugin_installer_creates_owned_bundle_and_config` failed because `assets/app-icon.png` was not copied.
- Slice 1 GREEN: same focused test passed after adding project-owned icon, installer copy, and corrected manifest/marketplace metadata.
- Slice 2 RED metadata: `cargo test --test e2e_harness_scripts plugin_smoke_fake_auto_install_validates_marketplace_metadata -- --nocapture` failed because e2e evidence did not return `display_name` metadata.
- Slice 2 RED stale install: `cargo test --test e2e_harness_scripts plugin_smoke_rejects_stale_six_tool_install -- --nocapture` failed because stale six-tool failure did not identify missing current tools.
- Slice 2 GREEN focused: both focused e2e harness tests passed after metadata validation returned UI fields and tool validation reported missing expected MCP tools.
- Slice 2 GREEN surrounding: `cargo test --test e2e_harness_scripts plugin_smoke -- --nocapture` passed all six plugin-smoke harness tests.
- Slice 2 GREEN smoke: `scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/slice2-fake` passed.
- Slice 3 RED missing DISPLAY: `cargo test --test mcp_server mcp_server_hydrates_desktop_env_for_doctor -- --nocapture` failed because `environment.display_present` remained false when MCP started without `DISPLAY`.
- Slice 3 RED preserve explicit env: `cargo test --test mcp_server mcp_server_preserves_explicit_display_during_hydration -- --nocapture` failed after adding fixture session assertions because fixture session vars were not hydrated.
- Slice 3 GREEN focused: both MCP hydration tests passed after adding MCP startup desktop-env hydration with an allowlisted fixture/systemd/proc source chain.
- Slice 3 GREEN surrounding: `cargo test --test mcp_server -- --nocapture` passed all MCP protocol tests, and `cargo test --test list_windows_cli -- --nocapture` confirmed CLI no-display behavior remains degraded instead of hydrated.
- Slice 3 stdout/secret check: hydration tests parsed JSON-RPC stdout successfully and asserted fixture `XAUTHORITY`/DBus secret-like values were not serialized in doctor text.
- Slice 4 GREEN project checks: `make fmt && make check && make test && openspec validate fix-codex-plugin-ui-installation --strict` passed.
- Slice 4 GREEN standalone smokes: `scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/final-fake` and `scripts/e2e/codex-plugin-smoke.sh --live --log-dir target/e2e-logs/final-live` passed; live smoke validated metadata and tool discovery.
- Slice 4 user-local refresh: `scripts/uninstall-codex-plugin.sh` then `scripts/install-codex-plugin.sh` reinstalled `codex-computer-use-x11 0.1.0` under `/home/as/.codex/plugins/cache/codex-computer-use-x11/...` and refreshed `/home/as/.codex/config.toml`.
- Slice 4 installed verification: `scripts/e2e/codex-plugin-smoke.sh --fake --codex-home "$HOME/.codex" --no-auto-install --log-dir target/e2e-logs/user-local-refresh` passed, and direct JSON-RPC `tools/list` against the installed `latest` binary returned 14 tools including `x11_get_app_state`, pointer, accessibility, and target-context tools.
- Slice 4 in-process Codex note: the current Codex thread still exposes the stale six-tool `mcp__codex_computer_use_x11` schema and `x11_doctor` result from a pre-refresh server; Codex needs plugin reload/restart/new thread to pick up the refreshed installed bundle.

## TDD Exceptions

None
