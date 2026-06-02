# x11-integration-contract Specification

## Purpose
This specification defines the X11/EWMH integration contract for backend identity, window-id normalization, source-overlay compatibility, and future Codex Desktop Linux adaptation constraints.
## Requirements
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
Standalone project code that exercises external command or DBus probe behavior MUST use a command-runner seam, fake command output, fixtures, or a fake `PATH` fixture so tests can run without live X11 command dependencies.

#### Scenario: Test standalone external command behavior
- **GIVEN** standalone project code needs to test external command behavior
- **WHEN** tests are written for the standalone crate
- **THEN** a command-runner seam or fake `PATH` fixture is used
- **AND** tests can run without invoking live `wmctrl`, `xprop`, or `xdotool` binaries

#### Scenario: Test standalone DBus probe parsing
- **GIVEN** standalone project code needs to classify portal or screenshot provider availability from DBus introspection output
- **WHEN** tests are written for parser behavior
- **THEN** fixtures or fake command output are used for `busctl` or `gdbus` output
- **AND** tests cover both available method tables and successful empty tables

### Requirement: Source-overlay command style decision
Future source-overlay code MUST follow the target repo style of thin `Command::new(...)` wrappers plus pure parser/normalizer fixture tests unless an explicit design/ADR exception is accepted before introducing a dependency-injection runner into the target repo.

#### Scenario: Use default source-overlay command style
- **GIVEN** future source-overlay work needs external command behavior
- **WHEN** the design proposes the default source-overlay test approach
- **THEN** it uses thin command wrappers plus pure parser/normalizer fixture tests
- **AND** portal or screenshot introspection parsing is covered by pure fixtures before live Cinnamon/X11 smoke evidence is accepted

#### Scenario: Record dependency-injection runner exception
- **GIVEN** future source-overlay work proposes a dependency-injection runner inside the target repo
- **WHEN** the design accepts that exception
- **THEN** the design or ADR records the rationale before source-overlay code adds the runner

### Requirement: Source-overlay diagnostics vocabulary compatibility
The source overlay MUST keep doctor and capability reports aligned with the current Computer Use Linux diagnostic vocabulary and MUST NOT require new upstream readiness fields that are absent from the target repo.

#### Scenario: Use upstream readiness names
- **GIVEN** the standalone doctor report maps readiness into source-overlay concepts
- **WHEN** the source overlay updates the target repo diagnostics
- **THEN** it uses `can_query_windows`, `can_focus_apps`, `can_focus_windows`, and `can_send_development_input` semantics for readiness
- **AND** it does not require an upstream `can_send_targeted_input` readiness field
- **AND** any targeted-input explanation remains derived from focus, window query, input backend, blocker, and recommendation facts

#### Scenario: Preserve target report layers
- **GIVEN** source-overlay work updates Computer Use Linux diagnostics
- **WHEN** capability facts are mapped into the target report
- **THEN** portal facts stay compatible with `PortalReport`
- **AND** input backend facts stay compatible with `InputReport`
- **AND** window listing and focus facts stay compatible with `WindowingReport`
- **AND** readiness facts stay compatible with `ReadinessReport`

### Requirement: Strict portal interface detection
The source overlay MUST classify portal interfaces by required methods or properties, not by `busctl` or `gdbus` exit status alone.

#### Scenario: Empty RemoteDesktop table is unavailable
- **GIVEN** `busctl introspect` for `org.freedesktop.portal.RemoteDesktop` exits successfully
- **AND** the introspection output contains no RemoteDesktop methods or properties
- **WHEN** the source overlay computes portal input readiness
- **THEN** `PortalReport.remote_desktop.ok` is false
- **AND** portal input is not added to preferred input capabilities from that empty table
- **AND** the diagnostic detail explains that required RemoteDesktop methods or properties were absent

#### Scenario: RemoteDesktop method table is available
- **GIVEN** portal RemoteDesktop introspection contains concrete methods or properties for session creation and input notification
- **WHEN** the source overlay computes portal input readiness
- **THEN** `PortalReport.remote_desktop.ok` may be true
- **AND** portal input readiness is based on those concrete methods or properties rather than command exit status alone

#### Scenario: Screenshot method table is available
- **GIVEN** portal Screenshot introspection includes the `Screenshot` method
- **WHEN** the source overlay computes screenshot capability facts
- **THEN** the portal screenshot fact is available
- **AND** version 2 is sufficient for basic screenshot availability
- **AND** version 3-only properties are not required for the basic screenshot path

