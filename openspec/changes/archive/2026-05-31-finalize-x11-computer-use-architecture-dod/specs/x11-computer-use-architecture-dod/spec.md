## ADDED Requirements

### Requirement: Final architecture decision ledger
The project MUST provide a final v1 architecture decision ledger for `codex-computer-use-x11` that records every required decision topic before the change can be archived. The ledger MUST include backend identity, window model, command execution seam, shell-out versus native X11 thresholds, diagnostics/readiness vocabulary, input safety invariant, keyboard/pointer backend priority, AT-SPI correlation, screenshot/root-coordinate model, `get_app_state` composition, standalone-plugin/source-overlay strategy, licensing/upstream policy, and Cinnamon extension/Wayland scope.

#### Scenario: Decision ledger contains every required topic
- **GIVEN** the final architecture/DoD documentation exists
- **WHEN** the DoD checker validates architecture decisions
- **THEN** it finds a recorded decision for canonical backend id `x11-ewmh`
- **AND** it finds decisions for the window model, command seam, shell-out thresholds, diagnostics/readiness, input safety, input backend priority, AT-SPI, screenshot coordinates, app state, plugin/overlay strategy, licensing/upstreaming, and Cinnamon out-of-scope boundaries
- **AND** validation fails if any decision topic is missing or only present in chat/backlog text

#### Scenario: Backend identity is precise and upstream-compatible
- **GIVEN** the final decision ledger is checked
- **WHEN** it describes the backend identity
- **THEN** it states that `WindowInfo.backend` uses `x11-ewmh`
- **AND** it states that ambiguous backend id `x11` is not used for the canonical backend
- **AND** it records that `x11-ewmh` should be registered as a late generic fallback after more specific target backends unless an upstream review requires a compatible alias

#### Scenario: Unsafe global injection is not presented as targeted safety
- **GIVEN** the final decision ledger is checked
- **WHEN** it describes keyboard and pointer input
- **THEN** it states that `abs_pointer`, `ydotool`, and `xdotool` are global desktop injectors
- **AND** it states that direct `xdotool --window`/XSendEvent delivery is not a trusted targeted-safety boundary
- **AND** it requires verified target focus and bounds before targeted keyboard or pointer injection

### Requirement: Final capability matrix DoD
The project MUST provide a fine-grained Computer Use v1 capability matrix with one row for each required final capability. Each row MUST record whether the capability is required for v1, current status, concrete test or documentation evidence, and degraded behavior. A required row MUST be either `pass` or `degraded` with a concrete reason; missing rows, empty evidence, or unsupported required statuses MUST fail validation.

#### Scenario: Required matrix rows are complete
- **GIVEN** the final capability matrix is validated
- **WHEN** the DoD checker reads the matrix
- **THEN** it finds rows for doctor/capabilities, list windows, focused window, focus window with verification, safe target resolution, `get_app_state` with X11 target context, keyboard `type_text`, keyboard `press_key`, pointer click, pointer scroll, pointer drag, stock `activate_window`, stock `mousemove` absence handling, Cinnamon X11 input backend, screenshot/global provider, screenshot/window crop/bounds, AT-SPI tree, AT-SPI action/value set, terminal context selectors, standalone Codex MCP plugin, source overlay, E2E from Codex, and uninstall/rollback
- **AND** every required row has non-empty evidence
- **AND** every degraded required row has a non-empty degraded behavior reason

#### Scenario: Missing capability evidence fails before archive
- **GIVEN** a matrix fixture omits a required row or leaves its evidence empty
- **WHEN** the DoD checker validates it
- **THEN** the checker exits non-zero
- **AND** the error names the missing row or missing evidence field
- **AND** archive readiness is blocked until the row is completed or explicitly documented as degraded with a reason

#### Scenario: Final answer is precise, not absolute
- **GIVEN** all required final matrix rows pass DoD validation
- **WHEN** a maintainer asks whether this is full Computer Use
- **THEN** the docs answer yes for the documented Cinnamon/X11 v1 baseline with listed evidence
- **AND** the docs answer degraded or unsupported for Cinnamon Wayland, unstable Cinnamon extension work, unavailable AT-SPI/screenshot/input layers, and OS/window-manager limitations
- **AND** the docs state that unsafe targeted input without verification remains unsupported

