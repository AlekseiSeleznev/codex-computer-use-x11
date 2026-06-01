## TDD Strategy

Future apply MUST use vertical RED -> GREEN -> REFACTOR slices through public behavior: `codex-computer-use-x11 doctor --json`, e2e fixture metadata, fake/live evidence JSON, matrix validation, and docs checks. The first RED tests should assert the currently wrong behavior (`NO_AT_BRIDGE=0` fixture evidence and generic `atspi_tree_extraction_unavailable`) before production code/docs are changed.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | Doctor bridge-disabled diagnostic state | Add/adjust Rust doctor tests so bus=true, tree=false, env `NO_AT_BRIDGE=1` expects `atspi_gtk_bridge_disabled_by_environment`; current code should fail with generic `atspi_tree_extraction_unavailable`. | `cargo test doctor_accessibility` or targeted doctor test passes; `doctor --json` preserves `atspi_bus_available=true`, `tree_available=false`, `reason_category=environment_limitation`. | New state shares constants/helpers where practical and does not remove bootstrap fields. |
| 2 | Doctor recommendation and sanitized env facts | Add tests expecting recommendation to mention removing/uninheriting `NO_AT_BRIDGE`, restarting affected session/process, and controlled GTK fixture verification; expect no arbitrary env dump. | Targeted doctor tests pass and live/fake `doctor --json` parses as JSON with sanitized bridge facts only. | No private runtime/socket/env values are serialized beyond allowed bridge keys. |
| 3 | Controlled GTK fixture child env | Add e2e harness tests with parent `NO_AT_BRIDGE=1`; expect GTK fixture metadata to show `NO_AT_BRIDGE` absent, causing current `NO_AT_BRIDGE=0` assertion to fail. | `cargo test --test e2e_harness_scripts` targeted fixture tests pass. | Environment helper is narrow, child-process only, and leaves parent/global env unchanged. |
| 4 | Fake/validator bridge-disabled evidence | Add fake evidence/validator tests for AT-SPI degraded row with `atspi_gtk_bridge_disabled_by_environment` and `environment_limitation`; ensure missing live fixture is not `code_failure` by default. | Matrix validator accepts the bridge-disabled degraded fixture and still rejects malformed/missing categories where required. | Validator error messages remain actionable and reason-category taxonomy stays stable. |
| 5 | Troubleshooting and retest docs | Add docs tests/grep assertions for the new section, package/gsettings/process/`NO_AT_BRIDGE` checks, controlled GTK fixture verification, and absence of `NO_AT_BRIDGE=0` as recommended enablement. | `cargo test --test packaging_docs` or targeted docs test passes. | Docs stay X11-only, no secrets, no real-window fallback, and commands remain safe. |
| 6 | Final verification | Run full verification after TDD slices. | Existing checks may fail before implementation. | `make fmt`, `make check`, `make test`, `cargo run --quiet -- doctor --json | python3 -m json.tool`, fake smoke/matrix validation, fake-live industrial validation where appropriate, `openspec validate repair-atspi-tree-extraction-diagnostics --type change --strict`, and `openspec validate --all --strict` pass or blockers are recorded. | No unrelated dirty files or local secret files are staged/tracked. |

## Mocking / Boundary Policy

- Use Rust unit tests and fake `ProbeFacts`/fake environment maps for doctor behavior.
- Use fake e2e evidence and fixture self-tests for bridge-disabled/bridge-enabled behavior.
- Live validation may use only controlled GTK/Tk fixtures with unique title/class/process metadata.
- Do not inspect, screenshot, input into, or correlate AT-SPI trees from real user windows as fallback.
- Do not read `.secrets.local.env` or require external credentials.

## Required Checks

Planning checks:

- `openspec validate repair-atspi-tree-extraction-diagnostics --type change --strict`
- `openspec validate --all --strict`

Future apply/verify checks:

- `make fmt`
- `make check`
- `make test`
- `cargo run --quiet -- doctor --json | python3 -m json.tool`
- Targeted doctor JSON assertions for `NO_AT_BRIDGE=1` bridge-disabled diagnosis
- Fake plugin smoke and matrix validation
- Fake-live industrial fixture smoke and matrix validation when controlled fixture code is available
- Controlled live GTK fixture evidence on Cinnamon/X11 when safe; otherwise record exact `missing_fixture_setup` or `environment_limitation`
- OpenSpec strict validation
- `git status --short` confirming no unrelated or secret files before any future checkpoint

