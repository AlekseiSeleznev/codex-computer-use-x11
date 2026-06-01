## 1. Test-first e2e harness scaffolding

- [x] 1.1 Add failing public-interface integration test for missing standalone plugin install diagnostics and failure log/evidence retention.
- [x] 1.2 Add minimal `scripts/e2e/codex-plugin-smoke.sh` wrapper and shared runner skeleton until the missing-install test passes.
- [x] 1.3 Add failing public-interface integration test for fake plugin auto-install and marketplace/cache metadata validation.
- [x] 1.4 Implement fake plugin auto-install, metadata validation, and evidence/log writing until the metadata test passes.
- [x] 1.5 Add failing public-interface integration test for capability matrix missing-evidence validation.
- [x] 1.6 Implement `validate-matrix --evidence <file>` and shared matrix constants until missing/degraded matrix tests pass.

## 2. Standalone plugin MCP and fake X11/input smoke

- [x] 2.1 Add failing public-interface integration test for MCP startup from installed `.mcp.json` and deterministic namespaced tool discovery.
- [x] 2.2 Implement MCP stdio initialize/tools-list smoke and unnamespaced-tool rejection until the tool-discovery test passes.
- [x] 2.3 Add failing public-interface integration test for fake X11 doctor/list/focused/focus routing and strict RemoteDesktop header-only false-positive handling.
- [x] 2.4 Implement fake X11 command fixture and doctor/window/focus MCP calls until the fake window-route test passes.
- [x] 2.5 Add failing public-interface integration test for `x11_get_app_state`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, `x11_drag`, and `x11_accessibility_tree` fake smoke evidence.
- [x] 2.6 Implement app-state, keyboard, pointer, and AT-SPI pass/degraded matrix evidence with fake `xdotool` log assertions until the input/app-state test passes.

## 3. Source-overlay fake/live smoke

- [x] 3.1 Add failing public-interface integration test for `scripts/e2e/codex-source-overlay-smoke.sh --fake` reversible status/install/uninstall against a fixture target.
- [x] 3.2 Add source-overlay wrapper and implement fake target fixture smoke until final fake status is clean and evidence/logs pass.
- [x] 3.3 Implement live source-overlay target resolution, stock target tool vocabulary inspection, uninstall-on-failure, optional target cargo checks, and final clean target checks.
- [x] 3.4 Run live/degraded source-overlay smoke against the configured target when available and record exact evidence in `test-plan.md`.

## 4. Documentation and user-facing smoke guidance

- [x] 4.1 Add failing docs/check test that requires e2e fake/live usage, evidence file shape, capability matrix groups, and manual Codex Desktop stock-tool fallback steps.
- [x] 4.2 Add `docs/e2e-harness.md` and any README/integration-contract links needed until docs checks pass.
- [x] 4.3 Ensure docs and logs avoid secret values and use only variable names such as `CODEX_HOME` and `CODEX_DESKTOP_LINUX_FULL_PATH`.

## 5. Verification, evidence, and archive readiness

- [x] 5.1 Run focused e2e harness tests and record RED/GREEN evidence in `test-plan.md`.
- [x] 5.2 Run direct fake smoke scripts for standalone plugin and source overlay; record evidence paths and outcomes in `test-plan.md`.
- [x] 5.3 Run `make fmt`, `make check`, `make test`, and `openspec validate add-codex-x11-e2e-test-harness --type change --strict`; fix issues before marking complete.
- [x] 5.4 Verify real target checkout final status is clean and no generated e2e logs or local temp secret/config files are staged.
- [x] 5.5 Update `tasks.md` and `test-plan.md` with final evidence summary, then run `/opsx:verify` equivalent before archive.
