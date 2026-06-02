## MODIFIED Requirements

### Requirement: Standalone MCP stdio server mode
The CLI MUST provide `codex-computer-use-x11 mcp` as a local stdio MCP server mode that communicates with JSON-RPC over stdin/stdout and exposes only project-owned `x11_*` tools in deterministic order.

#### Scenario: List standalone x11 tools
- **GIVEN** the `codex-computer-use-x11` binary is built
- **WHEN** an MCP client starts `codex-computer-use-x11 mcp`, initializes the server, and sends a `tools/list` request
- **THEN** the server returns a valid JSON-RPC response
- **AND** the response lists `x11_doctor`, `x11_list_windows`, `x11_focused_window`, `x11_focus_window`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, `x11_drag`, and `x11_accessibility_tree` in deterministic order
- **AND** every tool includes a description and JSON input schema
- **AND** no tool is named `doctor`, `list_windows`, `focused_window`, `activate_window`, `type_text`, `press_key`, `click`, `scroll`, `drag`, `accessibility_tree`, or `computer-use`

#### Scenario: Keep existing CLI usage distinct from MCP mode
- **GIVEN** a developer invokes an existing supported JSON command such as `doctor --json`
- **WHEN** the CLI handles the command
- **THEN** the command behavior remains the existing one-shot CLI behavior
- **AND** it does not require MCP initialization
- **AND** unsupported command usage still exits non-zero with stderr rather than starting the MCP server accidentally

### Requirement: x11 MCP tool calls wrap existing JSON capabilities
The MCP server MUST implement tool calls by reusing the standalone project's existing JSON report builders for doctor, window listing, focused-window, verified focus, verified keyboard input, verified pointer input, and AT-SPI window-correlation behavior. Tool results MUST be valid MCP tool results whose text content contains one JSON object from the underlying capability.

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

#### Scenario: Reject missing accessibility-tree argument as a tool error
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_accessibility_tree` without a `window_id` argument
- **THEN** the server returns an MCP tool result with `isError` true
- **AND** the result explains the missing `window_id` argument
- **AND** no AT-SPI collection command is attempted
