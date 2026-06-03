## TDD Strategy

Use the project-local `tdd` skill with vertical RED -> GREEN -> REFACTOR slices. Each behavior-changing task starts from an observable public interface: CLI JSON, MCP tool JSON, E2E harness command output/evidence JSON, or docs tests. Do not batch all tests before all code. Record RED/GREEN evidence in this file during apply before marking tasks complete.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | CLI `get-app-state --json` default screenshot output contains no inline blob and references a PNG path | Add one `tests/get_app_state_cli.rs` test with fake `gdbus` provider; run `cargo test --test get_app_state_cli app_state_default_screenshot_is_path_only -- --nocapture`; expect failure because current JSON has `screenshot.data_url` and no path | Implement path-oriented `ScreenshotCapture`; same command passes and verifies no `data:image`/`;base64,` plus file exists/non-empty PNG | Keep test through CLI only; extract output path verification helpers only after GREEN |
| 2 | CLI `--screenshot-output <path>` and invalid path degrade only screenshot layer | Add one test for caller path success and one for invalid parent; run targeted `cargo test --test get_app_state_cli app_state_screenshot_output_path`; expect unsupported flag or wrong success semantics | Parse `--screenshot-output`, preflight/resolve path, populate `screenshot_error` without dropping window/accessibility diagnostics; targeted tests pass | Reuse path preflight patterns; no screenshot-crop behavior change |
| 3 | `--no-screenshot` remains supported and no-inline | Add/extend a regression test for `get-app-state --no-screenshot --json`; run targeted cargo test; expect current pass or adjust only if schema changes | Ensure new struct/serialization keeps no-screenshot output stable; targeted test passes | No refactor unless compatibility needs it |
| 4 | MCP `x11_get_app_state` default and `screenshot_output` are no-inline | Add `tests/mcp_server.rs` test calling `x11_get_app_state` with fake screenshot provider args; run targeted cargo test; expect current output contains data_url or lacks argument support | Add MCP arguments and safe default serialization; targeted MCP test passes | Keep tool name and existing arguments stable |
| 5 | E2E harness rejects/sanitizes raw app-state inline blobs and records screenshot paths | Add/extend `tests/e2e_harness_scripts.rs` for app-state summary/raw evidence with screenshot path and no `data_url`; run targeted cargo test; expect fixture summary assumptions fail until harness updated | Update summarizer/fake-live calls to pass screenshot output path and assert raw/summarized evidence has no inline blob | Preserve existing fake/fake-live industrial validation behavior |
| 6 | Controlled real-live fixture runner uses neutral fixture identity and metadata | Add/extend fixture self-test and selection tests expecting titles/classes without `Codex` and metadata with bridge facts; run targeted cargo test or `python3 scripts/e2e/codex-x11-e2e.py fixture-selection-self-test`; expect current `codex`/`Codex` identity fails | Rename/rework runner identity and metadata; targeted tests pass | Keep fake fixtures deterministic and real-live cleanup reliable |
| 7 | Docs explain safe app-state evidence and real-live fixture retests | Add docs grep tests in `tests/packaging_docs.rs`; run targeted cargo test; expect missing wording/options | Update `docs/e2e-harness.md`, `docs/troubleshooting.md`, `docs/release-checklist.md`; targeted docs tests pass | Keep docs secret-free and scoped to Cinnamon/X11 |

## Mocking / Boundary Policy

- Use fake `PATH` command fixtures for `wmctrl`, `xprop`, `gdbus`, and MCP process boundaries, consistent with existing tests.
- Do not mock internal Rust collaborators. Exercise public CLI/MCP or harness entrypoints.
- Fake screenshot providers may write minimal PNG files and return provider failure strings; tests assert observable JSON and file effects.
- Real-live evidence remains optional during automated apply unless a safe Cinnamon/X11 desktop is available; fake/fake-live tests must not be mislabeled as primary real-live evidence.

## Required Checks

- `openspec validate --all --strict`
- `make fmt`
- `make check`
- `make test`
- Targeted CLI tests for get-app-state screenshot JSON safety.
- Targeted MCP tests for `x11_get_app_state` screenshot output path/no-inline behavior.
- Targeted harness/docs tests for real-live controlled fixture runner metadata, neutral titles/classes, and no inline app-state evidence.
- Existing screenshot-crop regression tests proving path-only behavior remains unchanged.
- If safe real Cinnamon/X11 desktop is available during verify: run controlled real-live fixture smoke and industrial matrix validation; otherwise report limitation and rely on deterministic fake/fake-live coverage until manual retest.

## Evidence Log

- RED: `cargo test --test get_app_state_cli keeps_screenshot_when_window_target_missing -- --nocapture` failed because default JSON still contained `screenshot.data_url` with a `data:image/png;base64,...` payload.
- GREEN: `cargo test --test get_app_state_cli -- --nocapture` passed after path-oriented app-state screenshot serialization, `--screenshot-output`, invalid path degradation, `--no-screenshot`, and explicit `--inline-screenshot` coverage.
- GREEN: `cargo test --test mcp_server -- --nocapture` passed, including `x11_get_app_state` no-screenshot compatibility and `screenshot_output` path/no-inline MCP behavior.
- GREEN: `cargo test --test e2e_harness_scripts -- --nocapture` passed, including app-state summary sanitization/path metadata, neutral controlled fixture identity, fake smoke, and fake-live industrial fixture evidence.
- GREEN: `cargo test --test packaging_docs -- --nocapture` passed for path-only app-state docs, real-live fixture retest docs, and `NO_AT_BRIDGE=1` remediation guidance.
- GREEN: `cargo test --test screenshot_coordinate_cli -- --nocapture` passed, proving screenshot-crop path-only behavior and output integrity remained intact.
- GREEN: `make fmt`, `make check`, `make test`, and `openspec validate --all --strict` passed.
- GREEN: `scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/fix-app-state-screenshot-evidence-safety-verify-20260601T165405Z/plugin-fake` passed, and `scripts/e2e/codex-x11-e2e.py validate-matrix --evidence .../evidence.json` passed.
- GREEN: `scripts/e2e/codex-plugin-smoke.sh --live --industrial --fake-live-fixtures --log-dir target/e2e-logs/fix-app-state-screenshot-evidence-safety-verify-20260601T165405Z/plugin-live-industrial-fake` passed, and `validate-matrix --industrial` passed.
- CHECK: recursive scan of the generated verification run directory found no `data:image` or `;base64,` screenshot payload markers.
- LIMITATION: real controlled Cinnamon/X11 retest against the newly built plugin was not run during apply because this change was not installed into Codex in this workflow; deterministic fake/fake-live industrial verification was run instead. The prior REAL LIVE retest evidence remains the motivating defect sample.

## TDD Exceptions

None.
