## ADDED Requirements

### Requirement: README provides safe v1 quick start
The project MUST provide a README quick start that explains the v1 posture as Codex-first, Cinnamon/X11-first, generic X11/EWMH, and separates standalone plugin usage from source-overlay usage. The quick start MUST include commands that match actual script names and MUST state that Cinnamon Wayland and Cinnamon/Muffin extensions are out of scope for v1.

#### Scenario: User can identify supported delivery paths
- **GIVEN** a user opens `README.md`
- **WHEN** they read the quick-start and delivery-path sections
- **THEN** the README describes the standalone user-local Codex MCP plugin path
- **AND** it describes the reversible source-overlay path for the Codex Desktop Linux target checkout
- **AND** it does not imply that Cinnamon Wayland, a Cinnamon/Muffin extension, or native packaging is supported in v1

#### Scenario: README commands match public scripts
- **GIVEN** the repository contains `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/status-codex-source-overlay.sh`, `scripts/install-codex-source-overlay.sh`, `scripts/uninstall-codex-source-overlay.sh`, and e2e smoke scripts
- **WHEN** documentation checks inspect README command snippets
- **THEN** each referenced project script exists at the documented path
- **AND** each documented `--help` or `--dry-run` command exits successfully or is explicitly documented as live/environment-dependent
- **AND** no README snippet references a removed stock target tool such as `focus_window` or `mousemove` as a required target capability

### Requirement: Install and uninstall docs are executable and rollback-first
The project MUST provide install/uninstall documentation for both v1 delivery paths. The docs MUST prefer dry-run/fake checks before live mutation, MUST show rollback commands, MUST describe the files or directories owned by the project, and MUST avoid sudo or system-wide writes except where future upstream target documentation explicitly owns them.

#### Scenario: Standalone plugin docs show user-local install and rollback
- **GIVEN** a user wants to test the standalone Codex MCP plugin
- **WHEN** they follow the install documentation
- **THEN** they can run a dry-run install command before mutating real `CODEX_HOME`
- **AND** the docs state that installer-owned state is under the user-local Codex plugin marketplace/cache namespace for `codex-computer-use-x11`
- **AND** the docs show `scripts/uninstall-codex-plugin.sh` as the rollback command
- **AND** the docs do not instruct the user to modify `/opt`, `openai-bundled`, or bundled `computer-use` cache entries for the standalone path

#### Scenario: Source-overlay docs show status, install, target checks, and uninstall
- **GIVEN** a user has a Codex Desktop Linux target checkout
- **WHEN** they follow the source-overlay documentation
- **THEN** the docs show `scripts/status-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"` before install
- **AND** the docs show `scripts/install-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"`
- **AND** the docs show target test or smoke commands while the overlay is applied
- **AND** the docs show `scripts/uninstall-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"` and final status/`git status --short` checks

### Requirement: Troubleshooting covers degraded layers without fabricating success
The project MUST provide troubleshooting documentation for Cinnamon/X11 readiness, missing external commands, plugin installation, source-overlay drift, screenshot/AT-SPI degraded layers, strict RemoteDesktop false positives, and e2e evidence failures. Troubleshooting MUST distinguish deterministic fake/dry-run checks from optional live checks.

#### Scenario: Doctor and dependency troubleshooting is layered
- **GIVEN** `cargo run -- doctor --json` reports degraded or unavailable capabilities
- **WHEN** a user reads troubleshooting documentation
- **THEN** the docs explain how to inspect X11 session variables, `wmctrl`, `xprop`, `xdotool`, `ydotool`, screenshot provider, AT-SPI, and portal readiness separately
- **AND** the docs state that an empty RemoteDesktop introspection table is not enough to mark portal input available
- **AND** the docs recommend fake/dry-run evidence when live desktop capabilities are unavailable

#### Scenario: Source-overlay drift troubleshooting preserves target safety
- **GIVEN** `scripts/status-codex-source-overlay.sh` reports `state=drifted`
- **WHEN** a user reads troubleshooting documentation
- **THEN** the docs explain that drift means owned markers, generated backend content, anchors, or metadata do not match expectations
- **AND** the docs direct the user to inspect target git status before reinstalling or uninstalling
- **AND** the docs do not recommend overwriting unowned target code or native X11 backend files blindly

