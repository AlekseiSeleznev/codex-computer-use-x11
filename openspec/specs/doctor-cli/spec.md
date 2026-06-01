# doctor-cli Specification

## Purpose
This specification defines the `codex-computer-use-x11 doctor --json` command as the project's machine-readable smoke-test and capability/readiness diagnostics surface for the standalone X11/EWMH integration path.
## Requirements
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

### Requirement: Doctor strict portal and screenshot facts
The doctor command MUST distinguish screenshot providers from RemoteDesktop input capability, MUST NOT treat an empty successful portal introspection table as proof that a portal interface is available, and MUST gather the live non-invasive portal/screenshot/AT-SPI probe outputs needed by app-state diagnostics when those commands are available.

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

#### Scenario: Live doctor probes feed app-state diagnostics
- **GIVEN** `busctl` or `gdbus` is available on `PATH`
- **WHEN** `doctor --json` gathers live non-invasive DBus diagnostics
- **THEN** it records RemoteDesktop, portal Screenshot, GNOME Shell-compatible screenshot, and AT-SPI bus reachability from actual introspection or call output when the probes can run
- **AND** failed or unavailable probes become structured unavailable/degraded facts rather than panics
- **AND** no secret values, screenshots, or target-checkout writes are produced by those probes

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

### Requirement: Canonical doctor CLI spec purpose metadata
The `doctor-cli` canonical specification MUST have a non-placeholder `## Purpose` section that describes the spec's role as the source of truth for the `codex-computer-use-x11 doctor --json` smoke-test and capability-readiness report surface.

#### Scenario: Replace bootstrap purpose placeholder
- **GIVEN** `openspec/specs/doctor-cli/spec.md` is the canonical doctor CLI specification
- **WHEN** the spec purpose metadata is inspected
- **THEN** the `## Purpose` section does not contain `TBD`
- **AND** it does not say it was created by archiving `bootstrap-codex-computer-use-x11`
- **AND** it describes the doctor CLI JSON report and capability/readiness diagnostic contract

#### Scenario: Preserve doctor CLI requirements
- **GIVEN** this maintenance change updates the canonical purpose metadata
- **WHEN** the canonical `doctor-cli` spec is compared before and after the change
- **THEN** existing normative requirements and scenarios remain semantically unchanged
- **AND** no Rust code or runtime CLI behavior is required to change

### Requirement: Doctor X11 readiness taxonomy
The doctor JSON report MUST expose a stable X11 readiness taxonomy that separates blocking failures, acceptable degraded X11-baseline limitations, optional enrichments, and unsupported out-of-scope paths without removing existing bootstrap-compatible fields.

#### Scenario: X11 baseline remains ready with optional enrichments degraded
- **GIVEN** `wmctrl`, `xprop`, `xdotool`, `DISPLAY`, and X11/EWMH window probes are usable
- **AND** AT-SPI tree extraction or RemoteDesktop portal probing is unavailable
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** `readiness.ok` is true when no X11-baseline blocker exists
- **AND** `readiness.blockers` is empty
- **AND** the report includes machine-readable degraded-but-acceptable entries for unavailable optional enrichments
- **AND** the report does not classify RemoteDesktop portal absence as an X11-baseline blocker

#### Scenario: X11 window probing blocker is distinct from optional degradation
- **GIVEN** the current environment has no usable `DISPLAY` or cannot run required X11/EWMH window probes
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** `readiness.ok` is false
- **AND** `readiness.blockers` contains a machine-readable X11 window-probing blocker
- **AND** optional AT-SPI, screenshot, ydotool, or portal facts do not hide the blocker

#### Scenario: Unsupported Wayland path is explicit and non-blocking for X11 diagnosis
- **GIVEN** the session facts indicate Wayland or an unavailable RemoteDesktop portal path
- **WHEN** `doctor --json` computes supported-scope readiness
- **THEN** the report records the unsupported or out-of-scope path separately from X11 blockers
- **AND** the recommendation explains that Wayland support is outside this change's runtime scope
- **AND** the report does not require portal input or screenshot paths to pass the Cinnamon/X11 baseline

### Requirement: Doctor AT-SPI diagnostic states
The doctor JSON report MUST distinguish AT-SPI bus unavailability, bus reachability, tree extraction unavailability, working tree extraction with no matching app subtree, ambiguous matches, and controlled-fixture pass evidence.

