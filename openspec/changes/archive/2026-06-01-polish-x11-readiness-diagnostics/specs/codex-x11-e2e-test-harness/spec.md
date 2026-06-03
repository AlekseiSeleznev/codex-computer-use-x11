## ADDED Requirements

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
