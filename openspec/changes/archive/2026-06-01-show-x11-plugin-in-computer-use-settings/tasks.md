## 1. Target patcher TDD slices

- [x] 1.1 Add RED Node test in target `scripts/patch-linux-window-ui.test.js` proving a Computer Use settings fixture receives an `X11 Computer Use` row backed by `codex-computer-use-x11`.
- [x] 1.2 Implement and export `applyX11ComputerUseSettingsRowPatch` in target patcher code with a minimal row lookup/injection for the current minified settings shape.
- [x] 1.3 Add/verify idempotence and bundled-row preservation assertions for the row patch.
- [x] 1.4 Add/verify drift warning test proving unexpected settings page shape is left unchanged with a clear warning.

## 2. Target patch registration and smoke coverage

- [x] 2.1 Register the row patch in target `scripts/patches/core/all-linux/webview/computer-use-ui/patch.js` for `computer-use-settings-*.js`, gated by `enableComputerUseUi`.
- [x] 2.2 Add a focused descriptor/smoke assertion that the row patch is part of the Computer Use UI patch set.
- [x] 2.3 Run focused GREEN checks for the new patcher tests and descriptor/smoke assertions.

## 3. Verification evidence and OpenSpec completion

- [x] 3.1 Run full target patcher verification: `node --test scripts/patch-linux-window-ui.test.js`.
- [x] 3.2 Run target smoke verification (`tests/scripts_smoke.sh` or focused feasible subset) or record exact blocker.
- [x] 3.3 Attempt live or extracted-asset UI readiness verification and record visible/degraded evidence for `X11 Computer Use`.
- [x] 3.4 Update `test-plan.md` Evidence Log with T1-T6 RED/GREEN and verification evidence.
- [x] 3.5 Run `openspec validate show-x11-plugin-in-computer-use-settings --type change --strict`.
- [x] 3.6 Check git status in both repositories and checkpoint implementation/planning changes; do not archive or push without explicit approval.
- [x] 3.7 Follow up on live UI feedback: support the installed memoized settings bundle shape and remove the standalone X11 row's accidental dependency on bundled `computerUseAvailability.available`.
- [x] 3.8 Follow up on second live UI feedback: compare target behavior with installed/available plugin sources, switch the X11 row lookup to installed-first (`installedPlugins` with `availablePlugins` fallback), migrate stale available-only patched assets, and verify the installed app asset.
