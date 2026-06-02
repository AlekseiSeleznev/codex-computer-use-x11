## ADDED Requirements

### Requirement: Fixture bridge environment is sanitized and self-tested
The e2e harness MUST launch the controlled GTK AT-SPI fixture with a safe bridge environment, record sanitized environment facts, and provide fake/self-test coverage for bridge-disabled and bridge-enabled fixture paths without changing the global user environment.

#### Scenario: Fixture launch removes inherited NO_AT_BRIDGE
- **GIVEN** the Codex or harness parent environment contains `NO_AT_BRIDGE=1`
- **WHEN** live smoke starts the controlled GTK AT-SPI fixture
- **THEN** the fixture subprocess environment removes `NO_AT_BRIDGE`
- **AND** the parent process environment and global user environment are not modified
- **AND** the fixture metadata records `NO_AT_BRIDGE` as absent for the fixture process
- **AND** the metadata records `GTK_MODULES=gail:atk-bridge` when that override was applied for the fixture

#### Scenario: Fake bridge-disabled evidence is classified as environment limitation
- **GIVEN** fake smoke or validator fixtures include doctor/accessibility evidence with `atspi_bus_available=true`, `tree_available=false`, and `NO_AT_BRIDGE` present
- **WHEN** the matrix validator evaluates the AT-SPI row
- **THEN** the row may be `degraded` with `reason_category=environment_limitation`
- **AND** the reason or evidence references `atspi_gtk_bridge_disabled_by_environment`
- **AND** the validator does not classify the absence of a real live GTK fixture implementation as `code_failure` solely because the environment is bridge-disabled

#### Scenario: Missing live fixture code remains a setup limitation
- **GIVEN** a live run has no controlled GTK fixture code available or cannot start it safely
- **WHEN** fixture-dependent AT-SPI validation is summarized
- **THEN** the row uses `missing_fixture_setup` or the precise dependency/environment category supported by the evidence
- **AND** no real user window is selected as a fallback AT-SPI target
- **AND** the summary tells the operator to run the controlled GTK fixture path after correcting bridge environment

## MODIFIED Requirements

### Requirement: Live plugin smoke verifies fixture-backed capability rows
Live standalone plugin smoke MUST exercise fixture-backed tool calls for keyboard input, pointer input, window listing/focus, target context/release, screenshot, `get_app_state`, GTK AT-SPI, and optional overlay lifecycle. Each exercised capability row MUST include the concrete fixture id, tool call, evidence path, status, and reason. GTK AT-SPI fixture evidence MUST record bridge-environment facts with `NO_AT_BRIDGE` absent rather than `NO_AT_BRIDGE=0`.

#### Scenario: Tk fixture backs keyboard and pointer rows
- **GIVEN** live smoke starts a Tk text/pointer fixture with a unique title
- **WHEN** the harness calls `x11_focus_window`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, and `x11_drag` against that fixture
- **THEN** keyboard and pointer capability rows are `pass` when the fixture event/value evidence matches expectations
- **AND** each input report proves verified target focus or bounds before input is sent
- **AND** no input evidence references a non-fixture application window

#### Scenario: GTK bridge fixture backs AT-SPI row
- **GIVEN** live smoke starts a GTK fixture with `GTK_MODULES=gail:atk-bridge` when needed and with `NO_AT_BRIDGE` absent
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
