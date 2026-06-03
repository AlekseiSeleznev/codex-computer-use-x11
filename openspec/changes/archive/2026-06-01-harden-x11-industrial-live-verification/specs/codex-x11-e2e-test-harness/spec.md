## ADDED Requirements

### Requirement: Live plugin smoke orchestrates controlled fixtures
Live standalone plugin smoke MUST create or select controlled fixture windows for fixture-dependent capabilities instead of marking those capabilities degraded solely because no fixture was orchestrated. The harness MUST use unique fixture titles/classes, readiness probes, timeouts, and cleanup traps for Tk keyboard/pointer/focus/target/release, GTK AT-SPI, optional overlay, screenshot crop, and app-state checks.

#### Scenario: Live smoke starts and cleans controlled fixtures
- **GIVEN** a developer runs `scripts/e2e/codex-plugin-smoke.sh --live`
- **WHEN** fixture-backed checks are enabled for the current desktop session
- **THEN** the harness starts fixture windows with unique `codex-x11-*` titles or classes
- **AND** it waits for each fixture readiness signal before tool calls
- **AND** it records fixture process ids and window ids in the run evidence
- **AND** it tears down all fixture processes and overlay state on success or failure

#### Scenario: Missing fixture setup is not an accepted pass
- **GIVEN** live smoke cannot start a required fixture because a dependency or display capability is unavailable
- **WHEN** capability matrix validation evaluates the evidence
- **THEN** the affected capability is not reported as `pass`
- **AND** the reason identifies `missing_fixture_setup` or a more specific dependency cause
- **AND** the validator distinguishes that reason from expected environment degradation and code failure

### Requirement: Live plugin smoke verifies fixture-backed capability rows
Live standalone plugin smoke MUST exercise fixture-backed tool calls for keyboard input, pointer input, window listing/focus, target context/release, screenshot, `get_app_state`, GTK AT-SPI, and optional overlay lifecycle. Each exercised capability row MUST include the concrete fixture id, tool call, evidence path, status, and reason.

#### Scenario: Tk fixture backs keyboard and pointer rows
- **GIVEN** live smoke starts a Tk text/pointer fixture with a unique title
- **WHEN** the harness calls `x11_focus_window`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, and `x11_drag` against that fixture
- **THEN** keyboard and pointer capability rows are `pass` when the fixture event/value evidence matches expectations
- **AND** each input report proves verified target focus or bounds before input is sent
- **AND** no input evidence references a non-fixture application window

#### Scenario: GTK bridge fixture backs AT-SPI row
- **GIVEN** live smoke starts a GTK fixture with `GTK_MODULES=gail:atk-bridge` and `NO_AT_BRIDGE=0`
- **WHEN** the harness calls `x11_accessibility_tree` against the GTK fixture
- **THEN** the AT-SPI capability row is `pass` when the returned tree contains the expected accessible role or name
- **AND** Tk `NoAccessibilityMatch` evidence remains fixture-specific degraded evidence rather than the semantic AT-SPI pass path

#### Scenario: Screenshot and app-state target only fixtures
- **GIVEN** live smoke has selected a controlled fixture window and resolved its bounds
- **WHEN** the harness calls screenshot crop and `x11_get_app_state`
- **THEN** screenshot evidence stores image bytes as files under `target/e2e-logs/<run-id>/`
- **AND** app-state evidence records sanitized layer summaries or file paths rather than dumping full screenshot data URLs into ordinary logs
- **AND** the capability rows identify the fixture window used for the check

#### Scenario: Optional overlay lifecycle is fixture-scoped
- **GIVEN** overlay checks are enabled with `CODEX_X11_ENABLE_TK_OVERLAY=1`
- **WHEN** the harness targets a controlled fixture with overlay and then releases it
- **THEN** overlay evidence records `overlay.shown=true` and release hide evidence when the provider works
- **AND** overlay degradation is explicit when the provider is unavailable
- **AND** overlay helper windows are not selected as input or screenshot targets

### Requirement: Industrial evidence matrix classification
The E2E evidence schema and matrix validator MUST classify each fixture-backed capability with canonical machine JSON statuses `pass`, `degraded`, or `fail` and machine-readable reason categories that distinguish expected environment limitations, missing fixture setup, and actual code failure. Missing fixture setup MUST NOT be counted as acceptable industrial pass evidence.

#### Scenario: Environment limitation is degraded with evidence
- **GIVEN** GTK accessibility dependencies are unavailable in a live desktop session
- **WHEN** live smoke records the AT-SPI fixture outcome
- **THEN** the AT-SPI row status is `degraded`
- **AND** the reason category is `environment_limitation`
- **AND** evidence names the missing dependency or bridge condition

#### Scenario: Missing fixture setup blocks industrial acceptance
- **GIVEN** live smoke skipped keyboard input because no safe text fixture was started
- **WHEN** `validate-matrix` runs in industrial mode
- **THEN** validation fails or marks the run not industrial-ready
- **AND** the reason category is `missing_fixture_setup`
- **AND** the result is not normalized to acceptable degraded evidence

#### Scenario: Code failure is a fail
- **GIVEN** a controlled fixture is ready and the required tool call returns `success=false` for a non-environment reason
- **WHEN** the matrix validator evaluates the evidence
- **THEN** the affected row status is `fail`
- **AND** the reason category is `code_failure`
- **AND** validation exits non-zero for an industrial acceptance run
