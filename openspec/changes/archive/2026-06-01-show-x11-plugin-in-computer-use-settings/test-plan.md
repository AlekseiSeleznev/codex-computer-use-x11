## TDD Strategy

Use the project-local `tdd` discipline with vertical slices against the public patcher/test interfaces in the target checkout. Each slice starts with one focused Node test or shell smoke assertion that fails against the current target code, then the minimal patcher/descriptor code is added until the same command passes. Internal minified-string helpers may be pure functions, but tests should call exported patcher functions or existing smoke scripts rather than poking private closures.

The implementation changes are in `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`; OpenSpec artifacts and evidence log live in this repository. No secrets or external services are needed.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| T1 | Target patcher exports and applies an X11 settings row patch for current minified Computer Use settings shape. | Add one Node test in `scripts/patch-linux-window-ui.test.js`, then run `node --test scripts/patch-linux-window-ui.test.js --test-name-pattern "X11 Computer Use settings"`; expected RED: missing export/function or no `codex-computer-use-x11` row injection. | Same command passes after adding `applyX11ComputerUseSettingsRowPatch` and exporting it. | Keep matching limited to the settings row construction; do not change existing Computer Use gate behavior. |
| T2 | Row patch is idempotent and preserves bundled `Any App` and Chrome row markers. | Add idempotence assertions to the same focused test command; expected RED: duplicate injection or missing preservation checks. | Same focused command passes after the patch returns unchanged when marker exists and preserves existing row literals. | Avoid brittle exact byte equality except after apply-twice; assert behavior markers. |
| T3 | Row patch warns and skips on drifted/unexpected settings page shape. | Add warning/unchanged test and run the same focused Node command; expected RED: no warning helper behavior or function changes drifted source. | Same focused command passes with a single clear warning and unchanged source. | Warning text should be specific enough for maintainers but not block unrelated builds. |
| T4 | Target patch registry applies the row patch to `computer-use-settings-*.js` only when Computer Use UI is enabled. | Add descriptor/smoke assertion and run `node --test scripts/patch-linux-window-ui.test.js --test-name-pattern "Computer Use UI"` or `tests/scripts_smoke.sh` focused smoke if available; expected RED: descriptor not registered. | Command passes after updating `scripts/patches/core/all-linux/webview/computer-use-ui/patch.js`. | Keep patch id/order near other Computer Use UI webview patches. |
| T5 | Full target patcher verification remains green. | No separate RED expected; this is the surrounding regression check after T1-T4. | `node --test scripts/patch-linux-window-ui.test.js` passes. Run `tests/scripts_smoke.sh` or a focused subset if runtime permits; otherwise record exact blocker. | Do not broaden implementation if unrelated target tests fail for pre-existing reasons. |
| T6 | OpenSpec and live/degraded UI verification are recorded. | No separate RED expected; final evidence gate. | `openspec validate show-x11-plugin-in-computer-use-settings --type change --strict` passes. Live UI check is attempted if safe; otherwise exact blocker is recorded. | Keep final evidence tied to automated patcher proof when live app cache/restart blocks visual proof. |

## Mocking / Boundary Policy

- Use minified webview fixture strings and the target patcher public export as the boundary for webview behavior.
- Use `captureWarns`/existing test helpers for warning assertions.
- Do not mock the standalone plugin installer; the row patch should operate on plugin id lookup only.
- Do not mutate `$HOME/.codex/plugins/cache/openai-bundled/computer-use` or bundled marketplace paths.
- Live UI verification may inspect the current app or patched extracted assets, but must not require storing secrets or destructive app state.

## Required Checks

- Focused RED/GREEN Node tests for T1-T4 in `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`.
- Full target patcher test: `node --test scripts/patch-linux-window-ui.test.js`.
- Target smoke check: `tests/scripts_smoke.sh` or focused subset when feasible; record blockers if too slow/unavailable.
- OpenSpec validation: `openspec validate show-x11-plugin-in-computer-use-settings --type change --strict`.
- Git status checks for both repositories.
- No Rust `make fmt/check/test` required unless standalone Rust code changes.

## Evidence Log

