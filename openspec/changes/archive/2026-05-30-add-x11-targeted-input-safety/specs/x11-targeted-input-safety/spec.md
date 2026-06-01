# x11-targeted-input-safety Specification Delta

## ADDED Requirements

### Requirement: Verified-focus targeted keyboard CLI
The standalone CLI MUST provide safe targeted keyboard input commands that resolve a target X11/EWMH window, activate it, verify exact active-window focus, and only then invoke the selected keyboard input backend. The commands MUST write a single JSON object to stdout when a report can be produced, MUST keep `backend` equal to `x11-ewmh`, and MUST NOT require external credentials or modify the Codex Desktop Linux target checkout.

#### Scenario: Type text after exact focus verification
- **GIVEN** `wmctrl -lpGx` lists a target window
- **AND** `xprop -root _NET_ACTIVE_WINDOW` initially reports another window
- **AND** the focus activation step succeeds
- **AND** a fresh active-window lookup reports the target window id
- **WHEN** a developer runs `codex-computer-use-x11 type-text --window-id <target> --text "hello" --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** `success` is `true`
- **AND** `input_sent` is `true`
- **AND** `focus.exact_window_focused` is `true`
- **AND** the keyboard backend invocation happens after the verified focus result
- **AND** stderr is empty on success

#### Scenario: Press a key after exact focus verification
- **GIVEN** `wmctrl -lpGx` lists a target window
- **AND** focus activation is verified through a fresh active-window lookup
- **WHEN** a developer runs `codex-computer-use-x11 press-key --window-id <target> --key Enter --json`
- **THEN** the command exits with status code 0
- **AND** `success` is `true`
- **AND** `input_sent` is `true`
- **AND** diagnostics identify the keyboard backend and arguments used without exposing unrelated local data

#### Scenario: Reject unsupported targeted input usage
- **GIVEN** a developer invokes `type-text`, `press-key`, or unsupported flags without `--json`
- **WHEN** the CLI handles the invocation
- **THEN** the command exits with a non-zero status code
- **AND** it writes usage or error text to stderr
- **AND** no focus or keyboard input command is attempted

### Requirement: Window target resolution for input
Targeted keyboard input MUST resolve exactly one current listed window before focus activation. The standalone target selectors MUST include `window_id` and MAY include title, `wm_class`, and pid selectors when they can be resolved from the current `list-windows --json` model. Ambiguous or stale targets MUST fail safely and MUST NOT invoke focus or keyboard input.

#### Scenario: Resolve by exact window id
- **GIVEN** the current window listing includes a window id `0x00000002`
- **WHEN** targeted input is requested with `--window-id 0x2`
- **THEN** the target resolves to that window
- **AND** focus activation may proceed to the verification gate

#### Scenario: Refuse ambiguous title target
- **GIVEN** the current window listing contains two windows whose titles contain `Editor`
- **WHEN** targeted input is requested with `--title Editor`
- **THEN** the command exits with a non-zero status code
- **AND** stdout is valid JSON when JSON output can be produced
- **AND** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `AmbiguousTarget`
- **AND** diagnostics include candidate window ids
- **AND** no activation or keyboard input command is attempted

#### Scenario: Refuse stale target window
- **GIVEN** a caller requests a window id that is absent from the current listing
- **WHEN** targeted input handles the request
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `WindowNotFound`
- **AND** no activation or keyboard input command is attempted

### Requirement: Input is never sent when focus is unverified
Targeted keyboard input MUST treat focus verification as the safety boundary because X11 keyboard injectors are global or best-effort direct-event mechanisms. If activation fails, exact active-window verification fails, or active-window lookup is unavailable, the implementation MUST NOT invoke the keyboard input backend.

#### Scenario: Focus verification mismatch blocks typing
- **GIVEN** `wmctrl -lpGx` lists the requested target window
- **AND** the activation command exits successfully
- **AND** a fresh active-window lookup reports a different window id
- **WHEN** targeted `type-text` handles the request
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `FocusNotVerified`
- **AND** diagnostics explain the focused window mismatch
- **AND** the keyboard backend command is not invoked

#### Scenario: Focus command failure blocks key press
- **GIVEN** the requested target window is listed
- **AND** every focus activation attempt fails or cannot be verified
- **WHEN** targeted `press-key` handles the request
- **THEN** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` is not `null`
- **AND** the keyboard backend command is not invoked