## Evidence Log

Planning evidence:

- `openspec validate repair-atspi-tree-extraction-diagnostics --type change --strict` passed after proposal/spec/grill/design creation.

Apply evidence:

- Filled below with RED/GREEN/REFACTOR command outputs and evidence paths.


### 2026-06-01 — Doctor bridge-disabled diagnostics slice

- RED: `cargo test doctor_accessibility_reports_bridge_disabled_environment --lib` failed because bus-reachable/tree-unavailable still produced `atspi_tree_extraction_unavailable`.
- GREEN: `cargo test doctor_accessibility --lib` passed after adding sanitized `accessibility.bridge_env`, `atspi_gtk_bridge_disabled_by_environment`, and bridge-specific recommendations/next-step text.
- GREEN: The non-bridge case stays distinct: `doctor_accessibility_keeps_non_bridge_tree_unavailable_when_env_absent` passed and preserves `atspi_tree_extraction_unavailable`.

### 2026-06-01 — Controlled GTK fixture bridge env and e2e evidence slice

- RED: `cargo test controlled_fixture_manager_creates_metadata_and_cleanup_records --test e2e_harness_scripts` failed when parent `NO_AT_BRIDGE=1` was inherited into the GTK fixture metadata.
- GREEN: The e2e harness now builds a child fixture environment that removes `NO_AT_BRIDGE` for GTK and records `NO_AT_BRIDGE=null`, `NO_AT_BRIDGE_PRESENT=false`, and `GTK_MODULES=gail:atk-bridge`; targeted fixture and fake-live industrial tests passed.
- GREEN: `cargo test industrial_matrix_validator_rejects_missing_fixture_setup_and_code_failure --test e2e_harness_scripts` passed with bridge-disabled AT-SPI degraded evidence classified as `environment_limitation`.

### 2026-06-01 — Troubleshooting docs slice

- RED: `cargo test troubleshooting_docs_explain_atspi_bridge_disabled_remediation --test packaging_docs` failed because the troubleshooting guide lacked the bus-reachable/tree-unavailable section and still documented stale `NO_AT_BRIDGE=0` enablement.
- GREEN: Added the dedicated AT-SPI troubleshooting section and updated fixture guidance to unset/remove `NO_AT_BRIDGE`; targeted packaging docs tests passed.


### 2026-06-01 — Verification evidence

- GREEN: `make fmt` passed.
- GREEN: `make check` passed.
- GREEN: `make test` passed: 49 library tests plus all integration suites, including 19 e2e harness tests and 9 packaging docs tests.
- GREEN: `NO_AT_BRIDGE=1 GTK_MODULES=gail:atk-bridge cargo run --quiet -- doctor --json` parsed with `python3 -m json.tool` and asserted `atspi_bus_available=true`, `tree_available=false`, `diagnostic_state=atspi_gtk_bridge_disabled_by_environment`, `reason_category=environment_limitation`, and sanitized bridge env.
- GREEN: fake plugin smoke evidence `target/e2e-logs/repair-atspi-20260601T155828Z/fake/standalone_plugin-fake-20260601T155828Z-754735/evidence.json` passed `scripts/e2e/codex-x11-e2e.py validate-matrix`.
- GREEN: fake-live industrial fixture smoke evidence `target/e2e-logs/repair-atspi-20260601T155828Z/fake-live/standalone_plugin-live-20260601T155828Z-755105/evidence.json` passed `scripts/e2e/codex-x11-e2e.py validate-matrix --industrial` and recorded GTK fixture env with `NO_AT_BRIDGE=null` and `NO_AT_BRIDGE_PRESENT=false`.
- GREEN: `openspec validate repair-atspi-tree-extraction-diagnostics --type change --strict` passed.
- GREEN: `openspec validate --all --strict` passed with 19 passed, 0 failed.
- GREEN: `git status --short` before checkpoint contained only intended implementation/docs/OpenSpec files and no local secret files.

## TDD Exceptions

None.
