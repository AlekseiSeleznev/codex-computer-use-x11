# standalone-codex-mcp-plugin Specification

## Purpose
This specification defines the standalone Codex MCP plugin and user-local installer contract for `codex-computer-use-x11`, including the `x11_*` MCP tool surface, owned plugin namespace, reversible marketplace/cache/config installation, direct MCP smoke path, and rollback safety.
## Requirements
### Requirement: Standalone MCP stdio server mode
The CLI MUST provide `codex-computer-use-x11 mcp` as a local stdio MCP server mode that communicates with JSON-RPC over stdin/stdout, exposes only project-owned `x11_*` tools in deterministic order, and keeps target-window group state scoped to that MCP server process.

#### Scenario: List standalone x11 tools
- **GIVEN** the `codex-computer-use-x11` binary is built
- **WHEN** an MCP client starts `codex-computer-use-x11 mcp`, initializes the server, and sends a `tools/list` request
- **THEN** the server returns a valid JSON-RPC response
- **AND** the response lists `x11_doctor`, `x11_list_windows`, `x11_focused_window`, `x11_focus_window`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, `x11_drag`, `x11_accessibility_tree`, `x11_get_app_state`, `x11_target_window`, `x11_release_window`, and `x11_target_context` in deterministic order
- **AND** every tool includes a description and JSON input schema
- **AND** no tool is named `doctor`, `list_windows`, `focused_window`, `activate_window`, `type_text`, `press_key`, `click`, `scroll`, `drag`, `accessibility_tree`, `get_app_state`, `target_window`, `release_window`, `target_context`, or `computer-use`

#### Scenario: Keep existing CLI usage distinct from MCP mode
- **GIVEN** a developer invokes an existing supported JSON command such as `doctor --json`
- **WHEN** the CLI handles the command
- **THEN** the command behavior remains the existing one-shot CLI behavior
- **AND** it does not require MCP initialization
- **AND** unsupported command usage still exits non-zero with stderr rather than starting the MCP server accidentally

#### Scenario: Keep MCP target state process-scoped
- **GIVEN** one MCP client process has targeted window `0x2`
- **WHEN** a separate `codex-computer-use-x11 mcp` process starts and receives `x11_target_context`
- **THEN** the second process returns an empty target context unless it targets its own windows
- **AND** it does not read or mutate another MCP process's in-memory target group state

### Requirement: x11 MCP tool calls wrap existing JSON capabilities
The MCP server MUST implement tool calls by reusing the standalone project's existing JSON report builders for doctor, window listing, focused-window, verified focus, verified keyboard input, verified pointer input, AT-SPI window-correlation behavior, app-state composition, and target-window group state. Tool results MUST be valid MCP tool results whose text content contains one JSON object from the underlying capability.

#### Scenario: Call x11_doctor
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_doctor` with no arguments
- **THEN** the server returns a successful MCP tool result
- **AND** the first text content item is valid JSON
- **AND** that JSON includes `project` equal to `codex-computer-use-x11`
- **AND** that JSON includes `backend` equal to `x11-ewmh`
- **AND** the report shape matches the `doctor-cli` capability instead of inventing a second doctor schema

#### Scenario: Call x11_list_windows
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_list_windows` with no arguments
- **THEN** the server returns an MCP tool result whose text content is a valid `list-windows --json` report
- **AND** no external credentials are required
- **AND** headless or degraded X11 state is reported in the JSON diagnostics instead of panicking

#### Scenario: Call x11_focused_window
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_focused_window` with no arguments
- **THEN** the server returns an MCP tool result whose text content is a valid `focused-window --json` report
- **AND** no-active or active-not-in-list state remains structured diagnostic JSON

#### Scenario: Call x11_focus_window with normalized id
- **GIVEN** an MCP client has initialized the standalone server
- **AND** a window id can be represented as decimal, short hexadecimal, or zero-padded hexadecimal
- **WHEN** the client calls `x11_focus_window` with `window_id` set to one of those forms
- **THEN** the server normalizes the id through the shared X11 id normalizer
- **AND** the tool result contains the same success or `FocusNotVerified` JSON semantics as `focus-window --window-id <id> --json`
- **AND** focus success is never reported unless a fresh active-window lookup verifies the requested id

#### Scenario: Reject missing focus argument as a tool error
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_focus_window` without a `window_id` argument
- **THEN** the server returns an MCP tool result with `isError` true
- **AND** the result explains the missing `window_id` argument
- **AND** no activation command is attempted

#### Scenario: Call x11_accessibility_tree with window id
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_accessibility_tree` with `window_id` set to a decimal or hexadecimal X11 id
- **THEN** the server normalizes the id through the shared X11 id normalizer
- **AND** the tool result text content is a valid `accessibility-tree --window-id <id> --json` report
- **AND** `isError` is false only when the report has `success` true and a confident correlation
- **AND** ambiguous, missing-window, or AT-SPI unavailable states are returned as structured JSON tool errors

#### Scenario: Call x11_get_app_state with selectors
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_get_app_state` with `window_id`, `pid`, `wm_class`, or `title` selector arguments
- **THEN** the server normalizes any `window_id` through the shared X11 id normalizer
- **AND** the tool result text content is a valid `get-app-state --json` report
- **AND** `isError` remains false when only a composed layer is degraded, such as missing window target, screenshot failure, or unavailable AT-SPI
- **AND** the JSON fields report those layer failures through `window_error`, `screenshot_error`, or `accessibility_error`