- 2026-05-31 T1 RED: `node --test scripts/patch-linux-window-ui.test.js --test-name-pattern "X11 Computer Use settings"` in `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` failed because `applyX11ComputerUseSettingsRowPatch` was not exported/implemented.
- 2026-05-31 T1/T2/T3 GREEN: added `applyX11ComputerUseSettingsRowPatch`, exported it from `scripts/patches/computer-use.js` and `scripts/patch-linux-window-ui.js`, and verified row injection, bundled row preservation, idempotence, and drift warning tests.
- 2026-05-31 T4 RED/GREEN: descriptor list test first failed because `linux-x11-computer-use-settings-row` was absent from the expected core descriptor set; after registering the descriptor in target `scripts/patches/core/all-linux/webview/computer-use-ui/patch.js` and updating the expectation, the focused Node command passed.
- 2026-05-31 syntax verification: `node --check scripts/patch-linux-window-ui.js && node --check scripts/patch-linux-window-ui.test.js && node --check scripts/patches/computer-use.js && node --check scripts/patches/core/all-linux/webview/computer-use-ui/patch.js` passed.
- 2026-05-31 T5 full target patcher verification: `node --test scripts/patch-linux-window-ui.test.js` passed with 155/155 tests.
- 2026-05-31 T5 target smoke verification: `tests/scripts_smoke.sh` passed with "All script smoke tests passed".
- 2026-05-31 T6 extracted-asset verification: copied `/tmp/codex-asar-extract`, ran `CODEX_LINUX_ENABLE_COMPUTER_USE_UI=1 node scripts/patch-linux-window-ui.js --report-json <tmp>/patch-report.json <tmp>`, and confirmed `computer-use-settings-Bj9s3CiH.js` contains `codex-computer-use-x11`, `X11 Computer Use`, and `Standalone X11/EWMH desktop control tools`; report row `linux-x11-computer-use-settings-row` status was `applied`.
- 2026-05-31 OpenSpec validation: `openspec validate show-x11-plugin-in-computer-use-settings --type change --strict` passed.
- 2026-05-31 target implementation checkpoint: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` committed `3e7b13e` (`Add X11 Computer Use settings row patch`) and target `git status --short` was clean afterward.
- 2026-05-31 live UI feedback: installed `/opt/codex-desktop/content/webview/assets/computer-use-settings-aHZZtKP_.js` used a newer memoized `S.push(...)` shape, and the first implementation also gated X11 on bundled `r.available`. Added a current-shape regression test and changed the row insertion so standalone `codex-computer-use-x11` is appended when that plugin is present, independent of bundled Computer Use availability.
- 2026-05-31 follow-up verification: `node --check scripts/patches/computer-use.js`, `node --check scripts/patch-linux-window-ui.test.js`, `node --test scripts/patch-linux-window-ui.test.js --test-name-pattern "X11 Computer Use settings|memoized settings|default core patch descriptors"`, and `tests/scripts_smoke.sh` passed in the target repo. A dry run against the installed asset showed `hasX11=true`, `hasPlugin=true`, `hasStandaloneGate=true`, and `gatedOnBundledAvailability=false`.
- 2026-05-31 local installed-app patch: backed up `/opt/codex-desktop/content/webview/assets/computer-use-settings-aHZZtKP_.js` to `/opt/codex-desktop/content/webview/assets/computer-use-settings-aHZZtKP_.js.bak-20260531223804` and installed the patched asset with X11 markers; a Codex Desktop restart/reload is required for the running webview to load it.


- 2026-05-31 second live UI feedback investigation: compared the installed `computer-use-settings-aHZZtKP_.js` behavior with the target checkout and found that the page's existing detail flow uses `installedPlugins`, while the first X11 row patch only searched `availablePlugins`; `codex plugin list` showed `codex-computer-use-x11@codex-computer-use-x11` installed and enabled, so the row could remain absent even though the plugin was installed.
- 2026-05-31 installed-first source fix: target `applyX11ComputerUseSettingsRowPatch` now resolves `codexLinuxX11Plugin` from `d.installedPlugins ?? []` first and falls back to `d.availablePlugins`; the patch also migrates older available-only patched assets before treating the marker as idempotent.
- 2026-05-31 installed-first regression verification: `node --check scripts/patches/computer-use.js`, `node --check scripts/patch-linux-window-ui.test.js`, `node --test scripts/patch-linux-window-ui.test.js --test-name-pattern "X11 Computer Use settings|memoized settings|migrates older X11|default core patch descriptors"` passed with 157/157 tests, including the new stale-patch migration test.
- 2026-05-31 full target smoke verification after installed-first fix: `tests/scripts_smoke.sh` passed with "All script smoke tests passed" in `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`.
- 2026-05-31 local installed-app migration: backed up `/opt/codex-desktop/content/webview/assets/computer-use-settings-aHZZtKP_.js` to `/opt/codex-desktop/content/webview/assets/computer-use-settings-aHZZtKP_.js.bak-20260531225952`, migrated the live asset to installed-first lookup, and verified `hasX11=true`, `hasPlugin=true`, `hasInstalledLookup=true`, `hasAvailableOnly=false`. The running desktop still needs a full process restart to clear already-loaded Electron/app-server state.
- 2026-05-31 target implementation checkpoint: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` committed `3dd399f` (`Resolve X11 Computer Use installed plugin lookup`).

## TDD Exceptions

None.
