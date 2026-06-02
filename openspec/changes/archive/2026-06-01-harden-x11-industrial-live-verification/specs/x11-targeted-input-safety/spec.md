## ADDED Requirements

### Requirement: Live input harness targets only controlled fixtures
Live keyboard and pointer verification MUST send input only to controlled fixture windows that the harness created or explicitly selected by unique fixture title/class and verified window id. The harness MUST refuse to call input tools when the resolved target is absent, ambiguous, stale, an overlay/helper window, or a real user application outside the fixture allowlist.

#### Scenario: Ambiguous fixture selection blocks input
- **GIVEN** live smoke expects one Tk input fixture
- **AND** window listing finds zero or more than one matching controlled fixture candidate
- **WHEN** the harness prepares to call `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, or `x11_drag`
- **THEN** the harness does not call the input tool
- **AND** the capability row is not `pass`
- **AND** evidence explains the missing or ambiguous fixture target

#### Scenario: Real user app is rejected as input target
- **GIVEN** window listing includes a non-fixture application window such as a browser, terminal, messenger, password manager, or editor
- **WHEN** live smoke resolves targets for input checks
- **THEN** the non-fixture window is not eligible for keyboard or pointer operations
- **AND** the harness records only sanitized selection diagnostics
- **AND** no input tool invocation targets that user application window id

#### Scenario: Fixture cleanup runs after input failure
- **GIVEN** an input tool call against a controlled fixture fails after the fixture was started
- **WHEN** the live smoke exits
- **THEN** cleanup traps terminate the fixture process or close the fixture window
- **AND** any target-window state for the fixture is released
- **AND** the failure evidence remains available under the run log directory
