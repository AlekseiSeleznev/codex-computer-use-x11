# standalone-codex-mcp-plugin Specification Delta

## MODIFIED Requirements

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
