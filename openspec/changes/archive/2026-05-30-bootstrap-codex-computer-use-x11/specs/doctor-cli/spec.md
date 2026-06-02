## ADDED Requirements

### Requirement: Doctor JSON command
The CLI binary MUST be named exactly `codex-computer-use-x11`, and it MUST provide a `doctor --json` command that writes a single valid JSON object to stdout and identifies the bootstrap project without requiring any live X11 probes.

#### Scenario: Produce the bootstrap doctor report
- **GIVEN** the `codex-computer-use-x11` CLI is built from the standalone project
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** the command exits with status code 0
- **AND** stdout is valid JSON
- **AND** stderr is empty on success
- **AND** the JSON includes `project`, `version`, `backend`, `readiness`, `capabilities`, and `checks`
- **AND** `project` equals `codex-computer-use-x11`
- **AND** `version` equals the package version declared in root `Cargo.toml`
- **AND** `backend` equals `x11-ewmh`

### Requirement: Doctor readiness shape
The doctor JSON report MUST expose a stable bootstrap readiness shape so tests can assert behavior before real X11/EWMH probing exists.

#### Scenario: Inspect readiness and checks
- **GIVEN** the bootstrap implementation does not yet call `wmctrl`, `xprop`, or `xdotool`
- **WHEN** `doctor --json` returns its report
- **THEN** `readiness` is an object with boolean `ok` and array `blockers` fields
- **AND** `capabilities` is an object with `implemented` and `planned` arrays of strings
- **AND** `capabilities.implemented` contains `doctor-json`
- **AND** `capabilities.planned` is non-empty to signal future X11/EWMH work without requiring design-owned planned capability names yet
- **AND** `checks` is an array of objects
- **AND** `checks` contains at least one bootstrap self-check entry
- **AND** every check object includes string `name`, boolean `ok`, and string `detail` fields

### Requirement: Doctor report is a standalone bootstrap surface
The bootstrap doctor JSON report MUST be a standalone smoke-test surface for this project, and it MUST NOT be coupled to the upstream target repo `doctor_report()` shape unless a later design or ADR explicitly chooses that coupling.

#### Scenario: Avoid premature upstream doctor coupling
- **GIVEN** the target repo has its own `doctor_report()` model
- **WHEN** `codex-computer-use-x11 doctor --json` returns its bootstrap report
- **THEN** the report satisfies this spec's fields and shapes
- **AND** the report is not required to be a strict subset of the upstream `doctor_report()` JSON

### Requirement: Doctor command is non-invasive
The doctor command MUST be safe for planning and smoke tests, and it MUST NOT modify the filesystem, patch the integration target, or require external credentials.

#### Scenario: Run doctor without external access
- **GIVEN** `.secrets.local.env` is absent or unread
- **WHEN** a developer runs `codex-computer-use-x11 doctor --json`
- **THEN** the command does not require secret values
- **AND** the command does not write to the path that `CODEX_DESKTOP_LINUX_FULL_PATH` resolves to, including when the variable is unset and the documented development-machine default is used in its place
- **AND** the command reports unavailable future capabilities as bootstrap diagnostics instead of failing on missing X11 tools
