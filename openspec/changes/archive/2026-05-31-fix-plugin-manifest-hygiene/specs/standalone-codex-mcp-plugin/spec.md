## ADDED Requirements

### Requirement: Generated plugin manifest metadata accuracy
The installer MUST generate standalone plugin metadata whose repository links and user-facing descriptions match the current project repository and full standalone `x11_*` MCP tool surface.

#### Scenario: Manifest homepage points at the actual repository
- **GIVEN** the standalone plugin installer generates `.codex-plugin/plugin.json`
- **WHEN** a developer or Codex reads the generated `homepage` value
- **THEN** it points to the actual `AlekseiSeleznev/codex-computer-use-x11` repository URL
- **AND** it does not point at a misspelled or stale repository owner

#### Scenario: Manifest description covers all exposed standalone tools
- **GIVEN** the MCP server exposes the standalone tools documented by `tools/list`
- **WHEN** the installer generates plugin interface metadata
- **THEN** the long description names the supported tool groups for doctor, window listing/focus, keyboard input, pointer actions, accessibility tree, app state, and target-window context
- **AND** the description does not imply that only the first six `x11_*` tools are available

#### Scenario: Manifest prompts guide users to representative current tools
- **GIVEN** the generated plugin manifest includes default prompts
- **WHEN** a user browses the plugin metadata
- **THEN** the prompts mention representative inspection and action paths from the current standalone tool surface
- **AND** the prompts remain within the project-owned `x11_*` namespace
