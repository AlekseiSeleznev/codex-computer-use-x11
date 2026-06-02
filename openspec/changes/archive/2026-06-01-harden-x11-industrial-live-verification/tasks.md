## 1. Screenshot crop output correctness

- [x] 1.1 Add a RED CLI/fake-provider test proving `screenshot-crop --output relative/path --json` resolves the output path against cwd before the provider call and reports the resolved absolute path.
- [x] 1.2 Implement cwd-relative output path resolution and invalid/unavailable output parent preflight without changing ADR 0008 crop rectangle semantics.
- [x] 1.3 Add a RED fake-provider test where `gdbus ScreenshotArea` returns false and creates no file; assert `success=false`, structured `error_code`, provider detail diagnostics, and no captured-file success claim.
- [x] 1.4 Implement provider status parsing and missing-output failure handling for screenshot crop.
- [x] 1.5 Add RED tests for empty output, unreadable output when feasible, and non-PNG output; assert distinct structured failures.
- [x] 1.6 Implement output integrity verification for readable non-empty PNG signature before `success=true`.
- [x] 1.7 Add/keep a GREEN positive test where a fake provider writes a valid PNG and screenshot crop succeeds with output metadata and no inline image data.
- [x] 1.8 Refactor screenshot output preflight/postflight helpers while all `tests/screenshot_coordinate_cli.rs` tests remain green.

## 2. Controlled live fixture manager

- [x] 2.1 Add RED harness tests or fake-live fixtures proving the runner can create run-scoped Tk and GTK fixture metadata, readiness files, process ids, and cleanup records under the selected log directory.
- [x] 2.2 Commit deterministic fixture scripts under `scripts/e2e/` or `scripts/e2e/fixtures/` for Tk text/pointer and GTK AT-SPI, using unique run-scoped titles/classes and no secret access.
- [x] 2.3 Implement fixture lifecycle management with timeouts, readiness probes, `try/finally` cleanup, and shell `trap` coverage from `codex-plugin-smoke.sh` when needed.
- [x] 2.4 Add RED tests for cleanup on fixture startup failure and tool-call failure, then implement cleanup of fixture processes, overlay state, and target-window state.

## 3. Safe fixture target selection

- [x] 3.1 Add RED tests for missing fixture, duplicate fixture, stale fixture, overlay/helper candidate, and real user application candidate in live window listings.
- [x] 3.2 Implement fixture allowlist resolution requiring exactly one controlled target per fixture role before keyboard, pointer, screenshot, app-state, target, or overlay tool calls.
- [x] 3.3 Implement `missing_fixture_setup` and `unsafe_target_selection` evidence reasons when selection is not safe.
- [x] 3.4 Verify no input/pointer/screenshot/app-state tool call is made when fixture selection is missing, ambiguous, stale, or unsafe.

## 4. Fixture-backed live capability checks

- [x] 4.1 Add RED fake-live tests for Tk-backed focus, ASCII/Cyrillic typing, Backspace, Enter, click, scroll, drag, target context, and release rows.
- [x] 4.2 Implement Tk fixture-backed live MCP calls for `x11_focus_window`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, `x11_drag`, `x11_target_window`, `x11_target_context`, and `x11_release_window`.
- [x] 4.3 Add RED fake-live tests for GTK bridge env metadata and expected accessible node evidence.
- [x] 4.4 Implement GTK fixture launch with `GTK_MODULES=gail:atk-bridge` and `NO_AT_BRIDGE=0`, `x11_accessibility_tree` evidence, and Tk no-match as fixture-specific degraded evidence only.
- [x] 4.5 Add RED fake-live tests for fixture-scoped screenshot crop and `x11_get_app_state` evidence.
- [x] 4.6 Implement screenshot/app-state fixture checks that store screenshots as files/paths and sanitize app-state screenshot data URLs from ordinary evidence/logs.
- [x] 4.7 Add RED fake-live tests for overlay enabled lifecycle and overlay helper exclusion.
- [x] 4.8 Implement optional `CODEX_X11_ENABLE_TK_OVERLAY=1` overlay check, release/hide verification, and explicit overlay degraded evidence when provider is unavailable.

## 5. Industrial evidence schema and matrix validation

- [x] 5.1 Add RED validator fixtures for canonical statuses `pass`, `degraded`, and `fail` plus reason categories `fixture_pass`, `environment_limitation`, `missing_fixture_setup`, `code_failure`, `unsafe_target_selection`, `malformed_evidence`, and `not_evaluated`.
- [x] 5.2 Implement schema/version handling for industrial evidence while preserving compatibility with existing fake and metadata-only evidence under default validation.
- [x] 5.3 Add and implement an explicit industrial validation profile, such as `validate-matrix --industrial`, that fails on missing fixture setup, code failure, unsafe target selection, malformed evidence, missing rows, and unevaluated required fixture-backed rows.
- [x] 5.4 Ensure environment limitations are accepted as degraded only when fixture orchestration was attempted and concrete dependency/display/toolkit evidence is present.
- [x] 5.5 Update `scripts/e2e/codex-plugin-smoke.sh --live` CLI/help and Python runner output so freshness smoke and industrial acceptance semantics are unambiguous.

## 6. Documentation and release evidence guidance

- [x] 6.1 Add RED docs tests or grep checks expecting industrial live verification commands, fixture safety language, matrix reason categories, and screenshot-by-path guidance.
- [x] 6.2 Update `docs/e2e-harness.md` to document fake mode, metadata live smoke, industrial live fixture mode, controlled fixtures, evidence schema, and safe target rules.
- [x] 6.3 Update `docs/troubleshooting.md` with screenshot output integrity failures, fixture dependency degradation, GTK bridge requirements, and no-user-app targeting guidance.
- [x] 6.4 Update `docs/release-checklist.md` and any final DoD guidance to require industrial validation before production readiness claims while preserving fake evidence requirements.
- [x] 6.5 Ensure docs and evidence examples use variable names only, do not mention secret values, and do not embed huge screenshot data URLs.

## 7. Verification and cleanup

- [x] 7.1 Run the targeted RED/GREEN commands from `test-plan.md` for screenshot, harness, matrix, app-state, AT-SPI, target-window, input, and pointer behavior; record evidence paths or outputs in `test-plan.md` Evidence Log during apply.
- [x] 7.2 Run deterministic fake plugin smoke and `validate-matrix` default profile to prove backward compatibility.
- [x] 7.3 Run industrial matrix validation against committed fixture evidence and at least one generated fake-live/industrial evidence file.
- [x] 7.4 When a safe Cinnamon/X11 desktop is available, run live industrial plugin smoke against controlled fixtures only; record pass/degraded/fail evidence under `target/e2e-logs/<run-id>/` and confirm cleanup.
- [x] 7.5 Run `make fmt`, `make check`, `make test`, and `openspec validate --all --strict` before marking apply complete.
- [x] 7.6 Confirm `git status --short` is clean or contains only intentional tracked planning/evidence updates; do not stage or commit `.secrets.local.env` or uncontrolled screenshots/log payloads.
