## ADDED Requirements

### Requirement: Standalone MCP stdio server mode
The CLI MUST provide `codex-computer-use-x11 mcp` as a local stdio MCP server mode that communicates with JSON-RPC over stdin/stdout and exposes only project-owned `x11_*` tools in deterministic order.

#### Scenario: List standalone x11 tools
- **GIVEN** the `codex-computer-use-x11` binary is built
- **WHEN** an MCP client starts `codex-computer-use-x11 mcp`, initializes the server, and sends a `tools/list` request
- **THEN** the server returns a valid JSON-RPC response
- **AND** the response lists `x11_doctor`, `x11_list_windows`, `x11_focused_window`, and `x11_focus_window` in deterministic order
- **AND** every tool includes a description and JSON input schema
- **AND** no tool is named `doctor`, `list_windows`, `focused_window`, `activate_window`, or `computer-use`

#### Scenario: Keep existing CLI usage distinct from MCP mode
- **GIVEN** a developer invokes an existing supported JSON command such as `doctor --json`
- **WHEN** the CLI handles the command
- **THEN** the command behavior remains the existing one-shot CLI behavior
- **AND** it does not require MCP initialization
- **AND** unsupported command usage still exits non-zero with stderr rather than starting the MCP server accidentally

### Requirement: x11 MCP tool calls wrap existing JSON capabilities
The MCP server MUST implement tool calls by reusing the standalone project's existing JSON report builders for doctor, window listing, focused-window, and verified focus behavior. Tool results MUST be valid MCP tool results whose text content contains one JSON object from the underlying capability.

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
The installer MUST create a user-local Codex plugin bundle under an owned `codex-computer-use-x11` namespace and MUST NOT write to `/opt`, `openai-bundled`, or bundled `computer-use` cache paths.

#### Scenario: Install owned plugin bundle files
- **GIVEN** `CODEX_HOME` points at an empty temporary Codex home
- **AND** an executable `codex-computer-use-x11` binary is available to install
- **WHEN** a developer runs `scripts/install-codex-plugin.sh`
- **THEN** the installer creates an owned cache entry under `$CODEX_HOME/plugins/cache/codex-computer-use-x11/codex-computer-use-x11/<version>/`
- **AND** that entry contains `.codex-plugin/plugin.json`, `.mcp.json`, and `bin/codex-computer-use-x11`
- **AND** `.mcp.json` starts the copied binary with argument `mcp`
- **AND** `latest` points to the installed version

#### Scenario: Write owned marketplace metadata
- **GIVEN** `CODEX_HOME` points at a temporary Codex home
- **WHEN** a developer runs `scripts/install-codex-plugin.sh`
- **THEN** the installer writes an owned local marketplace root for `codex-computer-use-x11`
- **AND** the root contains `.agents/plugins/marketplace.json`
- **AND** the marketplace JSON contains exactly the owned plugin entry for `codex-computer-use-x11`
- **AND** the marketplace plugin path resolves to the owned cache `latest` entry
- **AND** no marketplace metadata under `openai-bundled` is changed

#### Scenario: Enable plugin through user-local Codex config
- **GIVEN** `CODEX_HOME` points at a temporary Codex home with an existing `config.toml`
- **WHEN** a developer runs `scripts/install-codex-plugin.sh`
- **THEN** the installer preserves unrelated config content
- **AND** it adds or updates `[marketplaces.codex-computer-use-x11]` with a local source path to the owned marketplace root
- **AND** it adds or updates `[plugins."codex-computer-use-x11@codex-computer-use-x11"]` with `enabled = true`
- **AND** it does not add secrets or real credential values to tracked files or logs

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
