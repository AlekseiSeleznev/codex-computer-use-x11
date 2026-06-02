# x11-get-app-state-integration Specification

## Purpose
Defines the standalone `get-app-state` contract for composing X11/EWMH window context, path-oriented screenshot evidence, AT-SPI accessibility data, layer diagnostics, and MCP-facing app-state responses.

## Requirements
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
When screenshot capture is requested, app-state MUST use a standalone screenshot provider boundary that is compatible with the existing Codex Desktop Linux screenshot concepts. Screenshot failures MUST be reported in `screenshot_error` without preventing window diagnostics or accessibility diagnostics from being returned. By default, JSON reports MUST NOT serialize screenshot pixels, screenshot data URLs, or base64 screenshot payloads; successful screenshots MUST be represented by path-oriented metadata or by an explicit disabled/unavailable diagnostic.

#### Scenario: Capture screenshot through GNOME Shell-compatible provider
- **GIVEN** `org.gnome.Shell.Screenshot` exposes a screenshot method through the local session bus
- **AND** the provider writes a valid PNG file
- **WHEN** `get-app-state --json` captures a screenshot without inline opt-in
- **THEN** `screenshot.mime_type` equals `image/png`
- **AND** `screenshot.source` identifies the GNOME Shell-compatible provider
- **AND** `screenshot.width` and `screenshot.height` are positive integers when dimensions can be determined
- **AND** `screenshot.path` or an equivalent screenshot artifact path references the captured PNG file
- **AND** the referenced file exists, is readable, is non-empty, and has a PNG signature
- **AND** the JSON does not contain `screenshot.data_url`, `data:image`, or a base64 screenshot payload by default

#### Scenario: Report screenshot provider failure as degraded layer
- **GIVEN** no supported standalone screenshot provider is available or the provider call fails
- **WHEN** `get-app-state --json` builds the report
- **THEN** `screenshot` is null
- **AND** `screenshot_error` explains the provider failure
- **AND** `window_context`, `window_error`, `accessibility_tree`, and `accessibility_error` still reflect their own layers
- **AND** the command emits valid JSON when the non-screenshot layers can be serialized

### Requirement: Source-overlay compatibility guidance
The standalone app-state implementation MUST document that target-repo integration should improve the stock `get_app_state` path through `x11-ewmh` windowing, screenshot provider reuse, AT-SPI correlation reuse, and diagnostics reuse. It MUST NOT modify the target checkout or introduce unnamespaced competing target tools in this change.

#### Scenario: Document target integration path
- **GIVEN** future work adapts this project into the Codex Desktop Linux target checkout
- **WHEN** maintainers inspect README or the integration contract after this change
- **THEN** the docs state that the target checkout should reuse the existing stock `get_app_state` response shape
- **AND** the docs state that `x11-ewmh` should feed the existing windowing and target-resolution path
- **AND** the docs do not instruct maintainers to add a competing stock `x11_get_app_state` tool to the bundled Computer Use plugin

### Requirement: Evidence summaries read diagnostics layers correctly
App-state evidence summarization MUST read layer status from `diagnostics.layers`, not from a top-level `layers` field. Summary tooling MUST fail or mark evidence degraded when the expected diagnostics path is absent rather than silently misclassifying app-state readiness.

#### Scenario: Summary extracts diagnostics.layers
- **GIVEN** an app-state JSON report contains `diagnostics.layers` with `window`, `screenshot`, and `accessibility` layer entries
- **WHEN** evidence summary tooling evaluates the report
- **THEN** it reads layer status from `diagnostics.layers`
- **AND** the summary records screenshot pass and accessibility degraded according to those entries
- **AND** it does not look for a top-level `.layers` field

#### Scenario: Missing diagnostics.layers is explicit degraded evidence
- **GIVEN** an app-state JSON report lacks `diagnostics.layers`
- **WHEN** evidence summary tooling evaluates the report
- **THEN** the summary records a concrete degraded or failure reason
- **AND** the report is not counted as fully passing by omission

### Requirement: App-state evidence mode can omit screenshot bytes
The app-state CLI, MCP wrapper, or e2e summarizer MUST support a no-screenshot-data evidence mode that preserves screenshot metadata such as MIME type, source, dimensions, and capture status while omitting large base64 `data_url` payloads from live summaries and durable evidence files.

#### Scenario: Evidence summary omits base64 screenshot data
- **GIVEN** app-state captures a PNG screenshot successfully
- **WHEN** evidence summary mode is enabled
- **THEN** the summary records screenshot `mime_type`, `source`, `width`, `height`, and capture status
- **AND** the summary does not include the full `data:image/png;base64,...` payload
- **AND** raw screenshot bytes are written only to an explicit artifact path when requested

