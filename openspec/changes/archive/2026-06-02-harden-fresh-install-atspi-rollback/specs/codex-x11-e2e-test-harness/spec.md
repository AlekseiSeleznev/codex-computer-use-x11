## ADDED Requirements

### Requirement: Fake fresh-install smoke proves install doctor uninstall cycle
The e2e harness MUST provide a fake no-GUI smoke path that verifies a fresh install can activate the expected state, doctor can report AT-SPI readiness through fixture facts, and uninstall restores the recorded before-state.

#### Scenario: Fake fresh install doctor uninstall cycle passes
- **GIVEN** a fake home, fake Codex Desktop target, fake gsettings command, fake activation-environment commands, and fake AT-SPI collector probe
- **WHEN** the fake smoke runs fresh install, doctor, and uninstall
- **THEN** fresh install writes a rollback manifest for plugin, environment, gsettings, source, and live-asset fixture state
- **AND** doctor reports the fixture AT-SPI tree as available instead of degraded
- **AND** uninstall restores the fake before-state
- **AND** the smoke exits successfully without a GUI, live X11, or sudo

#### Scenario: Fake smoke captures drift blocker
- **GIVEN** a fake live asset is modified after install and before uninstall
- **WHEN** the fake smoke runs uninstall
- **THEN** uninstall reports a drift blocker in JSON
- **AND** it does not overwrite the drifted asset blindly

### Requirement: Live-safe checklist records install and rollback evidence
The live verification checklist MUST record safe, non-secret evidence for the current Cinnamon/X11 machine when live validation is available. It MUST target controlled fixtures and MUST not print secrets or inline screenshots.

#### Scenario: Live checklist covers current X11 functionality
- **GIVEN** live Cinnamon/X11 validation is available
- **WHEN** the live-safe checklist is executed
- **THEN** evidence records `x11_doctor`
- **AND** evidence records `x11_get_app_state include_screenshot=true` with screenshot data referenced by path or summarized without inline data
- **AND** evidence records `x11_accessibility_tree` against a controlled fixture or clearly reports missing fixture setup
- **AND** evidence records the provider takeover marker in the live asset when live patching was requested
- **AND** evidence records full uninstall restore or explicit drift/blocker results

#### Scenario: Live unavailable is reported as limitation
- **GIVEN** live X11, sudo/live assets, or controlled fixtures are unavailable
- **WHEN** verification runs
- **THEN** the report identifies the exact unavailable layer
- **AND** fake smoke evidence remains usable for CI-like verification
- **AND** unavailable live evidence is not reported as a pass
