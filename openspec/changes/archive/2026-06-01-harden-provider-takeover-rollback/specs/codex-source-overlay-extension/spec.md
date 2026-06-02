## MODIFIED Requirements

### Requirement: Live asset backup and patch report
When the takeover installer mutates live Codex Desktop assets, it MUST create restorable backups before writing patched assets and MUST write a machine-readable patch report. The recorded backup metadata MUST be sufficient for uninstall to restore the original bytes, owner, group, mode, size, and checksum or to stop with a safe blocker before modifying the live asset.

#### Scenario: Backup live Computer Use settings asset before patch
- **GIVEN** the live asset directory contains `computer-use-settings-*.js`
- **WHEN** the takeover installer patches live assets
- **THEN** each live asset written by the installer is first copied to a timestamped backup path
- **AND** the backup path is recorded in the provider takeover manifest and patch report
- **AND** the backup metadata records original checksum, size, owner, group, and mode when available
- **AND** the patched asset contains an owned provider-takeover marker
- **AND** the report includes the restart hint required for the running Electron process to load the changed asset

#### Scenario: Dry-run reports target and live asset backup plan without mutation
- **GIVEN** a valid target checkout and live asset path
- **WHEN** a developer runs the takeover installer in dry-run or report-only mode
- **THEN** the command reports planned target and live-asset mutations
- **AND** it reports the backup paths and manifest path that would be used
- **AND** it does not write target source files, live assets, backups, or persistent settings

#### Scenario: Install failure rolls back completed takeover writes
- **GIVEN** the takeover installer has already written one owned source file or live asset during the current install transaction
- **AND** a later source file or live asset write fails
- **WHEN** the installer handles the failure
- **THEN** it attempts to restore every file changed in the current transaction from the transaction backup manifest
- **AND** it leaves a failure report that records which restores succeeded or failed
- **AND** it does not claim the provider takeover is installed unless all required writes and manifest updates succeeded

### Requirement: Rollback restores bundled mode
The installer MUST provide rollback or restore behavior that removes only owned X11 takeover mutations and returns the target and live settings assets to bundled Computer Use mode. Rollback MUST be available through a one-command provider-takeover uninstaller that mirrors the one-command installer.

#### Scenario: One-command rollback restores all owned takeover surfaces
- **GIVEN** `scripts/install-x11-provider-takeover.sh` applied the standalone plugin, provider source overlay, and live asset patch
- **WHEN** a developer runs `scripts/uninstall-x11-provider-takeover.sh` for the same target and Codex home
- **THEN** the standalone plugin cache, marketplace, and owned config sections are removed
- **AND** target source files are restored from the provider takeover manifest backups
- **AND** live assets are restored from recorded backups when those backups exist
- **AND** owned provider-takeover metadata is removed only after source and live asset restore succeeds
- **AND** the rollback report records the restored files and the resulting bundled-mode state

#### Scenario: Rollback refuses unsafe live asset drift
- **GIVEN** a live asset backup is recorded in the provider takeover manifest
- **AND** the current live asset no longer contains the owned provider-takeover marker or its current checksum does not match the expected installed checksum
- **WHEN** a developer runs the provider-takeover uninstaller
- **THEN** the uninstaller stops before overwriting that live asset
- **AND** the report identifies the drifted asset and required manual action
- **AND** unrelated plugin caches, bundled Computer Use files, and unrelated live assets remain unchanged

#### Scenario: Rollback is safe when takeover is absent
- **GIVEN** the target checkout, standalone plugin state, and live assets do not contain owned takeover markers
- **WHEN** a developer runs provider-takeover rollback
- **THEN** the command exits successfully with a no-op clean report
- **AND** it does not remove bundled Computer Use files, Chrome files, unrelated plugins, unrelated backups, or unrelated target changes

#### Scenario: Rollback reports missing manifest instead of blind deletion
- **GIVEN** a target checkout or live asset still contains owned provider-takeover markers
- **AND** the provider takeover manifest is missing or lacks a backup for a marked file
- **WHEN** a developer runs provider-takeover rollback
- **THEN** the command fails with a safe blocker that names the missing manifest or backup
- **AND** it does not blindly delete marked code or modify live assets without a restorable backup