#### Scenario: Call x11_get_app_state without screenshot
- **GIVEN** an MCP client passes `include_screenshot` false to `x11_get_app_state`
- **WHEN** the server builds the app-state report
- **THEN** the result JSON has `screenshot` null
- **AND** `screenshot_error` null
- **AND** the rest of the app-state diagnostics are still returned

#### Scenario: Reject malformed app-state arguments as a tool error
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** it calls `x11_get_app_state` with a non-string/non-number `window_id` or invalid argument type
- **THEN** the server returns an MCP tool result with `isError` true
- **AND** no X11, screenshot, or AT-SPI external command is attempted for that malformed request

#### Scenario: Call x11_target_window
- **GIVEN** an MCP client has initialized the standalone server
- **AND** the current X11 listing contains window `0x2`
- **WHEN** the client calls `x11_target_window` with `window_id` set to `0x2`, `group` set to `data-entry`, `color` set to `green`, and `overlay` set to false
- **THEN** the server returns a successful MCP tool result
- **AND** the first text content item is valid target-window JSON
- **AND** that JSON records window `2` in group `data-entry`
- **AND** later `x11_target_context` in the same MCP process includes that saved target

#### Scenario: Call x11_release_window
- **GIVEN** an MCP client has targeted window `0x2` in the current MCP process
- **WHEN** the client calls `x11_release_window` with `window_id` set to `0x2`
- **THEN** the server returns a successful MCP tool result
- **AND** the result JSON states that the target was released
- **AND** later `x11_target_context` in the same MCP process no longer includes window `2`

