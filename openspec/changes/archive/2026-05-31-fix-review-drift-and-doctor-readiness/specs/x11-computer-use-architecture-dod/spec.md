## MODIFIED Requirements

### Requirement: Final architecture decision ledger
The project MUST provide a final v1 architecture decision ledger for `codex-computer-use-x11` that records every required decision topic before the change can be archived. The ledger MUST include backend identity, window model, command execution seam, shell-out versus native X11 thresholds, diagnostics/readiness vocabulary, input safety invariant, keyboard/pointer backend priority, AT-SPI correlation, screenshot/root-coordinate model, `get_app_state` composition, standalone-plugin/source-overlay strategy, licensing/upstream policy, and Cinnamon extension/Wayland scope. Every top-level ADR file referenced by `ARCHITECTURE.md` or `adr/README.md`, including superseded historical ADRs, MUST exist as a tracked `adr/NNNN-*.md` file or the reference MUST be reconciled before validation can pass.

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

#### Scenario: Architecture ADR references are tracked
- **GIVEN** `ARCHITECTURE.md` and `adr/README.md` reference durable ADR files by `adr/NNNN-*.md` path
- **WHEN** the DoD checker validates architecture decisions
- **THEN** every referenced top-level ADR path exists in the repository
- **AND** every in-force ADR listed in the architecture snapshot has status and rationale available in its tracked ADR file
- **AND** superseded ADRs referenced as historical context also have tracked ADR files or the reference is removed from the snapshot/index
- **AND** validation fails with the missing ADR path when a referenced ADR file is absent

### Requirement: Machine-checkable final DoD validator
The project MUST provide a public final DoD validator command that checks the architecture decision ledger, capability matrix, research refresh, license/upstream references, required validation command list, and tracked ADR-reference consistency. The validator MUST run without live GUI access, without sudo, and without reading `.secrets.local.env`.

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

#### Scenario: Validator rejects missing tracked ADR references
- **GIVEN** `ARCHITECTURE.md` or `adr/README.md` references `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md`
- **AND** that file is absent from the repository
- **WHEN** a developer runs the final DoD validator from the repository root
- **THEN** validation exits non-zero
- **AND** stderr identifies the missing ADR reference
- **AND** no secret files or live desktop services are required to detect the issue
