## ADDED Requirements

### Requirement: Backup manifest covers source overlay and live asset writes
The source-overlay and provider-takeover installers MUST write a rollback manifest before each mutation that records before-state and after-state for source target files, live webview assets, ownership, mode, and sha256. The manifest MUST distinguish installer-changed state from state that was already present.

#### Scenario: Source overlay records file metadata before patching
- **GIVEN** a valid Codex Desktop Linux target checkout
- **WHEN** the source-overlay installer plans a marker-block or generated-file write
- **THEN** it records the target path, existence, ownership, mode, and sha256 before mutation
- **AND** it records the intended owner, mode, and sha256 after mutation
- **AND** the manifest entry identifies whether the installer changed the file or found it already applied

#### Scenario: Live asset patch records root-owned metadata
- **GIVEN** a live webview asset is selected for provider takeover patching
- **WHEN** live asset patching is authorized and available
- **THEN** the installer backs up the asset before patching
- **AND** it records ownership, mode, and sha256 for the live asset and backup
- **AND** it reports the live asset marker used to prove takeover patching

#### Scenario: Partial install manifest supports rollback
- **GIVEN** installation fails after completing one source or live-asset mutation
- **WHEN** rollback reads the manifest
- **THEN** it can identify completed writes and their before-state
- **AND** it does not assume later planned writes occurred

### Requirement: Rollback reports drift and blockers instead of blind restoration
Rollback and uninstall for source overlay, provider takeover, and live assets MUST compare current state with manifest after-state before restoring before-state. They MUST report drift or blockers when current bytes, ownership, or mode do not match the installer-owned state.

#### Scenario: Rollback restores unchanged installer-owned live asset
- **GIVEN** a manifest records a live asset before-state and installer after-state
- **AND** the current live asset still matches the recorded installer after-state
- **WHEN** rollback runs in write mode
- **THEN** it restores the recorded before-state bytes, ownership, and mode
- **AND** it records the restoration in the rollback report

#### Scenario: Rollback blocks on live asset drift
- **GIVEN** a manifest records a live asset after-state
- **AND** the current live asset sha256 differs from the recorded installer after-state
- **WHEN** rollback evaluates the asset
- **THEN** it refuses blind restoration for that asset
- **AND** it reports the drift as a blocker with the affected path and expected metadata

#### Scenario: Rollback absent overlay is idempotent
- **GIVEN** no source overlay or provider takeover manifest is present
- **WHEN** rollback runs with `--dry-run --report-json`
- **THEN** it emits a JSON report stating that no owned takeover state was found
- **AND** it does not delete unowned target files or live assets
