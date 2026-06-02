## MODIFIED Requirements

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

## ADDED Requirements

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
