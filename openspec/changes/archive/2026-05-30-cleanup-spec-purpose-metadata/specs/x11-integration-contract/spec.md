## ADDED Requirements

### Requirement: Canonical X11 integration spec purpose metadata
The `x11-integration-contract` canonical specification MUST have a non-placeholder `## Purpose` section that describes the spec's role as the source of truth for X11/EWMH backend identity, source-overlay compatibility, and integration-contract constraints.

#### Scenario: Replace bootstrap purpose placeholder
- **GIVEN** `openspec/specs/x11-integration-contract/spec.md` is the canonical X11 integration contract specification
- **WHEN** the spec purpose metadata is inspected
- **THEN** the `## Purpose` section does not contain `TBD`
- **AND** it does not say it was created by archiving `bootstrap-codex-computer-use-x11`
- **AND** it describes the X11/EWMH backend and source-overlay integration contract

#### Scenario: Preserve X11 integration requirements
- **GIVEN** this maintenance change updates the canonical purpose metadata
- **WHEN** the canonical `x11-integration-contract` spec is compared before and after the change
- **THEN** existing normative requirements and scenarios remain semantically unchanged
- **AND** no Rust code, target-checkout write, or source-overlay behavior change is required
