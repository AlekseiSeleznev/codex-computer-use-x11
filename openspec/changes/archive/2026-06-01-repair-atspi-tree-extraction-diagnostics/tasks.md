## 1. Doctor bridge-disabled diagnostics

- [x] 1.1 Add RED doctor tests for bus reachable, tree unavailable, `NO_AT_BRIDGE=1` producing `atspi_gtk_bridge_disabled_by_environment` instead of generic tree-unavailable.
- [x] 1.2 Add RED doctor tests for bus reachable, tree unavailable, `NO_AT_BRIDGE` absent preserving non-bridge `atspi_tree_extraction_unavailable`.
- [x] 1.3 Implement sanitized bridge-env facts and diagnostic-state priority in `src/doctor.rs`.
- [x] 1.4 Implement bridge-specific recommendation and `readiness.recommended_next_step` wording.
- [x] 1.5 Validate `doctor --json` emits valid JSON and does not serialize unrelated/private environment values.

## 2. Controlled GTK fixture environment

- [x] 2.1 Add RED e2e harness tests proving parent `NO_AT_BRIDGE=1` is not inherited by the GTK fixture subprocess.
- [x] 2.2 Implement fixture child-env construction that removes `NO_AT_BRIDGE` for GTK fixtures and sets/records `GTK_MODULES=gail:atk-bridge` only for the fixture process when needed.
- [x] 2.3 Update GTK fixture metadata to record `NO_AT_BRIDGE` absent instead of defaulting to `0`.
- [x] 2.4 Update existing fake-live evidence/tests that currently assert `NO_AT_BRIDGE=0`.

## 3. E2E evidence and validator behavior

- [x] 3.1 Add fake evidence tests for `atspi_gtk_bridge_disabled_by_environment` with `reason_category=environment_limitation`.
- [x] 3.2 Ensure matrix validation accepts bridge-disabled degraded AT-SPI evidence while still rejecting malformed/missing reason categories.
- [x] 3.3 Ensure missing controlled GTK fixture setup remains `missing_fixture_setup` or precise environment limitation, never real-window fallback.
- [x] 3.4 Run targeted e2e harness tests for fixture self-test, fake-live industrial smoke, and validator behavior.

## 4. Troubleshooting and retest docs

- [x] 4.1 Add “AT-SPI bus reachable but tree extraction unavailable” documentation covering package, gsettings, process, and `NO_AT_BRIDGE` checks.
- [x] 4.2 Replace docs that recommend `NO_AT_BRIDGE=0` with guidance to remove/unset `NO_AT_BRIDGE` for GTK fixture/application processes.
- [x] 4.3 Document restart/remediation and controlled GTK fixture verification without changing global user environment from the harness.
- [x] 4.4 Add/adjust docs tests so stale `NO_AT_BRIDGE=0` enablement guidance fails.

## 5. Verification

- [x] 5.1 Run `make fmt`.
- [x] 5.2 Run `make check`.
- [x] 5.3 Run `make test`.
- [x] 5.4 Run `cargo run --quiet -- doctor --json | python3 -m json.tool` and targeted JSON assertions for bridge-disabled diagnostics.
- [x] 5.5 Run fake smoke and matrix validation.
- [x] 5.6 Run fake-live industrial fixture smoke and matrix validation, or record exact controlled-fixture blocker.
- [x] 5.7 Run `openspec validate repair-atspi-tree-extraction-diagnostics --type change --strict` and `openspec validate --all --strict`.
- [x] 5.8 Confirm `git status --short` contains only intended files and no secrets before requesting any future commit/archive permission.
