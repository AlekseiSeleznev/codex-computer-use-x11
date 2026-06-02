## ADDED Requirements

### Requirement: Fresh install activates standalone plugin and accessibility baseline
The user-local standalone installer MUST install the owned plugin bundle and, when requested for the Cinnamon/X11 baseline, MUST safely activate required accessibility environment state without enabling screen readers or overwriting unrelated user settings.

#### Scenario: Install records plugin and activation before-state
- **GIVEN** a user runs the standalone installer in fresh-install mode
- **WHEN** the installer plans owned plugin files and accessibility activation changes
- **THEN** it records before-state for every owned `$CODEX_HOME` plugin cache, marketplace, and config path it may change
- **AND** it records before-state for relevant user systemd and dbus activation environment variables
- **AND** it records before-state for `org.gnome.desktop.interface toolkit-accessibility`
- **AND** the manifest marks whether each value was changed by the installer or was already acceptable

#### Scenario: Install neutralizes disabling bridge environment only when safe
- **GIVEN** the user activation environment contains `NO_AT_BRIDGE=1`
- **WHEN** fresh install activates the Cinnamon/X11 accessibility baseline
- **THEN** the installer records the original value in the backup manifest
- **AND** it removes or neutralizes `NO_AT_BRIDGE=1` only for user activation environments it owns or can safely update
- **AND** it reports any environment it cannot safely update as a blocker or degraded setup item

#### Scenario: Install does not enable Orca implicitly
- **GIVEN** fresh install activates AT-SPI and toolkit accessibility
- **WHEN** the installer applies accessibility setup
- **THEN** it MUST NOT enable Orca or any screen-reader autostart setting
- **AND** it MUST NOT require screen-reader activation for the doctor AT-SPI probe to pass

### Requirement: Standalone uninstall restores manifest-owned user state
The standalone uninstaller MUST restore only installer-owned plugin and accessibility activation changes from the manifest, MUST be idempotent for absent or partial installs, and MUST support dry-run and report-json output.

#### Scenario: Uninstall restores changed activation environment
- **GIVEN** a backup manifest records that the installer changed `NO_AT_BRIDGE`, `GTK_MODULES`, or `QT_ACCESSIBILITY`
- **WHEN** a user runs uninstall with write mode
- **THEN** the uninstaller restores the recorded previous values or absence state
- **AND** it leaves values unchanged when the manifest says they were already acceptable before install

#### Scenario: Dry-run reports without mutation
- **GIVEN** a manifest-backed install is present
- **WHEN** a user runs standalone uninstall with `--dry-run --report-json`
- **THEN** stdout contains one JSON report describing planned removals and restorations
- **AND** no `$CODEX_HOME`, gsettings, or activation-environment state is changed

#### Scenario: Partial install can be rolled back
- **GIVEN** installation failed after writing some plugin paths and recording a partial manifest
- **WHEN** a user runs standalone uninstall
- **THEN** the uninstaller removes or restores only manifest-owned changes that were completed
- **AND** it reports missing or already-restored items as idempotent outcomes rather than failures
