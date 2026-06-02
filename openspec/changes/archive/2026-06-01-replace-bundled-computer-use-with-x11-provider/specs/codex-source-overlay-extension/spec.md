## ADDED Requirements

### Requirement: Installer applies X11 provider takeover overlay
The project installer MUST be able to apply an X11 provider takeover overlay over the configured Codex Desktop Linux target checkout. The command MUST accept `--provider x11 --mode takeover`, MUST resolve the target from `--target`, `CODEX_DESKTOP_LINUX_FULL_PATH`, or the documented local default, and MUST fail clearly before mutation when required target files or anchors are missing.

#### Scenario: Apply takeover overlay to target checkout
- **GIVEN** `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` or `CODEX_DESKTOP_LINUX_FULL_PATH` points to a valid target checkout
- **WHEN** a developer runs the installer with `--provider x11 --mode takeover`
- **THEN** the installer applies only owned provider-takeover patch content to the target checkout
- **AND** the patch report records the target path, target git commit when available, provider `x11`, mode `takeover`, and every file changed
- **AND** the report includes a restart hint for Codex Desktop

#### Scenario: Refuse unsupported provider or mode
- **GIVEN** a valid target checkout
- **WHEN** a developer runs the installer with an unsupported provider or unsupported mode
- **THEN** the command exits with a non-zero status code
- **AND** stderr explains the supported values including `--provider x11 --mode takeover`
- **AND** no target source files or live assets are changed

### Requirement: Live asset backup and patch report
When the takeover installer mutates live Codex Desktop assets, it MUST create restorable backups before writing patched assets and MUST write a machine-readable patch report.

#### Scenario: Backup live Computer Use settings asset before patch
- **GIVEN** the live asset directory contains `computer-use-settings-*.js`
- **WHEN** the takeover installer patches live assets
- **THEN** each live asset written by the installer is first copied to a timestamped backup path
- **AND** the backup path is recorded in the patch report
- **AND** the patched asset contains an owned provider-takeover marker
- **AND** the report includes the restart hint required for the running Electron process to load the changed asset

#### Scenario: Report diagnostic-only dry run
- **GIVEN** a valid target checkout and live asset path
- **WHEN** a developer runs the takeover installer in dry-run or report-only mode
- **THEN** the command reports planned target and live-asset mutations
- **AND** it records whether baseline bundled Computer Use diagnostics can be collected
- **AND** it does not write target source files, live assets, backups, or persistent settings

### Requirement: Rollback restores bundled mode
The installer MUST provide rollback or restore behavior that removes only owned X11 takeover mutations and returns the target and live settings assets to bundled Computer Use mode.

#### Scenario: Roll back owned takeover overlay
- **GIVEN** the X11 provider takeover overlay has been applied by the installer
- **WHEN** a developer runs rollback for the same target
- **THEN** owned provider-takeover marker blocks are removed from target source files
- **AND** live assets are restored from recorded backups when those backups exist
- **AND** unrelated user changes and unrelated target patches remain untouched
- **AND** the rollback report records the restored files and the resulting bundled-mode state

#### Scenario: Rollback is safe when takeover is absent
- **GIVEN** the target checkout and live assets do not contain owned takeover markers
- **WHEN** a developer runs rollback
- **THEN** the command exits successfully or with a clear no-op status
- **AND** it does not remove bundled Computer Use files, Chrome files, unrelated plugins, or unrelated backups
