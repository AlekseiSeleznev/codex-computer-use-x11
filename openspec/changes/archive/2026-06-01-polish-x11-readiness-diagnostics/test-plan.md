## TDD Strategy

Future apply MUST use vertical RED -> GREEN -> REFACTOR slices through public interfaces: `codex-computer-use-x11 doctor --json`, MCP/CLI tool evidence, `scripts/e2e/codex-plugin-smoke.sh`, matrix validation commands, screenshot output files, and documentation/retest commands. Tests should cover parser/model boundaries with fake commands and fixtures before relying on live desktop evidence.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | Doctor readiness taxonomy and JSON compatibility | Add fixture/unit tests for blockers/degraded/optional/unsupported categories; expect missing fields or wrong `readiness.ok` aggregation to fail. | `cargo test doctor_readiness` or equivalent passes; `codex-computer-use-x11 doctor --json` emits valid JSON preserving existing fields. | No removed/renamed bootstrap fields; categories are additive and documented. |
| 2 | Doctor/AT-SPI diagnostic states and redaction | Add tests for bus unavailable, bus reachable/tree unavailable, no-match, ambiguous, fixture pass, and private path redaction; expect current output to collapse states or leak paths. | AT-SPI diagnostic tests pass and live `doctor --json` uses stable codes without private socket/runtime paths. | Shared outcome constants/types avoid string drift across doctor/app-state/e2e. |
| 3 | Matrix reason categories | Add validator fixture JSON files for environment limitation, missing fixture setup, code failure, unsupported out-of-scope, and expected fake-fixture limitation; expect missing/incorrect categories to fail. | `scripts/e2e/validate-matrix ...` accepts valid fixtures and rejects missing/incorrect category fixtures, including industrial mode. | Validator errors are actionable and mention row/path/category. |
| 4 | Metadata-only live smoke safety classification | Add fake/live-safe fixture evidence where no controlled fixture exists; expect rows to be misclassified before implementation. | Metadata-only smoke reports `missing_fixture_setup` and warns against real-app input while preserving environment diagnostics. | No code path sends input/pointer/overlay without unique controlled target proof. |
| 5 | Fake screenshot semantics and real crop integrity | Add tests for fake provider pass and/or missing fake provider degraded limitation plus provider-success-without-output failure. | Fake smoke screenshot row either passes with fixture output or degrades with expected limitation; missing real output remains failure. | Screenshot validation stays path-based and does not embed image bytes in normal summaries. |
| 6 | Controlled live fixture uniqueness and cleanup | Add fake-live/controlled fixture tests for unique target selection, ambiguous target rejection, overlay hide, target release, fixture process stop, and stale target cleanup. | Fake-live industrial smoke and controlled live smoke record fixture ids and cleanup results; ambiguous/missing fixtures block unsafe actions. | Cleanup is trap-safe and evidence is emitted on success and failure. |
| 7 | Documentation and full retest guidance | Add docs/readme checks or grep-based tests for PASS/DEGRADED/FAIL, Wayland out of scope, safe retest, and evidence requirements; expect missing text before docs update. | Documentation checks pass and manual review confirms instructions do not require secrets or unsafe real-app input. | Docs avoid stale command names and broken illustrative links. |

## Mocking / Boundary Policy

- Mock external desktop commands, DBus outputs, AT-SPI trees, screenshot providers, and Codex home/plugin installation through fake `PATH`, fake evidence JSON, and controlled fixtures.
- Do not mock internal parser/model logic under test; feed it fixture command outputs instead.
- Live tests may use only controlled fixtures with unique titles/classes/process markers for input, pointer, screenshot, target, app-state, AT-SPI, and overlay evidence.
- No test reads `.secrets.local.env` or requires external credentials.

## Required Checks

Planning checks:

- `openspec validate polish-x11-readiness-diagnostics --type change --strict`
- `openspec validate --all --strict`

Future apply/verify checks:

- `make fmt`
- `make check`
- `make test`
- `codex-computer-use-x11 doctor --json` parsed as valid JSON with expected readiness fields
- Fake plugin smoke with matrix validation
- Fake-live industrial fixture smoke with matrix validation
- Controlled live fixture smoke when Cinnamon/X11 display is available; otherwise report exact environment blocker/limitation
- Documentation/release checklist checks for safe full retest guidance
- `git status --short` with no staged/tracked secrets

## Evidence Log

- Planning evidence to fill before this change is considered planning-complete: OpenSpec strict validation outputs and checkpoint commit hashes.
- Apply evidence will be filled during future `/opsx:apply` runs with RED/GREEN command outputs or paths to durable logs.


### 2026-06-01 — Doctor readiness and AT-SPI diagnostics slice

