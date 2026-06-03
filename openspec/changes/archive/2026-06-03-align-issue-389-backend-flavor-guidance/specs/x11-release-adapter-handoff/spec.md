## MODIFIED Requirements

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
