## MODIFIED Requirements

### Requirement: AT-SPI collector probe exposes canonical doctor facts
The AT-SPI correlation layer MUST provide a lightweight probe result that doctor can consume without duplicating or diverging from collector logic. The probe result MUST include canonical match outcome, candidate count, whether any tree was obtainable, whether a controlled fixture passed, and sanitized degraded reasons. A successful collector output with one or more candidates MUST be interpreted the same way for doctor probing and for window-scoped accessibility-tree collection.

#### Scenario: Probe reports obtainable tree without requiring a target selector
- **GIVEN** the AT-SPI bus is reachable
- **AND** the collector can enumerate application or window candidates
- **WHEN** the lightweight probe runs for doctor diagnostics
- **THEN** the probe returns `tree_available=true`
- **AND** it returns `candidate_count` equal to the number of candidate roots considered
- **AND** it returns a canonical `match_outcome` value

#### Scenario: Probe uses the same collector success contract as accessibility tree
- **GIVEN** the collector process returns valid JSON with `ok=true`
- **AND** the JSON contains one or more AT-SPI candidates that the accessibility-tree path can score
- **WHEN** the lightweight doctor probe parses that collector output
- **THEN** it treats the collector as available
- **AND** it reports `match_outcome=tree_available`
- **AND** it does not collapse the result to `collector_unavailable`

#### Scenario: Controlled fixture pass is distinguishable from ambient candidates
- **GIVEN** a controlled GTK fixture is configured for live validation
- **AND** the collector finds the expected fixture semantic node
- **WHEN** the lightweight probe runs
- **THEN** the probe records `controlled_fixture_pass=true`
- **AND** it still reports the ambient candidate count separately
- **AND** the report does not expose uncontrolled user-window text beyond sanitized diagnostics

#### Scenario: Probe preserves bridge-disabled reason
- **GIVEN** the process or activation environment contains `NO_AT_BRIDGE=1`
- **WHEN** the lightweight probe evaluates AT-SPI readiness
- **THEN** the probe returns a bridge-disabled canonical outcome
- **AND** it does not collapse the result into a generic tree-unavailable state
