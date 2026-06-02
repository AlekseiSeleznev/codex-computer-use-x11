## ADDED Requirements

### Requirement: Takeover settings row replaces bundled Any App surface
The Codex Desktop Linux `Settings -> Computer use` page MUST support a takeover row mode that presents `X11 Computer Use` as the active Any App Computer Use provider while preserving non-takeover bundled behavior.

#### Scenario: Takeover hides bundled Any App row
- **GIVEN** provider takeover mode is active for X11
- **AND** the X11 provider resolves from installed or available plugin data
- **WHEN** the user opens `Settings -> Computer use`
- **THEN** the Control section shows an `X11 Computer Use` row for the Any App provider surface
- **AND** it does not show a second active bundled `Any App` row for `computer-use`
- **AND** the `Google Chrome` row remains governed by its existing Chrome plugin lookup behavior

#### Scenario: Bundled mode remains available after rollback
- **GIVEN** provider takeover mode has been rolled back or is not configured
- **WHEN** the user opens `Settings -> Computer use`
- **THEN** the page uses the bundled `computer-use` `Any App` row decision from the unmodified baseline
- **AND** no X11 takeover alias remains in the settings row payload
- **AND** the page does not require `codex-computer-use-x11` to render the bundled row

### Requirement: Runtime diagnostic view for settings payload
The settings UI patch MUST expose a developer-readable diagnostic report for the Computer Use settings payload and row decisions when takeover diagnostics are enabled.

#### Scenario: Diagnostic report includes row decisions
- **GIVEN** takeover diagnostics are enabled
- **WHEN** the settings page computes Computer Use rows
- **THEN** the diagnostic report includes one row-decision record for the bundled provider candidate
- **AND** it includes one row-decision record for the X11 provider candidate
- **AND** each row-decision record identifies the source collection used for lookup, such as `installedPlugins`, `availablePlugins`, or `none`
- **AND** each row-decision record identifies whether the row was shown, hidden, disabled, or unavailable

#### Scenario: Diagnostics are off by default
- **GIVEN** no takeover diagnostic option is configured
- **WHEN** the settings page computes Computer Use rows
- **THEN** normal users do not see raw plugin payload diagnostics in the settings page
- **AND** no diagnostic file is written unless the installer or patch mode explicitly requests it
