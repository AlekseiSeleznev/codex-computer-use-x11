# x11-packaging-docs-upstreaming Specification

## Purpose
Defines the documentation, attribution, upstreaming, troubleshooting, and release-checklist contract for the v1 X11/EWMH handoff.
## Requirements
### Requirement: README provides safe v1 quick start
The project MUST provide a README quick start that explains the v1 posture as Codex-first, Cinnamon/X11-first, generic X11/EWMH, and separates standalone plugin usage from reversible source-overlay usage. The quick start MUST include commands that match actual script names, MUST state that Cinnamon Wayland and Cinnamon/Muffin extensions are out of scope for v1, and MUST avoid stale wording that describes implemented source-overlay tooling as merely future or read-only.

#### Scenario: User can identify supported delivery paths
- **GIVEN** a user opens `README.md`
- **WHEN** they read the quick-start and delivery-path sections
- **THEN** the README describes the standalone user-local Codex MCP plugin path
- **AND** it describes the reversible source-overlay path for the Codex Desktop Linux target checkout
- **AND** it makes clear that source-overlay scripts modify only owned marker blocks when explicitly invoked and are reversible through the documented uninstall command
- **AND** it does not imply that source overlay is only future work or that the target checkout is always read-only during explicit overlay install/uninstall operations
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
The project MUST provide troubleshooting documentation for Cinnamon/X11 readiness, missing X11 commands, plugin installation, source-overlay drift, screenshot/AT-SPI degraded layers, and e2e evidence failures. Troubleshooting MUST distinguish deterministic fake/dry-run checks from optional live checks, and MUST NOT present RemoteDesktop portal or Wayland remediation as part of the current standalone `x11-ewmh` plugin readiness path.

#### Scenario: Doctor and dependency troubleshooting is X11-scoped
- **GIVEN** `cargo run -- doctor --json` reports degraded or unavailable X11-baseline capabilities
- **WHEN** a user reads troubleshooting documentation
- **THEN** the docs explain how to inspect X11 session variables, `wmctrl`, `xprop`, `xdotool`, `ydotool`, screenshot provider, and AT-SPI separately
- **AND** the docs recommend fake/dry-run evidence when live desktop capabilities are unavailable
- **AND** the docs do not tell users to fix RemoteDesktop portal or Wayland capabilities as a readiness step for the standalone X11 plugin

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
The project MUST provide a release checklist that gates v1 handoff on OpenSpec validation, project checks, docs checks, fake e2e evidence, optional live evidence, source-overlay rollback, license review, and Git cleanliness. The checklist MUST avoid requiring secrets or private endpoints and MUST list commands that remain valid after the individual OpenSpec change that introduced them has been archived.

#### Scenario: Release checklist includes required verification commands
- **GIVEN** a maintainer prepares a v1 handoff or archive
- **WHEN** they read the release checklist
- **THEN** it includes `openspec validate --all --strict`, `make fmt`, `make check`, and `make test`
- **AND** it includes fake plugin and source-overlay e2e smoke commands with capability-matrix validation
- **AND** it includes optional live source-overlay smoke guidance only when the target checkout is available and clean
- **AND** it includes final `git status --short` checks for both project and target checkout
- **AND** it does not require validating an already-archived change by active change name as a release gate

#### Scenario: Release checklist preserves secret handling
- **GIVEN** release evidence is being collected
- **WHEN** a maintainer follows the release checklist
- **THEN** the checklist says not to read, print, commit, or archive `.secrets.local.env`
- **AND** it records variable names such as `CODEX_DESKTOP_LINUX_FULL_PATH` without storing private values
- **AND** it requires logs/evidence to avoid tokens, credentials, and private local configuration contents

### Requirement: Documentation examples avoid broken illustrative links
Project documentation that shows illustrative local paths MUST NOT use Markdown link syntax for files that are intentionally absent from this repository. Such examples MUST be rendered as code literals, plain text, or explicitly marked placeholders so local Markdown link checks do not confuse them with required tracked files.

#### Scenario: Skill template examples are not broken links
- **GIVEN** `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md` contains example glossary or context paths
- **WHEN** documentation checks inspect Markdown links
- **THEN** illustrative example paths such as `src/ordering/CONTEXT.md`, `src/payments/CONTEXT.md`, and `docs/glossary.md` are not encoded as local Markdown links unless those files exist
- **AND** the examples remain readable as path examples for future projects

### Requirement: Documentation explains X11-only production readiness semantics
README, troubleshooting, and retest documentation MUST explain PASS, DEGRADED, FAIL, doctor readiness, controlled-fixture evidence, and Wayland out-of-scope product status for the Cinnamon/X11 baseline. Documentation MUST state that RemoteDesktop portal and Wayland support are not current standalone plugin readiness diagnostics and that their absence does not degrade the `x11-ewmh` doctor baseline.

#### Scenario: Reader can interpret pass and degraded rows
- **GIVEN** a developer opens the production-readiness or troubleshooting documentation
- **WHEN** they read the capability matrix guidance
- **THEN** PASS means the capability has concrete evidence for the stated delivery path and fixture mode
- **AND** DEGRADED means a documented X11-baseline limitation with a reason category and evidence path, not hidden success
- **AND** FAIL means a code, safety, cleanup, or integrity issue that blocks production-readiness claims
- **AND** missing RemoteDesktop portal or Wayland support is not described as a DEGRADED doctor-readiness row for the standalone X11 plugin

#### Scenario: Reader can run safe full retest
- **GIVEN** a developer wants to retest the installed plugin
- **WHEN** they follow the documented safe full retest instructions
- **THEN** the instructions avoid `.secrets.local.env` and external credentials
- **AND** they identify fake smoke, controlled live fixture smoke, optional metadata-only smoke, doctor JSON validation, and matrix validation commands
- **AND** they warn that input/pointer/overlay checks must target controlled fixtures only

