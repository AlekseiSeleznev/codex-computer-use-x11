## MODIFIED Requirements

### Requirement: Troubleshooting covers degraded layers without fabricating success
The project MUST provide troubleshooting documentation for Cinnamon/X11 readiness, missing X11 commands, plugin installation, source-overlay drift, screenshot/AT-SPI degraded layers, and e2e evidence failures. Troubleshooting MUST distinguish deterministic fake/dry-run checks from optional live checks, and MUST NOT present RemoteDesktop portal or Wayland remediation as part of the current standalone `x11-ewmh` plugin readiness path.

#### Scenario: Doctor and dependency troubleshooting is X11-scoped
- **GIVEN** `cargo run -- doctor --json` reports degraded or unavailable X11-baseline capabilities
- **WHEN** a user reads troubleshooting documentation
- **THEN** the docs explain how to inspect X11 session variables, `wmctrl`, `xprop`, `xdotool`, `ydotool`, screenshot provider, and AT-SPI separately
- **AND** the docs recommend fake/dry-run evidence when live desktop capabilities are unavailable
- **AND** the docs do not tell users to fix RemoteDesktop portal or Wayland capabilities as a readiness step for the standalone X11 plugin

#### Scenario: Source-overlay drift troubleshooting preserves target safety
- **GIVEN** `scripts/status-codex-source-overlay.sh` reports `state=drifted`
- **WHEN** a user reads troubleshooting documentation
- **THEN** the docs explain that drift means owned markers, generated backend content, anchors, or metadata do not match expectations
- **AND** the docs direct the user to inspect target git status before reinstalling or uninstalling
- **AND** the docs do not recommend overwriting unowned target code or native X11 backend files blindly

### Requirement: Documentation explains X11-only production readiness semantics
README, troubleshooting, and retest documentation MUST explain PASS, DEGRADED, FAIL, doctor readiness, controlled-fixture evidence, and Wayland out-of-scope product status for the Cinnamon/X11 baseline. Documentation MUST state that RemoteDesktop portal and Wayland support are not current standalone plugin readiness diagnostics and that their absence does not degrade the `x11-ewmh` doctor baseline.

#### Scenario: Reader can interpret pass and degraded rows
- **GIVEN** a developer opens the production-readiness or troubleshooting documentation
- **WHEN** they read the capability matrix guidance
- **THEN** PASS means the capability has concrete evidence for the stated delivery path and fixture mode
- **AND** DEGRADED means a documented X11-baseline limitation with a reason category and evidence path, not hidden success
- **AND** FAIL means a code, safety, cleanup, or integrity issue that blocks production-readiness claims
- **AND** missing RemoteDesktop portal or Wayland support is not described as a DEGRADED doctor-readiness row for the standalone X11 plugin

#### Scenario: Reader can run safe full retest
- **GIVEN** a developer wants to retest the installed plugin
- **WHEN** they follow the documented safe full retest instructions
- **THEN** the instructions avoid `.secrets.local.env` and external credentials
- **AND** they identify fake smoke, controlled live fixture smoke, optional metadata-only smoke, doctor JSON validation, and matrix validation commands
- **AND** they warn that input/pointer/overlay checks must target controlled fixtures only

#### Scenario: Wayland status is unambiguous without doctor noise
- **GIVEN** a reader wants to understand Wayland or portal scope
- **WHEN** they consult README, troubleshooting, or retest documentation
- **THEN** the documentation states that Wayland support and portal-required runtime paths are outside the current standalone X11 plugin scope
- **AND** the documentation does not describe RemoteDesktop portal absence or `WAYLAND_DISPLAY` presence as current `doctor --json` readiness degraded reasons, optional enrichments, blockers, or next-step recommendations
- **AND** the documentation directs users to validate the X11 `x11-ewmh` baseline rather than fixing RemoteDesktop portal or Wayland for this plugin
