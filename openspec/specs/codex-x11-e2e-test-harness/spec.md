# codex-x11-e2e-test-harness Specification

## Purpose
Defines the end-to-end harness contract for validating standalone plugin and source-overlay delivery paths through fake, live metadata, and controlled fixture evidence with capability-matrix semantics.

## Requirements
### Requirement: Standalone plugin smoke validates Codex-facing installation
The e2e harness MUST provide `scripts/e2e/codex-plugin-smoke.sh` as a public script for validating the standalone plugin delivery path. The script MUST support a no-GUI fake mode and a live mode, MUST validate the owned Codex marketplace/cache metadata under `CODEX_HOME`, MUST start the plugin MCP server through the installed `.mcp.json` command, and MUST fail with a clear diagnostic when the plugin directory or metadata is missing.

#### Scenario: Missing plugin directory fails clearly
- **GIVEN** a fake `CODEX_HOME` that does not contain the `codex-computer-use-x11` marketplace plugin directory
- **WHEN** a developer runs `scripts/e2e/codex-plugin-smoke.sh --fake --codex-home <dir>`
- **THEN** the command exits with a non-zero status code
- **AND** stderr or the JSON evidence identifies the missing standalone plugin installation
- **AND** a failure log is written under the selected e2e log directory

#### Scenario: Marketplace metadata points to owned plugin
- **GIVEN** a fake `CODEX_HOME` populated by the project-owned plugin installer
- **WHEN** a developer runs `scripts/e2e/codex-plugin-smoke.sh --fake --codex-home <dir>`
- **THEN** the smoke validates that `plugins/marketplaces/codex-computer-use-x11/.agents/plugins/marketplace.json` contains plugin `codex-computer-use-x11`
- **AND** the marketplace plugin path resolves to the owned cache namespace `plugins/cache/codex-computer-use-x11/codex-computer-use-x11/latest`
- **AND** the installed `.codex-plugin/plugin.json` and `.mcp.json` are valid JSON with the expected plugin name and MCP command

#### Scenario: MCP server starts from installed manifest
- **GIVEN** a fake `CODEX_HOME` with an installed standalone plugin and an executable plugin binary
- **WHEN** the plugin smoke starts the MCP server using the installed `.mcp.json`
- **THEN** the smoke completes MCP `initialize` and `tools/list` over stdio
- **AND** the response contains `x11_doctor`, `x11_list_windows`, `x11_focused_window`, `x11_focus_window`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, `x11_drag`, `x11_accessibility_tree`, `x11_get_app_state`, `x11_target_window`, `x11_release_window`, and `x11_target_context`
- **AND** the response does not expose unnamespaced stock tool names such as `doctor`, `activate_window`, `type_text`, `press_key`, `click`, `scroll`, `drag`, or `get_app_state`

### Requirement: Fake plugin smoke verifies safe tool routing without GUI
In fake mode, the plugin smoke MUST avoid real desktop mutation by using isolated fake command fixtures and MUST prove the MCP tool routes for doctor, window listing/focus, app state, keyboard input, and pointer input. Fake mode MUST verify the strict RemoteDesktop portal false-positive case where `busctl` exits successfully but the introspection output contains no RemoteDesktop methods.

#### Scenario: Fake command fixture exercises X11 window routes
- **GIVEN** fake `wmctrl`, `xprop`, `xdotool`, and `busctl` commands on a temporary `PATH`
- **WHEN** the plugin smoke calls `x11_doctor`, `x11_list_windows`, `x11_focused_window`, and `x11_focus_window` through MCP
- **THEN** the evidence records a valid `x11-ewmh` doctor report
- **AND** at least one fake X11 window is listed and the focused window is readable
- **AND** focus activation is verified through a fresh fake active-window lookup without touching the real desktop
- **AND** the RemoteDesktop portal is not marked available solely because `busctl` returned status code 0