#### Scenario: Call x11_target_context
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_target_context` with no arguments
- **THEN** the server returns a successful MCP tool result
- **AND** the result text content is valid target-context JSON
- **AND** stale saved targets are validated against the current listing before the context is returned

#### Scenario: Reject malformed target-window arguments as a tool error
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** it calls `x11_target_window` with a non-string/non-number `window_id` or invalid `color`
- **THEN** the server returns an MCP tool result with `isError` true
- **AND** no target state is saved for the malformed request

### Requirement: MCP protocol robustness
The MCP server MUST handle common JSON-RPC control flow robustly enough for Codex smoke testing: initialize, initialized notification, tools/list, tools/call, parse errors, and unknown methods.

#### Scenario: Initialize returns server metadata and tools capability
- **GIVEN** an MCP client starts the server over stdio
- **WHEN** the client sends an `initialize` request
- **THEN** the server responds with JSON-RPC `result`
- **AND** the result includes server info for `codex-computer-use-x11`
- **AND** the result declares a tools capability

#### Scenario: Ignore initialized notification without response
- **GIVEN** the server has received a valid initialize request
- **WHEN** the client sends a `notifications/initialized` notification without an id
- **THEN** the server does not emit a JSON-RPC response for that notification
- **AND** it continues to answer subsequent `tools/list` and `tools/call` requests

#### Scenario: Return JSON-RPC errors for malformed or unknown requests
- **GIVEN** an MCP client sends malformed JSON or an unsupported method
- **WHEN** the server processes the input line
- **THEN** it returns a JSON-RPC error response when an id is available or appropriate
- **AND** it keeps protocol errors separate from successful tool result JSON
- **AND** it does not print diagnostic noise to stdout outside JSON-RPC messages

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

### Requirement: Installer dry-run and idempotence
The installer MUST support `--dry-run` without filesystem writes and MUST be idempotent when run repeatedly for the same version.

#### Scenario: Dry run does not write files
- **GIVEN** `CODEX_HOME` points at an empty temporary directory
- **WHEN** a developer runs `scripts/install-codex-plugin.sh --dry-run`
- **THEN** the command exits successfully
- **AND** it prints the planned owned cache, marketplace, and config updates
- **AND** it does not create `$CODEX_HOME/plugins`, marketplace files, plugin files, or config entries

#### Scenario: Repeated install is idempotent
- **GIVEN** a temporary Codex home has already been installed by `scripts/install-codex-plugin.sh`
- **WHEN** the developer runs `scripts/install-codex-plugin.sh` again for the same version and binary
- **THEN** the command exits successfully
- **AND** the owned cache and marketplace metadata remain valid
- **AND** the config file contains only one enabled plugin section for `codex-computer-use-x11@codex-computer-use-x11`
- **AND** the config file contains only one marketplace section for `codex-computer-use-x11`

### Requirement: Safe uninstall of owned plugin files
The uninstall script MUST remove only files and config sections owned by the `codex-computer-use-x11` namespace and MUST preserve bundled, curated, primary-runtime, and unrelated local plugins.

#### Scenario: Uninstall removes owned files only
- **GIVEN** a temporary Codex home contains an installed `codex-computer-use-x11` plugin
- **AND** it also contains unrelated plugin cache, marketplace, and config entries
- **WHEN** a developer runs `scripts/uninstall-codex-plugin.sh`
- **THEN** the owned `codex-computer-use-x11` plugin cache entry is removed
- **AND** the owned local marketplace root is removed
- **AND** the owned config sections are removed
- **AND** unrelated plugin files and config sections remain unchanged

#### Scenario: Uninstall dry run does not write files
- **GIVEN** a temporary Codex home contains an installed `codex-computer-use-x11` plugin
- **WHEN** a developer runs `scripts/uninstall-codex-plugin.sh --dry-run`
- **THEN** the command exits successfully
- **AND** it prints the planned owned removals
- **AND** installed files and config sections remain present

#### Scenario: Uninstall is safe when plugin is absent
- **GIVEN** `CODEX_HOME` points at a temporary Codex home without the standalone plugin
- **WHEN** a developer runs `scripts/uninstall-codex-plugin.sh`
- **THEN** the command exits successfully
- **AND** unrelated files are not removed
- **AND** the result explains that there was no owned install to remove or that removal was already complete

### Requirement: Plugin verification guidance
The project MUST document and verify both a direct MCP stdio smoke path and a Codex plugin refresh path so progress is not blocked if the host Codex app requires restart or lazy tool loading.

#### Scenario: Verify direct MCP stdio without installing into real HOME
- **GIVEN** the project is checked out on a development machine
- **WHEN** a developer runs the documented MCP smoke command against `codex-computer-use-x11 mcp`
- **THEN** the command proves that `tools/list` exposes the `x11_*` tools
- **AND** at least `x11_doctor` can be called without modifying real `HOME`

#### Scenario: Verify live user-local install when approved
- **GIVEN** the developer explicitly allows user-local `$CODEX_HOME` writes
- **WHEN** the installer is run without `--dry-run`
- **THEN** the plugin installs without sudo
- **AND** the project records either successful visibility/call evidence for `x11_*` tools after Codex refresh or exact restart/inspection instructions when the current process cannot load new plugin tools dynamically
- **AND** uninstall instructions are available for rollback

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

### Requirement: Fresh install activates standalone plugin and accessibility baseline
The user-local standalone installer MUST install the owned plugin bundle and, when requested for the Cinnamon/X11 baseline, MUST safely activate required accessibility environment state without enabling screen readers or overwriting unrelated user settings.

#### Scenario: Install records plugin and activation before-state
- **GIVEN** a user runs the standalone installer in fresh-install mode
- **WHEN** the installer plans owned plugin files and accessibility activation changes
- **THEN** it records before-state for every owned `$CODEX_HOME` plugin cache, marketplace, and config path it may change
- **AND** it records before-state for relevant user systemd and dbus activation environment variables
- **AND** it records before-state for `org.gnome.desktop.interface toolkit-accessibility`
- **AND** the manifest marks whether each value was changed by the installer or was already acceptable

#### Scenario: Install neutralizes disabling bridge environment only when safe
- **GIVEN** the user activation environment contains `NO_AT_BRIDGE=1`
- **WHEN** fresh install activates the Cinnamon/X11 accessibility baseline
- **THEN** the installer records the original value in the backup manifest
- **AND** it removes or neutralizes `NO_AT_BRIDGE=1` only for user activation environments it owns or can safely update
- **AND** it reports any environment it cannot safely update as a blocker or degraded setup item

#### Scenario: Install does not enable Orca implicitly
- **GIVEN** fresh install activates AT-SPI and toolkit accessibility
- **WHEN** the installer applies accessibility setup
- **THEN** it MUST NOT enable Orca or any screen-reader autostart setting
- **AND** it MUST NOT require screen-reader activation for the doctor AT-SPI probe to pass

### Requirement: Standalone uninstall restores manifest-owned user state
The standalone uninstaller MUST restore only installer-owned plugin and accessibility activation changes from the manifest, MUST be idempotent for absent or partial installs, and MUST support dry-run and report-json output.

#### Scenario: Uninstall restores changed activation environment
- **GIVEN** a backup manifest records that the installer changed `NO_AT_BRIDGE`, `GTK_MODULES`, or `QT_ACCESSIBILITY`
- **WHEN** a user runs uninstall with write mode
- **THEN** the uninstaller restores the recorded previous values or absence state
- **AND** it leaves values unchanged when the manifest says they were already acceptable before install

#### Scenario: Dry-run reports without mutation
- **GIVEN** a manifest-backed install is present
- **WHEN** a user runs standalone uninstall with `--dry-run --report-json`
- **THEN** stdout contains one JSON report describing planned removals and restorations
- **AND** no `$CODEX_HOME`, gsettings, or activation-environment state is changed

#### Scenario: Partial install can be rolled back
- **GIVEN** installation failed after writing some plugin paths and recording a partial manifest
- **WHEN** a user runs standalone uninstall
- **THEN** the uninstaller removes or restores only manifest-owned changes that were completed
- **AND** it reports missing or already-restored items as idempotent outcomes rather than failures

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

