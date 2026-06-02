# x11-pointer-actions Specification Delta

## ADDED Requirements

### Requirement: Standalone pointer action CLI
The standalone CLI MUST provide `click`, `scroll`, and `drag` JSON commands for X11/EWMH pointer actions. Each command MUST write one JSON report when a report can be produced, MUST keep `backend` equal to `x11-ewmh`, MUST distinguish targeted from explicitly global/unverified actions, and MUST NOT require external credentials or modify the Codex Desktop Linux target checkout.

#### Scenario: Click after target verification
- **GIVEN** `wmctrl -lpGx` lists a target window with known bounds
- **AND** the requested global/root X11 point is inside those bounds
- **AND** focus activation is verified through a fresh active-window lookup
- **WHEN** a developer runs `codex-computer-use-x11 click --window-id <target> --x <x> --y <y> --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** `success` is `true`
- **AND** `input_sent` is `true`
- **AND** `targeted` is `true`
- **AND** `focus.exact_window_focused` is `true`
- **AND** stderr is empty on success

#### Scenario: Scroll after target verification
- **GIVEN** `wmctrl -lpGx` lists a target window with known bounds
- **AND** the requested global/root X11 point is inside those bounds
- **AND** focus activation is verified
- **WHEN** a developer runs `codex-computer-use-x11 scroll --window-id <target> --x <x> --y <y> --direction down --amount 3 --json`
- **THEN** the command exits with status code 0
- **AND** `success` is `true`
- **AND** `input_sent` is `true`
- **AND** diagnostics identify the selected backend and direction mapping

#### Scenario: Drag after target verification
- **GIVEN** `wmctrl -lpGx` lists a target window with known bounds
- **AND** both requested global/root X11 drag endpoints are inside those bounds
- **AND** focus activation is verified
- **WHEN** a developer runs `codex-computer-use-x11 drag --window-id <target> --start-x <x1> --start-y <y1> --end-x <x2> --end-y <y2> --json`
- **THEN** the command exits with status code 0
- **AND** `success` is `true`
- **AND** `input_sent` is `true`
- **AND** diagnostics identify a down/move/up drag sequence

#### Scenario: Reject unsupported pointer usage
- **GIVEN** a developer invokes `click`, `scroll`, or `drag` without `--json` or with unsupported flags
- **WHEN** the CLI handles the invocation
- **THEN** the command exits with a non-zero status code
- **AND** it writes usage or error text to stderr
- **AND** no focus or pointer input command is attempted

### Requirement: Pointer safety gates
Targeted pointer actions MUST resolve exactly one current window, require usable bounds, validate requested global/root X11 coordinates inside those bounds, focus and verify the exact active window when a target is present, and refuse safely before invoking any pointer backend when a gate fails.

#### Scenario: Refuse ambiguous pointer target
- **GIVEN** the current window listing contains two windows whose titles contain `Editor`
- **WHEN** a targeted pointer action is requested with `--title Editor`
- **THEN** the command exits with a non-zero status code
- **AND** stdout is valid JSON when JSON output can be produced
- **AND** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `AmbiguousTarget`
- **AND** diagnostics include candidate window ids
- **AND** no activation or pointer command is attempted

#### Scenario: Refuse missing bounds
- **GIVEN** the requested target window resolves but has no known bounds
- **WHEN** a targeted pointer action is handled
- **THEN** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `MissingBounds`
- **AND** no focus or pointer backend command is attempted

#### Scenario: Refuse point outside target bounds
- **GIVEN** the requested target window resolves with known bounds
- **AND** the requested point is outside those bounds
- **WHEN** a targeted `click` or `scroll` action is handled
- **THEN** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `PointOutsideTargetBounds`
- **AND** no focus or pointer backend command is attempted

#### Scenario: Refuse drag endpoint outside target bounds
- **GIVEN** the requested target window resolves with known bounds
- **AND** one drag endpoint is outside those bounds
- **WHEN** the targeted `drag` action is handled
- **THEN** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `PointOutsideTargetBounds`
- **AND** no focus or pointer backend command is attempted

#### Scenario: Focus verification mismatch blocks pointer input
- **GIVEN** `wmctrl -lpGx` lists the requested target window with bounds
- **AND** the activation command exits successfully
- **AND** a fresh active-window lookup reports a different window id
- **WHEN** a targeted pointer action handles the request
- **THEN** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `FocusNotVerified`
- **AND** the pointer backend command is not invoked

### Requirement: Standalone pointer backend semantics
The standalone pointer backend MUST use active/global-context X11 pointer commands only after the selected safety mode has been established. Targeted actions MUST use active-context injection after verified focus. Explicit global actions MUST be marked as unverified and non-window-isolated. Backend diagnostics MUST identify the command, arguments, safety mode, and whether direct per-window events were avoided.

#### Scenario: Click uses active-context xdotool
- **GIVEN** a targeted click has passed target, bounds, and focus verification
- **WHEN** the standalone pointer backend invokes the click
- **THEN** it invokes `xdotool mousemove --sync <x> <y> click --repeat <count> <button>` or an equivalent active-context invocation
- **AND** it does not include `--window <id>` in the pointer command
- **AND** diagnostics state that X11 pointer injection is global and was guarded by verification

#### Scenario: Scroll maps directions to bounded wheel clicks
- **GIVEN** a targeted scroll has passed verification
- **WHEN** the standalone pointer backend invokes the scroll
- **THEN** it moves to the requested point before sending wheel clicks
- **AND** `up`, `down`, `left`, and `right` map to X11 wheel buttons 4, 5, 6, and 7 respectively or an equivalent documented mapping
- **AND** `amount` is clamped or refused according to a finite safety limit

#### Scenario: Drag emits bounded down move up sequence
- **GIVEN** a targeted drag has passed verification
- **WHEN** the standalone pointer backend invokes the drag
- **THEN** it emits a finite left-button down, move, and up sequence
- **AND** it refuses drags whose absolute delta exceeds the configured safety limit unless an accepted later spec adds an explicit override
- **AND** diagnostics include the start and end points without exposing unrelated local data

#### Scenario: Report missing pointer backend
- **GIVEN** the requested target window is listed, bounded, and focus can be verified
- **AND** `xdotool` is unavailable on `PATH`
- **WHEN** a targeted pointer action handles the request
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `InputBackendUnavailable`
- **AND** diagnostics explain that no standalone pointer backend was available

### Requirement: Explicit global pointer mode
Pointer actions without a target MUST be allowed only when the caller supplies `--global`. Global pointer reports MUST clearly mark `targeted` as false, `verification_mode` as `global_unverified`, and `input_sent` according to whether the backend command actually ran.

#### Scenario: Missing target without global mode is refused
- **GIVEN** a caller omits all window target selectors
- **AND** the caller does not pass `--global`
- **WHEN** a pointer action handles the request
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `MissingTarget`
- **AND** diagnostics explain that pointer injection is not window-isolated

#### Scenario: Explicit global click is marked unverified
- **GIVEN** a caller supplies `--global` with finite coordinates
- **WHEN** `click` handles the request
- **THEN** it may run without target resolution or focus activation
- **AND** the JSON report has `targeted` equal to `false`
- **AND** `verification_mode` equals `global_unverified`
- **AND** diagnostics include a degraded reason or warning that the action was not window-isolated

### Requirement: Pointer MCP tools wrap the safe CLI behavior
The standalone MCP server MUST expose `x11_click`, `x11_scroll`, and `x11_drag` tool calls that reuse the same safe pointer action behavior as the CLI. MCP tool results MUST be valid MCP tool results whose text content contains one JSON object from the underlying pointer action capability.

#### Scenario: MCP click requires target or global mode
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_click` with coordinates but without a window target selector and without `global` true
- **THEN** the server returns an MCP tool result with `isError` true
- **AND** the result JSON has `input_sent` false
- **AND** no pointer command is attempted

#### Scenario: MCP click reports safe success
- **GIVEN** an MCP client calls `x11_click` with `window_id`, `x`, and `y`
- **AND** target, bounds, focus, and backend verification succeed
- **WHEN** the server returns the tool result
- **THEN** `isError` is false
- **AND** the text content is valid JSON
- **AND** `success` is true
- **AND** `input_sent` is true

#### Scenario: MCP scroll reports safety failures as tool errors
- **GIVEN** an MCP client calls `x11_scroll` with a target and point outside target bounds
- **WHEN** the server returns the tool result
- **THEN** `isError` is true
- **AND** the text content is valid JSON
- **AND** `success` is false
- **AND** `input_sent` is false
- **AND** `error_code` explains why input was refused
