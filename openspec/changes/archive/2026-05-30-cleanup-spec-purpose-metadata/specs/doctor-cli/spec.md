## ADDED Requirements

### Requirement: Canonical doctor CLI spec purpose metadata
The `doctor-cli` canonical specification MUST have a non-placeholder `## Purpose` section that describes the spec's role as the source of truth for the `codex-computer-use-x11 doctor --json` smoke-test and capability-readiness report surface.

#### Scenario: Replace bootstrap purpose placeholder
- **GIVEN** `openspec/specs/doctor-cli/spec.md` is the canonical doctor CLI specification
- **WHEN** the spec purpose metadata is inspected
- **THEN** the `## Purpose` section does not contain `TBD`
- **AND** it does not say it was created by archiving `bootstrap-codex-computer-use-x11`
- **AND** it describes the doctor CLI JSON report and capability/readiness diagnostic contract

#### Scenario: Preserve doctor CLI requirements
- **GIVEN** this maintenance change updates the canonical purpose metadata
- **WHEN** the canonical `doctor-cli` spec is compared before and after the change
- **THEN** existing normative requirements and scenarios remain semantically unchanged
- **AND** no Rust code or runtime CLI behavior is required to change
