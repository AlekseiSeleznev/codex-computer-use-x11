# x11-window-listing Specification

## Purpose
This specification defines the standalone `codex-computer-use-x11 list-windows --json` command as the project's X11/EWMH window-listing surface, including `wmctrl -lpGx` parsing, upstream-compatible `WindowInfo` output, sidecar diagnostics, focus detection, degraded behavior, and fixture-backed testability.
## Requirements
### Requirement: List windows JSON command
The CLI MUST provide `codex-computer-use-x11 list-windows --json` as the public standalone X11/EWMH window-listing surface. The command MUST write one valid JSON object to stdout when a listing report can be produced, MUST use the canonical backend id `x11-ewmh`, and MUST NOT require external credentials or modify the Codex Desktop Linux target checkout.

#### Scenario: Produce a window listing report
- **GIVEN** the standalone CLI is built from the project
- **AND** an X11 display is available
- **AND** `wmctrl` can return one or more rows from `wmctrl -lpGx`
- **WHEN** a developer runs `codex-computer-use-x11 list-windows --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** stderr is empty on success
- **AND** the JSON includes `project`, `version`, `backend`, `windows`, and `diagnostics`
- **AND** `project` equals `codex-computer-use-x11`
- **AND** `backend` equals `x11-ewmh`
- **AND** `windows` is an array of primary window objects

#### Scenario: Reject unsupported list-windows usage
- **GIVEN** a developer invokes `codex-computer-use-x11 list-windows` without `--json` or with unsupported flags
- **WHEN** the CLI handles the invocation
- **THEN** the command exits with a non-zero status code
- **AND** it writes usage or error text to stderr
- **AND** it is not required to write a JSON listing report to stdout

### Requirement: Upstream-compatible primary window model
Each primary window object in `list-windows --json` MUST map X11/EWMH observations into the target repo's `WindowInfo`-compatible shape without adding X11-only fields to the primary object. X11-only raw ids, provenance, PID reliability, command source, warnings, and degraded reasons MUST live in `diagnostics` or another sidecar/report field.

#### Scenario: Map a normal wmctrl row to WindowInfo fields
- **GIVEN** `wmctrl -lpGx` returns a normal application window row with id, workspace, pid, x, y, width, height, class, host, and title fields
- **WHEN** `list-windows --json` serializes that row
- **THEN** the primary window object includes numeric `window_id`
- **AND** it includes nullable `title`, `app_id`, `wm_class`, `pid`, `bounds`, `workspace`, and `client_type` fields
- **AND** it includes boolean `focused` and `hidden` fields
- **AND** it includes string `backend` equal to `x11-ewmh`
- **AND** `bounds.width` and `bounds.height` are positive integers when bounds are present
- **AND** X11-only raw id or PID reliability details are not embedded as extra primary window fields

#### Scenario: Preserve Unicode and whitespace in titles
- **GIVEN** a `wmctrl -lpGx` row has a title containing spaces, Cyrillic text, emoji, or other multibyte characters
- **WHEN** the parser creates a primary window object
- **THEN** the title is preserved as one string without truncating or splitting on internal whitespace
- **AND** the parser still extracts the preceding fixed columns correctly

#### Scenario: Preserve negative coordinates
- **GIVEN** a `wmctrl -lpGx` row reports negative X or Y coordinates for a window on a multi-monitor desktop
- **WHEN** the parser creates bounds
- **THEN** `bounds.x` or `bounds.y` preserves the negative signed coordinate
- **AND** width and height remain positive unsigned dimensions

### Requirement: Robust wmctrl parsing and id normalization
The implementation MUST parse `wmctrl -lpGx` output through a deterministic parser that reuses the shared canonical X11 window-id normalization behavior, rejects malformed rows without panicking, and records parse failures as diagnostics.

#### Scenario: Normalize padded and unpadded X11 ids
- **GIVEN** one `wmctrl` row reports a window id as `0x5624b36`
- **AND** another observation reports the same id as `0x05624b36`
- **WHEN** the listing parser normalizes both ids
- **THEN** both observations map to the same numeric `window_id`

#### Scenario: Reject invalid dimensions without unsigned wraparound
- **GIVEN** a `wmctrl -lpGx` row contains zero, negative, or non-numeric width or height
- **WHEN** the parser evaluates the row
- **THEN** the row is omitted or represented as degraded according to the listing report contract
- **AND** the command does not serialize unsigned wraparound dimensions
- **AND** diagnostics explain that invalid geometry was encountered

#### Scenario: Degrade malformed rows without aborting the whole report
- **GIVEN** `wmctrl -lpGx` returns a mix of valid rows and malformed rows
- **WHEN** `list-windows --json` builds the report
- **THEN** valid windows are still returned
- **AND** malformed rows are counted or described in diagnostics
- **AND** the command exits with status code 0 when a valid JSON report can be produced

### Requirement: Focus and EWMH enrichment are explicit and bounded
The listing report MUST identify the focused window when `_NET_ACTIVE_WINDOW` can be read, and any `_NET_WM_WINDOW_TYPE` or `_NET_WM_STATE_HIDDEN` enrichment MUST be explicit, bounded, cached, lazy, or otherwise designed to avoid unconditional slow N+1 process spawning for every listing call.

#### Scenario: Mark focused window from active id
- **GIVEN** `xprop -root _NET_ACTIVE_WINDOW` returns an active X11 window id
- **AND** `wmctrl -lpGx` returns a row with the same normalized id
- **WHEN** `list-windows --json` serializes the windows
- **THEN** exactly that matching window has `focused` set to true
- **AND** non-matching windows have `focused` set to false

#### Scenario: Degrade when active id is unavailable
- **GIVEN** `wmctrl -lpGx` returns windows
- **AND** `_NET_ACTIVE_WINDOW` cannot be read or parsed
- **WHEN** `list-windows --json` serializes the windows
- **THEN** the report remains valid JSON
- **AND** windows are still listed
- **AND** diagnostics explain that focused-window detection was unavailable

#### Scenario: Bound optional window type lookups
- **GIVEN** multiple windows are returned by `wmctrl -lpGx`
- **WHEN** the implementation needs `_NET_WM_WINDOW_TYPE` or `_NET_WM_STATE_HIDDEN`
- **THEN** the design uses lazy, bounded, cached, or explicitly optional lookup behavior
- **AND** the MVP does not spawn an unbounded `xprop -id` process for every window on every listing call without a recorded design justification
- **AND** unknown `client_type` or `hidden` state is represented conservatively with diagnostics rather than fabricated certainty

### Requirement: PID reliability and non-application windows are visible
The listing report MUST account for X11 PID and window-type uncertainty. PID values that are zero, service-like, remote-host, or otherwise unreliable MUST NOT be treated as verified target identity, and desktop, dock, panel, or project-owned internal overlay/helper windows MUST be filtered or marked so consumers can avoid unsafe targeting.

#### Scenario: Mark unreliable PID in sidecar diagnostics
- **GIVEN** a `wmctrl -lpGx` row reports PID `0`, PID `2`, or a PID whose client machine does not match the local host when host information is available
- **WHEN** `list-windows --json` builds the report
- **THEN** the primary window object does not claim a verified reliable PID
- **AND** sidecar diagnostics record that PID reliability is false or unknown for that window

#### Scenario: Avoid treating desktop and dock windows as normal targets
- **GIVEN** a window is identified as desktop, dock, panel, or another non-application target through class, type, or bounded EWMH enrichment
- **WHEN** the listing report is serialized
- **THEN** the window is either filtered from primary application targets or marked in diagnostics as non-application
- **AND** the report does not silently present it as an ordinary safe application target without a warning

#### Scenario: Exclude or mark project-owned overlay windows
- **GIVEN** a `wmctrl -lpGx` row has a class, application id, or title that identifies it as a `codex-computer-use-x11` overlay/helper window
- **WHEN** `list-windows --json` builds the report
- **THEN** the overlay/helper window is not presented as an ordinary safe application target
- **AND** diagnostics include metadata that identifies the row as project-owned internal UI
- **AND** target-window and input consumers can avoid selecting the overlay/helper as a normal app window

### Requirement: Degraded and no-display behavior
The command MUST produce structured degraded JSON whenever safe host inspection and serialization are still possible, including when `DISPLAY` is absent, `wmctrl` is missing, or X11/EWMH probing fails.

#### Scenario: Run without DISPLAY
- **GIVEN** the process environment has no usable `DISPLAY`
- **WHEN** a developer runs `codex-computer-use-x11 list-windows --json`
- **THEN** the command exits with status code 0 when it can emit a JSON report
- **AND** stdout is valid JSON
- **AND** `windows` is an empty array
- **AND** diagnostics include a blocker or degraded reason explaining that X11 window listing is unavailable without a display

#### Scenario: Run without wmctrl
- **GIVEN** `DISPLAY` is present
- **AND** `wmctrl` is not available on `PATH`
- **WHEN** a developer runs `codex-computer-use-x11 list-windows --json`
- **THEN** the command exits with status code 0 when it can emit a JSON report
- **AND** `windows` is an empty array
- **AND** diagnostics include a blocker or degraded reason explaining that `wmctrl` is required for the MVP listing backend

#### Scenario: Report wmctrl command failure
- **GIVEN** `wmctrl` is available but `wmctrl -lpGx` exits unsuccessfully
- **WHEN** `list-windows --json` handles the failure
- **THEN** stdout remains valid JSON when serialization is possible
- **AND** stderr is empty on successful JSON report emission
- **AND** diagnostics include the command failure without exposing unrelated sensitive local data

### Requirement: Listing behavior is testable without live X11
Implementation work for X11 window listing MUST cover parser, command, and CLI behavior with fixtures, fake command output, a command-runner seam, or a fake `PATH` before live Cinnamon/X11 smoke evidence is used to mark behavior complete.

#### Scenario: Test parser fixtures before live smoke
- **GIVEN** implementation adds `wmctrl -lpGx` parsing behavior
- **WHEN** tests are written for the parser
- **THEN** fixtures cover normal windows, Unicode titles, negative coordinates, malformed rows, invalid dimensions, and unreliable PID cases
- **AND** those tests run without invoking live `wmctrl`, `xprop`, or an X11 display

#### Scenario: Test CLI through fake command behavior
- **GIVEN** implementation adds `list-windows --json`
- **WHEN** CLI tests exercise success and degraded paths
- **THEN** they use fake command output, fake `PATH`, or an equivalent seam for external commands
- **AND** live Cinnamon/X11 smoke testing is recorded only after unit and CLI tests are green