#### Scenario: No target does not imply safe targeted input
- **GIVEN** a caller omits all window target selectors
- **WHEN** targeted keyboard input handles the request
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `MissingTarget`
- **AND** diagnostics explain that global/unverified input is not window-isolated and is out of scope for this safe targeted command

### Requirement: Standalone keyboard backend semantics
The standalone keyboard input backend MUST use active-context injection after verified focus and MUST NOT use `xdotool --window` direct events as proof of targeted safety. Keyboard backend diagnostics MUST identify whether `xdotool type` or `xdotool key` was selected and MUST report layout/Unicode limitations as degraded evidence rather than silently claiming full Unicode correctness.

#### Scenario: Type text uses active-context xdotool
- **GIVEN** focus verification has succeeded for the requested target
- **WHEN** the standalone `type-text` command invokes the keyboard backend
- **THEN** it invokes `xdotool type --clearmodifiers <text>` or an equivalent active-context invocation
- **AND** it does not include `--window <id>` in the xdotool typing command
- **AND** diagnostics state that X11 direct-to-window events are not the safety boundary

#### Scenario: Press key uses active-context xdotool
- **GIVEN** focus verification has succeeded for the requested target
- **WHEN** the standalone `press-key` command invokes the keyboard backend
- **THEN** it invokes `xdotool key --clearmodifiers <key>` or an equivalent active-context invocation
- **AND** it does not include `--window <id>` in the xdotool key command

#### Scenario: Report missing keyboard backend
- **GIVEN** the requested target window is listed and focus can be verified
- **AND** `xdotool` is unavailable on `PATH`
- **WHEN** targeted keyboard input handles the request
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `input_sent` is `false`
- **AND** `error_code` equals `InputBackendUnavailable`
- **AND** diagnostics explain that no standalone keyboard input backend was available

#### Scenario: Record non-US and non-BMP behavior
- **GIVEN** the implementation is verified on Cinnamon/X11 or through a fake backend that preserves literal arguments
- **WHEN** text containing Cyrillic and at least one non-BMP/emoji character is tested
- **THEN** evidence records whether the selected backend typed the intended text or degraded
- **AND** any Unicode/layout limitation is documented in the test-plan evidence without bypassing focus verification

### Requirement: Targeted input MCP tools wrap the safe CLI behavior
The standalone MCP server MUST expose `x11_type_text` and `x11_press_key` tool calls that reuse the same safe targeted keyboard input behavior as the CLI. MCP tool results MUST be valid MCP tool results whose text content contains one JSON object from the underlying targeted input capability.

#### Scenario: MCP type text requires a target
- **GIVEN** an MCP client has initialized the standalone server
- **WHEN** the client calls `x11_type_text` without a window target selector
- **THEN** the server returns an MCP tool result with `isError` true
- **AND** the result JSON has `input_sent` false
- **AND** no keyboard input command is attempted

#### Scenario: MCP type text reports safe success
- **GIVEN** an MCP client calls `x11_type_text` with `window_id` and `text`
- **AND** focus verification succeeds
- **AND** the keyboard backend exits successfully
- **WHEN** the server returns the tool result
- **THEN** `isError` is false
- **AND** the text content is valid JSON
- **AND** `success` is true
- **AND** `input_sent` is true

#### Scenario: MCP press key reports focus safety failures as tool errors
- **GIVEN** an MCP client calls `x11_press_key` with `window_id` and `key`
- **AND** focus verification fails
- **WHEN** the server returns the tool result
- **THEN** `isError` is true
- **AND** the text content is valid JSON
- **AND** `success` is false
- **AND** `input_sent` is false
- **AND** `error_code` explains why input was refused
