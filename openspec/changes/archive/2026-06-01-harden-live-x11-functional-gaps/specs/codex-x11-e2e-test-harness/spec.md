## ADDED Requirements

### Requirement: Live harness verifies exact Unicode text fidelity
Live e2e mode MUST verify the actual value inserted into a safe text field for Cyrillic and other non-ASCII text, not only key-release events or backend exit status. The evidence MUST identify the selected keyboard route and mark the keyboard capability degraded when exact text value does not match.

#### Scenario: Cyrillic value matches expected text
- **GIVEN** live mode starts a safe text fixture with a readable text value endpoint or event log
- **WHEN** the harness calls `x11_type_text` with Cyrillic text such as `Привет`
- **THEN** the fixture value equals the requested Cyrillic string at the checked insertion point
- **AND** the evidence records the keyboard route used
- **AND** the keyboard capability row is `pass` for Unicode fidelity

#### Scenario: Cyrillic key events without exact value are degraded
- **GIVEN** live mode sends Cyrillic text
- **AND** the fixture event log only shows layout-dependent Latin keysyms or the final value differs from the requested string
- **WHEN** capability matrix validation runs
- **THEN** the keyboard capability row is `degraded`
- **AND** the reason states that exact Cyrillic fidelity was not proven

### Requirement: Live harness includes GTK AT-SPI positive fixture
Live e2e mode MUST include an AT-SPI-positive GTK fixture or documented GTK-safe application and MUST validate that `x11_accessibility_tree` can return a matched subtree for that fixture. Tkinter windows MAY remain in the fixture set for keyboard and pointer checks but MUST NOT be the sole acceptance evidence for AT-SPI.

#### Scenario: GTK fixture AT-SPI pass is recorded
- **GIVEN** live mode starts or selects a GTK fixture with stable title and accessible controls
- **WHEN** the harness targets the GTK fixture with `x11_accessibility_tree`
- **THEN** a high- or medium-confidence subtree is returned
- **AND** expected accessible control names or roles are present
- **AND** the AT-SPI capability row records pass with GTK evidence

#### Scenario: Tk AT-SPI no-match is documented separately
- **GIVEN** Tkinter safe windows are present for keyboard and pointer checks
- **WHEN** AT-SPI matching returns `NoAccessibilityMatch` for Tk windows
- **THEN** the harness records that as Tk fixture limitation evidence
- **AND** it does not lower correlation thresholds or use bounds-only matching to pass AT-SPI

### Requirement: Live harness verifies overlay lifecycle
Live e2e mode MUST verify that target overlay display, release/hide behavior, and overlay listing exclusion work when overlay is requested. Overlay provider failure MAY remain degraded only when the failure is explicit and target state lifecycle still passes.

#### Scenario: Overlay shown and hidden in live mode
- **GIVEN** live mode has a safe target window with valid bounds
- **WHEN** the harness runs `x11_target_window` with overlay enabled
- **THEN** the tool report has `overlay.requested=true` and `overlay.shown=true`
- **AND** subsequent listing excludes project overlay windows from target candidates
- **WHEN** the harness runs `x11_release_window`
- **THEN** overlay diagnostics show hide requested or completed
- **AND** follow-up target context is empty or no longer contains the released target

#### Scenario: Overlay failure is explicit degraded evidence
- **GIVEN** overlay is requested in live mode
- **AND** the provider cannot show a border
- **WHEN** capability matrix validation runs
- **THEN** target context lifecycle may pass
- **AND** overlay status is `degraded` with the provider warning
- **AND** the failure is not silently omitted from readiness evidence

### Requirement: Capability matrix records pass and degraded rows with concrete evidence
The e2e harness MUST update machine-readable capability matrix rows for live and fake modes so every required v1 group has `pass` or `degraded` status with concrete evidence paths, tool names, and reasons. Missing rows or summary extraction bugs MUST fail validation.

#### Scenario: Live matrix includes concrete degraded reasons
- **GIVEN** live mode produces evidence for keyboard, AT-SPI, overlay, app-state, and portal readiness
- **WHEN** matrix validation runs
- **THEN** each row has a status of `pass` or `degraded`
- **AND** degraded rows include concrete evidence such as log paths, tool names, and observed error codes
- **AND** missing rows fail validation

#### Scenario: Evidence summary uses no-screenshot-data output
- **GIVEN** live app-state captured a screenshot with a large data URL
- **WHEN** the harness writes durable summary files
- **THEN** summary files omit the full base64 screenshot data
- **AND** they retain screenshot status and metadata needed to validate the screenshot capability
