## ADDED Requirements

### Requirement: Codex Desktop Linux feature installer
The project MUST provide a local installer for the optional Codex Desktop Linux `x11-ewmh-computer-use` Linux Feature. The installer MUST be fully opt-in, MUST default to the documented local target checkout only when no explicit `--target` is supplied, MUST support dry-run and machine-readable report output, and MUST NOT change users who do not run the installer.

#### Scenario: Dry-run reports planned feature install without mutation
- **GIVEN** a Codex Desktop Linux target checkout and install directory fixture
- **WHEN** a developer runs the feature installer with `--dry-run --report-json -`
- **THEN** the command exits with status code 0
- **AND** the JSON report identifies the target checkout, install directory, feature id `x11-ewmh-computer-use`, and planned surfaces
- **AND** no target feature files, install plugin files, marketplace files, app assets, update-builder files, or rollback manifest files are written

#### Scenario: Install enables the local feature and stages the standalone plugin
- **GIVEN** a Codex Desktop Linux target checkout fixture
- **AND** a Codex Desktop install directory fixture with an existing bundled `computer-use` plugin and marketplace entry
- **WHEN** a developer runs the feature installer with a local `codex-computer-use-x11` source or binary
- **THEN** the adapter scaffold is copied under the target checkout's local Linux Features area
- **AND** the target checkout's ignored feature config enables `x11-ewmh-computer-use`
- **AND** the install directory contains `resources/plugins/openai-bundled/plugins/codex-computer-use-x11`
- **AND** the marketplace contains an entry named `codex-computer-use-x11` pointing at `./plugins/codex-computer-use-x11`
- **AND** the pre-existing bundled `computer-use` plugin directory and marketplace entry remain present and unmodified

#### Scenario: Install patches live assets only as an owned optional surface
- **GIVEN** a Codex Desktop install directory with `resources/app.asar` and `content/webview`
- **WHEN** the installer is allowed to patch app assets
- **THEN** it records before-state for the live assets before mutation
- **AND** it applies only the enabled `x11-ewmh-computer-use` Linux Feature patch descriptors
- **AND** it refreshes extracted webview assets from the patched app state when applicable
- **AND** the report states whether app patching was applied, skipped, or blocked

### Requirement: Rollback-first feature install manifest
The feature installer MUST create a non-secret rollback manifest before mutating installer-owned surfaces and MUST record completed after-state for each mutation. The manifest MUST distinguish changed state from already-acceptable state, include checksums and backup paths for restorable file-like state, and support partial-install cleanup.

#### Scenario: Manifest records each installer-owned surface
- **GIVEN** a successful feature install
- **WHEN** a verifier reads the rollback manifest
- **THEN** it records entries for the target local feature scaffold, target feature config, staged plugin directory, marketplace file, update-builder feature/config when present, and optional app/webview assets when patched
- **AND** each completed entry records whether the installer changed it or found it already acceptable
- **AND** restorable entries include non-secret before-state, after-state, checksums, mode, and backup location where applicable
- **AND** the manifest does not include secrets, tokens, broad environment dumps, private URLs, or local secret file contents

#### Scenario: Partial install can be rolled back safely
- **GIVEN** an installation run records a completed subset of planned entries before failing
- **WHEN** the uninstaller or rollback path consumes the manifest
- **THEN** it attempts to restore only completed installer-owned entries
- **AND** it leaves unstarted planned entries alone
- **AND** it reports every restored, skipped, and blocked entry in machine-readable output

### Requirement: Codex Desktop Linux feature uninstaller
The project MUST provide a local uninstaller for the optional Codex Desktop Linux feature install. The uninstaller MUST be idempotent when no manifest or owned install is present, MUST restore installer-owned changes from the manifest, and MUST refuse blind restoration when current state has drifted from the recorded installer after-state.

#### Scenario: Uninstall restores a clean feature install
- **GIVEN** a fixture after a successful feature install
- **WHEN** a developer runs the feature uninstaller with `--report-json -`
- **THEN** the command exits with status code 0
- **AND** the staged `codex-computer-use-x11` plugin is removed or restored to its exact before-state
- **AND** the marketplace is restored to its exact before-state while preserving the bundled `computer-use` entry
- **AND** target/update-builder feature config and optional app/webview assets are restored to their recorded before-state
- **AND** the report identifies every restored and skipped entry

#### Scenario: Uninstall blocks on drift
- **GIVEN** a fixture after a successful feature install
- **AND** a current installer-owned file was modified after install
- **WHEN** a developer runs the feature uninstaller
- **THEN** the command exits with a non-zero status code
- **AND** it reports a drift blocker identifying the changed path and expected recorded after-state
- **AND** it does not overwrite the drifted path with stale before-state

#### Scenario: Uninstall dry-run does not mutate
- **GIVEN** a fixture after a successful feature install
- **WHEN** a developer runs the feature uninstaller with `--dry-run --report-json -`
- **THEN** the command exits with status code 0
- **AND** the JSON report lists planned restores/removals
- **AND** plugin, marketplace, feature config, manifest, and app assets remain unchanged
