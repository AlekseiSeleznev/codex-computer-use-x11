## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — accepted and remains in force for OpenSpec/Codex overlay workflow.
- `adr/0003-formalize-project-context-entrypoints.md` — accepted and remains in force for root constitution, architecture, ADR, and local-secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — accepted and remains in force; this change includes `grill.md`, `design-review.md`, `adr.md`, and `test-plan.md` before apply.
- `adr/0006-adopt-claude-artifact-review.md` — accepted and remains in force; session state currently has Claude review disabled.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — accepted and remains in force for safe local checkpoint commits in auto mode.
- `adr/0008-adopt-x11-root-coordinate-model.md` — accepted and remains in force; screenshot crop integrity and X11 root-coordinate semantics are preserved.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — accepted and remains in force; the change stays within Cinnamon/X11 `x11-ewmh` and does not add Wayland scope.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — accepted and remains in force; standalone plugin identity and localized provider takeover boundaries are unchanged.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md` secret handling: no `.secrets.local.env` access, no secret values in tracked artifacts, examples, logs, or reports.
- `CONSTITUTION.md` required technologies: Rust 2021/Cargo, root Makefile wrappers, Markdown OpenSpec artifacts with Gherkin-style scenarios, Bash helper scripts.
- `CONSTITUTION.md` verification: OpenSpec strict validation, `make fmt`, `make check`, `make test`, and machine-readable `doctor --json` validation before future apply completion.
- `ARCHITECTURE.md` lifecycle: proposal -> specs -> grill -> design -> design-review -> adr -> test-plan -> tasks -> apply -> verify -> archive.
- `ARCHITECTURE.md` X11-only baseline: Wayland and unsafe input without verification remain unsupported/out of scope.

## Decisions Evaluated

- **Additive readiness/evidence taxonomy instead of replacing doctor JSON:** accepted for backward compatibility with bootstrap consumers.
- **Use controlled fixtures as mandatory live safety boundary:** accepted because input/pointer/overlay against ambient apps is unsafe.
- **Keep Wayland/portal-required runtime paths out of scope:** accepted because ADR 0009 and user request explicitly scope this change to Cinnamon/X11.
- **Fake screenshot pass versus documented expected fake limitation:** left as implementation choice because both preserve real screenshot integrity and do not change architecture.
- **Durable ADR for reason categories:** rejected because reason categories are evidence-schema details within existing ADR 0009 DoD/evidence posture, not a new architecture direction.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None required. `ARCHITECTURE.md` already states the X11-only baseline, lifecycle gates, safe checkpoints, and relevant ADR relationships.

## No ADR Needed

- No durable ADR is needed because the change does not alter backend identity, coordinate model, provider takeover architecture, lifecycle rules, or supported runtime scope. It makes diagnostics, readiness, evidence, fixtures, and documentation stricter within the existing accepted architecture.