#### Scenario: No-screenshot-data does not hide screenshot failure
- **GIVEN** screenshot capture fails
- **WHEN** evidence summary mode is enabled
- **THEN** the summary records `screenshot_error` and the screenshot layer status
- **AND** it does not fabricate screenshot metadata

### Requirement: Portal diagnostics remain optional for working X11 path
Readiness, app-state messages, and recommended next steps MUST separate real X11/EWMH blockers from optional RemoteDesktop portal diagnostics. When X11/EWMH focus, input, screenshot, and app-state layers work, an incomplete RemoteDesktop portal MUST be reported as degraded/report-only and MUST NOT be the recommended blocker for the X11 path.

#### Scenario: Working X11 path is not blocked by portal absence
- **GIVEN** `x11_doctor` reports X11/EWMH focus/input prerequisites available
- **AND** RemoteDesktop portal introspection is unavailable or incomplete
- **WHEN** readiness summary and recommended next step are generated
- **THEN** RemoteDesktop portal status is marked degraded or optional for the X11 path
- **AND** the recommended next step focuses on any real X11 degraded layer, such as AT-SPI or Unicode fidelity
- **AND** the summary does not call portal absence a blocker for working X11/EWMH focus/input/screenshot

#### Scenario: Portal-only guidance remains separate
- **GIVEN** a future path requires portal integration
- **WHEN** readiness docs mention RemoteDesktop portal gaps
- **THEN** that guidance is clearly labeled as optional or future portal work
- **AND** it does not weaken the Cinnamon/X11 `x11-ewmh` readiness criteria from ADR 0009

### Requirement: Live app-state evidence is fixture scoped and sanitized
Live `get_app_state` verification MUST target a controlled fixture window and MUST write sanitized evidence that preserves window context, layer statuses, screenshot metadata, and artifact paths while avoiding full screenshot data URLs in ordinary logs or chat-oriented reports.

#### Scenario: App-state evidence references fixture target
- **GIVEN** live smoke selected a controlled GTK or Tk fixture window
- **WHEN** the harness calls `x11_get_app_state` for that window
- **THEN** evidence records the fixture id and selected window id
- **AND** the app-state window context matches the fixture title or class
- **AND** no non-fixture user application contents are required for the capability row

#### Scenario: Screenshot layer is summarized without data URL
- **GIVEN** `x11_get_app_state` includes a screenshot layer with image data
- **WHEN** the harness writes durable live evidence
- **THEN** ordinary logs and `evidence.json` omit the full `data:image/...;base64` payload
- **AND** they retain screenshot status, MIME/source metadata, dimensions when available, and an artifact file path when raw screenshot bytes are stored
- **AND** missing screenshot layer data is recorded as degraded or failure evidence instead of silently passing

#### Scenario: App-state layer classification feeds matrix
- **GIVEN** app-state diagnostics contain window, screenshot, and accessibility layer statuses
- **WHEN** matrix validation evaluates the `get_app_state` capability row
- **THEN** the row status reflects the layer outcomes and configured acceptance rules
- **AND** missing `diagnostics.layers` is classified as malformed evidence or degraded/fail, not as pass by omission

### Requirement: Metadata-only live smoke classifies missing fixtures safely
Live metadata-only app-state smoke MUST classify missing controlled fixture setup as a safety limitation, not as code failure or production pass evidence.

#### Scenario: No controlled fixture yields missing fixture setup
- **GIVEN** live metadata-only smoke is run without starting or selecting controlled fixtures
- **WHEN** app-state, screenshot, AT-SPI, keyboard, pointer, target, or overlay rows would require a safe target
- **THEN** those rows use `reason_category=missing_fixture_setup`
- **AND** the summary says it is not safe to test input against real user applications
- **AND** the run does not claim controlled live production readiness

#### Scenario: App-state layer degradation keeps usable metadata visible
- **GIVEN** a controlled X11 fixture target is selected
- **AND** screenshot or AT-SPI layers are degraded by environment limitations
- **WHEN** `x11_get_app_state` evidence is summarized
- **THEN** window context, target identity, and layer diagnostics remain visible
- **AND** degraded layers include canonical reason categories
- **AND** no full screenshot data URL is embedded in ordinary logs or summaries

### Requirement: App-state screenshot evidence is safe by default
The standalone app-state CLI and MCP tool MUST omit inline screenshot pixels from machine-readable JSON by default. A default `get-app-state --window-id <controlled_window_id> --json` report SHALL contain no `data:image` URI and no base64 screenshot payload; it SHALL include either a screenshot artifact path with metadata or an explicit `screenshot_error` / screenshot-disabled diagnostic while preserving usable non-screenshot layers.

