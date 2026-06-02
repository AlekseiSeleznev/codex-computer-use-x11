## MODIFIED Requirements

### Requirement: Doctor AT-SPI probe reflects real collector availability
The doctor JSON command MUST determine AT-SPI tree availability from the same lightweight accessibility collector semantics used by the window-scoped accessibility-tree path instead of a hardcoded unavailable value or a divergent success parser. The report MUST expose the probe facts needed to distinguish a working tree, a missing or disabled bridge, an unavailable collector, an ambiguous match, and a controlled-fixture pass.

#### Scenario: Working collector prevents false degraded doctor state
- **GIVEN** the AT-SPI bus is reachable
- **AND** the accessibility collector can return at least one candidate tree or a controlled fixture pass
- **WHEN** a caller runs `codex-computer-use-x11 doctor --json`
- **THEN** the doctor report records `tree_available` as `true`
- **AND** it records a non-negative `candidate_count`
- **AND** it records the collector `match_outcome`
- **AND** the X11 readiness summary does not report `atspi_tree_extraction_unavailable`

#### Scenario: Direct accessibility success and doctor probe agree
- **GIVEN** the same built binary can run `accessibility-tree --window-id <id> --json` for a resolved X11 window
- **AND** that command reports `success=true`, `correlation.status=matched`, and a non-empty `tree`
- **WHEN** a caller runs `codex-computer-use-x11 doctor --json` in the same effective accessibility environment
- **THEN** the doctor AT-SPI probe MUST NOT report `match_outcome=collector_unavailable`
- **AND** it MUST report `tree_available=true`
- **AND** it MUST expose a positive `candidate_count`
- **AND** readiness degraded reasons MUST NOT include `atspi_tree_extraction_unavailable`

#### Scenario: Bridge-disabled environment remains degraded
- **GIVEN** the AT-SPI bus is reachable
- **AND** the effective environment disables GTK bridge loading with `NO_AT_BRIDGE=1`
- **WHEN** a caller runs `codex-computer-use-x11 doctor --json`
- **THEN** the doctor report records `tree_available` as `false`
- **AND** it records a bridge-disabled AT-SPI reason instead of reporting a successful tree
- **AND** remediation recommends enabling the bridge and re-running a controlled verification

#### Scenario: Ambiguous collector match is safe degradation
- **GIVEN** the AT-SPI bus is reachable
- **AND** the collector returns candidates but cannot confidently select a requested or controlled window subtree
- **WHEN** a caller runs `codex-computer-use-x11 doctor --json`
- **THEN** the doctor report records the candidate count and ambiguous match outcome
- **AND** it classifies the accessibility layer as degraded
- **AND** it does not fabricate a successful tree match

### Requirement: Doctor probe facts are testable without live desktop state
Doctor AT-SPI probe behavior MUST be testable through fixtures or a collector seam so false-negative fixes can be verified without requiring the developer's live Cinnamon/X11 session.

#### Scenario: Fixture proves tree availability path
- **GIVEN** a test fixture whose collector probe returns a candidate tree
- **WHEN** the doctor fact gathering code builds the accessibility diagnostics
- **THEN** the resulting diagnostics mark tree availability true
- **AND** they include the fixture candidate count and match outcome

#### Scenario: Fixture proves unavailable path
- **GIVEN** a test fixture whose collector probe reports the AT-SPI bus unavailable
- **WHEN** the doctor fact gathering code builds the accessibility diagnostics
- **THEN** the resulting diagnostics mark tree availability false
- **AND** they preserve the unavailable reason without panicking

#### Scenario: Fixture proves accessibility-tree success maps to doctor success
- **GIVEN** a fixture collector output has `ok=true`, at least one candidate, and diagnostics equivalent to a successful accessibility-tree collector result
- **WHEN** the doctor probe consumes that fixture output
- **THEN** the resulting diagnostics mark tree availability true
- **AND** `match_outcome` equals `tree_available`
- **AND** `candidate_count` equals the fixture candidate count
