## MODIFIED Requirements

### Requirement: User-local Codex plugin bundle layout
The installer MUST create a user-local Codex plugin bundle under an owned `codex-computer-use-x11` namespace and MUST NOT write to `/opt`, `openai-bundled`, or bundled `computer-use` cache paths. The installed bundle MUST include the current project binary, MCP manifest, plugin manifest, and project-owned display assets needed by Codex UI.

#### Scenario: Install owned plugin bundle files
- **GIVEN** `CODEX_HOME` points at an empty temporary Codex home
- **AND** an executable `codex-computer-use-x11` binary is available to install
- **WHEN** a developer runs `scripts/install-codex-plugin.sh`
- **THEN** the installer creates an owned cache entry under `$CODEX_HOME/plugins/cache/codex-computer-use-x11/codex-computer-use-x11/<version>/`
- **AND** that entry contains `.codex-plugin/plugin.json`, `.mcp.json`, `bin/codex-computer-use-x11`, and `assets/app-icon.png`
- **AND** `.mcp.json` starts the copied binary with argument `mcp`
- **AND** `latest` points to the installed version

#### Scenario: Write owned marketplace metadata
- **GIVEN** `CODEX_HOME` points at a temporary Codex home
- **WHEN** a developer runs `scripts/install-codex-plugin.sh`
- **THEN** the installer writes an owned local marketplace root for `codex-computer-use-x11`
- **AND** the root contains `.agents/plugins/marketplace.json`
- **AND** the marketplace JSON contains exactly the owned plugin entry for `codex-computer-use-x11`
- **AND** the marketplace interface display name is `X11 Computer Use`
- **AND** the marketplace plugin path resolves to the owned cache `latest` entry
- **AND** no marketplace metadata under `openai-bundled` is changed

### Requirement: Generated plugin manifest metadata accuracy
The installer MUST generate standalone plugin metadata whose repository links, author/developer fields, user-facing descriptions, and display assets match the current project repository and full standalone `x11_*` MCP tool surface.

#### Scenario: Manifest identity matches the project
- **GIVEN** the standalone plugin installer generates `.codex-plugin/plugin.json`
- **WHEN** Codex reads the plugin metadata for the plugin details page
- **THEN** the manifest name is `codex-computer-use-x11`
- **AND** the interface display name is `X11 Computer Use`
- **AND** the author name is `AlekseiSeleznev`
- **AND** the interface developer name is `AlekseiSeleznev`
- **AND** the manifest homepage and interface website URL point to `https://github.com/AlekseiSeleznev/codex-computer-use-x11`
- **AND** no stale `AlekseiSelin` repository owner appears anywhere in the generated plugin manifest

#### Scenario: Manifest legal links are not invented
- **GIVEN** the project does not contain project-owned Privacy Policy or Terms of Service documents
- **WHEN** the installer generates plugin interface metadata
- **THEN** it omits `privacyPolicyURL`
- **AND** it omits `termsOfServiceURL`
- **AND** it does not point those fields at unrelated OpenAI, GitHub, or third-party policies

#### Scenario: Manifest description covers all exposed standalone tools
- **GIVEN** the MCP server exposes the standalone tools documented by `tools/list`
- **WHEN** the installer generates plugin interface metadata
- **THEN** the long description names the supported tool groups for doctor, window listing/focus, keyboard input, pointer actions, accessibility tree, app state, and target-window context
- **AND** the description does not imply that only the first six `x11_*` tools are available

#### Scenario: Manifest prompts guide users to representative current tools
- **GIVEN** the generated plugin manifest includes default prompts
- **WHEN** a user browses the plugin metadata
- **THEN** the prompts mention representative inspection and action paths from the current standalone tool surface
- **AND** the prompts include `x11_get_app_state` and `x11_target_window`
- **AND** the prompts remain within the project-owned `x11_*` namespace

#### Scenario: Manifest references project-owned logo
- **GIVEN** the installer generates `.codex-plugin/plugin.json`
- **WHEN** Codex renders the plugin card or details page
- **THEN** the interface logo points at `./assets/app-icon.png`
- **AND** the referenced file exists inside the installed plugin bundle
- **AND** the logo is copied from a tracked project-owned asset rather than a bundled OpenAI plugin asset

## ADDED Requirements

### Requirement: Standalone MCP process hydrates desktop environment
The standalone MCP server MUST recover the local graphical session environment needed for X11/EWMH checks when Codex starts the plugin process with a sparse environment. Hydration MUST use local non-secret environment sources only, MUST NOT print secret values, and MUST preserve explicit caller-provided environment variables.

#### Scenario: MCP doctor recovers DISPLAY from systemd user environment
- **GIVEN** the MCP server process starts without `DISPLAY`
- **AND** the user systemd manager environment contains `DISPLAY=:0`, `XDG_RUNTIME_DIR`, `DBUS_SESSION_BUS_ADDRESS`, and `XAUTHORITY`
- **WHEN** an MCP client calls `x11_doctor`
- **THEN** the doctor report evaluates the hydrated desktop environment
- **AND** `environment.display_present` is true
- **AND** X11/EWMH checks are not blocked solely because the original process environment lacked `DISPLAY`
- **AND** no secret environment values are serialized into the MCP response

#### Scenario: MCP preserves explicit caller environment
- **GIVEN** the MCP server process starts with `DISPLAY=:99`
- **AND** another local desktop environment source contains `DISPLAY=:0`
- **WHEN** an MCP client calls `x11_doctor`
- **THEN** the doctor report continues to use the explicit `DISPLAY=:99`
- **AND** hydration does not overwrite caller-provided non-empty desktop variables

#### Scenario: MCP reports missing desktop environment when hydration cannot help
- **GIVEN** the MCP server process starts without `DISPLAY`
- **AND** no local systemd or process environment source contains a graphical display
- **WHEN** an MCP client calls `x11_doctor`
- **THEN** the doctor report remains valid JSON
- **AND** readiness blockers explain that `DISPLAY` is unavailable
- **AND** the server does not panic or write non-JSON diagnostic noise to stdout

### Requirement: Plugin smoke validates current installed tool surface and metadata
The plugin e2e smoke MUST detect stale user-local installations whose binary, metadata, marketplace display name, or icon do not match the current repository contract.

#### Scenario: Smoke rejects stale six-tool install
- **GIVEN** a user-local plugin cache contains a stale `codex-computer-use-x11` binary that exposes only `x11_doctor`, `x11_list_windows`, `x11_focused_window`, `x11_focus_window`, `x11_type_text`, and `x11_press_key`
- **WHEN** `scripts/e2e/codex-plugin-smoke.sh --fake --codex-home <dir> --no-auto-install` validates that install
- **THEN** the smoke exits non-zero
- **AND** the evidence identifies the missing current `x11_*` tools
- **AND** the failure points at the selected log directory

#### Scenario: Smoke validates UI metadata
- **GIVEN** a fake `CODEX_HOME` populated by the current project-owned plugin installer
- **WHEN** `scripts/e2e/codex-plugin-smoke.sh --fake --codex-home <dir>` validates the install
- **THEN** the smoke verifies the plugin manifest identity, website URL, developer name, missing privacy/terms links, logo path, and marketplace display name
- **AND** the evidence records those checks without including secret values
