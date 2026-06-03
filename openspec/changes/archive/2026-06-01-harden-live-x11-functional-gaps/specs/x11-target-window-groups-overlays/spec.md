## ADDED Requirements

### Requirement: Standalone overlay provider shows non-focus target borders
The standalone target-window implementation MUST provide an overlay provider that can draw a visual border around target bounds in X11 root coordinates without stealing focus from the target application. The preferred v1 implementation SHALL use native X11 behavior through `x11rb` or a helper process that creates non-focus `override-redirect` border windows.

#### Scenario: Overlay provider shows border without stealing focus
- **GIVEN** `wmctrl -lpGx` lists target window `0x2` with valid X11 root-coordinate bounds
- **AND** exact active-window state before the overlay is known
- **WHEN** a caller runs `target-window --window-id 0x2 --overlay --json`
- **THEN** target state is saved
- **AND** `overlay.requested` is true
- **AND** `overlay.shown` is true
- **AND** `overlay.provider` identifies the standalone X11 overlay provider
- **AND** diagnostics show that the active target window was not replaced by the overlay window

#### Scenario: Overlay failure remains a warning
- **GIVEN** target resolution succeeds
- **AND** the overlay provider cannot connect to X11 or cannot create border windows
- **WHEN** `target-window --overlay --json` runs
- **THEN** the command exits successfully for the target save
- **AND** `overlay.requested` is true
- **AND** `overlay.shown` is false
- **AND** `overlay.warning` explains the provider failure

### Requirement: Overlay windows are identifiable and excluded from target listing
Overlay windows MUST use a recognizable title and class containing `codex-computer-use-x11-overlay` and MUST NOT appear as ordinary application targets in `x11_list_windows`, target resolution, app-state target selectors, or e2e capability windows.

#### Scenario: Overlay window is excluded from list-windows targets
- **GIVEN** a standalone overlay border window exists with title/class `codex-computer-use-x11-overlay`
- **WHEN** `list-windows --json` runs
- **THEN** the overlay window is absent from the `windows` target list
- **AND** diagnostics may count or mention excluded project overlay windows
- **AND** ordinary application windows remain visible

#### Scenario: Overlay cannot be selected as a target
- **GIVEN** an overlay border window exists
- **WHEN** a caller tries to target by title `codex-computer-use-x11-overlay`
- **THEN** the target resolver refuses the overlay as a non-application target
- **AND** no input, app-state, or overlay-on-overlay action is attempted

### Requirement: Release hides overlays for released targets
Releasing a target MUST hide or tear down the overlay associated with that target when an overlay was shown. Releasing all targets MUST hide all project-owned overlays. Overlay hide failure MUST be reported as a warning without preserving stale target state as usable.

#### Scenario: Release one target hides its overlay
- **GIVEN** target state contains window `0x2`
- **AND** an overlay is shown for window `0x2`
- **WHEN** `release-window --window-id 0x2 --json` runs
- **THEN** the target is released
- **AND** overlay diagnostics record hide requested or completed for window `0x2`
- **AND** follow-up `target-context --json` does not report the target as active

#### Scenario: Release all targets hides all overlays
- **GIVEN** target state contains overlays for two target windows
- **WHEN** `release-window --all --json` runs
- **THEN** all target state is cleared
- **AND** all project-owned overlays are hidden or hide warnings are reported
- **AND** follow-up listing excludes any remaining overlay helper windows from target candidates
