## ADDED Requirements

### Requirement: Fake screenshot smoke has explicit pass-or-expected-degraded semantics
Fake-mode screenshot evidence MUST either use a controlled fake screenshot provider fixture and pass output integrity checks or classify the missing fake provider as an expected fake-fixture limitation without weakening real screenshot-crop validation.

#### Scenario: Fake screenshot provider produces pass evidence
- **GIVEN** fake smoke provides a fake screenshot command or DBus fixture capable of writing a PNG output
- **WHEN** screenshot crop is exercised in fake mode
- **THEN** the row is `pass` only if the output file exists, is a valid image, and matches expected crop dimensions or metadata
- **AND** the summary references the file path rather than embedding image bytes

#### Scenario: Missing fake screenshot provider is documented degraded evidence
- **GIVEN** fake smoke does not provide fake `gdbus`, `busctl`, or equivalent screenshot fixture support
- **WHEN** screenshot crop is evaluated in fake mode
- **THEN** the row is `degraded` with a reason category for expected fake-fixture limitation
- **AND** the report states that this does not prove real screenshot failure
- **AND** real live screenshot-crop output integrity checks remain required for production evidence

### Requirement: Screenshot crop integrity remains strict
Screenshot crop success MUST continue to require a caller-visible output artifact with validated path handling, image readability, and expected bounds metadata.

#### Scenario: Provider success without output file fails integrity
- **GIVEN** a screenshot provider reports success
- **AND** the expected output file is missing, empty, unreadable, or outside the resolved output path
- **WHEN** screenshot crop evidence is validated
- **THEN** the screenshot row is `fail` with `reason_category=code_failure`
- **AND** the failure is not normalized to degraded fake limitation unless the run was explicitly fake mode without a fake provider fixture
