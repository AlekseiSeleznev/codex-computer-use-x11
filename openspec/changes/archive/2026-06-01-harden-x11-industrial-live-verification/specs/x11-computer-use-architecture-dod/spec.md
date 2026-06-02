## ADDED Requirements

### Requirement: Industrial live acceptance rejects harness omissions
The final Computer Use DoD and release checklist MUST distinguish industrial live acceptance from metadata-only live smoke. Industrial live acceptance SHALL require fixture-backed evidence for fixture-dependent capabilities, and MUST reject missing fixture orchestration as acceptable pass evidence.

#### Scenario: Metadata-only live smoke is not industrial acceptance
- **GIVEN** live plugin evidence contains marketplace metadata and MCP tool list checks
- **AND** fixture-dependent rows are degraded because no safe fixtures were orchestrated
- **WHEN** the final DoD validator or release checklist evaluates industrial readiness
- **THEN** the run is not accepted as industrial live verification
- **AND** the report states that metadata/tools smoke passed but fixture-backed capabilities remain unproven
- **AND** required next steps name the missing fixture-backed checks

#### Scenario: Fixture-backed pass supports industrial acceptance
- **GIVEN** live evidence includes controlled fixture pass rows for keyboard, pointer, window focus, screenshot, app-state, GTK AT-SPI, target context, and overlay when enabled
- **WHEN** the final DoD validator evaluates industrial readiness
- **THEN** the run can satisfy industrial live verification for the supported Cinnamon/X11 scope
- **AND** any remaining degraded rows are limited to real environment limitations with concrete reasons
- **AND** missing fixture setup does not appear among accepted degraded reasons

### Requirement: Industrial evidence keeps safety and privacy boundaries
Industrial acceptance evidence MUST prove no input, pointer, screenshot, app-state, or overlay operation targeted uncontrolled real user applications. Evidence and reports MUST avoid secret values, huge inline screenshots, and uncontrolled app content while preserving enough file paths and sanitized diagnostics to reproduce failures.

#### Scenario: Unsafe target evidence blocks release readiness
- **GIVEN** a live evidence file shows an input, pointer, screenshot, or app-state operation targeted a non-fixture user application window
- **WHEN** industrial DoD validation runs
- **THEN** validation exits non-zero
- **AND** the error names unsafe target selection as the blocker
- **AND** release readiness is blocked until fixture-scoped evidence replaces the unsafe evidence

#### Scenario: Screenshot artifacts are stored by path
- **GIVEN** live screenshot or app-state checks produce image evidence
- **WHEN** release evidence is summarized
- **THEN** the summary references image files under `target/e2e-logs/<run-id>/`
- **AND** it does not serialize full screenshots as huge inline data URLs
- **AND** it does not copy real secret values from local files or environment variables