### Requirement: Machine-checkable final DoD validator
The project MUST provide a public final DoD validator command that checks the architecture decision ledger, capability matrix, research refresh, license/upstream references, and required validation command list. The validator MUST run without live GUI access, without sudo, and without reading `.secrets.local.env`.

#### Scenario: Validator passes complete tracked DoD documents
- **GIVEN** the repository contains complete tracked final DoD documentation and matrix data
- **WHEN** a developer runs the final DoD validator from the repository root
- **THEN** the command exits with status code 0
- **AND** stdout reports that the final X11 Computer Use DoD is complete
- **AND** the validator does not require a live X11 display, Codex Desktop process, or secret file

#### Scenario: Validator rejects incomplete fixture
- **GIVEN** a fixture capability matrix is missing required v1 rows
- **WHEN** a test runs the final DoD validator against that fixture
- **THEN** validation exits non-zero
- **AND** stderr identifies the missing rows
- **AND** no tracked project files are modified by the validation attempt

#### Scenario: Release checklist includes final DoD validation
- **GIVEN** the final DoD validator exists
- **WHEN** release documentation is checked
- **THEN** `docs/release-checklist.md` includes the validator command
- **AND** the checklist keeps existing `make fmt`, `make check`, `make test`, fake e2e, OpenSpec validation, source-overlay rollback, license, secret-safety, git-clean, archive, and push gates

### Requirement: Fresh research and license refresh are captured
The final DoD artifacts MUST capture a 2026-05-31 research refresh for the current project repository, current target checkout, and relevant external references. The refresh MUST identify which ideas are kept, which are rejected, and how licenses affect copying versus runtime command invocation.

#### Scenario: Research refresh names current target facts
- **GIVEN** a maintainer reads the final architecture/DoD documentation
- **WHEN** they inspect the research refresh section
- **THEN** it records the current target checkout branch/status and the inspected `computer-use-linux/src/` files
- **AND** it records that current target stock vocabulary includes `activate_window`, `get_app_state`, `type_text`, `press_key`, `click`, `scroll`, and `drag`
- **AND** it records that v1 does not require a stock `focus_window` or stock `mousemove` tool unless future target research changes the contract

#### Scenario: License refresh separates invocation from copying
- **GIVEN** a maintainer evaluates `wmctrl`, `xdotool`, `ydotool`, and `x11rb`
- **WHEN** they read the final DoD license/upstream section
- **THEN** it states that runtime invocation of installed commands is different from copying or vendoring their source
- **AND** it records `wmctrl` and `ydotool` source as copy-unsafe without separate review
- **AND** it records `xdotool` and `x11rb` as usable references/dependencies only with their license obligations satisfied

### Requirement: DoD evidence integrates existing e2e matrix
The final DoD MUST consume and extend the existing e2e evidence posture rather than replacing it with a separate undocumented checklist. The final documentation MUST point from each capability row to existing tests, scripts, docs, or explicit degraded evidence, and the machine validator MUST ensure at least one concrete evidence reference is recorded for each required row.

#### Scenario: Capability rows link to existing project evidence
- **GIVEN** the final capability matrix is complete
- **WHEN** a maintainer inspects a required row
- **THEN** the row references concrete evidence such as Rust tests, CLI/MCP routes, e2e scripts, source-overlay smoke, documentation checks, OpenSpec specs, or manual/live degraded notes
- **AND** the row does not rely on unsupported chat-only assertions

#### Scenario: Existing e2e validation remains required
- **GIVEN** final DoD validation is added
- **WHEN** release and verification commands are listed
- **THEN** fake standalone plugin smoke and fake source-overlay smoke remain required
- **AND** `scripts/e2e/codex-x11-e2e.py validate-matrix` remains required for e2e evidence files
- **AND** the final DoD validator adds architecture/matrix completeness checks that the coarse e2e matrix does not cover
