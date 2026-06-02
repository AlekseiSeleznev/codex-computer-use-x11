## ADDED Requirements

### Requirement: Screenshot crop output integrity
The standalone screenshot-crop command MUST validate the screenshot provider outcome and the output file before reporting success. `success=true` SHALL be returned only when the provider reports success or equivalent completion and the output path exists, is readable, is non-empty, and begins with a PNG signature; provider false, missing output, empty output, unreadable output, or non-PNG output MUST produce `success=false` with a structured error.

#### Scenario: Provider false with no output fails
- **GIVEN** crop rectangle validation succeeds for a target window
- **AND** the screenshot provider returns a false result such as `(false, '<path>')`
- **AND** no file exists at the requested output path
- **WHEN** `screenshot-crop --window-id <fixture> --output <path> --json` completes
- **THEN** the command returns structured JSON with `success` equal to `false`
- **AND** `screenshot_invoked` is `true`
- **AND** `error_code` equals `ScreenshotOutputMissing` or a more specific provider-output error
- **AND** diagnostics include the provider false detail
- **AND** the report does not claim that a screenshot file was captured

#### Scenario: Provider true but empty output fails
- **GIVEN** crop rectangle validation succeeds
- **AND** the screenshot provider reports success
- **AND** the output file exists but has zero bytes
- **WHEN** `screenshot-crop` verifies the result
- **THEN** `success` is `false`
- **AND** `error_code` equals `ScreenshotOutputEmpty`
- **AND** diagnostics include the output path and verification failure without embedding image data

#### Scenario: Provider writes non-PNG output fails
- **GIVEN** crop rectangle validation succeeds
- **AND** the provider writes a non-empty file whose first bytes are not the PNG signature
- **WHEN** `screenshot-crop` verifies the result
- **THEN** `success` is `false`
- **AND** `error_code` equals `ScreenshotOutputInvalidFormat`
- **AND** diagnostics identify the expected PNG output contract

#### Scenario: Valid PNG output is the only success path
- **GIVEN** crop rectangle validation succeeds for a fixture window
- **AND** the screenshot provider writes a readable non-empty PNG file at the requested output path
- **WHEN** `screenshot-crop` verifies the result
- **THEN** `success` is `true`
- **AND** `output_path` equals the resolved output path
- **AND** diagnostics include output file size and format evidence
- **AND** stdout does not contain screenshot pixels or a data URL

### Requirement: Screenshot crop output path resolution
The screenshot-crop command MUST handle caller-provided output paths deterministically before invoking the provider. It MUST resolve relative paths against the process current working directory before provider invocation, MUST report the resolved absolute path in JSON, and MUST reject invalid or unavailable output locations with a structured preflight error.

#### Scenario: Relative output path is resolved before provider call
- **GIVEN** a caller passes `--output relative/crop.png`
- **WHEN** `screenshot-crop` validates output path handling
- **THEN** the command resolves the output path against the process current working directory
- **AND** the command reports the absolute resolved path in JSON
- **AND** the provider receives the resolved absolute path
- **AND** the command does not pass an ambiguous relative path to the provider while later reporting success for a missing file

#### Scenario: Unsafe output parent fails before provider call
- **GIVEN** the selected output path has a parent directory that does not exist or cannot be written
- **WHEN** `screenshot-crop` validates output path handling
- **THEN** `success` is `false`
- **AND** `screenshot_invoked` is `false`
- **AND** `error_code` equals `InvalidOutputPath` or `OutputPathUnavailable`
- **AND** diagnostics explain the path preflight failure
