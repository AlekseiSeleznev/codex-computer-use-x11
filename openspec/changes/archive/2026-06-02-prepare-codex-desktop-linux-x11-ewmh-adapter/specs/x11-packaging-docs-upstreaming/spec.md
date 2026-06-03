## ADDED Requirements

### Requirement: Docs cross-link optional upstream adapter readiness
The project documentation MUST mention release artifacts and adapter-ready documentation for a possible `codex-desktop-linux` Linux Feature integration without claiming that upstream integration has been merged.

#### Scenario: README links adapter contract without overstating status
- **GIVEN** a reader opens `README.md`
- **WHEN** they read release or upstreaming guidance
- **THEN** the README links `docs/codex-desktop-linux-x11-ewmh-adapter.md`
- **AND** it says the project prepared an adapter contract for optional `linux-features/x11-ewmh-computer-use` integration in `codex-desktop-linux`
- **AND** it does not claim that upstream has merged or enabled the integration by default

#### Scenario: Install docs mention release artifacts as adapter input
- **GIVEN** a reader opens `INSTALL_CODEX.md`
- **WHEN** they read install and release guidance
- **THEN** it points to the adapter contract document for downstream `codex-desktop-linux` integration planning
- **AND** it distinguishes user-local standalone install from upstream app staging
- **AND** it does not instruct standalone users to write into `/opt` or `openai-bundled`

### Requirement: Changelog records adapter-prep scope
The changelog MUST record packaging, documentation, scaffold, and test additions under an unreleased or target release section while preserving the rule that publishing a GitHub release requires explicit approval.

#### Scenario: Changelog lists unreleased adapter-prep changes
- **GIVEN** a maintainer opens `CHANGELOG.md`
- **WHEN** they read the newest section
- **THEN** it lists release tarball plus SHA256 packaging support
- **AND** it lists upstream adapter contract documentation
- **AND** it lists downstream adapter scaffold tests
- **AND** it does not state that a new GitHub release was already published
