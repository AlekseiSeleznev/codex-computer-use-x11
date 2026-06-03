## ADDED Requirements

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
