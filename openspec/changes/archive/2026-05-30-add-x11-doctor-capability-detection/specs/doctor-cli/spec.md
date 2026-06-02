## Proposal acceptance checklist for specs

- [x] Bootstrap compatibility table is present in `Doctor JSON command` / `Preserve bootstrap field compatibility`.
- [x] `project-bootstrap` is closed out of scope for this change because the doctor JSON expansion is additive and preserves existing bootstrap field paths and types.
- [x] No-display/headless behavior is specified in `Doctor headless and no-display behavior`.
- [x] Fixture-backed DBus parser cases are specified in `Doctor probe parser fixture coverage`.
- [x] Source-overlay acceptance is specified in the `x11-integration-contract` delta.
- [x] No top-level or upstream-required targeted-input readiness field is introduced.

## MODIFIED Requirements

### Requirement: Doctor JSON command
The CLI binary MUST be named exactly `codex-computer-use-x11`, and it MUST provide a `doctor --json` command that writes a single valid JSON object to stdout. The expanded report MUST remain additive for current bootstrap consumers by preserving existing top-level `project`, `version`, `backend`, `readiness`, `capabilities`, and `checks` paths and their bootstrap field types while adding richer capability facts.

#### Scenario: Produce the capability doctor report
- **GIVEN** the `codex-computer-use-x11` CLI is built from the standalone project
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** the command exits with status code 0 when a JSON report is emitted
- **AND** stdout is valid JSON
- **AND** stderr is empty on success
- **AND** the JSON includes `project`, `version`, `backend`, `readiness`, `capabilities`, and `checks`
- **AND** `project` equals `codex-computer-use-x11`
- **AND** `version` equals the package version declared in root `Cargo.toml`
- **AND** `backend` equals `x11-ewmh`
- **AND** richer capability facts are added without removing or renaming the bootstrap fields

#### Scenario: Preserve bootstrap field compatibility
- **GIVEN** an existing bootstrap smoke-test caller reads the doctor JSON report
- **WHEN** `doctor --json` returns the expanded report
- **THEN** `project` is still a string
- **AND** `version` is still a string
- **AND** `backend` is still the string `x11-ewmh`
- **AND** `readiness.ok` is still a boolean
- **AND** `readiness.blockers` is still an array of strings
- **AND** `capabilities.implemented` is still an array of strings
- **AND** `capabilities.planned` is still an array of strings
- **AND** `checks` is still an array of objects
- **AND** every check object keeps string `name`, boolean `ok`, and string `detail` fields

### Requirement: Doctor readiness shape
The doctor JSON report MUST expose a stable readiness shape for compatibility, and it MUST also report upstream-compatible readiness concepts for capability detection: `can_query_windows`, `can_focus_apps`, `can_focus_windows`, `can_send_development_input`, `blockers`, and `recommended_next_step`.

#### Scenario: Inspect readiness and checks
- **GIVEN** `doctor --json` returns its expanded report
- **WHEN** a developer inspects the JSON
- **THEN** `readiness` is an object with boolean `ok` and array `blockers` fields
- **AND** `readiness` includes boolean `can_query_windows`, `can_focus_apps`, `can_focus_windows`, and `can_send_development_input` fields
- **AND** `readiness` includes a string `recommended_next_step` field
- **AND** `capabilities` is an object with `implemented` and `planned` arrays of strings
- **AND** `capabilities.implemented` contains `doctor-json`
- **AND** this delta overrides the earlier canonical non-empty `capabilities.planned` constraint so `capabilities.planned` remains an array and may be empty only when all previously planned capabilities have become implemented or moved into named capability facts
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

### Requirement: Doctor report is a standalone bootstrap surface
The doctor JSON report MUST remain a standalone smoke-test and capability-detection surface for this project, and it MUST NOT be required to be a strict subset of the upstream target repo `doctor_report()` JSON unless a later design or ADR explicitly chooses that coupling. Where this report uses upstream-compatible vocabulary, it MUST preserve the semantics of the target repo's `ReadinessReport`, `PortalReport`, `InputReport`, and `WindowingReport` concepts.

