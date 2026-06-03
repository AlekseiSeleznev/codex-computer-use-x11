# x11-release-adapter-handoff Specification

## Purpose
Defines the release packaging, adapter contract, and copyable downstream Linux Feature scaffold required to hand off `codex-computer-use-x11` as an optional disabled-by-default X11/EWMH integration for Codex Desktop Linux.
## Requirements
### Requirement: Versioned release artifact with checksum
The project MUST provide a release packaging command that builds the current `VERSION` into a self-contained Codex plugin tarball and a matching SHA256 sidecar. The artifact name MUST include the project name, `v<VERSION>`, and target triple, and the checksum MUST verify the exact tarball bytes.

#### Scenario: Package current version into a stable artifact name
- **GIVEN** the repository `VERSION` file contains a semantic version
- **WHEN** a maintainer runs the release packaging command with a temporary output directory
- **THEN** it builds the release binary with Cargo
- **AND** it writes `codex-computer-use-x11-v<VERSION>-x86_64-unknown-linux-gnu.tar.gz` or the selected target-triple equivalent
- **AND** it writes a `.sha256` file next to that tarball
- **AND** the artifact filename includes the exact `VERSION` value

#### Scenario: Checksum verifies the tarball
- **GIVEN** the release packaging command has produced a tarball and `.sha256` file
- **WHEN** a verifier runs SHA256 verification against the sidecar file
- **THEN** the checksum verification succeeds
- **AND** the sidecar file names the tarball rather than an unrelated path

### Requirement: Release artifact contains a ready Codex plugin bundle
The release tarball MUST contain one top-level `codex-computer-use-x11/` directory with enough files for a downstream adapter to stage the plugin without reinterpreting the repository layout. The bundle MUST include `.mcp.json`, `.codex-plugin/plugin.json`, `assets/app-icon.png`, `bin/codex-computer-use-x11`, and release metadata.

#### Scenario: Extracted artifact has executable plugin binary
- **GIVEN** a release tarball produced by the packaging command
- **WHEN** a verifier extracts it into a temporary directory
- **THEN** `codex-computer-use-x11/bin/codex-computer-use-x11` exists
- **AND** the binary is executable
- **AND** running the extracted binary with `doctor --json` emits valid JSON or a structured degraded/blocker JSON report for the current environment

#### Scenario: Extracted MCP manifest starts the packaged binary
- **GIVEN** a release tarball produced by the packaging command
- **WHEN** a verifier reads `codex-computer-use-x11/.mcp.json`
- **THEN** the manifest includes server `codex-computer-use-x11`
- **AND** its command is `./bin/codex-computer-use-x11`
- **AND** its args are exactly `["mcp"]`
- **AND** its cwd is `.`

#### Scenario: Extracted plugin manifest matches project identity
- **GIVEN** a release tarball produced by the packaging command
- **WHEN** a verifier reads `codex-computer-use-x11/.codex-plugin/plugin.json`
- **THEN** the manifest name is `codex-computer-use-x11`
- **AND** its version equals the repository `VERSION`
- **AND** its interface display name is `X11 Computer Use`
- **AND** its short description identifies standalone `x11_*` tools for Linux X11/EWMH

#### Scenario: Release metadata records adapter handoff facts
- **GIVEN** a release tarball produced by the packaging command
- **WHEN** a verifier reads `codex-computer-use-x11/RELEASE-METADATA.json`
- **THEN** the metadata records plugin name `codex-computer-use-x11`
- **AND** it records the current version, command, args, display name, short description, baseline `x11-ewmh / Cinnamon X11`, source repository URL, release URL pattern, binary SHA256, and tarball SHA256 sidecar name

### Requirement: Release artifact excludes build, VCS, session, and local secret files
The packaging command MUST construct the artifact from an explicit bundle staging directory and MUST NOT include repository build outputs, VCS metadata, Codex session files, local backup files, or secret/local environment files.

#### Scenario: Forbidden files are absent from tarball listing
- **GIVEN** a release tarball produced by the packaging command
- **WHEN** a verifier lists every path inside the tarball
- **THEN** no path contains `.git/`
- **AND** no path contains `target/`
- **AND** no path contains `.codex/session/`
- **AND** no path matches `.secrets*`
- **AND** no path matches a local environment file such as `.env` or `*.local.env`
- **AND** no path matches a backup pattern such as `*.bak` or `*.bak.*`

### Requirement: Adapter contract documentation
The project MUST document the contract for a later upstream `codex-desktop-linux` Linux Feature adapter under `linux-features/x11-ewmh-computer-use/`. The documentation MUST say this repository remains the source of truth and the upstream adapter is a thin, disabled-by-default, fully opt-in adapter.

#### Scenario: Contract records upstream maintainer constraints
- **GIVEN** a maintainer opens the adapter contract document
- **WHEN** they read the scope and non-goals
- **THEN** it states that the upstream adapter must be disabled by default
- **AND** it must not modify core Computer Use behavior
- **AND** it must not replace the bundled `computer-use` plugin
- **AND** it must not change global doctor behavior
- **AND** it must not use submodules
- **AND** it must expose the existing namespaced `x11_*` plugin as `codex-computer-use-x11`

