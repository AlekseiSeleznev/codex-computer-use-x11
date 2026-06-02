## ADDED Requirements

### Requirement: Docs explain safe app-state screenshot evidence
Troubleshooting, E2E harness, and release documentation MUST explain that `get-app-state --json` no longer emits inline screenshot blobs by default. The docs MUST show how to request screenshot artifact paths, how `--no-screenshot` behaves, and that any retained inline mode is explicit opt-in and unsafe for durable evidence logs.

#### Scenario: Operator learns path-only app-state behavior
- **GIVEN** a developer reads the E2E harness or troubleshooting docs
- **WHEN** they look for `get-app-state` screenshot behavior
- **THEN** the docs state that default JSON contains no `data:image` screenshot blob
- **AND** the docs show the supported screenshot output path option or generated artifact path behavior
- **AND** the docs explain that `--no-screenshot` keeps window/accessibility/capability diagnostics usable without screenshot capture
- **AND** any inline screenshot opt-in is labeled unsafe for evidence logs

### Requirement: Docs explain controlled real-live fixture retests
The project docs MUST describe how to run controlled real-live Cinnamon/X11 fixture retests safely. The docs MUST distinguish real-live controlled fixture evidence from fake/fake-live evidence, describe fixture metadata and cleanup, and warn that fixture-dependent operations must never target real user applications as fallback.

#### Scenario: Operator runs controlled real-live retest safely
- **GIVEN** a developer follows the E2E harness documentation for an industrial real-live retest
- **WHEN** they start the controlled fixture runner
- **THEN** the docs identify expected metadata files, fixture roles, target selection rules, cleanup behavior, and evidence directory layout
- **AND** the docs state that fake or fake-live fixtures are not primary real-live evidence
- **AND** the docs warn that keyboard, pointer, screenshot, app-state, target, and overlay checks require controlled fixture windows

### Requirement: Docs preserve NO_AT_BRIDGE remediation guidance
Troubleshooting docs MUST preserve and update the Cinnamon/X11 `NO_AT_BRIDGE=1` diagnostic guidance. They MUST explain that the disabling contract is presence-based for common GTK/ATK bridge integrations, that controlled GTK fixture processes should remove `NO_AT_BRIDGE`, and that diagnostic repair should not mutate global user environment silently.

#### Scenario: Operator fixes bridge-disabled fixture diagnostics
- **GIVEN** a diagnostic report shows `NO_AT_BRIDGE=1` or an AT-SPI bridge-disabled outcome
- **WHEN** the operator reads troubleshooting docs
- **THEN** the docs explain to remove `NO_AT_BRIDGE` from the controlled GTK fixture/application process environment
- **AND** the docs say to restart the affected fixture/Codex session as needed
- **AND** the docs recommend rerunning controlled GTK fixture self-test or real-live fixture smoke before claiming AT-SPI pass evidence
