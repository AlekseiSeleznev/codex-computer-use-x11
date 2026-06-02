## MODIFIED Requirements

### Requirement: Doctor detects AT-SPI bridge-disabled environment
The doctor JSON report MUST distinguish a reachable AT-SPI bus from GTK/ATK bridge-disabled risk when the process or controlled fixture environment contains `NO_AT_BRIDGE`. This diagnosis MUST be additive to existing bootstrap fields, MUST NOT make the Cinnamon/X11 window/input baseline fail by itself, and MUST NOT override a successful bounded collector result that proves a tree is available in the same effective runtime path.

#### Scenario: Bus reachable and NO_AT_BRIDGE only degrades when tree extraction fails
- **GIVEN** AT-SPI bus reachability is true
- **AND** the doctor environment contains `NO_AT_BRIDGE=1`
- **AND** tree extraction is unavailable or no application trees can be enumerated
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** accessibility facts include `atspi_bus_available=true`
- **AND** accessibility facts include `tree_available=false`
- **AND** the diagnostic state is `atspi_gtk_bridge_disabled_by_environment`
- **AND** the reason category is `environment_limitation`
- **AND** the report records a sanitized bridge-env fact showing that `NO_AT_BRIDGE` is present without exposing unrelated environment values
- **AND** the X11 baseline readiness remains governed by X11/EWMH blockers, not by optional semantic accessibility enrichment alone

#### Scenario: NO_AT_BRIDGE is diagnostic when collector proves a tree
- **GIVEN** AT-SPI bus reachability is true
- **AND** the doctor environment contains `NO_AT_BRIDGE=1`
- **AND** the bounded doctor collector returns valid output with one or more candidates or tree nodes using the same collector success contract as `accessibility-tree`
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** accessibility facts include `tree_available=true`
- **AND** `match_outcome` equals `tree_available`
- **AND** the diagnostic state is `tree_extraction_available`
- **AND** readiness degraded reasons do not include `atspi_tree_extraction_unavailable`
- **AND** the report may still record a sanitized bridge-env fact showing that `NO_AT_BRIDGE` is present as setup-risk context

#### Scenario: Recommendation names remediation and controlled verification
- **GIVEN** the diagnostic state is `atspi_gtk_bridge_disabled_by_environment`
- **WHEN** doctor recommendations are serialized
- **THEN** the recommendation says to remove or avoid inheriting `NO_AT_BRIDGE=1` for GTK fixture/application processes
- **AND** it says to restart the affected Cinnamon/Codex session or fixture process so the bridge environment changes take effect
- **AND** it says to verify semantic accessibility with the controlled GTK fixture before claiming AT-SPI pass evidence
- **AND** it does not recommend Wayland or portal-required runtime paths as the remediation for the X11 baseline

#### Scenario: Bridge-enabled tree failure remains distinct
- **GIVEN** AT-SPI bus reachability is true
- **AND** `NO_AT_BRIDGE` is absent from the probed environment
- **AND** tree extraction remains unavailable
- **WHEN** `doctor --json` emits accessibility facts
- **THEN** the diagnostic state remains `atspi_tree_extraction_unavailable` or a more specific non-bridge-disabled state
- **AND** the recommendation mentions package, gsettings, process, and controlled fixture checks instead of incorrectly blaming `NO_AT_BRIDGE`

### Requirement: Doctor AT-SPI probe reflects real collector availability
The doctor JSON command MUST determine AT-SPI tree availability from the same lightweight accessibility collector semantics used by the window-scoped accessibility-tree path instead of a hardcoded unavailable value, an environment-only short-circuit, or a divergent success parser. The report MUST expose the probe facts needed to distinguish a working tree, a missing or disabled bridge, an unavailable collector, invalid collector output, an ambiguous match, and a controlled-fixture pass.

#### Scenario: Working collector prevents false degraded doctor state
- **GIVEN** the AT-SPI bus is reachable
- **AND** the accessibility collector can return at least one candidate tree or a controlled fixture pass
- **WHEN** a caller runs `codex-computer-use-x11 doctor --json`
- **THEN** the doctor report records `tree_available` as `true`
- **AND** it records a positive `candidate_count` when candidates are present
- **AND** it records `match_outcome=tree_available`
- **AND** the X11 readiness summary does not report `atspi_tree_extraction_unavailable`

