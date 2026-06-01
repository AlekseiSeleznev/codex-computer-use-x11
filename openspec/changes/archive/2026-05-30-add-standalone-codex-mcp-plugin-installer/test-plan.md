## TDD Strategy

Apply the project-local `tdd` skill with vertical public-interface slices. Each behavior starts with one failing test/check, then the minimum production code needed for GREEN, then refactor only while the focused slice and relevant surrounding checks are GREEN. MCP tests launch the built `codex-computer-use-x11` binary in `mcp` mode and exchange JSON-RPC over stdio. Installer tests execute the public `scripts/install-codex-plugin.sh` and `scripts/uninstall-codex-plugin.sh` entry points against temporary `CODEX_HOME`/config paths and an env-provided binary. No production MCP or installer behavior should be implemented before the corresponding RED evidence is recorded here.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | `codex-computer-use-x11 mcp` initializes and `tools/list` returns deterministic `x11_*` registry | Add one integration test `mcp_server_lists_x11_tools` that starts the binary, sends `initialize`, initialized notification, and `tools/list`; run `cargo test mcp_server_lists_x11_tools`. Expected RED: unsupported `mcp` command, process exits, or missing tools. | Add CLI `mcp` arm and minimal MCP initialize/tools-list handler; same command passes and reports the four `x11_*` tools only. | Keep registry static and public-surface tested; do not add tool-call behavior until the next slice. |
| 2 | MCP `x11_doctor` tool call wraps existing doctor JSON | Add one integration test `mcp_server_calls_x11_doctor` through stdio `tools/call`; run `cargo test mcp_server_calls_x11_doctor`. Expected RED: `tools/call` unsupported or no doctor content. | Implement `tools/call` dispatch for `x11_doctor`; same command passes and first text content parses as existing doctor JSON. | Avoid duplicating doctor schema; use existing report builder/serializer. |
| 3 | MCP window/focus tool calls expose existing list/focused/focus behavior and argument errors | Add tests `mcp_server_calls_window_tools` and `mcp_focus_window_requires_window_id`; run `cargo test mcp_server_calls_window_tools mcp_focus_window_requires_window_id`. Expected RED: calls unsupported or missing argument does not return tool error. | Implement `x11_list_windows`, `x11_focused_window`, and `x11_focus_window`; same commands pass, missing `window_id` is `isError:true`, and valid focus calls contain existing focus JSON semantics. | Keep protocol errors separate from tool/business errors; preserve existing focus safety. |
| 4 | Installer dry-run prints planned owned changes and writes nothing | Add one script integration test `plugin_installer_dry_run_writes_nothing`; run `cargo test plugin_installer_dry_run_writes_nothing`. Expected RED: install script missing or creates files. | Add `scripts/install-codex-plugin.sh --dry-run` with no writes/builds; same command passes. | Dry-run must stay strict and should not create `CODEX_HOME`, config, cache, or marketplace paths. |
| 5 | Installer creates owned plugin bundle, marketplace, and config in temp `CODEX_HOME` | Add test `plugin_installer_creates_owned_bundle_and_config`; run `cargo test plugin_installer_creates_owned_bundle_and_config`. Expected RED: files/config missing. | Implement non-dry install using env-provided binary, generated manifests, cache `latest`, marketplace JSON/link, and owned config sections; same command passes. | Keep generated JSON/TOML deterministic except timestamps; do not print full config. |
| 6 | Repeated install is idempotent | Add test `plugin_installer_repeated_install_is_idempotent`; run `cargo test plugin_installer_repeated_install_is_idempotent`. Expected RED: duplicate config sections, broken latest link, or changed unrelated content. | Replace owned files/sections atomically enough for tests; same command passes after two installs. | Preserve unrelated config and avoid accumulating duplicate sections. |
| 7 | Uninstall removes only owned files/sections and supports dry-run/absent state | Add tests `plugin_uninstaller_removes_only_owned_files` and `plugin_uninstaller_dry_run_and_absent_are_safe`; run `cargo test plugin_uninstaller`. Expected RED: uninstall script missing, removes unrelated sentinel, or dry-run writes. | Implement uninstall with owned-path and owned-section removal only; same commands pass. | Do not recursively remove parent marketplace/cache roots unless empty and owned-safe. |
| 8 | README and final live/direct smoke evidence | Add or update docs check manually, then run `openspec validate add-standalone-codex-mcp-plugin-installer --type change --strict`, `make fmt`, `make check`, `make test`, direct MCP smoke, installer `--dry-run`, live install, and rollback/inspection as applicable. Expected RED only if previous slices missed integration or documentation issues. | All required checks pass; live install writes only owned `$CODEX_HOME` paths and either `x11_*` tools are visible/callable after refresh or exact restart/inspection instructions are recorded. | No refactor if it changes public tool names, manifest layout, or config sections without spec/design update. |

