## ADDED Requirements

### Requirement: Keyboard key names are normalized before injection
The targeted keyboard implementation MUST normalize common user-facing key aliases to the X11 keysyms expected by the active-context backend before invoking the backend. At minimum, `Enter` SHALL map to `Return` and `Backspace` / `Backspace` variants SHALL map to `BackSpace`. The report MUST expose the requested key, normalized key, backend route, and whether input was sent.

#### Scenario: Enter alias is sent as Return
- **GIVEN** a target window resolves and exact focus verification succeeds
- **WHEN** a caller requests `press-key --key Enter --json`
- **THEN** the backend invocation uses `Return`
- **AND** the keyboard report records `requested_key` as `Enter`
- **AND** the keyboard report records `normalized_key` as `Return`
- **AND** `input_sent` is true

#### Scenario: Backspace alias is sent as BackSpace
- **GIVEN** a target window resolves and exact focus verification succeeds
- **WHEN** a caller requests `press-key --key Backspace --json`
- **THEN** the backend invocation uses `BackSpace`
- **AND** the keyboard report records the alias normalization
- **AND** stderr is empty on success

### Requirement: Xdotool semantic key errors fail the backend
The targeted keyboard backend MUST treat `xdotool` stderr that contains semantic key failure phrases, including `No such key name` and `Ignoring it`, as a backend failure even when the process exits with status code 0. The implementation MUST NOT report `success=true` or `input_sent=true` for such a backend attempt.

#### Scenario: Exit zero with No such key name is failure
- **GIVEN** a target window resolves and exact focus verification succeeds
- **AND** the fake `xdotool` exits with status code 0 while writing `No such key name` to stderr
- **WHEN** targeted `press-key` handles the request
- **THEN** the command emits valid JSON
- **AND** `success` is false
- **AND** `input_sent` is false
- **AND** `error_code` equals `InputBackendFailed`
- **AND** diagnostics include the stderr phrase

#### Scenario: Exit zero with Ignoring it is failure
- **GIVEN** a target window resolves and exact focus verification succeeds
- **AND** the fake `xdotool` exits with status code 0 while writing `Ignoring it` to stderr
- **WHEN** targeted `type-text` handles the request
- **THEN** `success` is false
- **AND** `input_sent` is false
- **AND** diagnostics explain that the backend refused at least one key

### Requirement: Non-ASCII typing uses verified-focus Unicode routes
For `type-text` values containing non-ASCII text, the implementation MUST keep verified focus as the safety boundary and MUST try an X11 Unicode keysym route before any clipboard fallback. The primary Unicode route SHOULD invoke active-context `xdotool key --clearmodifiers Uxxxx ...` keysyms derived from Unicode scalar values. The implementation MUST NOT use `xdotool --window` as a targeted-safety boundary and MUST NOT make `ydotool` the primary Unicode fix.

#### Scenario: Cyrillic text uses Unicode keysyms after focus verification
- **GIVEN** a target window resolves and exact focus verification succeeds
- **AND** the text to type is `Привет`
- **WHEN** targeted `type-text` handles the request
- **THEN** the keyboard route is `xdotool-unicode-keysyms` or a more specific active-context Unicode keysym route
- **AND** the backend arguments include `U041F`, `U0440`, `U0438`, `U0432`, `U0435`, and `U0442`
- **AND** the backend invocation does not include `--window <id>`
- **AND** diagnostics state that focus verification, not direct XSendEvent, was the safety boundary

#### Scenario: Non-BMP text records unsupported keysym degradation
- **GIVEN** a target window resolves and exact focus verification succeeds
- **AND** the text contains a Unicode scalar that the chosen X11 keysym route cannot type exactly
- **WHEN** targeted `type-text` handles the request
- **THEN** the report records a concrete degraded reason or falls back through the explicit clipboard-paste route
- **AND** the report does not silently claim exact fidelity for unsupported characters

### Requirement: Clipboard fallback is explicit and recoverable
If the Unicode keysym route cannot provide exact non-ASCII text fidelity, the implementation MAY use a clipboard-paste fallback only after verified target focus. The fallback MUST use an explicit route such as `clipboard-paste`, MUST prefer `xclip` or `xsel` when available, MUST preserve and restore the previous clipboard contents when possible, and MUST fail with diagnostics instead of leaving unrecoverable clipboard mutation unreported.

#### Scenario: Clipboard fallback restores previous clipboard
- **GIVEN** a target window resolves and exact focus verification succeeds
- **AND** the Unicode keysym route reports that exact text fidelity is unavailable
- **AND** a clipboard command is available and can read the previous clipboard value
- **WHEN** targeted `type-text` uses the fallback
- **THEN** the report records `route=clipboard-paste`
- **AND** the paste command happens only after focus verification
- **AND** diagnostics record that previous clipboard content was restored
- **AND** `input_sent` is true only if paste succeeds

#### Scenario: Clipboard fallback failure does not hide mutation risk
- **GIVEN** a target window resolves and exact focus verification succeeds
- **AND** the implementation attempts clipboard fallback
- **AND** previous clipboard restoration fails
- **WHEN** the command emits its report
- **THEN** `success` is false or degraded according to the route contract
- **AND** diagnostics include `clipboard_restore_failed` or equivalent warning
- **AND** the report does not claim an unrecoverable clipboard mutation was safe
