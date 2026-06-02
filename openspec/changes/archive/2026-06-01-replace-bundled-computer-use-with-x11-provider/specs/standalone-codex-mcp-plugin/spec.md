## ADDED Requirements

### Requirement: Takeover preserves standalone plugin identity
X11 provider takeover MUST consume the existing standalone plugin identity and metadata without renaming the plugin, changing the MCP tool namespace, or rewriting bundled plugin ownership.

#### Scenario: Standalone plugin remains owned during takeover
- **GIVEN** the takeover installer or settings resolver selects the X11 provider
- **WHEN** it reads plugin metadata for `codex-computer-use-x11`
- **THEN** the plugin id remains `codex-computer-use-x11`
- **AND** the marketplace remains `codex-computer-use-x11`
- **AND** the MCP tools remain in the `x11_*` namespace
- **AND** no tool is renamed to an unqualified stock Computer Use tool as part of settings takeover

#### Scenario: Bundled marketplace paths are not rewritten
- **GIVEN** the local Codex plugin cache contains bundled `openai-bundled/computer-use` data
- **WHEN** X11 provider takeover is enabled
- **THEN** the installer does not overwrite `$CODEX_HOME/plugins/cache/openai-bundled/computer-use`
- **AND** it does not change bundled marketplace metadata to point at the X11 plugin
- **AND** rollback does not remove standalone plugin cache files unless the standalone plugin uninstall command is explicitly invoked