### Requirement: License and attribution notes classify reuse boundaries
The project MUST provide license and attribution documentation that lists the refreshed reference projects and command dependencies, their observed SPDX status where available, and the allowed reuse policy. The documentation MUST distinguish invoking external runtime commands from copying/vendoring source code.

#### Scenario: Copy-safe and copy-unsafe references are explicit
- **GIVEN** a maintainer evaluates whether to copy or adapt external code
- **WHEN** they read the license/attribution notes
- **THEN** MIT and Apache-2.0 references are marked potentially copy-safe only with required attribution or NOTICE handling
- **AND** NOASSERTION, no-license, GPL, AGPL, and unclear-license references are marked copy-unsafe for MIT upstream code unless a later explicit license decision changes scope
- **AND** Linux Mint/Cinnamon/Muffin sources are described as ideas/specification references only for this MIT-targeted project

#### Scenario: Runtime command invocation is not treated as vendoring
- **GIVEN** project code or docs mention `wmctrl`, `xdotool`, `ydotool`, or `x11rb`
- **WHEN** a maintainer reads the license/attribution notes
- **THEN** the notes state that invoking an installed command at runtime is distinct from copying or vendoring its source code
- **AND** the notes flag `wmctrl` GPL and `ydotool` AGPL source copying as disallowed without separate review
- **AND** the notes flag `xdotool` BSD-3-Clause and `x11rb` Apache-2.0 as requiring normal attribution/license compliance if code or library dependency reuse is introduced

### Requirement: Upstreaming guide separates backend and packaging targets
The project MUST provide an upstreaming guide with a target matrix that separates backend/windowing contributions from Codex Desktop packaging and wrapper integration. The guide MUST identify `agent-sh/computer-use-linux` as the expected backend-upstream target and `codex-desktop-linux-full` as the local packaging/integration target, unless fresh research supersedes that mapping.

#### Scenario: Backend-upstream and wrapper-integration targets are distinct
- **GIVEN** a maintainer prepares upstream work
- **WHEN** they read the upstream target matrix
- **THEN** backend/windowing, diagnostics, AT-SPI correlation, screenshot integration, and input-safety changes are mapped to the Computer Use Linux backend lineage
- **AND** Codex Desktop packaging, Linux feature toggles, launcher/update-manager wiring, and bundled-plugin staging are mapped to the Codex Desktop Linux wrapper/integration lineage
- **AND** the guide warns not to mix these targets in one pull request unless a later design explicitly accepts that coupling

#### Scenario: Source overlay remains reversible until upstreamed
- **GIVEN** the source overlay exists as local integration evidence
- **WHEN** a maintainer reads the upstreaming guide
- **THEN** the guide states that the overlay is a reversible staging mechanism, not a long-lived fork
- **AND** it requires clean install/test/uninstall evidence before claiming upstream-readiness
- **AND** it preserves existing stock tool vocabulary such as `activate_window`, `get_app_state`, `type_text`, `press_key`, `click`, `scroll`, and `drag`

### Requirement: Release checklist gates v1 handoff evidence
The project MUST provide a release checklist that gates v1 handoff on OpenSpec validation, project checks, docs checks, fake e2e evidence, optional live evidence, source-overlay rollback, license review, and Git cleanliness. The checklist MUST avoid requiring secrets or private endpoints.

#### Scenario: Release checklist includes required verification commands
- **GIVEN** a maintainer prepares a v1 handoff or archive
- **WHEN** they read the release checklist
- **THEN** it includes `openspec validate --all --strict`, `make fmt`, `make check`, and `make test`
- **AND** it includes fake plugin and source-overlay e2e smoke commands with capability-matrix validation
- **AND** it includes optional live source-overlay smoke guidance only when the target checkout is available and clean
- **AND** it includes final `git status --short` checks for both project and target checkout

#### Scenario: Release checklist preserves secret handling
- **GIVEN** release evidence is being collected
- **WHEN** a maintainer follows the release checklist
- **THEN** the checklist says not to read, print, commit, or archive `.secrets.local.env`
- **AND** it records variable names such as `CODEX_DESKTOP_LINUX_FULL_PATH` without storing private values
- **AND** it requires logs/evidence to avoid tokens, credentials, and private local configuration contents
