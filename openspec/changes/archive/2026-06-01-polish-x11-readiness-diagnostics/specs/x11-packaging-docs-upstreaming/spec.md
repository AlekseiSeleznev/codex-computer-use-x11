## ADDED Requirements

### Requirement: Documentation explains X11-only production readiness semantics
README, troubleshooting, and retest documentation MUST explain PASS, DEGRADED, FAIL, doctor readiness, controlled-fixture evidence, and Wayland out-of-scope status for the Cinnamon/X11 baseline.

#### Scenario: Reader can interpret pass and degraded rows
- **GIVEN** a developer opens the production-readiness or troubleshooting documentation
- **WHEN** they read the capability matrix guidance
- **THEN** PASS means the capability has concrete evidence for the stated delivery path and fixture mode
- **AND** DEGRADED means a documented limitation with a reason category and evidence path, not hidden success
- **AND** FAIL means a code, safety, cleanup, or integrity issue that blocks production-readiness claims

#### Scenario: Reader can run safe full retest
- **GIVEN** a developer wants to retest the installed plugin
- **WHEN** they follow the documented safe full retest instructions
- **THEN** the instructions avoid `.secrets.local.env` and external credentials
- **AND** they identify fake smoke, controlled live fixture smoke, optional metadata-only smoke, doctor JSON validation, and matrix validation commands
- **AND** they warn that input/pointer/overlay checks must target controlled fixtures only

#### Scenario: Wayland status is unambiguous
- **GIVEN** a reader sees Wayland or portal degraded diagnostics
- **WHEN** they consult troubleshooting documentation
- **THEN** the documentation states that Wayland support and portal-required runtime paths are outside the current X11-only scope
- **AND** RemoteDesktop portal facts may be used as diagnostics only
- **AND** absence of those paths is not a blocker for the Cinnamon/X11 `x11-ewmh` baseline
