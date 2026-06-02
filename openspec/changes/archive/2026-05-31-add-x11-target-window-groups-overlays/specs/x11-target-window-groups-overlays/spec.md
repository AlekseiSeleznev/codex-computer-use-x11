# x11-target-window-groups-overlays Specification Delta

## ADDED Requirements

### Requirement: Target-window state commands
The standalone CLI MUST provide JSON target-window state commands that save, inspect, and release resolved X11/EWMH targets through the existing selector vocabulary. Target saving MUST reuse the current safe target-resolution semantics for `window_id`, `pid`, `wm_class`, and `title`, MUST reject ambiguous selectors, and MUST NOT require external credentials or modify the Codex Desktop Linux target checkout.

#### Scenario: Save target by window id
- **GIVEN** `wmctrl -lpGx` lists window `0x2` with title `Editor`, reliable pid `1234`, and bounds in X11 root coordinates
- **AND** target state is empty
- **WHEN** a developer runs `codex-computer-use-x11 target-window --window-id 0x2 --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** `success` is true
- **AND** `target.window.window_id` equals `2`
- **AND** `target.window.backend` equals `x11-ewmh`
- **AND** `state.groups[0].windows` contains the saved target
- **AND** `overlay.requested` is false

#### Scenario: Inspect target context
- **GIVEN** target state contains a saved target for window `0x2`
- **AND** the current X11 listing still contains window `0x2`
- **WHEN** a developer runs `codex-computer-use-x11 target-context --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** `success` is true
- **AND** the report includes the active group id
- **AND** the report includes the saved target as not stale

#### Scenario: Release one target
- **GIVEN** target state contains saved targets for windows `0x2` and `0x3`
- **WHEN** a developer runs `codex-computer-use-x11 release-window --window-id 0x2 --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** the released target id refers to window `2`
- **AND** target state no longer contains window `2`
- **AND** target state still contains window `3`

#### Scenario: Release all targets
- **GIVEN** target state contains two saved targets in one or more groups
- **WHEN** a developer runs `codex-computer-use-x11 release-window --all --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** the report states that two targets were released
- **AND** target state contains no saved targets

#### Scenario: Refuse ambiguous selector
- **GIVEN** the current X11 listing contains two windows whose titles contain `Editor`
- **WHEN** a developer runs `codex-computer-use-x11 target-window --title Editor --json`
- **THEN** the command exits with a non-zero status code
- **AND** stdout is valid JSON
- **AND** `success` is false
- **AND** `error_code` includes `AmbiguousTarget` or equivalent ambiguity detail
- **AND** no target state is saved or overwritten

### Requirement: Window groups and active targets
The target-window implementation MUST support groups of targeted windows with deterministic active-group and active-window tracking. Adding the same current window to the same group MUST be idempotent, and group operations MUST remain testable without a live X server.

#### Scenario: Create group on first target
- **GIVEN** target state is empty
- **AND** `wmctrl -lpGx` lists window `0x2` with title `Spreadsheet`
- **WHEN** a developer runs `codex-computer-use-x11 target-window --window-id 0x2 --group data-entry --color green --json`
- **THEN** the command exits with status code 0
- **AND** `state.active_group_id` equals `data-entry`
- **AND** the `data-entry` group color is `green`
- **AND** the saved target becomes the active window for that group

#### Scenario: Idempotent add of existing window
- **GIVEN** group `data-entry` already contains a saved target for window `0x2`
- **AND** the current X11 listing still contains window `0x2`
- **WHEN** a developer runs `codex-computer-use-x11 target-window --window-id 0x2 --group data-entry --color green --json` again
- **THEN** the command exits with status code 0
- **AND** group `data-entry` contains one target for window `2`
- **AND** the report states that the target was updated or already existed instead of duplicated

#### Scenario: Add second window to group
- **GIVEN** group `data-entry` already contains active target window `0x2`
- **AND** the current X11 listing contains window `0x3` with title `Email`
- **WHEN** a developer runs `codex-computer-use-x11 target-window --window-id 0x3 --group data-entry --color blue --json`
- **THEN** the command exits with status code 0
- **AND** group `data-entry` contains targets for windows `2` and `3`
- **AND** the newly targeted window becomes the active window for the group
- **AND** the previously targeted window remains in the group but is not active

### Requirement: Stale target detection
Target-context and target-window operations MUST validate saved targets against a fresh listing before reporting them as current. A saved target whose X11 window id is no longer present MUST be marked or removed as stale, and stale state MUST NOT be reused for input, app-state, or overlay operations.

#### Scenario: Mark vanished window stale
- **GIVEN** target state contains saved target window `0x2`
- **AND** the current X11 listing no longer contains window `0x2`
- **WHEN** a developer runs `codex-computer-use-x11 target-context --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** diagnostics include window `2` in `stale_removed` or equivalent stale-target evidence
- **AND** state no longer presents window `2` as an active usable target

#### Scenario: Clear active target after stale removal
- **GIVEN** group `data-entry` has active target window `0x2`
- **AND** no other window in that group remains valid
- **WHEN** stale validation runs
- **THEN** the group remains present or is reported empty according to the state contract
- **AND** `active_window_id` for that group is null
- **AND** the report warns that the previous active target vanished

### Requirement: Optional overlay boundary
Target-window operations MUST support an optional colored overlay boundary that is independent from target-state success. When overlay display is unsupported or fails, the target MUST still be saved if target resolution succeeds, and the report MUST include a warning/degraded overlay result instead of treating visual display as required.

#### Scenario: Request overlay and provider succeeds
- **GIVEN** `wmctrl -lpGx` lists window `0x2` with valid bounds
- **AND** an overlay provider can show a green border for the target bounds
- **WHEN** a developer runs `codex-computer-use-x11 target-window --window-id 0x2 --group data-entry --color green --overlay --json`
- **THEN** the command exits with status code 0
- **AND** the target is saved in group `data-entry`
- **AND** `overlay.requested` is true
- **AND** `overlay.shown` is true
- **AND** overlay diagnostics identify the provider that accepted the show request

#### Scenario: Overlay failure is warning
- **GIVEN** `wmctrl -lpGx` lists window `0x2` with valid bounds
- **AND** the overlay provider is unsupported or fails to show a border
- **WHEN** a developer runs `codex-computer-use-x11 target-window --window-id 0x2 --overlay --json`
- **THEN** the command exits with status code 0
- **AND** the target is saved
- **AND** `overlay.requested` is true
- **AND** `overlay.shown` is false
- **AND** `overlay.warning` explains the unsupported or failed provider

#### Scenario: Release hides overlay when available
- **GIVEN** target state contains window `0x2`
- **AND** an overlay provider has a shown border for that target
- **WHEN** a developer runs `codex-computer-use-x11 release-window --window-id 0x2 --json`
- **THEN** the command exits with status code 0
- **AND** the target is released
- **AND** overlay diagnostics state that the border hide was requested or completed
