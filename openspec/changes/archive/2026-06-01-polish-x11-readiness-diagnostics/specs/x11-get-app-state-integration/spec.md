## ADDED Requirements

### Requirement: Metadata-only live smoke classifies missing fixtures safely
Live metadata-only app-state smoke MUST classify missing controlled fixture setup as a safety limitation, not as code failure or production pass evidence.

#### Scenario: No controlled fixture yields missing fixture setup
- **GIVEN** live metadata-only smoke is run without starting or selecting controlled fixtures
- **WHEN** app-state, screenshot, AT-SPI, keyboard, pointer, target, or overlay rows would require a safe target
- **THEN** those rows use `reason_category=missing_fixture_setup`
- **AND** the summary says it is not safe to test input against real user applications
- **AND** the run does not claim controlled live production readiness

#### Scenario: App-state layer degradation keeps usable metadata visible
- **GIVEN** a controlled X11 fixture target is selected
- **AND** screenshot or AT-SPI layers are degraded by environment limitations
- **WHEN** `x11_get_app_state` evidence is summarized
- **THEN** window context, target identity, and layer diagnostics remain visible
- **AND** degraded layers include canonical reason categories
- **AND** no full screenshot data URL is embedded in ordinary logs or summaries
