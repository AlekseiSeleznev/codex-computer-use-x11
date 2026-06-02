## ADDED Requirements

### Requirement: AT-SPI diagnostics use canonical match outcome codes
AT-SPI correlation diagnostics MUST use canonical outcome codes that distinguish bus/probe availability, extraction, no-match, ambiguous-match, and fixture-backed pass states.

#### Scenario: Canonical outcomes cover every AT-SPI probe result
- **GIVEN** an AT-SPI accessibility-tree or app-state request is evaluated
- **WHEN** the result is serialized in CLI, MCP, app-state, doctor, or e2e evidence
- **THEN** the outcome is one of a documented canonical set covering bus unavailable, bus reachable, tree extraction unavailable, no matching subtree, ambiguous match, and matched subtree
- **AND** every non-pass outcome includes a reason and next diagnostic hint
- **AND** low-confidence or ambiguous data is not normalized into a successful subtree

#### Scenario: Controlled GTK fixture pass is distinguishable from live ambient success
- **GIVEN** live smoke starts a controlled GTK AT-SPI fixture with a unique title or class
- **WHEN** `x11_accessibility_tree` returns the expected role or name from that fixture
- **THEN** the evidence records a controlled-fixture pass outcome
- **AND** the fixture id, target window id, and correlation signals are included in sanitized evidence
- **AND** ambient user application windows are not used as fallback pass evidence

### Requirement: Cinnamon X11 recommendations are actionable
AT-SPI degraded diagnostics MUST include recommendations that are specific enough for Cinnamon/X11 troubleshooting without making AT-SPI mandatory for the X11 window/input baseline.

#### Scenario: Missing bridge produces setup recommendation
- **GIVEN** AT-SPI bus reachability or tree extraction is degraded because bridge support or accessibility enablement appears missing
- **WHEN** diagnostics are emitted
- **THEN** the recommendation identifies the likely setup category
- **AND** it avoids claiming a code failure without evidence
- **AND** it states whether the X11 baseline remains usable without semantic accessibility enrichment