#### Scenario: Default JSON has no inline screenshot blob
- **GIVEN** a controlled Cinnamon/X11 fixture window resolves by window id
- **AND** screenshot capture succeeds
- **WHEN** a developer runs `codex-computer-use-x11 get-app-state --window-id <controlled_window_id> --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** stdout does not contain `data:image`
- **AND** stdout does not contain `;base64,` for screenshot pixels
- **AND** `screenshot` contains path, MIME/type, bounds or dimensions when known, and provider/provenance metadata
- **AND** the referenced PNG file exists and is non-empty

#### Scenario: Screenshot disabled keeps other layers usable
- **GIVEN** a controlled Cinnamon/X11 fixture window resolves by window id
- **WHEN** a developer runs `codex-computer-use-x11 get-app-state --window-id <controlled_window_id> --no-screenshot --json`
- **THEN** the command exits with status code 0 when JSON can be emitted
- **AND** `screenshot` is null
- **AND** `screenshot_error` is null or a specific screenshot-disabled diagnostic
- **AND** `window_context`, diagnostics, and any available accessibility layer remain usable
- **AND** stdout contains no `data:image` or screenshot base64 payload

#### Scenario: Screenshot unavailable degrades only screenshot layer
- **GIVEN** a controlled Cinnamon/X11 fixture window resolves by window id
- **AND** the screenshot provider is unavailable or fails before a PNG can be written
- **WHEN** `get-app-state --window-id <controlled_window_id> --json` builds the report
- **THEN** `window_context` remains populated
- **AND** accessibility diagnostics remain populated according to AT-SPI availability
- **AND** `screenshot` is null or lacks a success path
- **AND** `screenshot_error` identifies the screenshot failure
- **AND** stdout contains no inline screenshot blob

### Requirement: App-state screenshot output path is caller-controllable
The standalone app-state CLI MUST provide an explicit way for callers to request where screenshot artifacts are written, such as `--screenshot-output <path>`. If no output path is supplied and screenshot capture is enabled, the implementation MUST use a deterministic safe generated evidence path under a caller/run log directory or documented temporary evidence directory; the JSON MUST report the resolved artifact path and not embed pixels.

#### Scenario: Caller supplied screenshot path is used
- **GIVEN** a caller passes `--screenshot-output target/e2e-logs/run/app-state.png`
- **AND** the parent directory exists and is writable
- **WHEN** `get-app-state --json` captures the screenshot
- **THEN** the provider writes the PNG to the resolved output path
- **AND** JSON reports that resolved path
- **AND** the reported file exists, is non-empty, and begins with the PNG signature
- **AND** no inline data URL appears in stdout

#### Scenario: Invalid screenshot output path fails the screenshot layer only
- **GIVEN** a caller passes `--screenshot-output` with a missing or unwritable parent directory
- **WHEN** `get-app-state --json` builds the report
- **THEN** the command still emits app-state JSON when non-screenshot layers can be serialized
- **AND** `screenshot` is null
- **AND** `screenshot_error` identifies the invalid output path
- **AND** `window_context` and accessibility diagnostics remain independent of that screenshot-layer failure

### Requirement: Inline app-state screenshots require explicit unsafe opt-in
If inline app-state screenshot serialization is retained for compatibility or debugging, it MUST require an explicit CLI/MCP opt-in and MUST be documented as unsafe for durable evidence logs. The opt-in MUST NOT be enabled by default and MUST NOT be used by the industrial evidence harness.

#### Scenario: Inline screenshot mode is explicit
- **GIVEN** the implementation supports an inline screenshot compatibility mode
- **WHEN** a developer runs `get-app-state --window-id <controlled_window_id> --json` without the inline flag
- **THEN** the JSON is path-only or diagnostic-only for screenshots
- **AND** no inline screenshot payload appears
- **WHEN** the developer reruns the command with the explicit inline opt-in
- **THEN** inline screenshot serialization may appear only in that opt-in output
- **AND** diagnostics or docs identify it as unsafe for durable evidence logs

### Requirement: Screenshot-crop remains path-only and unchanged
The existing `screenshot-crop` behavior MUST remain path-only and MUST NOT be changed to embed pixels or data URLs as part of app-state screenshot work. Regression coverage MUST prove screenshot-crop still writes caller-provided PNG files and emits metadata/diagnostics rather than inline image content.

#### Scenario: Screenshot-crop keeps path-only output
- **GIVEN** a controlled fixture window has valid bounds
- **WHEN** `screenshot-crop --window-id <controlled_window_id> --output <path> --json` succeeds
- **THEN** the JSON references the output path and output metadata
- **AND** the output file exists as a non-empty PNG
- **AND** stdout contains no `data:image` and no screenshot base64 payload