#### Scenario: Avoid strict upstream doctor coupling
- **GIVEN** the target repo has its own `doctor_report()` model
- **WHEN** `codex-computer-use-x11 doctor --json` returns its report
- **THEN** the report satisfies this spec's fields and shapes
- **AND** the report is not required to be a strict subset of the upstream `doctor_report()` JSON
- **AND** upstream-compatible fields use the same meaning as the target repo fields they reference

#### Scenario: Do not invent an upstream targeted-input field
- **GIVEN** the report explains whether targeted input is practically usable
- **WHEN** a consumer inspects the JSON
- **THEN** the report does not require a top-level or upstream-required `can_send_targeted_input` field
- **AND** any targeted-input explanation is derived from window query, focus, input backend, blocker, and recommendation facts
- **AND** any future derived targeted-input field remains additive, report-only, and outside top-level `readiness` unless a later accepted spec changes the contract

### Requirement: Doctor command is non-invasive
The doctor command MUST be safe for planning and smoke tests, and it MUST NOT modify the filesystem, patch the integration target, or require external credentials.

#### Scenario: Run doctor without external access
- **GIVEN** `.secrets.local.env` is absent or unread
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** the command does not require secret values
- **AND** the command does not write to the path that `CODEX_DESKTOP_LINUX_FULL_PATH` resolves to, including when the variable is unset and the documented development-machine default is used in its place
- **AND** the command reports unavailable capabilities as diagnostics instead of failing on missing X11 tools

#### Scenario: Inspect target path without patching it
- **GIVEN** `CODEX_DESKTOP_LINUX_FULL_PATH` is set or the documented development-machine default path exists
- **WHEN** `doctor --json` gathers local context
- **THEN** the command may report non-secret facts about the configured target path when needed
- **AND** it does not create, modify, stage, or delete files in the target checkout

## ADDED Requirements

### Requirement: Doctor capability detection report
The doctor JSON report MUST include structured capability facts for the local desktop session, installed tools, accessibility, X11/EWMH readiness, portal availability, screenshot providers, input backends, and degraded reasons. Degraded reasons MUST be exposed as an additive `readiness.degraded_reasons` array of strings so downstream automation has a stable machine-readable field distinct from blocking readiness failures.

#### Scenario: Report Cinnamon X11 environment facts
- **GIVEN** the process environment contains an X11 session with Cinnamon desktop variables and an X11 display
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes session or environment facts for session type, current desktop, desktop session, display, Wayland display presence, and runtime directory presence
- **AND** the report identifies the environment as compatible with the Cinnamon/X11 baseline without using `cinnamon` as the backend id
- **AND** `backend` remains `x11-ewmh`

#### Scenario: Report installed desktop-control tools
- **GIVEN** `wmctrl`, `xprop`, `xdotool`, and `ydotool` are available on `PATH`
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes installed-tool facts for each command
- **AND** each available command has an ok check or equivalent fact
- **AND** unavailable commands are reported as degraded reasons rather than causing a panic

#### Scenario: Report accessibility and input backend facts
- **GIVEN** AT-SPI, `/dev/uinput`, ydotool, and portal input probes have been evaluated
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes accessibility availability facts
- **AND** the accessibility facts distinguish at least AT-SPI bus reachability from whether accessibility is enabled for tree extraction
- **AND** the report includes distinct input backend facts for the target-repo `abs_pointer` capability and `/dev/uinput` device availability, `ydotool`, portal RemoteDesktop input, and X11-native `xdotool` candidate availability
- **AND** `readiness.can_send_development_input` is true when at least one supported upstream-shaped development input backend is verified: `abs_pointer` via read/write `/dev/uinput`, `ydotool` with a connectable socket, or portal RemoteDesktop with concrete methods or properties
- **AND** X11-native `xdotool` availability is reported as a separate candidate fact and does not by itself satisfy upstream-shaped `can_send_development_input` unless a later accepted design explicitly maps it