### Requirement: Screenshot provider capability mapping
The source overlay MUST allow screenshot capability reporting to distinguish provider provenance while still mapping to the target repo screenshot capability semantics.

#### Scenario: Cinnamon provides GNOME Shell compatible screenshot DBus
- **GIVEN** Cinnamon owns `org.gnome.Shell.Screenshot` and exposes screenshot methods
- **WHEN** the source overlay computes screenshot capabilities
- **THEN** it reports a GNOME Shell-compatible DBus screenshot provider as available
- **AND** it does not require the `gnome-shell` binary version check to pass before recognizing that provider
- **AND** it keeps this provider distinct from XDG Portal Screenshot availability in diagnostic facts

#### Scenario: Map screenshot providers to preferred screenshot capability
- **GIVEN** one or more screenshot providers are available
- **WHEN** the source overlay builds a capability map
- **THEN** the preferred screenshot backend is derived from available provider facts
- **AND** provider provenance remains visible in diagnostics or sidecar facts
- **AND** the mapping does not collapse portal and GNOME Shell-compatible DBus providers into one ambiguous boolean

### Requirement: Targeted input remains gated by verified focus
The source overlay MUST NOT enable window-targeted keyboard or pointer input solely because an input backend exists; targeted input remains gated by window query and exact focus verification behavior.

#### Scenario: Input backend without exact focus is degraded
- **GIVEN** a development input backend is available
- **AND** window listing or exact window focus verification is unavailable
- **WHEN** targeted input readiness is explained
- **THEN** the report does not claim verified targeted input readiness
- **AND** it explains which window query or focus capability is missing
- **AND** global or development input availability remains a separate fact

#### Scenario: Derived targeted-input fact is report-only
- **GIVEN** a later design adds a derived targeted-input fact
- **WHEN** that fact appears in diagnostics
- **THEN** it is additive and report-only
- **AND** it is nested under diagnostics or capability facts rather than top-level readiness
- **AND** upstream consumers are not required to honor it unless a later accepted spec changes the contract

### Requirement: Source-overlay acceptance record
The `x11-integration-contract` spec delta MUST explicitly record source-overlay acceptance for the Computer Use Linux doctor/report gaps, and follow-up design and implementation tasks MUST consume that acceptance rather than relying on an implicit design note.

#### Scenario: Verify strict portal and screenshot acceptance in the spec delta
- **GIVEN** a reviewer inspects this `x11-integration-contract` spec delta
- **WHEN** they look for source-overlay acceptance criteria
- **THEN** the delta includes acceptance for strict portal RemoteDesktop detection
- **AND** the delta includes acceptance for Screenshot version 2 and Cinnamon GNOME Shell-compatible DBus screenshot provider detection
- **AND** the delta states that a new durable ADR is not required unless ADR review identifies a hard-to-reverse architecture decision

#### Scenario: Carry source-overlay acceptance into later artifacts
- **GIVEN** the design and tasks for this change are created
- **WHEN** they consume this spec delta
- **THEN** they carry forward the strict portal and screenshot acceptance criteria
- **AND** they do not replace this spec delta with an implicit design-only note

#### Scenario: Preserve existing desktop-specific backends
- **GIVEN** source-overlay acceptance is recorded for doctor/report fixes
- **WHEN** future implementation applies the overlay to the target repo
- **THEN** existing GNOME, COSMIC, KWin, Hyprland, and i3 backend behavior remains out of scope unless directly affected by the diagnostics fix
- **AND** `x11-ewmh` remains a late fallback backend if a window backend is added later

### Requirement: Canonical X11 integration spec purpose metadata
The `x11-integration-contract` canonical specification MUST have a non-placeholder `## Purpose` section that describes the spec's role as the source of truth for X11/EWMH backend identity, source-overlay compatibility, and integration-contract constraints.

#### Scenario: Replace bootstrap purpose placeholder
- **GIVEN** `openspec/specs/x11-integration-contract/spec.md` is the canonical X11 integration contract specification
- **WHEN** the spec purpose metadata is inspected
- **THEN** the `## Purpose` section does not contain `TBD`
- **AND** it does not say it was created by archiving `bootstrap-codex-computer-use-x11`
- **AND** it describes the X11/EWMH backend and source-overlay integration contract

#### Scenario: Preserve X11 integration requirements
- **GIVEN** this maintenance change updates the canonical purpose metadata
- **WHEN** the canonical `x11-integration-contract` spec is compared before and after the change
- **THEN** existing normative requirements and scenarios remain semantically unchanged
- **AND** no Rust code, target-checkout write, or source-overlay behavior change is required

