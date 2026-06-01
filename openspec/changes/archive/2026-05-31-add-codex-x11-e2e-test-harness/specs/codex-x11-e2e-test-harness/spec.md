## ADDED Requirements

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

#### Scenario: Capability groups match backlog v1 coverage
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