#### Scenario: Report degraded reasons separately from blockers
- **GIVEN** one optional desktop-control capability is unavailable while an alternative supported backend is available
- **WHEN** `doctor --json` returns its report
- **THEN** `readiness.degraded_reasons` includes a string explaining the unavailable optional capability
- **AND** `readiness.blockers` includes only failures that block the current supported readiness target
- **AND** `readiness.degraded_reasons` is present even when it is empty

### Requirement: Doctor headless and no-display behavior
The doctor command MUST produce a structured JSON report in headless or no-display environments whenever safe host inspection and serialization are still possible.

#### Scenario: Run without DISPLAY
- **GIVEN** the process environment has no usable `DISPLAY`
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** the command exits with status code 0 when it can emit a JSON report
- **AND** stdout is valid JSON
- **AND** X11/EWMH-dependent facts are marked unavailable or blocked
- **AND** `readiness.can_query_windows` is false
- **AND** the report includes a blocker or recommended next step explaining that X11 window probing is unavailable without a display

#### Scenario: Distinguish no-display from invalid CLI usage
- **GIVEN** the process environment has no usable `DISPLAY`
- **WHEN** a developer invokes the supported `doctor --json` command
- **THEN** the command does not use a non-zero exit solely because the display is unavailable
- **AND** non-zero exits are reserved for unsupported CLI usage or failures that prevent any JSON report from being produced

### Requirement: Doctor CLI exit behavior
The CLI MUST use stable exit behavior so automation can distinguish successful JSON reports, unsupported usage, and failures that prevent JSON output.

#### Scenario: Emit JSON report successfully
- **GIVEN** the doctor command can construct and serialize a report
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** the command exits with status code 0
- **AND** stdout contains one JSON object
- **AND** stderr is empty

#### Scenario: Reject unsupported CLI usage
- **GIVEN** a developer invokes an unsupported command or unsupported flags
- **WHEN** the CLI handles the invocation
- **THEN** the command exits with a non-zero status code
- **AND** it writes usage or error text to stderr
- **AND** it is not required to write a JSON doctor report to stdout

#### Scenario: Fail when no JSON report can be produced
- **GIVEN** an internal runtime or serialization failure prevents producing a valid doctor JSON report
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** the command exits with a non-zero status code
- **AND** it reports the failure on stderr without printing partial or invalid JSON as a successful report

### Requirement: Doctor ydotool socket detection
The doctor command MUST evaluate ydotool socket candidates in a deterministic order, continue past stale or missing earlier paths, select the first connectable socket for primary readiness, and preserve enough checked-path detail for degraded diagnostics.

#### Scenario: Continue after stale YDOTOOL_SOCKET
- **GIVEN** `YDOTOOL_SOCKET` names a missing or stale socket path
- **AND** `$XDG_RUNTIME_DIR/.ydotool_socket` is missing
- **AND** `/tmp/.ydotool_socket` is connectable
- **WHEN** `doctor --json` returns its report
- **THEN** the ydotool socket check is ok
- **AND** the selected connectable socket is `/tmp/.ydotool_socket`
- **AND** the report records that earlier candidates were checked and unavailable
- **AND** readiness does not fail solely because `YDOTOOL_SOCKET` was stale

#### Scenario: Report no connectable ydotool socket
- **GIVEN** `ydotool` is installed but no candidate ydotool socket is connectable
- **WHEN** `doctor --json` returns its report
- **THEN** the ydotool socket fact is not ok
- **AND** the report lists checked candidates or equivalent diagnostic details
- **AND** the recommended next step explains how to start or expose `ydotoold` when no other input backend is verified

### Requirement: Doctor strict portal and screenshot facts
The doctor command MUST distinguish screenshot providers from RemoteDesktop input capability and MUST NOT treat an empty successful portal introspection table as proof that a portal interface is available.