## Mocking / Boundary Policy

- MCP tests use the public binary over stdio; they do not call private Rust functions.
- Installer tests execute public scripts and isolate filesystem effects with temporary `CODEX_HOME`, `CODEX_CONFIG_FILE`, and `CODEX_X11_PLUGIN_BINARY`.
- Fake or temporary filesystem state is acceptable for Codex cache/marketplace/config boundaries; do not write real HOME during automated tests.
- Live user-local install is supplementary and explicitly requested by the user; it must be followed by owned-path inspection and rollback instructions/evidence.
- Do not mock internal Rust modules controlled by this crate; system boundaries are stdio, filesystem, config file, and optional live Codex plugin refresh.

## Required Checks

- `openspec validate add-standalone-codex-mcp-plugin-installer --type change --strict` before apply and before archive.
- Per-slice RED and GREEN `cargo test <test-name>` commands.
- Final Rust checks: `make fmt`, `make check`, `make test`.
- Direct MCP smoke: start `target/debug/codex-computer-use-x11 mcp` or `cargo run -- mcp`, initialize, list tools, and call `x11_doctor`.
- Installer script smoke: `scripts/install-codex-plugin.sh --dry-run` from the repo root.
- Live install/rollback evidence when allowed: `scripts/install-codex-plugin.sh`, owned file/config inspection, Codex refresh/restart/tool visibility instructions or evidence, and `scripts/uninstall-codex-plugin.sh` available for rollback.
- Verify `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` remains unmodified after read-only research.
- Verify git status contains only expected change/archive files before checkpoint/archive/push.

## Evidence Log

