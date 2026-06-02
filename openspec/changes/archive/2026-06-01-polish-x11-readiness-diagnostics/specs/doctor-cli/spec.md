## ADDED Requirements

### Requirement: Doctor X11 readiness taxonomy
The doctor JSON report MUST expose a stable X11 readiness taxonomy that separates blocking failures, acceptable degraded X11-baseline limitations, optional enrichments, and unsupported out-of-scope paths without removing existing bootstrap-compatible fields.

#### Scenario: X11 baseline remains ready with optional enrichments degraded
- **GIVEN** `wmctrl`, `xprop`, `xdotool`, `DISPLAY`, and X11/EWMH window probes are usable
- **AND** AT-SPI tree extraction or RemoteDesktop portal probing is unavailable
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** `readiness.ok` is true when no X11-baseline blocker exists
- **AND** `readiness.blockers` is empty
- **AND** the report includes machine-readable degraded-but-acceptable entries for unavailable optional enrichments
- **AND** the report does not classify RemoteDesktop portal absence as an X11-baseline blocker

#### Scenario: X11 window probing blocker is distinct from optional degradation
- **GIVEN** the current environment has no usable `DISPLAY` or cannot run required X11/EWMH window probes
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** `readiness.ok` is false
- **AND** `readiness.blockers` contains a machine-readable X11 window-probing blocker
- **AND** optional AT-SPI, screenshot, ydotool, or portal facts do not hide the blocker

#### Scenario: Unsupported Wayland path is explicit and non-blocking for X11 diagnosis
- **GIVEN** the session facts indicate Wayland or an unavailable RemoteDesktop portal path
- **WHEN** `doctor --json` computes supported-scope readiness
- **THEN** the report records the unsupported or out-of-scope path separately from X11 blockers
- **AND** the recommendation explains that Wayland support is outside this change's runtime scope
- **AND** the report does not require portal input or screenshot paths to pass the Cinnamon/X11 baseline

### Requirement: Doctor AT-SPI diagnostic states
The doctor JSON report MUST distinguish AT-SPI bus unavailability, bus reachability, tree extraction unavailability, working tree extraction with no matching app subtree, ambiguous matches, and controlled-fixture pass evidence.

#### Scenario: Bus reachable but tree extraction unavailable
- **GIVEN** the AT-SPI bus can be reached
- **AND** tree extraction fails because accessibility is disabled, bridge support is missing, or the probe cannot enumerate application trees
- **WHEN** `doctor --json` emits accessibility facts
- **THEN** the AT-SPI diagnostic state is not merely `unavailable`
- **AND** it records bus reachability separately from tree extraction availability
- **AND** it recommends Cinnamon/X11 setup actions without marking the X11 window baseline failed

#### Scenario: Tree extraction works but no app subtree matches
- **GIVEN** AT-SPI tree extraction succeeds
- **AND** no extracted application subtree matches the selected or controlled target window
- **WHEN** `doctor --json` or fixture-backed diagnostics summarize AT-SPI
- **THEN** the diagnostic state identifies `no_matching_app_subtree` or an equivalent stable code
- **AND** the report preserves score or signal details sufficient to debug matching
- **AND** no arbitrary subtree is returned as a pass

#### Scenario: Ambiguous AT-SPI match is safe degradation
- **GIVEN** AT-SPI tree extraction succeeds
- **AND** multiple candidate subtrees cannot be distinguished with sufficient confidence
- **WHEN** AT-SPI diagnostics are serialized
- **THEN** the diagnostic state identifies ambiguity
- **AND** the report records candidate count or score facts without exposing secret values
- **AND** the X11 baseline may remain ready while semantic accessibility is degraded

### Requirement: Doctor report redacts private diagnostic paths
Doctor diagnostics MUST NOT serialize secret values or private socket/runtime absolute paths, while preserving stable labels that make readiness decisions reproducible.

#### Scenario: Private runtime paths are label-redacted
- **GIVEN** `YDOTOOL_SOCKET`, `XDG_RUNTIME_DIR`, or desktop bus paths contain user-private absolute paths
- **WHEN** `doctor --json` records checked diagnostic candidates
- **THEN** the serialized report uses stable labels such as `env:YDOTOOL_SOCKET` or `env:XDG_RUNTIME_DIR`
- **AND** it does not include the private absolute path values
- **AND** internal probing may still use the real paths locally
