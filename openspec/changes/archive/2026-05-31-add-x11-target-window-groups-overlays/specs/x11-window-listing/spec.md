# x11-window-listing Specification Delta

## MODIFIED Requirements

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
