## 1. Apply preflight and MCP registry slices

- [x] 1.1 Re-read project context and apply instructions: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, this change's artifacts, and `openspec instructions apply --change add-standalone-codex-mcp-plugin-installer --json`; run `openspec validate add-standalone-codex-mcp-plugin-installer --type change --strict` before production edits.
- [x] 1.2 RED/GREEN slice 1: add and satisfy the public MCP stdio integration test proving `initialize` plus `tools/list` returns deterministic `x11_doctor`, `x11_list_windows`, `x11_focused_window`, and `x11_focus_window`; record RED/GREEN evidence in `test-plan.md`.
- [x] 1.3 RED/GREEN slice 2: add and satisfy the public MCP `x11_doctor` `tools/call` test that wraps existing doctor JSON; record RED/GREEN evidence in `test-plan.md`.
- [x] 1.4 RED/GREEN slice 3: add and satisfy public MCP window/focus tool-call tests for `x11_list_windows`, `x11_focused_window`, `x11_focus_window`, and missing `window_id` errors; record RED/GREEN evidence in `test-plan.md`.

## 2. Installer and uninstaller TDD slices

- [x] 2.1 RED/GREEN slice 4: add and satisfy the public script test proving `scripts/install-codex-plugin.sh --dry-run` exits successfully and writes nothing under temp `CODEX_HOME`; record evidence in `test-plan.md`.
- [x] 2.2 RED/GREEN slice 5: add and satisfy the temp-`CODEX_HOME` script test proving install creates owned cache, `.codex-plugin/plugin.json`, `.mcp.json`, executable copy, marketplace metadata/link, and owned Codex config sections; record evidence in `test-plan.md`.
- [x] 2.3 RED/GREEN slice 6: add and satisfy the repeated-install idempotence test proving owned config sections are not duplicated and unrelated content remains; record evidence in `test-plan.md`.
- [x] 2.4 RED/GREEN slice 7: add and satisfy uninstall tests proving owned-only removal, dry-run no-write behavior, and absent-plugin safety; record evidence in `test-plan.md`.

## 3. Documentation, smoke, and safety verification

- [x] 3.1 Update `README.md` with `mcp`, install dry-run/install, Codex refresh/restart, direct MCP smoke, and uninstall/rollback instructions without printing or embedding secret values.
- [x] 3.2 Run required project checks: `make fmt`, `make check`, `make test`, and `openspec validate add-standalone-codex-mcp-plugin-installer --type change --strict`; record final evidence in `test-plan.md`.
- [x] 3.3 Run direct MCP stdio smoke against the built binary, proving `tools/list` exposes `x11_*` tools and `x11_doctor` returns JSON; record evidence in `test-plan.md`.
- [x] 3.4 Run installer `--dry-run` smoke from the repo root and verify it reports only owned paths; record evidence in `test-plan.md`.
- [x] 3.5 Run live user-local install per user request, inspect only owned cache/marketplace/config state, verify or document Codex refresh/restart/tool visibility instructions, and keep `scripts/uninstall-codex-plugin.sh` available for rollback; record evidence in `test-plan.md`.
- [x] 3.6 Verify `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` remains clean/read-only, update this task list to checked only after evidence exists, and leave git status ready for verify/archive.
