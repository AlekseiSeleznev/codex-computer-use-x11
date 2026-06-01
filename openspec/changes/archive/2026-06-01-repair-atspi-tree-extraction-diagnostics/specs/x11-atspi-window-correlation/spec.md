## ADDED Requirements

### Requirement: AT-SPI bridge-disabled state is canonical
AT-SPI diagnostics emitted by accessibility-tree, app-state, doctor, and e2e evidence MUST use a canonical bridge-disabled outcome when the AT-SPI bus is reachable but a GTK/ATK bridge-disabling environment prevents useful tree extraction.

#### Scenario: Bridge-disabled outcome is not collapsed into generic unavailable
- **GIVEN** an AT-SPI probe reaches the accessibility bus
- **AND** the probed process environment has `NO_AT_BRIDGE` present
- **AND** no usable GTK application tree is exposed
- **WHEN** the probe serializes its diagnostic state
- **THEN** the outcome is `atspi_gtk_bridge_disabled_by_environment`
- **AND** the report preserves `atspi_bus_available=true` and `tree_available=false`
- **AND** no arbitrary AT-SPI subtree is returned as a pass

#### Scenario: Safe degradation preserves X11 context
- **GIVEN** an X11/EWMH window target resolves successfully
- **AND** AT-SPI diagnostics report `atspi_gtk_bridge_disabled_by_environment`
- **WHEN** `x11_accessibility_tree` or `x11_get_app_state` emits a report
- **THEN** the report includes the resolved X11 target diagnostics
- **AND** the AT-SPI layer is degraded with `reason_category=environment_limitation`
- **AND** the report does not send input, pointer, overlay, screenshot, or app-state operations to an uncontrolled real user window as fallback

## MODIFIED Requirements

### Requirement: Industrial live AT-SPI uses GTK bridge fixture
Industrial live verification MUST treat a GTK fixture launched with accessibility bridge environment as the semantic AT-SPI pass path. The fixture subprocess MUST remove `NO_AT_BRIDGE` from its environment rather than setting it to `0`, because the disabling contract is presence-based for bridge suppression in common GTK/ATK bridge integrations. Tk/Tkinter AT-SPI no-match MAY be recorded as expected degraded fixture evidence, but MUST NOT be used as the only live AT-SPI acceptance signal and MUST NOT cause the matcher to relax confidence rules.

#### Scenario: GTK bridge environment is recorded
- **GIVEN** live smoke starts the GTK AT-SPI fixture
- **WHEN** the harness records fixture metadata
- **THEN** evidence includes that `GTK_MODULES=gail:atk-bridge` was set for the fixture process when required by the desktop environment
- **AND** evidence includes that `NO_AT_BRIDGE` was absent from the fixture process environment
- **AND** evidence includes the fixture title, process id when available, and selected window id

#### Scenario: GTK tree pass includes expected semantic node
- **GIVEN** the GTK fixture is ready and selected as the target window
- **WHEN** `x11_accessibility_tree` runs against the fixture
- **THEN** the report matches a high- or medium-confidence subtree
- **AND** the tree contains an expected role, name, action, or value from the fixture
- **AND** the AT-SPI capability row references that tree evidence as the pass reason

#### Scenario: Tk no-match is not the pass path
- **GIVEN** the Tk fixture is used for keyboard and pointer checks
- **WHEN** `x11_accessibility_tree` returns `NoAccessibilityMatch` for the Tk fixture
- **THEN** evidence records the Tk AT-SPI result as fixture-specific degraded evidence
- **AND** the industrial AT-SPI pass still requires the GTK bridge fixture or a documented accessible equivalent