#### Scenario: Accept Screenshot portal version 2
- **GIVEN** portal Screenshot introspection includes the `Screenshot` method and reports version 2
- **WHEN** `doctor --json` returns its report
- **THEN** the portal screenshot fact is available
- **AND** the report does not require version 3-only properties to mark basic screenshot availability

#### Scenario: Reject empty RemoteDesktop introspection
- **GIVEN** portal RemoteDesktop introspection exits successfully but contains no concrete RemoteDesktop methods or properties
- **WHEN** `doctor --json` returns its report
- **THEN** the portal RemoteDesktop input fact is unavailable
- **AND** `readiness.can_send_development_input` does not become true from that empty introspection table
- **AND** the report includes a degraded reason or check detail explaining that no RemoteDesktop methods were found

#### Scenario: Report Cinnamon GNOME Shell compatible screenshot provider
- **GIVEN** `org.gnome.Shell.Screenshot` is owned by the Cinnamon process and exposes screenshot methods
- **WHEN** `doctor --json` returns its report
- **THEN** the report includes a GNOME Shell-compatible DBus screenshot provider fact
- **AND** the report keeps that provider distinct from XDG Portal Screenshot availability
- **AND** the report does not require `gnome-shell --version` to succeed before recognizing this DBus screenshot provider

### Requirement: Doctor probe parser fixture coverage
Implementation work for doctor probes MUST be testable without relying on the live desktop by covering external command and DBus parser behavior with fixtures, fake command output, or fake `PATH` fixtures before live smoke evidence is used to mark the behavior complete.

#### Scenario: Parse empty portal introspection fixture
- **GIVEN** a fixture or fake command output representing a successful `busctl introspect` call with only an empty header table
- **WHEN** the portal parser evaluates RemoteDesktop availability
- **THEN** it reports RemoteDesktop unavailable
- **AND** it records that required methods or properties were absent

#### Scenario: Parse screenshot provider fixture
- **GIVEN** a fixture or fake command output representing `org.gnome.Shell.Screenshot` methods exposed by Cinnamon
- **WHEN** the screenshot provider parser evaluates availability
- **THEN** it reports the GNOME Shell-compatible DBus screenshot provider as available
- **AND** it preserves provider provenance separately from portal screenshot availability

#### Scenario: Detect command availability without live X11 commands
- **GIVEN** tests use a fake command runner or fake `PATH` fixture
- **WHEN** doctor tool detection is evaluated
- **THEN** installed and missing command cases are observable without invoking live `wmctrl`, `xprop`, `xdotool`, or `ydotool`

## Claude Specs Review Disposition

The final specs review returned `pass` with no `mustFix`, no warnings, and no user-facing questions. The `shouldFix` items are handled as follows:

- Re-asserted the baseline planned-capabilities/self-check expectations in the modified `Doctor readiness shape` scenario, with the planned array allowed to become empty only when planned capabilities are implemented or moved into named capability facts.
- Tightened accessibility facts to distinguish AT-SPI bus reachability from accessibility enabled state for tree extraction; design may choose exact field names.
- Defined `readiness.ok` aggregation as blocker-based and chose `readiness.degraded_reasons` as the stable additive degraded-reasons shape.
- Deferred the pre-existing canonical `Purpose` field cleanup for `openspec/specs/doctor-cli/spec.md` and `openspec/specs/x11-integration-contract/spec.md` to archive/spec-sync or a dedicated maintenance change, because OpenSpec delta specs do not update canonical Purpose text directly and this one-artifact run should not directly edit canonical specs outside the change. The later `tasks.md` artifact must include an explicit follow-up task or archive note so this cleanup does not remain open indefinitely.
- The final rerun still passed and carried two task-stage `shouldFix` items. The later `tasks.md` artifact must include a concrete named task for canonical Purpose cleanup, and design/tasks must enumerate which previously planned capabilities may leave `capabilities.planned` once implemented or moved into named capability facts.