#### Scenario: Contract documents supported staging modes
- **GIVEN** a maintainer opens the adapter contract document
- **WHEN** they read the staging section
- **THEN** it documents pinned release artifact plus SHA256 verification mode
- **AND** it documents local checkout build mode using `CODEX_X11_COMPUTER_USE_SOURCE=/path/to/codex-computer-use-x11`
- **AND** it states that sha256 must be verified before staging downloaded artifacts
- **AND** it states that local source mode builds with `cargo build --release` in that checkout
- **AND** it states that the adapter must fail clearly when no staging mode can produce a binary or plugin tree

#### Scenario: Contract preserves X11 baseline boundaries
- **GIVEN** a maintainer opens the adapter contract document
- **WHEN** they read readiness and baseline guidance
- **THEN** it states that X11/EWMH readiness remains inside this plugin's `x11_doctor` or `doctor --json`
- **AND** it states that RemoteDesktop and Wayland remain debug-only or out of scope for this X11 baseline
- **AND** it does not describe upstream global doctor changes as required integration work
#### Scenario: Contract records backend flavor evaluation path
- **GIVEN** a maintainer opens the adapter contract document after reviewing GitHub issue #389
- **WHEN** they read the upstream path guidance
- **THEN** it states that the current thin Linux Feature adapter remains the default upstream-ready path unless a later change proves a better fit
- **AND** it states that `agent-sh/computer-use-linux` selectable backend/flavor integration is an optional future evaluation path
- **AND** it states that any backend/flavor work must be proposed separately from the disabled-by-default adapter handoff
- **AND** it states that backend/flavor selection must not change default `codex-desktop-linux` Computer Use behavior for users who do not opt in

### Requirement: Copyable downstream adapter scaffold
The project MUST provide a copyable scaffold for a later upstream `linux-features/x11-ewmh-computer-use/` feature. The scaffold MUST be inert in this repository runtime and MUST include feature metadata, README, stage hook, patch descriptors when needed, and self-contained tests.

#### Scenario: Feature manifest is disabled by default
- **GIVEN** the adapter scaffold contains `feature.json`
- **WHEN** a verifier reads the manifest
- **THEN** its id is `x11-ewmh-computer-use`
- **AND** the id matches `^[a-z0-9][a-z0-9-]*$`
- **AND** `defaultEnabled` is false
- **AND** it exposes a stage hook entrypoint
- **AND** any patch descriptor entrypoint is narrow and feature-owned

#### Scenario: Scaffold README explains opt-in enablement and non-goals
- **GIVEN** the adapter scaffold contains `README.md`
- **WHEN** a maintainer reads it
- **THEN** it shows enabling through git-ignored `linux-features/features.json` with `x11-ewmh-computer-use`
- **AND** it identifies Linux Mint Cinnamon on X11 / `x11-ewmh` as the supported baseline
- **AND** it lists the `x11_*` tools exposed by the plugin
- **AND** it documents pinned artifact, direct binary, and local source modes
- **AND** it states no core Computer Use replacement, no Wayland/RemoteDesktop baseline, no default enablement, no submodule, and no global doctor changes

#### Scenario: Stage hook writes only app install resources
- **GIVEN** the adapter scaffold `stage.sh` runs inside an upstream Linux Feature build
- **WHEN** it stages the plugin from a verified tarball, direct binary, or local source build
- **THEN** it writes the plugin under `$INSTALL_DIR/resources/plugins/openai-bundled/plugins/codex-computer-use-x11`
- **AND** it writes or updates `$INSTALL_DIR/resources/plugins/openai-bundled/.agents/plugins/marketplace.json`
- **AND** it adds a marketplace entry named `codex-computer-use-x11` with local source path `./plugins/codex-computer-use-x11`
- **AND** it does not write user-home files from `stage.sh`
- **AND** it does not touch existing `computer-use` plugin files or global app doctor files

#### Scenario: Scaffold tests prove disabled feature behavior and staging safety
- **GIVEN** the adapter scaffold contains `test.js`
- **WHEN** the self-contained Node tests run in the upstream repository context
- **THEN** they prove the feature has no stage hook or patch descriptors when not enabled
- **AND** they prove the stage hook and patch descriptors are visible when `features.json` enables `x11-ewmh-computer-use`
- **AND** they stage a fake executable binary into a temporary install directory
- **AND** they assert `.mcp.json`, executable binary, and marketplace entry exist
- **AND** they assert an existing `computer-use` plugin fixture remains untouched

#### Scenario: Plugin gate patch is idempotent and narrow
- **GIVEN** the scaffold includes a plugin gate patch descriptor
- **WHEN** tests apply the patch to representative upstream bundle shapes
- **THEN** the patch adds only the `codex-computer-use-x11` Linux plugin gate
- **AND** applying the patch twice produces the same output as applying it once
- **AND** it does not change existing `computer-use` descriptor behavior
#### Scenario: Scaffold README separates adapter and backend flavor paths
- **GIVEN** the adapter scaffold contains `README.md`
- **WHEN** a maintainer reads the upstream alignment section
- **THEN** it states that this scaffold wires the separate `codex-computer-use-x11` plugin as an opt-in Linux Feature
- **AND** it identifies `agent-sh/computer-use-linux` selectable backend/flavor integration as a separate future investigation rather than scaffold behavior
- **AND** it states no backend/flavor experiment may require enabling this feature by default or modifying core Computer Use behavior in the scaffold

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

