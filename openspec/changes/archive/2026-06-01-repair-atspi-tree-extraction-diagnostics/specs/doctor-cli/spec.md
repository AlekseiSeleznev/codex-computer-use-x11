## ADDED Requirements

### Requirement: Doctor detects AT-SPI bridge-disabled environment
The doctor JSON report MUST distinguish a reachable AT-SPI bus from GTK/ATK bridge-disabled tree extraction when the process or controlled fixture environment contains `NO_AT_BRIDGE`. This diagnosis MUST be additive to existing bootstrap fields and MUST NOT make the Cinnamon/X11 window/input baseline fail by itself.

#### Scenario: Bus reachable but NO_AT_BRIDGE disables GTK bridge
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
