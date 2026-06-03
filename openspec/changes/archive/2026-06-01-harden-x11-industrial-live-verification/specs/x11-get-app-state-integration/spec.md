## ADDED Requirements

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