#### Scenario: Bus reachable but tree extraction unavailable
- **GIVEN** the AT-SPI bus can be reached
- **AND** tree extraction fails because accessibility is disabled, bridge support is missing, or the probe cannot enumerate application trees
- **WHEN** `doctor --json` emits accessibility facts
- **THEN** the AT-SPI diagnostic state is not merely `unavailable`
- **AND** it records bus reachability separately from tree extraction availability
- **AND** it recommends Cinnamon/X11 setup actions without marking the X11 window baseline failed

#### Scenario: Tree extraction works but no app subtree matches
- **GIVEN** AT-SPI tree extraction succeeds
- **AND** no extracted application subtree matches the selected or controlled target window
- **WHEN** `doctor --json` or fixture-backed diagnostics summarize AT-SPI
- **THEN** the diagnostic state identifies `no_matching_app_subtree` or an equivalent stable code
- **AND** the report preserves score or signal details sufficient to debug matching
- **AND** no arbitrary subtree is returned as a pass

#### Scenario: Ambiguous AT-SPI match is safe degradation
- **GIVEN** AT-SPI tree extraction succeeds
- **AND** multiple candidate subtrees cannot be distinguished with sufficient confidence
- **WHEN** AT-SPI diagnostics are serialized
- **THEN** the diagnostic state identifies ambiguity
- **AND** the report records candidate count or score facts without exposing secret values
- **AND** the X11 baseline may remain ready while semantic accessibility is degraded

### Requirement: Doctor report redacts private diagnostic paths
Doctor diagnostics MUST NOT serialize secret values or private socket/runtime absolute paths, while preserving stable labels that make readiness decisions reproducible.

#### Scenario: Private runtime paths are label-redacted
- **GIVEN** `YDOTOOL_SOCKET`, `XDG_RUNTIME_DIR`, or desktop bus paths contain user-private absolute paths
- **WHEN** `doctor --json` records checked diagnostic candidates
- **THEN** the serialized report uses stable labels such as `env:YDOTOOL_SOCKET` or `env:XDG_RUNTIME_DIR`
- **AND** it does not include the private absolute path values
- **AND** internal probing may still use the real paths locally

### Requirement: Doctor detects AT-SPI bridge-disabled environment
The doctor JSON report MUST distinguish a reachable AT-SPI bus from GTK/ATK bridge-disabled tree extraction when the process or controlled fixture environment contains `NO_AT_BRIDGE`. This diagnosis MUST be additive to existing bootstrap fields and MUST NOT make the Cinnamon/X11 window/input baseline fail by itself.

#### Scenario: Bus reachable but NO_AT_BRIDGE disables GTK bridge
- **GIVEN** AT-SPI bus reachability is true
- **AND** the doctor environment contains `NO_AT_BRIDGE=1`
- **AND** tree extraction is unavailable or no application trees can be enumerated
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** accessibility facts include `atspi_bus_available=true`
- **AND** accessibility facts include `tree_available=false`
- **AND** the diagnostic state is `atspi_gtk_bridge_disabled_by_environment`
- **AND** the reason category is `environment_limitation`
- **AND** the report records a sanitized bridge-env fact showing that `NO_AT_BRIDGE` is present without exposing unrelated environment values
- **AND** the X11 baseline readiness remains governed by X11/EWMH blockers, not by optional semantic accessibility enrichment alone

#### Scenario: Recommendation names remediation and controlled verification
- **GIVEN** the diagnostic state is `atspi_gtk_bridge_disabled_by_environment`
- **WHEN** doctor recommendations are serialized
- **THEN** the recommendation says to remove or avoid inheriting `NO_AT_BRIDGE=1` for GTK fixture/application processes
- **AND** it says to restart the affected Cinnamon/Codex session or fixture process so the bridge environment changes take effect
- **AND** it says to verify semantic accessibility with the controlled GTK fixture before claiming AT-SPI pass evidence
- **AND** it does not recommend Wayland or portal-required runtime paths as the remediation for the X11 baseline

#### Scenario: Bridge-enabled tree failure remains distinct
- **GIVEN** AT-SPI bus reachability is true
- **AND** `NO_AT_BRIDGE` is absent from the probed environment
- **AND** tree extraction remains unavailable
- **WHEN** `doctor --json` emits accessibility facts
- **THEN** the diagnostic state remains `atspi_tree_extraction_unavailable` or a more specific non-bridge-disabled state
- **AND** the recommendation mentions package, gsettings, process, and controlled fixture checks instead of incorrectly blaming `NO_AT_BRIDGE`

