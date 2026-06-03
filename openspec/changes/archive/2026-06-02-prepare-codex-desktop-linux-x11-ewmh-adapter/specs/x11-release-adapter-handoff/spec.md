## ADDED Requirements

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
