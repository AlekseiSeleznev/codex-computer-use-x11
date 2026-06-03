## MODIFIED Requirements

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

## ADDED Requirements

### Requirement: Documentation examples avoid broken illustrative links
Project documentation that shows illustrative local paths MUST NOT use Markdown link syntax for files that are intentionally absent from this repository. Such examples MUST be rendered as code literals, plain text, or explicitly marked placeholders so local Markdown link checks do not confuse them with required tracked files.

#### Scenario: Skill template examples are not broken links
- **GIVEN** `.codex/skills/grill-with-docs/CONTEXT-FORMAT.md` contains example glossary or context paths
- **WHEN** documentation checks inspect Markdown links
- **THEN** illustrative example paths such as `src/ordering/CONTEXT.md`, `src/payments/CONTEXT.md`, and `docs/glossary.md` are not encoded as local Markdown links unless those files exist
- **AND** the examples remain readable as path examples for future projects
