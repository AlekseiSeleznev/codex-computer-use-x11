# codex-computer-use-provider-takeover Specification

## Purpose
Defines the Codex Desktop Linux provider-takeover contract for presenting the standalone X11 Computer Use provider as the active Any App surface while preserving diagnosable bundled fallback and rollback behavior.
## Requirements
### Requirement: Baseline bundled Computer Use diagnostics
Codex Desktop Linux provider-takeover work MUST make the baseline bundled Computer Use settings state diagnosable before replacing any row. The diagnostic output MUST include the non-secret runtime facts needed to explain whether the bundled `Any App` row is shown, hidden, unavailable, or gated.

#### Scenario: Record bundled baseline payload
- **GIVEN** the Computer Use settings page receives plugin and availability data at runtime
- **WHEN** diagnostics are enabled for provider takeover investigation
- **THEN** the diagnostic payload records the plugin ids and marketplace names from `availablePlugins`
- **AND** it records the plugin ids and marketplace names from `installedPlugins`
- **AND** it records the non-secret `computerUseAvailability` fields used by the settings page
- **AND** it records the row decision for the bundled `computer-use` provider
- **AND** it does not record secret values, auth tokens, private URLs, or credential material

#### Scenario: Explain absent bundled Any App row
- **GIVEN** the bundled `Any App` row is not rendered on Linux
- **WHEN** a developer inspects the provider diagnostic report
- **THEN** the report identifies whether the absence came from plugin lookup, `computerUseAvailability`, host/platform gating, feature rollout gating, or settings row shape drift
- **AND** the report includes enough non-secret data to reproduce the decision in a fixture test

### Requirement: X11 takeover provider resolution
When configured with provider `x11` and mode `takeover`, Codex Desktop Linux MUST resolve `codex-computer-use-x11` as the active Computer Use provider for the settings UI and MUST hide or disable the bundled `computer-use` settings row for that provider surface.

#### Scenario: Choose installed X11 provider in takeover mode
- **GIVEN** takeover mode is configured with `--provider x11 --mode takeover`
- **AND** `installedPlugins` includes `codex-computer-use-x11@codex-computer-use-x11`
- **WHEN** the Computer Use settings page resolves provider rows
- **THEN** the active Computer Use provider row is backed by `codex-computer-use-x11`
- **AND** the bundled `computer-use` `Any App` row is not shown as the active provider row
- **AND** the row decision records that takeover mode selected the installed X11 provider

#### Scenario: Fall back to available X11 provider before reporting unavailable
- **GIVEN** takeover mode is configured with `--provider x11 --mode takeover`
- **AND** `installedPlugins` does not include `codex-computer-use-x11@codex-computer-use-x11`
- **AND** `availablePlugins` includes a `codex-computer-use-x11` plugin entry
- **WHEN** the Computer Use settings page resolves provider rows
- **THEN** the active Computer Use provider row is backed by the available X11 plugin entry
- **AND** the row uses the normal plugin install/enable control flow
- **AND** the row decision records the installed-first fallback path

#### Scenario: Report missing X11 provider instead of silently falling back
- **GIVEN** takeover mode is configured with `--provider x11 --mode takeover`
- **AND** neither `installedPlugins` nor `availablePlugins` contains `codex-computer-use-x11`
- **WHEN** the Computer Use settings page resolves provider rows
- **THEN** the bundled `computer-use` row remains hidden or disabled for takeover mode
- **AND** the X11 provider row shows a clear unavailable or diagnostic state
- **AND** the page does not silently fall back to showing only the Google Chrome row

### Requirement: Localized compatibility shim
Any compatibility alias needed for takeover MUST be localized to the settings/provider resolver layer and MUST NOT globally rename, rewrite, or publish the standalone X11 plugin as the bundled `computer-use` plugin.

#### Scenario: Compatibility alias stays in provider resolver
- **GIVEN** the provider resolver needs an alias so stock Computer Use UI components can treat the X11 provider as the active Any App provider
- **WHEN** takeover mode is applied
- **THEN** the alias is created only inside the settings/provider row decision or component payload
- **AND** the plugin catalog still identifies the standalone plugin as `codex-computer-use-x11@codex-computer-use-x11`
- **AND** no bundled marketplace entry or cache path is rewritten to point at the X11 plugin

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

