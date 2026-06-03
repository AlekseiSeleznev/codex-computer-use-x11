## ADDED Requirements

### Requirement: Takeover install is rollback-first across delivery surfaces
Provider takeover installation MUST preserve standalone plugin identity and bundled fallback while applying takeover state through manifest-backed source overlay, optional live asset patching, and settings/provider resolver changes.

#### Scenario: Fresh takeover install activates X11 provider without masquerade
- **GIVEN** takeover mode is selected for `codex-computer-use-x11`
- **WHEN** fresh install applies provider takeover
- **THEN** the installed standalone plugin remains identified as `codex-computer-use-x11@codex-computer-use-x11`
- **AND** bundled `openai-bundled/computer-use` marketplace and cache paths are not rewritten as the X11 plugin
- **AND** takeover compatibility aliases are localized to the settings/provider resolver payload

#### Scenario: Takeover install records bundled fallback before-state
- **GIVEN** provider takeover will hide or disable the bundled Any App row
- **WHEN** the installer plans takeover state
- **THEN** it records non-secret bundled provider row facts needed for fallback diagnostics
- **AND** it records the exact source and live assets that will be patched
- **AND** rollback can restore bundled mode without deleting unrelated standalone plugin files

### Requirement: Takeover uninstall restores bundled mode from manifest
Provider takeover uninstall MUST restore the bundled Computer Use settings/provider mode from the manifest, MUST not remove unrelated standalone plugin files, and MUST explain drift or missing-manifest cases.

#### Scenario: Uninstall restores bundled settings mode
- **GIVEN** a manifest-backed takeover install is present and current files match installer after-state
- **WHEN** provider takeover uninstall runs
- **THEN** bundled Computer Use row behavior is restored from recorded before-state
- **AND** X11 takeover aliases are removed from the settings/provider resolver payload
- **AND** standalone `x11_*` MCP tools remain governed by standalone plugin install state rather than takeover rollback

#### Scenario: Missing manifest blocks blind takeover rollback
- **GIVEN** live or source takeover markers are present
- **AND** no matching rollback manifest can be found
- **WHEN** provider takeover uninstall runs
- **THEN** it reports a missing-manifest blocker
- **AND** it does not blindly delete marker-looking content or bundled assets
