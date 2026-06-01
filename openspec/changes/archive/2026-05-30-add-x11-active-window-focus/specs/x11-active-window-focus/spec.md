## ADDED Requirements

### Requirement: Focused window JSON command
The CLI MUST provide `focused-window --json` as a read-only command that reports the current X11/EWMH active window using the project `WindowInfo` shape whenever the active window can be matched to the current `wmctrl -lpGx` listing.

#### Scenario: Report the matched focused window
- **GIVEN** `wmctrl -lpGx` lists a window with id `0x00000002`
- **AND** `xprop -root _NET_ACTIVE_WINDOW` reports `window id # 0x2`
- **WHEN** a developer runs `codex-computer-use-x11 focused-window --json`
- **THEN** the command exits with status code 0
- **AND** stdout is a single valid JSON object
- **AND** `focused_window.window_id` equals `2`
- **AND** `focused_window.focused` is `true`
- **AND** `diagnostics.active_window` equals `2`
- **AND** stderr is empty

#### Scenario: Report no active window without failing JSON output
- **GIVEN** `wmctrl -lpGx` can list windows
- **AND** `xprop -root _NET_ACTIVE_WINDOW` reports `window id # 0x0`
- **WHEN** a developer runs `codex-computer-use-x11 focused-window --json`
- **THEN** the command exits with status code 0
- **AND** `focused_window` is `null`
- **AND** `diagnostics.active_window` is `null`
- **AND** `diagnostics.degraded_reasons` explains that no active X11 window was reported

#### Scenario: Report an active id that is not in the listing
- **GIVEN** `wmctrl -lpGx` lists windows that do not include `0x00000003`
- **AND** `xprop -root _NET_ACTIVE_WINDOW` reports `window id # 0x3`
- **WHEN** a developer runs `codex-computer-use-x11 focused-window --json`
- **THEN** the command exits with status code 0
- **AND** `focused_window` is `null`
- **AND** `diagnostics.active_window` equals `3`
- **AND** `diagnostics.degraded_reasons` explains that the active window could not be matched to the current window listing

### Requirement: Shared X11 window-id normalization for focus commands
The focus commands MUST reuse the shared X11 window-id normalizer so decimal, short hexadecimal, and zero-padded hexadecimal inputs resolve to the same `u64` window identity.

#### Scenario: Accept equivalent id formats
- **GIVEN** `wmctrl -lpGx` lists a window with raw id `0x00000002`
- **WHEN** a developer runs `codex-computer-use-x11 focus-window --window-id 0x2 --json`
- **THEN** the requested window id is interpreted as `2`
- **AND** the same command with `--window-id 0x00000002` targets the same window
- **AND** the same command with `--window-id 2` targets the same window

#### Scenario: Reject invalid window ids before activation
- **GIVEN** the developer provides `--window-id not-a-window`
- **WHEN** a developer runs `codex-computer-use-x11 focus-window --window-id not-a-window --json`
- **THEN** the command exits with a non-zero status code
- **AND** no activation command is attempted
- **AND** stderr explains that the X11 window id is invalid

### Requirement: Verified focus activation command
The CLI MUST provide `focus-window --window-id <id> --json` that attempts to activate a listed X11 window and reports success only after a fresh active-window lookup verifies the requested window id.

#### Scenario: Activation is verified by active-window identity
- **GIVEN** `wmctrl -lpGx` lists a window with id `0x00000002`
- **AND** the activation command exits successfully
- **AND** a fresh `xprop -root _NET_ACTIVE_WINDOW` lookup reports `window id # 0x2`
- **WHEN** a developer runs `codex-computer-use-x11 focus-window --window-id 0x2 --json`
- **THEN** the command exits with status code 0
- **AND** `success` is `true`
- **AND** `exact_window_focused` is `true`
- **AND** `requested_window.window_id` equals `2`
- **AND** `focused_window.window_id` equals `2`
- **AND** `error_code` is `null`

