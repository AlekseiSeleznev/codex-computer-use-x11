## ADDED Requirements

### Requirement: Window-scoped accessibility tree CLI
The standalone CLI MUST provide `accessibility-tree --window-id <id> --json` to resolve an X11/EWMH window, correlate it with AT-SPI candidates, and emit a machine-readable report. The command MUST return a subtree only when the correlation is confident and MUST otherwise return structured ambiguous or degraded diagnostics without panicking.

#### Scenario: Return correlated subtree for a confident match
- **GIVEN** the current X11 window listing contains a window with id `0x2`, reliable pid metadata, title `Preferences`, class `org.gnome.Settings`, bounds, and verified focus state
- **AND** AT-SPI candidates include one application/window subtree whose pid, name, and bounds match those window signals
- **WHEN** a developer runs `codex-computer-use-x11 accessibility-tree --window-id 0x2 --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** `success` is `true`
- **AND** `correlation.status` is `matched`
- **AND** `correlation.confidence` is `high`
- **AND** `tree` contains AT-SPI nodes for the matched candidate
- **AND** `error_code` is null

#### Scenario: Refuse missing window before AT-SPI collection
- **GIVEN** the current X11 window listing does not contain window id `0x99`
- **WHEN** a developer runs `codex-computer-use-x11 accessibility-tree --window-id 0x99 --json`
- **THEN** the command exits with a non-zero status code
- **AND** stdout is valid JSON
- **AND** `success` is `false`
- **AND** `input_sent` is absent or `false`
- **AND** `error_code` equals `WindowNotFound`
- **AND** no AT-SPI tree command is attempted

#### Scenario: Degrade when AT-SPI is unavailable
- **GIVEN** the requested X11 window resolves from the current listing
- **AND** AT-SPI collection is unavailable on this desktop session
- **WHEN** `accessibility-tree --window-id <id> --json` is handled
- **THEN** the command exits with a non-zero status code
- **AND** `success` is `false`
- **AND** `correlation.status` is `degraded`
- **AND** `error_code` equals `AtspiUnavailable`
- **AND** `tree` is empty
- **AND** diagnostics explain the AT-SPI blocker while preserving the X11 window listing diagnostics

### Requirement: Correlation matcher confidence and ambiguity
The matcher MUST score AT-SPI candidates using multiple signals and MUST report confidence and reasons. Reliable sidecar PID metadata, title/name similarity, wm_class/app-name similarity, bounds overlap, and focused-window state MUST be independent signals. The matcher MUST NOT select an arbitrary candidate when the top candidates are ambiguous or below threshold.

#### Scenario: Reliable pid plus title produces high confidence
- **GIVEN** a target window has reliable pid `4242`, title `Editor Alpha`, wm_class `code.Code`, and bounds
- **AND** AT-SPI candidates include one candidate with pid `4242`, name `Editor Alpha`, and overlapping bounds
- **WHEN** the matcher evaluates the candidates
- **THEN** it selects that candidate
- **AND** `confidence` is `high`
- **AND** `reasons` mention reliable pid, title/name, and bounds overlap

#### Scenario: Unreliable pid requires non-PID evidence
- **GIVEN** a target window has sidecar PID reliability `unreliable`
- **AND** AT-SPI candidates include one candidate whose pid equals the raw window pid but whose title/class/bounds do not match
- **AND** another candidate has matching title/class/bounds but a different pid
- **WHEN** the matcher evaluates the candidates
- **THEN** it selects the title/class/bounds candidate when the non-PID score reaches the threshold
- **AND** `confidence` is `medium`
- **AND** `reasons` state that PID was not treated as reliable evidence

#### Scenario: Ambiguous candidates are refused
- **GIVEN** two AT-SPI candidates have equivalent title/class/bounds evidence for the same target window
- **WHEN** the matcher evaluates the candidates
- **THEN** no subtree is returned
- **AND** `correlation.status` is `ambiguous`
- **AND** `success` is `false`
- **AND** `error_code` equals `AmbiguousAccessibilityMatch`
- **AND** diagnostics include the candidate object references and scores needed for disambiguation

#### Scenario: Browser multi-process candidate can match without PID
- **GIVEN** a browser window's X11 pid differs from the AT-SPI application pid
- **AND** the window title, class/app name, and bounds overlap identify exactly one AT-SPI candidate
- **WHEN** the matcher evaluates the candidates
- **THEN** it may select that candidate with `medium` confidence
- **AND** `reasons` include non-PID evidence
- **AND** the report does not claim a reliable PID match

#### Scenario: Terminal child process pid is not mistaken for the semantic owner
- **GIVEN** a terminal window has a terminal app/window pid and a foreground child process pid
- **AND** AT-SPI exposes the terminal application/window node rather than the foreground child process
- **WHEN** the matcher evaluates the candidates
- **THEN** it prefers the terminal application/window evidence over the child process pid alone
- **AND** it reports medium or high confidence only when title/class/bounds or terminal context corroborate the match

### Requirement: AT-SPI subtree report shape
The accessibility-tree report MUST include stable, automation-friendly fields for the selected window, correlation result, candidate diagnostics, and tree nodes. Tree nodes MUST include object reference, role, name, optional bounds, states, actions, value/editability indicators when available, depth, parent index, and child count. The report MUST avoid secret values and unrelated local environment data.

#### Scenario: Report includes match diagnostics
- **GIVEN** an AT-SPI correlation attempt has at least one candidate
- **WHEN** the CLI or MCP report is serialized
- **THEN** the JSON includes `project`, `version`, `backend`, `success`, `window`, `correlation`, `tree`, `error_code`, `note`, and `diagnostics`
- **AND** `correlation` includes `status`, `confidence`, `score`, `reasons`, and the matched object reference when present
- **AND** diagnostics include candidate summaries with scores but not secret values

#### Scenario: Tree size is bounded
- **GIVEN** the matched AT-SPI subtree is large
- **WHEN** the report is built
- **THEN** the implementation applies documented node and depth limits
- **AND** diagnostics state whether the tree was truncated
- **AND** the command remains responsive enough for MCP smoke testing