#### Scenario: Fake input smoke routes keyboard and pointer tools
- **GIVEN** fake X11 window bounds and fake `xdotool` that writes invoked arguments to the e2e log directory
- **WHEN** the plugin smoke calls `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, and `x11_drag` through MCP against the fake target window
- **THEN** every tool returns a structured MCP tool result
- **AND** each input action records verified target focus or an explicit degraded reason before any fake input is logged
- **AND** the evidence records the selected input backend or route, such as standalone `xdotool`, and confirms no portal route was selected when RemoteDesktop methods were absent

#### Scenario: App state smoke returns layered evidence
- **GIVEN** the fake command fixture can resolve a target X11 window
- **WHEN** the plugin smoke calls `x11_get_app_state` through MCP
- **THEN** the evidence includes window context or a clear window degraded reason
- **AND** screenshot and AT-SPI layers are recorded as pass or degraded independently
- **AND** a missing screenshot or AT-SPI layer does not hide valid window-context evidence

### Requirement: Source overlay smoke is reversible and stock-tool aware
The e2e harness MUST provide `scripts/e2e/codex-source-overlay-smoke.sh` as a public script for validating the source-overlay delivery path. The script MUST support fake and live modes, MUST run overlay status/install/uninstall checks without sudo, MUST never modify `/opt/codex-desktop` directly, MUST return the real target checkout to its pre-smoke clean state in live mode, and MUST use target-stock tool vocabulary such as `activate_window` instead of assuming a stock `focus_window` tool.

#### Scenario: Fake source overlay smoke applies and uninstalls a fixture target
- **GIVEN** fake mode with no GUI dependency
- **WHEN** a developer runs `scripts/e2e/codex-source-overlay-smoke.sh --fake`
- **THEN** the smoke creates or uses a temporary target fixture with the expected `computer-use-linux` structure
- **AND** `status-codex-source-overlay.sh` reports `state=clean` before install
- **AND** `install-codex-source-overlay.sh` applies owned marker blocks and the generated `x11_ewmh.rs` backend
- **AND** `uninstall-codex-source-overlay.sh` removes owned content
- **AND** final status reports `state=clean`

#### Scenario: Live source overlay smoke leaves real target clean
- **GIVEN** live mode is run against a real target checkout selected by `--target`, `CODEX_DESKTOP_LINUX_FULL_PATH`, or the documented local default
- **AND** the target checkout starts with a clean `git status --short`
- **WHEN** the source-overlay smoke runs status, install, target checks, uninstall, and final status
- **THEN** target mutations occur only inside the reversible source-overlay interval
- **AND** the final target `git status --short` is clean
- **AND** failure during target checks still attempts uninstall before reporting the failure

#### Scenario: Stock target tool vocabulary is reflected in source evidence
- **GIVEN** the current target source exposes `activate_window`, `get_app_state`, `click`, `scroll`, `drag`, `press_key`, and `type_text`
- **WHEN** source-overlay smoke evaluates stock tool coverage
- **THEN** the evidence maps window focus to stock `activate_window`
- **AND** it records absence of stock `focus_window` or `mousemove` as a documented non-blocking fact instead of failing solely for those absent names
- **AND** it does not require a competing stock `x11_get_app_state` tool in the target repo

### Requirement: Capability matrix evidence covers every v1 group
The e2e harness MUST produce machine-readable capability-matrix evidence for both standalone plugin and source-overlay paths. Each required v1 capability group MUST be marked `pass` or `degraded` with a concrete reason; the harness MUST fail when any group is missing evidence or silently omitted.

#### Scenario: Missing capability evidence fails the smoke
- **GIVEN** a fake evidence file where one required v1 capability group has no standalone or source-overlay status
- **WHEN** the e2e harness validates capability-matrix coverage
- **THEN** validation fails with `missing evidence` for that capability group
- **AND** the failure names the missing group and delivery path

#### Scenario: Degraded capability evidence is explicit and accepted
- **GIVEN** fake mode cannot access a real screenshot provider or AT-SPI bus
- **WHEN** capability-matrix validation runs
- **THEN** screenshot and AT-SPI groups may be marked `degraded`
- **AND** each degraded entry includes the reason and the tool/check that produced it
- **AND** all other implemented fake evidence remains visible in the same JSON report

#### Scenario: Capability groups match v1 coverage
- **GIVEN** a completed plugin and source-overlay smoke run
- **WHEN** a developer inspects the machine-readable evidence
- **THEN** it contains entries for doctor/capabilities, window listing/focus, `get_app_state`, keyboard input, pointer input, screenshot, AT-SPI, and install/rollback
- **AND** each entry separately records standalone plugin evidence and source-overlay evidence

### Requirement: E2E logs are durable and safe
Every e2e run MUST write logs and JSON evidence under `target/e2e-logs/` by default or a caller-specified log directory. Logs MUST be written on success and failure, MUST avoid secret values, and MUST include enough command/output context to reproduce failures without requiring Codex chat history.

#### Scenario: Failure retains logs
- **GIVEN** a smoke run fails during marketplace validation or MCP startup
- **WHEN** the command exits non-zero
- **THEN** the log directory contains a run log and a JSON evidence file
- **AND** stderr points to the log directory
- **AND** the logs include the failed check name, command boundary, and sanitized diagnostic output

#### Scenario: Fake mode runs without GUI and without sudo
- **GIVEN** a CI environment without `DISPLAY` or a real desktop session
- **WHEN** a developer runs the fake plugin and source-overlay smoke scripts
- **THEN** both scripts can complete using isolated fixtures
- **AND** neither script requires sudo, reads `.secrets.local.env`, or writes outside the repository temp/log directories and the caller-provided fake `CODEX_HOME`

#### Scenario: Live mode documents manual Codex Desktop steps when direct stock runner is unavailable
- **GIVEN** a live source-overlay smoke cannot find a stable direct Codex Desktop stock tool-call runner
- **WHEN** the script completes its machine-checkable target/source checks
- **THEN** the evidence records the stock runner as degraded with a concrete reason
- **AND** the documentation lists manual Codex Desktop smoke steps for stock `doctor`, `list_windows`, `focused_window`, `activate_window`, `get_app_state`, keyboard, pointer, screenshot, AT-SPI, and rollback evidence

### Requirement: Live harness verifies exact Unicode text fidelity
Live e2e mode MUST verify the actual value inserted into a safe text field for Cyrillic and other non-ASCII text, not only key-release events or backend exit status. The evidence MUST identify the selected keyboard route and mark the keyboard capability degraded when exact text value does not match.

#### Scenario: Cyrillic value matches expected text
- **GIVEN** live mode starts a safe text fixture with a readable text value endpoint or event log
- **WHEN** the harness calls `x11_type_text` with Cyrillic text such as `Привет`
- **THEN** the fixture value equals the requested Cyrillic string at the checked insertion point
- **AND** the evidence records the keyboard route used
- **AND** the keyboard capability row is `pass` for Unicode fidelity

#### Scenario: Cyrillic key events without exact value are degraded
- **GIVEN** live mode sends Cyrillic text
- **AND** the fixture event log only shows layout-dependent Latin keysyms or the final value differs from the requested string
- **WHEN** capability matrix validation runs
- **THEN** the keyboard capability row is `degraded`
- **AND** the reason states that exact Cyrillic fidelity was not proven

### Requirement: Live harness includes GTK AT-SPI positive fixture
Live e2e mode MUST include an AT-SPI-positive GTK fixture or documented GTK-safe application and MUST validate that `x11_accessibility_tree` can return a matched subtree for that fixture. Tkinter windows MAY remain in the fixture set for keyboard and pointer checks but MUST NOT be the sole acceptance evidence for AT-SPI.

#### Scenario: GTK fixture AT-SPI pass is recorded
- **GIVEN** live mode starts or selects a GTK fixture with stable title and accessible controls
- **WHEN** the harness targets the GTK fixture with `x11_accessibility_tree`
- **THEN** a high- or medium-confidence subtree is returned
- **AND** expected accessible control names or roles are present
- **AND** the AT-SPI capability row records pass with GTK evidence

#### Scenario: Tk AT-SPI no-match is documented separately
- **GIVEN** Tkinter safe windows are present for keyboard and pointer checks
- **WHEN** AT-SPI matching returns `NoAccessibilityMatch` for Tk windows
- **THEN** the harness records that as Tk fixture limitation evidence
- **AND** it does not lower correlation thresholds or use bounds-only matching to pass AT-SPI

### Requirement: Live harness verifies overlay lifecycle
Live e2e mode MUST verify that target overlay display, release/hide behavior, and overlay listing exclusion work when overlay is requested. Overlay provider failure MAY remain degraded only when the failure is explicit and target state lifecycle still passes.

#### Scenario: Overlay shown and hidden in live mode
- **GIVEN** live mode has a safe target window with valid bounds
- **WHEN** the harness runs `x11_target_window` with overlay enabled
- **THEN** the tool report has `overlay.requested=true` and `overlay.shown=true`
- **AND** subsequent listing excludes project overlay windows from target candidates
- **WHEN** the harness runs `x11_release_window`
- **THEN** overlay diagnostics show hide requested or completed
- **AND** follow-up target context is empty or no longer contains the released target

#### Scenario: Overlay failure is explicit degraded evidence
- **GIVEN** overlay is requested in live mode
- **AND** the provider cannot show a border
- **WHEN** capability matrix validation runs
- **THEN** target context lifecycle may pass
- **AND** overlay status is `degraded` with the provider warning
- **AND** the failure is not silently omitted from readiness evidence

### Requirement: Capability matrix records pass and degraded rows with concrete evidence
The e2e harness MUST update machine-readable capability matrix rows for live and fake modes so every required v1 group has `pass` or `degraded` status with concrete evidence paths, tool names, and reasons. Missing rows or summary extraction bugs MUST fail validation.

#### Scenario: Live matrix includes concrete degraded reasons
- **GIVEN** live mode produces evidence for keyboard, AT-SPI, overlay, app-state, and portal readiness
- **WHEN** matrix validation runs
- **THEN** each row has a status of `pass` or `degraded`
- **AND** degraded rows include concrete evidence such as log paths, tool names, and observed error codes
- **AND** missing rows fail validation

#### Scenario: Evidence summary uses no-screenshot-data output
- **GIVEN** live app-state captured a screenshot with a large data URL
- **WHEN** the harness writes durable summary files
- **THEN** summary files omit the full base64 screenshot data
- **AND** they retain screenshot status and metadata needed to validate the screenshot capability

### Requirement: Live plugin smoke orchestrates controlled fixtures
Live standalone plugin smoke MUST create or select controlled fixture windows for fixture-dependent capabilities instead of marking those capabilities degraded solely because no fixture was orchestrated. The harness MUST use unique fixture titles/classes, readiness probes, timeouts, and cleanup traps for Tk keyboard/pointer/focus/target/release, GTK AT-SPI, optional overlay, screenshot crop, and app-state checks.

#### Scenario: Live smoke starts and cleans controlled fixtures
- **GIVEN** a developer runs `scripts/e2e/codex-plugin-smoke.sh --live`
- **WHEN** fixture-backed checks are enabled for the current desktop session
- **THEN** the harness starts fixture windows with unique `codex-x11-*` titles or classes
- **AND** it waits for each fixture readiness signal before tool calls
- **AND** it records fixture process ids and window ids in the run evidence
- **AND** it tears down all fixture processes and overlay state on success or failure

#### Scenario: Missing fixture setup is not an accepted pass
- **GIVEN** live smoke cannot start a required fixture because a dependency or display capability is unavailable
- **WHEN** capability matrix validation evaluates the evidence
- **THEN** the affected capability is not reported as `pass`
- **AND** the reason identifies `missing_fixture_setup` or a more specific dependency cause
- **AND** the validator distinguishes that reason from expected environment degradation and code failure

### Requirement: Live plugin smoke verifies fixture-backed capability rows
Live standalone plugin smoke MUST exercise fixture-backed tool calls for keyboard input, pointer input, window listing/focus, target context/release, screenshot, `get_app_state`, GTK AT-SPI, and optional overlay lifecycle. Each exercised capability row MUST include the concrete fixture id, tool call, evidence path, status, and reason. GTK AT-SPI fixture evidence MUST record bridge-environment facts with `NO_AT_BRIDGE` absent rather than `NO_AT_BRIDGE=0`.

#### Scenario: Tk fixture backs keyboard and pointer rows
- **GIVEN** live smoke starts a Tk text/pointer fixture with a unique title
- **WHEN** the harness calls `x11_focus_window`, `x11_type_text`, `x11_press_key`, `x11_click`, `x11_scroll`, and `x11_drag` against that fixture
- **THEN** keyboard and pointer capability rows are `pass` when the fixture event/value evidence matches expectations
- **AND** each input report proves verified target focus or bounds before input is sent
- **AND** no input evidence references a non-fixture application window

#### Scenario: GTK bridge fixture backs AT-SPI row
- **GIVEN** live smoke starts a GTK fixture with `GTK_MODULES=gail:atk-bridge` when needed and with `NO_AT_BRIDGE` absent
- **WHEN** the harness calls `x11_accessibility_tree` against the GTK fixture
- **THEN** the AT-SPI capability row is `pass` when the returned tree contains the expected accessible role or name
- **AND** Tk `NoAccessibilityMatch` evidence remains fixture-specific degraded evidence rather than the semantic AT-SPI pass path

#### Scenario: Screenshot and app-state target only fixtures
- **GIVEN** live smoke has selected a controlled fixture window and resolved its bounds
- **WHEN** the harness calls screenshot crop and `x11_get_app_state`
- **THEN** screenshot evidence stores image bytes as files under `target/e2e-logs/<run-id>/`
- **AND** app-state evidence records sanitized layer summaries or file paths rather than dumping full screenshot data URLs into ordinary logs
- **AND** the capability rows identify the fixture window used for the check

#### Scenario: Optional overlay lifecycle is fixture-scoped
- **GIVEN** overlay checks are enabled with `CODEX_X11_ENABLE_TK_OVERLAY=1`
- **WHEN** the harness targets a controlled fixture with overlay and then releases it
- **THEN** overlay evidence records `overlay.shown=true` and release hide evidence when the provider works
- **AND** overlay degradation is explicit when the provider is unavailable
- **AND** overlay helper windows are not selected as input or screenshot targets

### Requirement: Industrial evidence matrix classification
The E2E evidence schema and matrix validator MUST classify each fixture-backed capability with canonical machine JSON statuses `pass`, `degraded`, or `fail` and machine-readable reason categories that distinguish expected environment limitations, missing fixture setup, and actual code failure. Missing fixture setup MUST NOT be counted as acceptable industrial pass evidence.

#### Scenario: Environment limitation is degraded with evidence
- **GIVEN** GTK accessibility dependencies are unavailable in a live desktop session
- **WHEN** live smoke records the AT-SPI fixture outcome
- **THEN** the AT-SPI row status is `degraded`
- **AND** the reason category is `environment_limitation`
- **AND** evidence names the missing dependency or bridge condition

#### Scenario: Missing fixture setup blocks industrial acceptance
- **GIVEN** live smoke skipped keyboard input because no safe text fixture was started
- **WHEN** `validate-matrix` runs in industrial mode
- **THEN** validation fails or marks the run not industrial-ready
- **AND** the reason category is `missing_fixture_setup`
- **AND** the result is not normalized to acceptable degraded evidence

#### Scenario: Code failure is a fail
- **GIVEN** a controlled fixture is ready and the required tool call returns `success=false` for a non-environment reason
- **WHEN** the matrix validator evaluates the evidence
- **THEN** the affected row status is `fail`
- **AND** the reason category is `code_failure`
- **AND** validation exits non-zero for an industrial acceptance run

### Requirement: Evidence rows use canonical reason categories
The e2e evidence schema and matrix validator MUST require every non-pass row to include a stable `reason_category` that distinguishes expected environment limitations, missing fixture setup, code failure, unsupported out-of-scope paths, and documented fake-fixture limitations.

#### Scenario: Missing fixture setup is not environment degradation
- **GIVEN** live metadata-only smoke runs without controlled fixtures
- **WHEN** the matrix validator evaluates fixture-backed keyboard, pointer, AT-SPI, screenshot, app-state, target, or overlay rows
- **THEN** rows skipped because no safe fixture was started use `reason_category=missing_fixture_setup`
- **AND** industrial readiness is not reported as pass for those rows
- **AND** the summary explains that testing real user applications would be unsafe

#### Scenario: Environment limitation remains acceptable degraded evidence
- **GIVEN** a controlled fixture is started or attempted safely
- **AND** a desktop dependency such as AT-SPI tree extraction or optional overlay display is unavailable
- **WHEN** evidence is written
- **THEN** the row may be `degraded` with `reason_category=environment_limitation`
- **AND** the evidence names the unavailable dependency or probe outcome
- **AND** the validator distinguishes this from code failure

#### Scenario: Code failure fails the matrix
- **GIVEN** fixture setup succeeds
- **AND** a tool call, parser, cleanup, safety check, or output integrity assertion violates its expected behavior
- **WHEN** matrix validation runs
- **THEN** the affected row is `fail` with `reason_category=code_failure`
- **AND** the overall run is not accepted as production-ready

#### Scenario: Wayland and portal-required paths are out of scope
- **GIVEN** the environment exposes Wayland or lacks RemoteDesktop portal support
- **WHEN** X11-only evidence is summarized
- **THEN** Wayland or portal-required runtime paths are classified as unsupported/out of scope when mentioned
- **AND** their absence does not block Cinnamon/X11 baseline readiness
- **AND** no row implies that Wayland support was tested or required

### Requirement: Controlled live fixtures prove uniqueness and cleanup
Live fixture-backed smoke MUST prove it targeted only controlled fixtures and cleaned target, overlay, and process state on success and failure.

#### Scenario: Fixture target uniqueness is proven before input
- **GIVEN** live smoke intends to send keyboard, pointer, screenshot, app-state, target, or overlay operations
- **WHEN** it resolves a target window
- **THEN** the evidence proves the target title, class, process, or marker is unique to the current run fixture
- **AND** ambiguous or multiple matching fixture candidates block input rather than selecting a real user app
- **AND** no input or overlay operation falls back to an ambient non-fixture window

#### Scenario: Cleanup evidence is recorded
- **GIVEN** live smoke started controlled fixtures or showed overlays
- **WHEN** the run exits successfully or with failure
- **THEN** it attempts to hide overlays, release target context, stop fixture processes, and clear stale target state
- **AND** evidence records cleanup status for each cleanup action
- **AND** stale target context after cleanup is a failed or degraded row with a concrete reason

### Requirement: Evidence summaries are readable and path-based
E2E summaries MUST be concise, safe to inspect in logs, and link to durable evidence paths instead of embedding large screenshot data or raw secret-bearing environment values.

#### Scenario: Screenshot evidence is referenced by path
- **GIVEN** screenshot or app-state evidence captures image bytes
- **WHEN** the harness writes JSON summaries and logs
- **THEN** ordinary summaries include file paths, dimensions, status, and integrity metadata
- **AND** they do not inline screenshot data URLs or base64 payloads
- **AND** missing or degraded screenshots include a reason category and the tool/check that produced it

### Requirement: Fixture bridge environment is sanitized and self-tested
The e2e harness MUST launch the controlled GTK AT-SPI fixture with a safe bridge environment, record sanitized environment facts, and provide fake/self-test coverage for bridge-disabled and bridge-enabled fixture paths without changing the global user environment.

#### Scenario: Fixture launch removes inherited NO_AT_BRIDGE
- **GIVEN** the Codex or harness parent environment contains `NO_AT_BRIDGE=1`
- **WHEN** live smoke starts the controlled GTK AT-SPI fixture
- **THEN** the fixture subprocess environment removes `NO_AT_BRIDGE`
- **AND** the parent process environment and global user environment are not modified
- **AND** the fixture metadata records `NO_AT_BRIDGE` as absent for the fixture process
- **AND** the metadata records `GTK_MODULES=gail:atk-bridge` when that override was applied for the fixture

#### Scenario: Fake bridge-disabled evidence is classified as environment limitation
- **GIVEN** fake smoke or validator fixtures include doctor/accessibility evidence with `atspi_bus_available=true`, `tree_available=false`, and `NO_AT_BRIDGE` present
- **WHEN** the matrix validator evaluates the AT-SPI row
- **THEN** the row may be `degraded` with `reason_category=environment_limitation`
- **AND** the reason or evidence references `atspi_gtk_bridge_disabled_by_environment`
- **AND** the validator does not classify the absence of a real live GTK fixture implementation as `code_failure` solely because the environment is bridge-disabled

#### Scenario: Missing live fixture code remains a setup limitation
- **GIVEN** a live run has no controlled GTK fixture code available or cannot start it safely
- **WHEN** fixture-dependent AT-SPI validation is summarized
- **THEN** the row uses `missing_fixture_setup` or the precise dependency/environment category supported by the evidence
- **AND** no real user window is selected as a fallback AT-SPI target
- **AND** the summary tells the operator to run the controlled GTK fixture path after correcting bridge environment

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