- RED: `cargo test doctor_readiness` initially failed because `Readiness` lacked additive X11 taxonomy fields and `AccessibilityReport` lacked canonical diagnostic-state fields (see `/tmp/polish-doctor-red.log`).
- GREEN: `cargo test doctor_readiness` passed after adding additive readiness taxonomy fields.
- GREEN: `cargo test doctor_accessibility_reports_canonical_diagnostic_states` passed after adding AT-SPI diagnostic states for bus unavailable, tree extraction unavailable, no matching app subtree, ambiguous match, and controlled fixture pass.
- GREEN: `cargo test doctor` passed (33 lib doctor tests, 4 doctor CLI tests, 2 MCP doctor tests).
- GREEN: `cargo run --quiet -- doctor --json` was parsed by `python3 -m json.tool` and checked for additive readiness fields; output reported valid JSON and no private path values were introduced.


### 2026-06-01 — Evidence matrix, fake screenshot, app-state, and controlled fixture safety slices

- RED: `cargo test matrix_validator_requires_canonical_reason_categories_for_non_pass_rows` failed because non-pass matrix rows did not require `reason_category`.
- GREEN: Added canonical reason-category validation and `not_evaluated` defaults; `cargo test matrix_validator` passed.
- Decision for task 3.1: keep fake screenshot as explicit `expected_fake_fixture_limitation` when fake `gdbus`/screenshot fixture is unavailable; do not add a fake screenshot provider in this apply slice.
- GREEN: `scripts/e2e/codex-plugin-smoke.sh --fake` produced screenshot row `degraded` with `reason_category=expected_fake_fixture_limitation`; `scripts/e2e/codex-x11-e2e.py validate-matrix` accepted the evidence.
- RED/GREEN: `cargo test plugin_smoke_live_metadata_only_records_missing_fixture_setup` now proves metadata-only live smoke classifies fixture-dependent rows as `missing_fixture_setup` and warns that it is not safe to test input against real user applications.
- RED/GREEN: `cargo test app_state_summary_requires_diagnostics_layers_and_sanitizes_screenshot` now proves degraded app-state layer summaries carry reason categories and still omit screenshot data URLs.
- GREEN: `cargo test plugin_smoke_live_industrial_fake_fixtures_records_fixture_backed_rows` proves fixture uniqueness, fixture-backed pass rows, overlay hide/release, target context cleanup, and process cleanup evidence.
- GREEN: `cargo test --test e2e_harness_scripts` passed (19 tests).


### 2026-06-01 — Production readiness documentation slice

- RED: `cargo test production_readiness_docs_define_x11_pass_degraded_fail_and_safe_retest` initially failed while the README/troubleshooting/release checklist lacked complete PASS/DEGRADED/FAIL, X11-only scope, safe retest, controlled-fixture, and no-inline-screenshot guidance.
- GREEN: `cargo test production_readiness_docs_define_x11_pass_degraded_fail_and_safe_retest` passed after updating `/home/as/ai-projects/codex-computer-use-x11/README.md`, `/home/as/ai-projects/codex-computer-use-x11/docs/troubleshooting.md`, and `/home/as/ai-projects/codex-computer-use-x11/docs/release-checklist.md`.
- GREEN: `cargo test --test packaging_docs` passed (8 tests), confirming the production readiness docs and existing packaging/release docs checks.


### 2026-06-01 — Final verification checkpoint

- GREEN: `make fmt`, `make check`, and `make test` passed. `make test` covered 47 library tests plus all integration suites, including `tests/e2e_harness_scripts.rs` (19 tests) and `tests/packaging_docs.rs` (8 tests).
- GREEN: `cargo run --quiet -- doctor --json` emitted valid machine-readable JSON parsed by `python3 -m json.tool`; readiness included additive fields `required_baseline`, `blockers_detailed`, `degraded_acceptable`, `optional_enrichments`, and `unsupported_out_of_scope`.
- GREEN: fake plugin smoke produced evidence at `target/e2e-logs/standalone_plugin-fake-20260601T151050Z-672474/evidence.json`; `scripts/e2e/codex-x11-e2e.py validate-matrix --evidence ...` passed.
- GREEN: deterministic fake-live industrial fixture smoke produced evidence at `target/e2e-logs/standalone_plugin-live-20260601T151051Z-672658/evidence.json`; `scripts/e2e/codex-x11-e2e.py validate-matrix --industrial --evidence ...` passed.
- LIMITATION RECORDED: real live industrial smoke without `--fake-live-fixtures` produced `target/e2e-logs/standalone_plugin-live-20260601T151115Z-673170/evidence.json`; industrial validation intentionally rejected it because fixture-dependent rows were `reason_category=missing_fixture_setup`. This records the current environment/tooling limitation: no explicit safe live controlled fixture was configured, so the harness refused real-user-app fallback.
- GREEN: supplemental controlled-fixture helpers passed: `fixture-self-test` at `target/e2e-logs/fixture-self-test-20260601T151140Z` and `selection-self-test` scenarios `ok`, `missing`, `duplicate`, `stale`, `overlay-helper`, and `user-app` under `target/e2e-logs/selection-self-test-20260601T151141Z-*`.
- GREEN: `openspec validate polish-x11-readiness-diagnostics --type change --strict` passed.
- GREEN: `openspec validate --all --strict` passed with 19 passed, 0 failed.
- GREEN: `git status --short` was clean before marking verification tasks complete; no unrelated or secret files were present.

## TDD Exceptions

None.

