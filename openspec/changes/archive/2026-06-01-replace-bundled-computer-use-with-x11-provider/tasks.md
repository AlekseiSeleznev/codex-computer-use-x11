## 1. Target baseline and diagnostic TDD slices

- [x] 1.1 Add RED target fixture test for bundled Computer Use baseline diagnostics in `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/scripts/patch-linux-window-ui.test.js` and record the expected failure for T1.
- [x] 1.2 Implement the minimal target provider diagnostic/resolver boundary needed for T1 so bundled `computer-use` row decisions explain shown, hidden, gated, unavailable, and absent states.
- [x] 1.3 Add RED target fixture test for sanitized runtime payload diagnostics (`installedPlugins`, `availablePlugins`, `computerUseAvailability`, row decisions, marker version) and record the expected failure for T2.
- [x] 1.4 Implement diagnostic serialization/report support with diagnostics off by default and no secret/private-value output.
- [x] 1.5 Run and record focused GREEN target commands for T1-T2.

## 2. Target X11 takeover provider behavior

- [x] 2.1 Add RED target fixture test proving `--provider x11 --mode takeover` hides or disables the bundled `computer-use` Any App row while preserving Google Chrome row behavior.
- [x] 2.2 Implement takeover resolver behavior for bundled row suppression without changing bundled marketplace/cache identity.
- [x] 2.3 Add RED target fixture tests proving X11 takeover selects `codex-computer-use-x11` from `installedPlugins` first, falls back to `availablePlugins`, and reports unavailable when absent instead of silently falling back.
- [x] 2.4 Implement X11 provider row selection and any row-local compatibility shim while preserving plugin id `codex-computer-use-x11`, marketplace `codex-computer-use-x11`, and `x11_*` MCP tools.
- [x] 2.5 Add/verify migration behavior for prior side-by-side `codexLinuxX11Plugin` markers so takeover uses a distinct marker and does not treat old static injection as takeover success.
- [x] 2.6 Run and record focused GREEN target commands for T3-T4.

## 3. Target patch registration and smoke coverage

- [x] 3.1 Register takeover diagnostic/provider patch descriptors under the target Computer Use UI patch set, gated by the existing opt-in Computer Use UI posture unless tests justify a narrower explicit takeover gate.
- [x] 3.2 Add descriptor/smoke assertions proving takeover descriptors apply only to the intended `computer-use-settings-*.js` assets and fail soft on upstream shape drift.
- [x] 3.3 Run target full patcher verification: `node --test scripts/patch-linux-window-ui.test.js`.
- [x] 3.4 Run target smoke verification (`tests/scripts_smoke.sh` or a focused feasible subset) and record exact blocker if unavailable.

## 4. Current repo installer/status/rollback implementation

- [x] 4.1 Add RED current-repo overlay-manager test for `--provider x11 --mode takeover` install/report behavior in `tests/source_overlay_scripts.rs` and record expected failure for T5.
- [x] 4.2 Extend `scripts/codex-source-overlay.py` and shell wrappers to accept provider/mode options, dry-run/status/report paths, strict unsupported-provider/mode refusal, and target path resolution.
- [x] 4.3 Implement takeover source-overlay apply/status reporting with target path, target commit when available, marker version, changed files, drift classification, and restart hint.
- [x] 4.4 Add RED current-repo overlay-manager test for live-asset backup metadata, restore, no-op absent rollback, and drift refusal; record expected failure for T6.
- [x] 4.5 Implement explicit live-asset patching option with timestamped backups, patch report entries, checksum or size metadata, and restart hint.
- [x] 4.6 Implement rollback/restore that removes only owned takeover source markers, restores recorded live backups when safe, no-ops when takeover is absent, and does not remove standalone plugin cache files.
- [x] 4.7 Run and record GREEN current-repo installer tests for T5-T6.
- [x] 4.8 Fix and cover fresh-target `--dry-run --patch-live-assets` so it uses the overlay patcher without mutating the target before install.

## 5. Integrated verification and evidence

- [x] 5.1 Run current repo `make fmt`, `make check`, and `make test` after installer changes, or record exact blocker.
- [x] 5.2 Run `openspec validate replace-bundled-computer-use-with-x11-provider --type change --strict` and record output.
- [x] 5.3 Apply the takeover overlay to the real target checkout only after fixture tests are GREEN; record target report path, changed files, and target git status.
- [x] 5.4 Attempt extracted-asset or live-asset verification with backups/report when explicitly enabled; record whether the running Codex Desktop process requires full restart.
- [x] 5.5 Verify rollback against target source and any explicitly patched live assets; record bundled-mode restoration evidence.
- [x] 5.6 Update `test-plan.md` Evidence Log with T1-T7 RED/GREEN commands, report paths, live/degraded UI findings, and rollback evidence.
- [x] 5.7 Check final git status in `/home/as/ai-projects/codex-computer-use-x11` and `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`; checkpoint implementation/planning changes locally but do not archive, push, merge, or PR without explicit approval.
- [x] 5.8 Record fresh `/home/as/Документы/AI_PROJECTS/codex-desktop-linux` reinstall, provider takeover, rebuild, and user UI confirmation evidence.