#### Scenario: Wayland status is unambiguous without doctor noise
- **GIVEN** a reader wants to understand Wayland or portal scope
- **WHEN** they consult README, troubleshooting, or retest documentation
- **THEN** the documentation states that Wayland support and portal-required runtime paths are outside the current standalone X11 plugin scope
- **AND** the documentation does not describe RemoteDesktop portal absence or `WAYLAND_DISPLAY` presence as current `doctor --json` readiness degraded reasons, optional enrichments, blockers, or next-step recommendations
- **AND** the documentation directs users to validate the X11 `x11-ewmh` baseline rather than fixing RemoteDesktop portal or Wayland for this plugin

### Requirement: Troubleshooting explains bus-reachable tree-unavailable AT-SPI
Troubleshooting and retest documentation MUST include a dedicated Cinnamon/X11 section for `atspi_bus_available=true` with `tree_available=false`, including the bridge-disabled environment path and safe controlled-fixture verification.

#### Scenario: Reader can diagnose NO_AT_BRIDGE bridge suppression
- **GIVEN** a developer sees `diagnostic_state=atspi_gtk_bridge_disabled_by_environment` or `atspi_tree_extraction_unavailable`
- **WHEN** they read troubleshooting documentation
- **THEN** the docs explain that AT-SPI bus reachability is different from GTK/ATK tree extraction
- **AND** the docs tell them to inspect package availability for `at-spi2-core`, `libatk-adaptor`, `libatk-bridge2.0-0t64`, and `libatspi2.0-0t64` or distribution equivalents
- **AND** the docs tell them to check toolkit accessibility settings and AT-SPI processes such as the bus launcher, registry daemon, and AT-SPI DBus daemon
- **AND** the docs identify inherited `NO_AT_BRIDGE=1` as a bridge-disable signal that should be removed or not inherited by GTK fixture/application processes

#### Scenario: Reader gets safe remediation steps
- **GIVEN** the operator wants to recover semantic AT-SPI evidence on Cinnamon/X11
- **WHEN** they follow the remediation section
- **THEN** it says not to change the global environment from the test harness
- **AND** it says to restart the affected Cinnamon/Codex session or fixture process after correcting bridge-related environment
- **AND** it says to run a controlled GTK fixture self-test or live fixture smoke before claiming AT-SPI pass evidence
- **AND** it warns that live checks must not target real user windows as fallback

#### Scenario: Documentation preserves baseline semantics
- **GIVEN** AT-SPI tree extraction remains unavailable after safe checks
- **WHEN** the operator reads the production readiness guidance
- **THEN** the docs state that this is expected degraded semantic accessibility enrichment for the Cinnamon/X11 baseline when X11 window/focus/input requirements still pass
- **AND** the docs state that a degraded AT-SPI row still needs a concrete `reason_category` and evidence path
- **AND** the docs do not expand scope to Wayland or portal-required runtime paths

### Requirement: Docs explain safe app-state screenshot evidence
Troubleshooting, E2E harness, and release documentation MUST explain that `get-app-state --json` no longer emits inline screenshot blobs by default. The docs MUST show how to request screenshot artifact paths, how `--no-screenshot` behaves, and that any retained inline mode is explicit opt-in and unsafe for durable evidence logs.

#### Scenario: Operator learns path-only app-state behavior
- **GIVEN** a developer reads the E2E harness or troubleshooting docs
- **WHEN** they look for `get-app-state` screenshot behavior
- **THEN** the docs state that default JSON contains no `data:image` screenshot blob
- **AND** the docs show the supported screenshot output path option or generated artifact path behavior
- **AND** the docs explain that `--no-screenshot` keeps window/accessibility/capability diagnostics usable without screenshot capture
- **AND** any inline screenshot opt-in is labeled unsafe for evidence logs

### Requirement: Docs explain controlled real-live fixture retests
The project docs MUST describe how to run controlled real-live Cinnamon/X11 fixture retests safely. The docs MUST distinguish real-live controlled fixture evidence from fake/fake-live evidence, describe fixture metadata and cleanup, and warn that fixture-dependent operations must never target real user applications as fallback.

#### Scenario: Operator runs controlled real-live retest safely
- **GIVEN** a developer follows the E2E harness documentation for an industrial real-live retest
- **WHEN** they start the controlled fixture runner
- **THEN** the docs identify expected metadata files, fixture roles, target selection rules, cleanup behavior, and evidence directory layout
- **AND** the docs state that fake or fake-live fixtures are not primary real-live evidence
- **AND** the docs warn that keyboard, pointer, screenshot, app-state, target, and overlay checks require controlled fixture windows

### Requirement: Docs preserve NO_AT_BRIDGE remediation guidance
Troubleshooting docs MUST preserve and update the Cinnamon/X11 `NO_AT_BRIDGE=1` diagnostic guidance. They MUST explain that the disabling contract is presence-based for common GTK/ATK bridge integrations, that controlled GTK fixture processes should remove `NO_AT_BRIDGE`, and that diagnostic repair should not mutate global user environment silently.

#### Scenario: Operator fixes bridge-disabled fixture diagnostics
- **GIVEN** a diagnostic report shows `NO_AT_BRIDGE=1` or an AT-SPI bridge-disabled outcome
- **WHEN** the operator reads troubleshooting docs
- **THEN** the docs explain to remove `NO_AT_BRIDGE` from the controlled GTK fixture/application process environment
- **AND** the docs say to restart the affected fixture/Codex session as needed
- **AND** the docs recommend rerunning controlled GTK fixture self-test or real-live fixture smoke before claiming AT-SPI pass evidence