- Apply preflight (2026-05-30): `git status --short` was clean; `openspec validate add-standalone-codex-mcp-plugin-installer --type change --strict` passed; `openspec instructions apply --change add-standalone-codex-mcp-plugin-installer --json` returned `state: ready` with 14 pending tasks.
- Slice 1 RED (2026-05-30): `cargo test mcp_server_lists_x11_tools` failed after adding `tests/mcp_server.rs` because `codex-computer-use-x11 mcp` was unsupported and the child process closed stdout unexpectedly.
- Slice 1 GREEN (2026-05-30): `cargo test mcp_server_lists_x11_tools` passed after adding `src/mcp.rs`, exposing `pub mod mcp`, updating usage, and adding the CLI `mcp` arm with initialize and `tools/list` support.
- Slice 2 RED (2026-05-30): `cargo test mcp_server_calls_x11_doctor` failed because `tools/call` returned JSON-RPC `unsupported method: tools/call`.
- Slice 2 GREEN (2026-05-30): `cargo test mcp_server_calls_x11_doctor` passed after adding `tools/call` dispatch for `x11_doctor`, MCP tool-result content, `structuredContent`, and `isError:false` around the existing doctor report.
- Slice 3 RED (2026-05-30): `cargo test mcp_server_calls_window_tools` failed because `x11_list_windows` returned `isError:true` from unsupported-tool handling; `cargo test mcp_focus_window_requires_window_id` failed because the error was `unsupported tool: x11_focus_window` instead of a `window_id` argument error.
- Slice 3 GREEN (2026-05-30): `cargo test mcp_server_calls_window_tools` and `cargo test mcp_focus_window_requires_window_id` passed after adding MCP dispatch for `x11_list_windows`, `x11_focused_window`, `x11_focus_window`, shared id parsing, and tool-error handling for missing `window_id` and non-success focus reports.
- Slice 4 RED (2026-05-30): `cargo test plugin_installer_dry_run_writes_nothing` failed because `scripts/install-codex-plugin.sh` did not exist.
- Slice 4 GREEN (2026-05-30): `cargo test plugin_installer_dry_run_writes_nothing` passed after adding `scripts/install-codex-plugin.sh --dry-run`, which prints owned cache/marketplace/config paths and creates no `CODEX_HOME`.
- Slice 5 RED (2026-05-30): `cargo test plugin_installer_creates_owned_bundle_and_config` failed because non-dry install exited with `non-dry install is not implemented yet`.
- Slice 5 GREEN (2026-05-30): `cargo test plugin_installer_creates_owned_bundle_and_config` passed after implementing user-local install with generated `.codex-plugin/plugin.json`, `.mcp.json`, copied executable, cache `latest` symlink, owned marketplace JSON/link, and owned Codex config sections while preserving unrelated config.
- Slice 6 RED (2026-05-30): `cargo test plugin_installer_repeated_install_is_idempotent` failed after simulating a stale non-symlink `latest` path because the installer did not replace it with a symlink.
- Slice 6 GREEN (2026-05-30): `cargo test plugin_installer_repeated_install_is_idempotent` passed after making install remove stale non-symlink `latest` and marketplace-plugin paths before recreating symlinks; repeated install also preserves unrelated config and keeps one owned plugin/marketplace section.
- Slice 7 RED (2026-05-30): `cargo test plugin_uninstaller` failed because `scripts/uninstall-codex-plugin.sh` did not exist.
- Slice 7 GREEN (2026-05-30): `cargo test plugin_uninstaller` passed after adding `scripts/uninstall-codex-plugin.sh` with owned cache/marketplace removal, owned config-section removal, dry-run no-write behavior, and absent-plugin success.
- Documentation (2026-05-30): `README.md` was updated with `cargo run -- mcp`, installer dry-run/install, Codex refresh/restart guidance, direct MCP smoke, and uninstall/rollback instructions without secret values.
- Final format/check/test (2026-05-30): initial `make fmt` failed on rustfmt diffs; after `cargo fmt`, `make fmt`, `make check`, and `make test` passed. `make test` covered 40 lib tests, 2 doctor CLI tests, 8 focus CLI tests, 3 list-windows CLI tests, 4 MCP server tests, 5 plugin installer tests, and doc tests.
- OpenSpec validation (2026-05-30): `openspec validate add-standalone-codex-mcp-plugin-installer --type change --strict` passed before and after implementation; rerun after final task/evidence updates is required before archive.
- Direct MCP smoke (2026-05-30): a Python JSON-RPC smoke against `target/debug/codex-computer-use-x11 mcp` initialized the server, listed `x11_doctor`, `x11_list_windows`, `x11_focused_window`, and `x11_focus_window`, and called `x11_doctor`, receiving JSON with `project=codex-computer-use-x11` and `backend=x11-ewmh`.
- Installer dry-run smoke (2026-05-30): `scripts/install-codex-plugin.sh --dry-run` exited 0 and printed only owned `codex-computer-use-x11` cache, marketplace, link, and config paths under `$CODEX_HOME`; no files were written by dry-run.
- Live user-local install (2026-05-30): `scripts/install-codex-plugin.sh` built release binary and installed without sudo to owned `~/.codex/plugins/cache/codex-computer-use-x11/.../0.1.0`, owned marketplace `~/.codex/plugins/marketplaces/codex-computer-use-x11/.agents/plugins/marketplace.json`, and owned config sections. Inspection confirmed executable binary, plugin/mcp/marketplace names, and config plugin/marketplace sections. `codex plugin list` reported `codex-computer-use-x11@codex-computer-use-x11` as `installed, enabled` version `0.1.0`. Rollback remains `scripts/uninstall-codex-plugin.sh`.
- Target checkout guard (2026-05-30): `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` remained clean after read-only research and live install; `git status --short` printed no changes.

## TDD Exceptions

None.