#### Scenario: Activation command success without focus verification is unsafe
- **GIVEN** `wmctrl -lpGx` lists a requested window with id `0x00000002`
- **AND** the activation command exits successfully
- **AND** a fresh `xprop -root _NET_ACTIVE_WINDOW` lookup reports `window id # 0x3`
- **WHEN** a developer runs `codex-computer-use-x11 focus-window --window-id 0x2 --json`
- **THEN** the command exits with a non-zero status code
- **AND** stdout is a single valid JSON object
- **AND** `success` is `false`
- **AND** `exact_window_focused` is `false`
- **AND** `error_code` equals `FocusNotVerified`
- **AND** `focused_window.window_id` equals `3` when that window is present in the current listing

#### Scenario: Requested window must be present in the current listing
- **GIVEN** `wmctrl -lpGx` lists windows that do not include id `0x00000099`
- **WHEN** a developer runs `codex-computer-use-x11 focus-window --window-id 0x99 --json`
- **THEN** the command exits with a non-zero status code
- **AND** no activation command is attempted
- **AND** `success` is `false`
- **AND** `error_code` equals `WindowNotFound`

### Requirement: Activation fallback diagnostics
The focus implementation MUST expose which activation command attempts were made and MUST try `xdotool windowactivate --sync` as a fallback when `wmctrl -ia` fails or cannot be verified, while still requiring the same final active-window verification.

#### Scenario: Use wmctrl as the first activation attempt
- **GIVEN** `wmctrl` and `xdotool` are both available
- **WHEN** a developer runs `codex-computer-use-x11 focus-window --window-id 0x2 --json`
- **THEN** the first activation attempt uses `wmctrl -ia 0x2`
- **AND** the result diagnostics record the `wmctrl` attempt

#### Scenario: Fallback to xdotool after wmctrl failure
- **GIVEN** `wmctrl -ia 0x2` exits unsuccessfully
- **AND** `xdotool windowactivate --sync 2` exits successfully
- **AND** a fresh active-window lookup reports `window id # 0x2`
- **WHEN** a developer runs `codex-computer-use-x11 focus-window --window-id 0x2 --json`
- **THEN** the command exits with status code 0
- **AND** diagnostics record both activation attempts in order
- **AND** `success` is `true`

#### Scenario: Fallback command success still requires verification
- **GIVEN** `wmctrl -ia 0x2` fails
- **AND** `xdotool windowactivate --sync 2` exits successfully
- **AND** a fresh active-window lookup reports `window id # 0x3`
- **WHEN** a developer runs `codex-computer-use-x11 focus-window --window-id 0x2 --json`
- **THEN** the command exits with a non-zero status code
- **AND** diagnostics record both activation attempts in order
- **AND** `error_code` equals `FocusNotVerified`

### Requirement: Focus verification is the targeted-input safety boundary
The project MUST NOT treat X11 window-targeted input as safe unless a focus operation has verified that the requested window is the active window; direct `xdotool --window` or command exit success MUST NOT be used as the safety boundary.

#### Scenario: Unverified focus keeps future targeted input unsafe
- **GIVEN** `focus-window --window-id 0x2 --json` returns `success: false`
- **AND** `error_code` equals `FocusNotVerified`
- **WHEN** a later targeted input path evaluates whether it may send input to window `0x2`
- **THEN** the focus result is not sufficient to permit targeted input
- **AND** the diagnostic note explains that exact focus verification failed

#### Scenario: Verified focus can be consumed by later input stages
- **GIVEN** `focus-window --window-id 0x2 --json` returns `success: true`
- **AND** `exact_window_focused` is `true`
- **WHEN** a later targeted input path evaluates whether focus is safe for window `0x2`
- **THEN** the focus result can be used as evidence that the requested X11 window was active at verification time
- **AND** input-specific authorization and backend checks still remain separate concerns
