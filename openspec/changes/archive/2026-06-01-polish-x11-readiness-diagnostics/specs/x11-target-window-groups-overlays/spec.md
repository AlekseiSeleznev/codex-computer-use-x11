## ADDED Requirements

### Requirement: Fixture-scoped target and overlay lifecycle evidence
Target-window and overlay evidence in live smoke MUST prove fixture scope, release behavior, overlay hiding, and stale-target cleanup.

#### Scenario: Release clears controlled target context
- **GIVEN** live smoke targets a controlled fixture window
- **WHEN** it calls release or cleanup for that target
- **THEN** follow-up target context is empty or no longer contains the released window
- **AND** stale target state is reported as fail or degraded with a concrete reason
- **AND** no later input, screenshot, or overlay operation uses the released target implicitly

#### Scenario: Overlay helper windows are never targets
- **GIVEN** an overlay is shown for a controlled fixture
- **WHEN** live smoke lists or resolves target candidates
- **THEN** project overlay helper windows are excluded from target candidates
- **AND** cleanup attempts to hide the overlay
- **AND** evidence records whether overlay hide succeeded or degraded
