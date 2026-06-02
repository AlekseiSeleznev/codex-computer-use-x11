## ADDED Requirements

### Requirement: Industrial live AT-SPI uses GTK bridge fixture
Industrial live verification MUST treat a GTK fixture launched with accessibility bridge environment as the semantic AT-SPI pass path. Tk/Tkinter AT-SPI no-match MAY be recorded as expected degraded fixture evidence, but MUST NOT be used as the only live AT-SPI acceptance signal and MUST NOT cause the matcher to relax confidence rules.

#### Scenario: GTK bridge environment is recorded
- **GIVEN** live smoke starts the GTK AT-SPI fixture
- **WHEN** the harness records fixture metadata
- **THEN** evidence includes that `GTK_MODULES=gail:atk-bridge` and `NO_AT_BRIDGE=0` were set for the fixture process
- **AND** evidence includes the fixture title, process id when available, and selected window id
- **AND** no secret environment values are recorded

#### Scenario: GTK tree pass includes expected semantic node
- **GIVEN** the GTK fixture is ready and selected as the target window
- **WHEN** `x11_accessibility_tree` runs against the fixture
- **THEN** the report returns a high- or medium-confidence tree
- **AND** the tree contains an expected role, name, action, or value from the fixture
- **AND** the AT-SPI capability row references that tree evidence as the pass reason

#### Scenario: Tk no-match remains degraded only for Tk
- **GIVEN** the Tk fixture is used for keyboard and pointer checks
- **WHEN** `x11_accessibility_tree` returns `NoAccessibilityMatch` for the Tk fixture
- **THEN** evidence records the Tk AT-SPI result as fixture-specific degraded evidence
- **AND** the industrial AT-SPI pass still requires the GTK bridge fixture or a documented accessible equivalent
- **AND** the matcher does not return a bounds-only or arbitrary subtree for Tk
