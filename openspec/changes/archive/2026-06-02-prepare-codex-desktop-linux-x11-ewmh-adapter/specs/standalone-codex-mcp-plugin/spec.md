## ADDED Requirements

### Requirement: Release bundle metadata stays installer-compatible
The release packaging path MUST reuse or match the standalone installer plugin bundle contract so that packaged `.mcp.json`, plugin manifest metadata, icon asset, and binary layout remain consistent with user-local installation.

#### Scenario: Packaged bundle and installer metadata agree on MCP command
- **GIVEN** a release artifact has been produced
- **AND** a user-local installer dry run can describe the standalone plugin bundle
- **WHEN** tests compare the packaged `.mcp.json` with the installer contract
- **THEN** both identify server `codex-computer-use-x11`
- **AND** both use command `./bin/codex-computer-use-x11`
- **AND** both use args `["mcp"]`
- **AND** both use cwd `.`

#### Scenario: Packaged bundle preserves standalone namespace
- **GIVEN** a release artifact has been produced
- **WHEN** tests inspect the packaged plugin manifest
- **THEN** the plugin name is `codex-computer-use-x11`
- **AND** the interface display name is `X11 Computer Use`
- **AND** the manifest exposes the standalone plugin as a separate namespaced plugin
- **AND** it does not rename the plugin to `computer-use`
- **AND** it does not require replacing the bundled `computer-use` plugin
