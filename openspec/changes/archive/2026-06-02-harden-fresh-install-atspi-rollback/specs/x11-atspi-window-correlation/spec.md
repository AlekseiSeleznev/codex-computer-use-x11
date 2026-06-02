## ADDED Requirements

### Requirement: AT-SPI collector probe exposes canonical doctor facts
The AT-SPI correlation layer MUST provide a lightweight probe result that doctor can consume without duplicating collector logic. The probe result MUST include canonical match outcome, candidate count, whether any tree was obtainable, whether a controlled fixture passed, and sanitized degraded reasons.

#### Scenario: Probe reports obtainable tree without requiring a target selector
- **GIVEN** the AT-SPI bus is reachable
- **AND** the collector can enumerate application or window candidates
- **WHEN** the lightweight probe runs for doctor diagnostics
- **THEN** the probe returns `tree_available=true`
- **AND** it returns `candidate_count` equal to the number of candidate roots considered
- **AND** it returns a canonical `match_outcome` value

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
