## MODIFIED Requirements

### Requirement: Doctor readiness shape
The doctor JSON report MUST expose a stable readiness shape for compatibility, and it MUST also report upstream-compatible readiness concepts for capability detection: `can_query_windows`, `can_focus_apps`, `can_focus_windows`, `can_send_development_input`, `blockers`, and `recommended_next_step`. The finalized Cinnamon/X11 v1 baseline MUST NOT leave implemented window-query or window-focus capabilities listed as merely planned.

#### Scenario: Inspect readiness and checks
- **GIVEN** `doctor --json` returns its expanded report
- **WHEN** a developer inspects the JSON
- **THEN** `readiness` is an object with boolean `ok` and array `blockers` fields
- **AND** `readiness` includes boolean `can_query_windows`, `can_focus_apps`, `can_focus_windows`, and `can_send_development_input` fields
- **AND** `readiness` includes a string `recommended_next_step` field
- **AND** `capabilities` is an object with `implemented` and `planned` arrays of strings
- **AND** `capabilities.implemented` contains `doctor-json`
- **AND** `capabilities.implemented` contains finalized v1 capability facts for X11/EWMH window querying and focus-with-verification when those behaviors are implemented in the repository
- **AND** `capabilities.planned` remains an array and MUST NOT contain stale entries for finalized v1 behaviors that have implementation and verification evidence
- **AND** `capabilities.planned` may be empty only when all previously planned capabilities have become implemented or moved into named capability facts
- **AND** `checks` is an array of objects
- **AND** `checks` contains at least one self-check or doctor-internal check entry
- **AND** every check object includes string `name`, boolean `ok`, and string `detail` fields

#### Scenario: Explain degraded readiness without panicking
- **GIVEN** one or more desktop-control probes are unavailable
- **WHEN** `doctor --json` returns its report
- **THEN** the command exits with status code 0 when a JSON report can still be produced
- **AND** unavailable capabilities are represented by readiness booleans, blockers, failed check entries, or capability facts
- **AND** the report includes a recommended next step instead of panicking

#### Scenario: Aggregate readiness ok from blockers
- **GIVEN** `doctor --json` returns its expanded report
- **WHEN** a consumer inspects `readiness.ok`
- **THEN** `readiness.ok` is false whenever `readiness.blockers` is non-empty
- **AND** `readiness.ok` is true only when all required baseline readiness checks for the current supported target have no blockers
- **AND** producing a JSON report successfully is not sufficient by itself to make `readiness.ok` true

#### Scenario: Report finalized X11 window readiness
- **GIVEN** `wmctrl` and `xprop` are available, `DISPLAY` is usable, and EWMH active-window/window-list probes succeed
- **WHEN** `doctor --json` computes readiness for the finalized Cinnamon/X11 v1 baseline
- **THEN** `readiness.can_query_windows` is true
- **AND** `readiness.can_focus_windows` is true because the repository implements focus-with-verification through the X11/EWMH window model
- **AND** `readiness.can_focus_apps` is true only when the report can map app-focus semantics to verified X11 window activation; otherwise the report explains the app/window distinction in degraded diagnostics without setting `can_focus_windows` false
- **AND** `capabilities.implemented` names the X11/EWMH window listing and focus-with-verification behaviors
- **AND** `capabilities.planned` does not contain the stale `x11-ewmh-windowing` placeholder

### Requirement: Doctor ydotool socket detection
The doctor command MUST evaluate ydotool socket candidates in a deterministic order, continue past stale or missing earlier paths, select the first connectable socket for primary readiness, and preserve enough checked-path detail for degraded diagnostics. Serialized reports MUST NOT expose machine-private paths derived from `YDOTOOL_SOCKET` or `XDG_RUNTIME_DIR`; they MUST use stable diagnostic labels for those environment-derived candidates while still allowing the public fallback `/tmp/.ydotool_socket` to be reported literally.

#### Scenario: Continue after stale YDOTOOL_SOCKET
- **GIVEN** `YDOTOOL_SOCKET` names a missing or stale socket path
- **AND** `$XDG_RUNTIME_DIR/.ydotool_socket` is missing
- **AND** `/tmp/.ydotool_socket` is connectable
- **WHEN** `doctor --json` returns its report
- **THEN** the ydotool socket check is ok
- **AND** the selected connectable socket is `/tmp/.ydotool_socket`
- **AND** the report records that earlier candidates were checked and unavailable using non-secret labels such as `env:YDOTOOL_SOCKET` and `env:XDG_RUNTIME_DIR/.ydotool_socket`
- **AND** the report does not serialize the private absolute values of `YDOTOOL_SOCKET` or `XDG_RUNTIME_DIR`
- **AND** readiness does not fail solely because `YDOTOOL_SOCKET` was stale

#### Scenario: Report no connectable ydotool socket
- **GIVEN** `ydotool` is installed but no candidate ydotool socket is connectable
- **WHEN** `doctor --json` returns its report
- **THEN** the ydotool socket fact is not ok
- **AND** the report lists checked candidates or equivalent diagnostic details without exposing private environment-derived absolute paths
- **AND** the recommended next step explains how to start or expose `ydotoold` when no other input backend is verified

#### Scenario: Redact private ydotool socket candidates in live facts
- **GIVEN** `YDOTOOL_SOCKET` is `/home/alice/private/ydotool.sock`
- **AND** `XDG_RUNTIME_DIR` is `/run/user/1000`
- **WHEN** `doctor --json` gathers live system facts
- **THEN** serialized ydotool candidate details include stable labels for those environment-derived candidates
- **AND** serialized ydotool candidate details do not include `/home/alice/private/ydotool.sock` or `/run/user/1000/.ydotool_socket`
- **AND** connection attempts may still use the real local paths internally to determine availability