#### Scenario: Direct accessibility success and doctor probe agree
- **GIVEN** the same built binary can run `accessibility-tree --window-id <id> --json` for a resolved X11 window
- **AND** that command reports `success=true`, `correlation.status=matched`, and a non-empty `tree`
- **WHEN** a caller runs `codex-computer-use-x11 doctor --json` in the same effective accessibility environment
- **THEN** the doctor AT-SPI probe MUST NOT report `match_outcome=collector_unavailable`
- **AND** it MUST report `tree_available=true`
- **AND** it MUST expose a positive `candidate_count` when the collector reports candidates
- **AND** readiness degraded reasons MUST NOT include `atspi_tree_extraction_unavailable`

#### Scenario: NO_AT_BRIDGE does not short-circuit proven collector success
- **GIVEN** the AT-SPI bus is reachable
- **AND** the effective environment contains `NO_AT_BRIDGE=1`
- **AND** the bounded collector returns valid candidates or tree nodes equivalent to a successful `accessibility-tree` collector result
- **WHEN** a caller runs `codex-computer-use-x11 doctor --json`
- **THEN** the doctor report records `tree_available=true`
- **AND** it records `match_outcome=tree_available`
- **AND** it does not classify the diagnostic state as `atspi_gtk_bridge_disabled_by_environment`
- **AND** it does not record `collector_unavailable` for that collector output

#### Scenario: Unset NO_AT_BRIDGE path accepts valid collector output
- **GIVEN** the AT-SPI bus is reachable
- **AND** `NO_AT_BRIDGE` is absent from the effective environment
- **AND** the bounded collector returns valid candidates or tree nodes equivalent to a successful `accessibility-tree` collector result
- **WHEN** a caller runs `env -u NO_AT_BRIDGE codex-computer-use-x11 doctor --json`
- **THEN** the doctor report records `tree_available=true`
- **AND** it records `match_outcome=tree_available`
- **AND** it does not record `collector_unavailable`

#### Scenario: Unavailable, invalid, or timed-out collector remains degraded
- **GIVEN** the AT-SPI bus is reachable
- **AND** the bounded collector is missing, returns invalid output, returns no usable candidates or tree nodes, or times out
- **WHEN** a caller runs `codex-computer-use-x11 doctor --json`
- **THEN** the doctor report records `tree_available=false`
- **AND** it records a diagnostic state or match outcome that identifies the unavailable, invalid, no-tree, or timed-out collector condition
- **AND** it does not fabricate `match_outcome=tree_available`

#### Scenario: Ambiguous collector match is safe degradation
- **GIVEN** the AT-SPI bus is reachable
- **AND** the collector returns candidates but cannot confidently select a requested or controlled window subtree when a target-specific match is required
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

#### Scenario: Fixture proves accessibility-tree success maps to doctor success with NO_AT_BRIDGE present
- **GIVEN** a fixture collector output has `ok=true`, at least one candidate or tree node, and diagnostics equivalent to a successful accessibility-tree collector result
- **AND** the test environment contains `NO_AT_BRIDGE=1`
- **WHEN** the doctor probe consumes that fixture output
- **THEN** the resulting diagnostics mark tree availability true
- **AND** `match_outcome` equals `tree_available`
- **AND** `candidate_count` equals the fixture candidate count when the fixture reports one

#### Scenario: Fixture proves env-u path maps valid collector output to success
- **GIVEN** a fixture collector output has `ok=true`, at least one candidate or tree node, and diagnostics equivalent to a successful accessibility-tree collector result
- **AND** `NO_AT_BRIDGE` is absent from the test environment
- **WHEN** the doctor probe consumes that fixture output
- **THEN** the resulting diagnostics mark tree availability true
- **AND** `match_outcome` equals `tree_available`
- **AND** the degraded reason `atspi_tree_extraction_unavailable` is absent

#### Scenario: Fixture preserves true collector degradation
- **GIVEN** a fixture collector output is invalid, unavailable, empty, or timed out
- **WHEN** the doctor probe consumes that fixture output
- **THEN** the resulting diagnostics mark tree availability false
- **AND** they preserve the corresponding degraded outcome without panicking
