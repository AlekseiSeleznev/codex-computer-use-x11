# x11-get-app-state-integration Specification Delta

## ADDED Requirements

### Requirement: Standalone app-state CLI report
The standalone CLI MUST provide `get-app-state --json` as a composed X11/EWMH state report. The report MUST keep `backend` equal to `x11-ewmh`, MUST include target-repo-compatible fields for `window_context`, `window_error`, `screenshot`, `screenshot_error`, `accessibility_tree`, `accessibility_error`, `diagnostics`, and `message`, and MUST produce one valid JSON object whenever a report can be serialized.

#### Scenario: Resolve window context by window id
- **GIVEN** `wmctrl -lpGx` lists window `0x2` with title `Editor`, reliable pid `1234`, and bounds in X11 root coordinates
- **AND** `_NET_ACTIVE_WINDOW` can be queried
- **WHEN** a developer runs `codex-computer-use-x11 get-app-state --window-id 0x2 --no-screenshot --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** `backend` equals `x11-ewmh`
- **AND** `window_context.window_id` equals `2`
- **AND** `window_context.backend` equals `x11-ewmh`
- **AND** `window_error` is null
- **AND** `message` states that the window target resolved

#### Scenario: Return screenshot while target is missing
- **GIVEN** no current window matches requested window id `0x99`
- **AND** a supported standalone screenshot provider can capture a screenshot
- **WHEN** a developer runs `codex-computer-use-x11 get-app-state --window-id 0x99 --json`
- **THEN** the command exits with status code 0 when JSON can be emitted
- **AND** `window_context` is null
- **AND** `window_error` explains that no window matched the requested id
- **AND** `screenshot` is present
- **AND** `screenshot_error` is null
- **AND** diagnostics preserve the window-listing degraded or blocker details instead of failing the entire report

#### Scenario: Omit screenshot when not requested
- **GIVEN** a caller does not want screenshot bytes in the app-state response
- **WHEN** the caller runs `get-app-state --no-screenshot --json`
- **THEN** the command emits a valid app-state JSON report
- **AND** `screenshot` is null
- **AND** `screenshot_error` is null
- **AND** `message` states that screenshot capture was not requested

### Requirement: Safe target resolution in app state
App-state target selectors MUST reuse the standalone safe target-resolution semantics for `window_id`, `pid`, `wm_class`, and `title`. Ambiguous or stale selectors MUST populate `window_error` and MUST NOT populate `window_context` with an arbitrary candidate.

#### Scenario: Refuse ambiguous title target
- **GIVEN** the current X11 listing contains two windows whose titles contain `Editor`
- **WHEN** a developer runs `codex-computer-use-x11 get-app-state --title Editor --no-screenshot --json`
- **THEN** the command exits with status code 0 when JSON can be emitted
- **AND** `window_context` is null
- **AND** `window_error` includes `AmbiguousTarget` or equivalent candidate ambiguity detail
- **AND** diagnostics include candidate window ids or target-resolution evidence
- **AND** no focus or input command is attempted

#### Scenario: No target selector is not a target-resolution failure
- **GIVEN** the caller passes no window target selector
- **WHEN** `get-app-state --json` builds the report
- **THEN** `window_context` is null
- **AND** `window_error` is null
- **AND** screenshot and global diagnostics are still reported when available

### Requirement: App-state accessibility correlation
When a target window resolves, app-state MUST reuse the existing X11-to-AT-SPI correlation behavior. A high-confidence match MUST populate `accessibility_tree`; ambiguous, unavailable, or low-confidence AT-SPI MUST populate `accessibility_error` without discarding usable `window_context` or `screenshot` data.

#### Scenario: Include matched accessibility tree
- **GIVEN** the requested X11 window resolves to reliable pid `1234`
- **AND** the AT-SPI collector returns a high-confidence candidate for that pid with a button node
- **WHEN** `get-app-state --window-id 0x2 --no-screenshot --json` builds the report
- **THEN** `window_context.window_id` equals `2`
- **AND** `accessibility_tree` contains the matched node data
- **AND** `accessibility_error` is null
- **AND** diagnostics include the correlation status, confidence, and candidate reasons

#### Scenario: Keep window context when accessibility is ambiguous
- **GIVEN** the requested X11 window resolves
- **AND** AT-SPI candidates are ambiguous or no candidate reaches the confidence threshold
- **WHEN** app-state builds the report
- **THEN** `window_context` remains populated
- **AND** `accessibility_tree` is empty
- **AND** `accessibility_error` explains the ambiguous or degraded AT-SPI state
- **AND** screenshot data remains present when screenshot capture succeeds

### Requirement: App-state screenshot capture
When screenshot capture is requested, app-state MUST use a standalone screenshot provider boundary that is compatible with the existing Codex Desktop Linux screenshot concepts. Screenshot failures MUST be reported in `screenshot_error` without preventing window diagnostics or accessibility diagnostics from being returned.

#### Scenario: Capture screenshot through GNOME Shell-compatible provider
- **GIVEN** `org.gnome.Shell.Screenshot` exposes a screenshot method through the local session bus
- **AND** the provider writes a valid PNG file
- **WHEN** `get-app-state --json` captures a screenshot
- **THEN** `screenshot.mime_type` equals `image/png`
- **AND** `screenshot.source` identifies the GNOME Shell-compatible provider
- **AND** `screenshot.width` and `screenshot.height` are positive integers
- **AND** `screenshot.data_url` starts with `data:image/png;base64,`

#### Scenario: Report screenshot provider failure as degraded layer
- **GIVEN** no supported standalone screenshot provider is available or the provider call fails
- **WHEN** `get-app-state --json` builds the report
- **THEN** `screenshot` is null
- **AND** `screenshot_error` explains the provider failure
- **AND** `window_context`, `window_error`, `accessibility_tree`, and `accessibility_error` still reflect their own layers

### Requirement: Source-overlay compatibility guidance
The standalone app-state implementation MUST document that target-repo integration should improve the stock `get_app_state` path through `x11-ewmh` windowing, screenshot provider reuse, AT-SPI correlation reuse, and diagnostics reuse. It MUST NOT modify the target checkout or introduce unnamespaced competing target tools in this change.

#### Scenario: Document target integration path
- **GIVEN** future work adapts this project into the Codex Desktop Linux target checkout
- **WHEN** maintainers inspect README or the integration contract after this change
- **THEN** the docs state that the target checkout should reuse the existing stock `get_app_state` response shape
- **AND** the docs state that `x11-ewmh` should feed the existing windowing and target-resolution path
- **AND** the docs do not instruct maintainers to add a competing stock `x11_get_app_state` tool to the bundled Computer Use plugin
