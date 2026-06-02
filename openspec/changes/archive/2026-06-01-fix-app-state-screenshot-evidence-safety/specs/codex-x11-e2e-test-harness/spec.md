## ADDED Requirements

### Requirement: Real-live controlled fixture runner is reusable
The E2E harness MUST provide a reusable real-live controlled fixture runner for manual and industrial Cinnamon/X11 retests. The runner MUST start controlled fixtures with run-scoped metadata, keep them alive for the whole retest, avoid unsafe titles/classes that project filters may exclude, record fixture PID/title/wm_class/window_id/metadata JSON, and clean up fixture processes and target-window/overlay state reliably.

#### Scenario: Runner starts fixtures with safe metadata
- **GIVEN** a developer requests a controlled real-live retest fixture run
- **WHEN** the harness starts Tk and GTK fixture roles
- **THEN** each fixture has a run-scoped title and wm_class that are clearly controlled
- **AND** fixture titles avoid project-owned or overlay-looking strings such as titles containing `Codex` when current filters exclude or special-case them
- **AND** the GTK fixture process environment has `NO_AT_BRIDGE` absent rather than set to `1`
- **AND** metadata JSON records PID, title, wm_class, readiness path, selected window id when available, and bridge-environment facts

#### Scenario: Runner keeps fixtures alive for retest
- **GIVEN** the fixture runner starts controlled windows
- **WHEN** the retest performs focus, input, pointer, screenshot, app-state, target-window, overlay, and AT-SPI checks
- **THEN** fixture processes remain alive until the runner cleanup phase
- **AND** each fixture-dependent tool call targets the recorded controlled window id or records a safe degraded setup reason
- **AND** no fake or fake-live fixture is used as the primary real-live evidence source for the real-live profile

#### Scenario: Runner cleans up on success and failure
- **GIVEN** a real-live controlled fixture run starts one or more fixture processes
- **WHEN** a tool call fails, times out, or the retest completes successfully
- **THEN** cleanup terminates fixture processes that the harness started
- **AND** cleanup releases target-window state and hides overlay state when applicable
- **AND** cleanup status is recorded in evidence JSON

### Requirement: Real-live fixture evidence is sanitized and target-safe
Real-live fixture retest evidence MUST record enough non-secret fixture metadata to reproduce target selection and classify failures, while refusing to target uncontrolled user applications for keyboard, pointer, screenshot, app-state, target-window, or overlay operations.

#### Scenario: Uncontrolled user windows are not fallback targets
- **GIVEN** window listing includes browser, terminal, editor, messenger, password manager, Codex, overlay, or other non-fixture windows
- **WHEN** the fixture runner resolves targets for mutating or screenshot/app-state checks
- **THEN** those non-fixture windows are not eligible fallback targets
- **AND** missing or ambiguous fixtures produce `missing_fixture_setup` or `unsafe_target_selection` evidence
- **AND** no input, pointer, screenshot, app-state, target, or overlay call is made against the non-fixture window id

#### Scenario: Sanitized evidence references files and metadata
- **GIVEN** a real-live controlled fixture retest captures app-state or screenshot evidence
- **WHEN** evidence is written under `target/e2e-logs/<run-id>/`
- **THEN** evidence references screenshot files by path and metadata
- **AND** evidence records fixture metadata and layer diagnostics
- **AND** evidence does not contain full screenshot data URLs, real secret values, or uncontrolled app content payloads
