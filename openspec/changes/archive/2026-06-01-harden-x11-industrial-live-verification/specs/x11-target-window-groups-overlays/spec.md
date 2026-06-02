## ADDED Requirements

### Requirement: Live overlay fixture acceptance
Live overlay verification MUST target a controlled fixture window and MUST record whether the overlay provider actually showed and hid project-owned overlay UI. Overlay lifecycle evidence MUST include target id, provider name, shown/hide status, release result, and any warning without treating overlay helper windows as normal application targets.

#### Scenario: Enabled overlay provider passes against fixture
- **GIVEN** `CODEX_X11_ENABLE_TK_OVERLAY=1` is set for the live overlay check
- **AND** a controlled fixture window has valid X11 root-coordinate bounds
- **WHEN** the harness calls `x11_target_window` with overlay requested
- **THEN** the tool report has `overlay.requested=true`
- **AND** `overlay.shown=true`
- **AND** `overlay.provider` identifies the provider such as `python-tk-overlay-helper`
- **AND** the capability row records pass evidence for overlay display

#### Scenario: Release hides enabled overlay
- **GIVEN** a controlled fixture has an active overlay target context
- **WHEN** the harness calls `x11_release_window` for that fixture
- **THEN** the release report indicates the target was released
- **AND** overlay diagnostics show the overlay is no longer shown or the helper was hidden
- **AND** follow-up target context does not contain an active target for the fixture

#### Scenario: Overlay helper is never selected for input
- **GIVEN** an overlay helper window appears in the X11 window listing
- **WHEN** live smoke resolves fixture targets for keyboard, pointer, screenshot, or app-state checks
- **THEN** the overlay helper is excluded from target candidates
- **AND** evidence records the exclusion as an internal/project-owned helper when relevant
