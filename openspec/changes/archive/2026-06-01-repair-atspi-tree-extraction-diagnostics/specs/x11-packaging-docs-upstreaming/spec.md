## ADDED Requirements

### Requirement: Troubleshooting explains bus-reachable tree-unavailable AT-SPI
Troubleshooting and retest documentation MUST include a dedicated Cinnamon/X11 section for `atspi_bus_available=true` with `tree_available=false`, including the bridge-disabled environment path and safe controlled-fixture verification.

#### Scenario: Reader can diagnose NO_AT_BRIDGE bridge suppression
- **GIVEN** a developer sees `diagnostic_state=atspi_gtk_bridge_disabled_by_environment` or `atspi_tree_extraction_unavailable`
- **WHEN** they read troubleshooting documentation
- **THEN** the docs explain that AT-SPI bus reachability is different from GTK/ATK tree extraction
- **AND** the docs tell them to inspect package availability for `at-spi2-core`, `libatk-adaptor`, `libatk-bridge2.0-0t64`, and `libatspi2.0-0t64` or distribution equivalents
- **AND** the docs tell them to check toolkit accessibility settings and AT-SPI processes such as the bus launcher, registry daemon, and AT-SPI DBus daemon
- **AND** the docs identify inherited `NO_AT_BRIDGE=1` as a bridge-disable signal that should be removed or not inherited by GTK fixture/application processes

#### Scenario: Reader gets safe remediation steps
- **GIVEN** the operator wants to recover semantic AT-SPI evidence on Cinnamon/X11
- **WHEN** they follow the remediation section
- **THEN** it says not to change the global environment from the test harness
- **AND** it says to restart the affected Cinnamon/Codex session or fixture process after correcting bridge-related environment
- **AND** it says to run a controlled GTK fixture self-test or live fixture smoke before claiming AT-SPI pass evidence
- **AND** it warns that live checks must not target real user windows as fallback

#### Scenario: Documentation preserves baseline semantics
- **GIVEN** AT-SPI tree extraction remains unavailable after safe checks
- **WHEN** the operator reads the production readiness guidance
- **THEN** the docs state that this is expected degraded semantic accessibility enrichment for the Cinnamon/X11 baseline when X11 window/focus/input requirements still pass
- **AND** the docs state that a degraded AT-SPI row still needs a concrete `reason_category` and evidence path
- **AND** the docs do not expand scope to Wayland or portal-required runtime paths
