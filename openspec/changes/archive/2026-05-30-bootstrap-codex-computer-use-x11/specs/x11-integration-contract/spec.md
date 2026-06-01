## ADDED Requirements

### Requirement: Canonical X11 EWMH backend identity
The integration contract MUST use `x11-ewmh` as the canonical backend identifier for the future generic X11/EWMH backend, and it MUST NOT use `x11` or `cinnamon` as the backend id.

#### Scenario: Name future X11 windows consistently
- **GIVEN** a future X11/EWMH backend reports a window to Codex Computer Use Linux
- **WHEN** that window is mapped into the upstream `WindowInfo` model
- **THEN** `WindowInfo.backend` is `x11-ewmh`
- **AND** `client_type` may describe the client/window type separately from the backend id

### Requirement: Source-overlay fallback order
A future source overlay MUST register `x11-ewmh` as a late fallback after existing desktop-specific window backends unless a later accepted ADR changes the registry strategy.

#### Scenario: Preserve desktop-specific backends
- **GIVEN** the target registry includes GNOME extension, GNOME introspect, COSMIC, KWin, Hyprland, and i3 backends
- **WHEN** `x11-ewmh` is added in a future source overlay
- **THEN** it is ordered after those existing desktop-specific backends
- **AND** it does not replace a more specific backend that can list or focus windows successfully

### Requirement: Upstream WindowInfo is the primary model
The integration contract MUST treat the upstream `WindowInfo` shape as the primary window data model and MUST keep X11-only reliability or provenance details in a sidecar/report by default.

#### Scenario: Map X11 metadata without extending WindowInfo
- **GIVEN** an X11 observation includes raw ids, command source, PID reliability, warnings, or degraded diagnostics
- **WHEN** the observation is converted for upstream window consumers
- **THEN** supported fields are mapped into upstream `WindowInfo`
- **AND** X11-only provenance or diagnostic fields are stored in a sidecar/report
- **AND** upstream `WindowInfo` is not expanded without a later design/ADR decision

### Requirement: Canonical X11 window-id normalization
The project MUST provide a shared X11 window-id normalizer that converts equivalent hexadecimal X11 id strings to the same canonical `u64` value.

#### Scenario: Normalize equivalent hex ids
- **GIVEN** one tool reports a window id as `0x5624b36`
- **AND** another tool reports the same window id as `0x05624b36`
- **WHEN** both strings are parsed by the shared normalizer
- **THEN** both results are the same `u64`
- **AND** the bootstrap normalizer does not expose command-formatting behavior; future `wmctrl` or `xdotool` formatting must remain separate from the canonical numeric parser

### Requirement: Standalone command testing seam
Standalone project code that exercises external command behavior MUST use either a command-runner seam or a fake `PATH` fixture so tests can run without live X11 command dependencies.

#### Scenario: Test standalone external command behavior
- **GIVEN** standalone project code needs to test external command behavior
- **WHEN** tests are written for the standalone crate
- **THEN** a command-runner seam or fake `PATH` fixture is used
- **AND** tests can run without invoking live `wmctrl`, `xprop`, or `xdotool` binaries

### Requirement: Source-overlay command style decision
Future source-overlay code MUST follow the target repo style of thin `Command::new(...)` wrappers plus pure parser/normalizer fixture tests unless an explicit design/ADR exception is accepted before introducing a dependency-injection runner into the target repo.

#### Scenario: Use default source-overlay command style
- **GIVEN** future source-overlay work needs external command behavior
- **WHEN** the design proposes the default source-overlay test approach
- **THEN** it uses thin command wrappers plus pure parser/normalizer fixture tests

#### Scenario: Record dependency-injection runner exception
- **GIVEN** future source-overlay work proposes a dependency-injection runner inside the target repo
- **WHEN** the design accepts that exception
- **THEN** the design or ADR records the rationale before source-overlay code adds the runner
