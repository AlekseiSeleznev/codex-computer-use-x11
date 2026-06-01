## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — accepted and remains in force for OpenSpec/Codex lifecycle.
- `adr/0003-formalize-project-context-entrypoints.md` — accepted and remains in force for root constitution, architecture snapshot, ADR history, and local-secret boundaries.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — accepted and remains in force; this change includes `grill.md`, `design-review.md`, `adr.md`, and `test-plan.md` before apply.
- `adr/0006-adopt-claude-artifact-review.md` — accepted and remains in force; current session state has Claude review disabled.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — accepted but user explicitly forbids commit/push/archive in this run, so no checkpoint commit is made without later explicit permission.
- `adr/0008-adopt-x11-root-coordinate-model.md` — accepted and remains in force for X11 root-coordinate and screenshot/app-state evidence semantics.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — accepted and remains in force; this change keeps AT-SPI semantic tree extraction as explicit degraded evidence when environment-limited and does not expand to Wayland.
- `adr/0010-adopt-x11-provider-takeover-shim.md` — accepted and remains in force; this change does not modify bundled `computer-use`, plugin identity, settings-provider takeover, or rollback behavior.

## Constitution / Architecture Rules Considered

- Secret handling: do not read/print/stage/commit `.secrets.local.env`; do not serialize unrelated environment values.
- Required technologies: Rust 2021/Cargo, root Makefile verification, Bash/Python helper scripts, Markdown OpenSpec artifacts.
- Verification: OpenSpec strict validation, `make fmt`, `make check`, `make test`, and `doctor --json` machine-readable validation are required before future completion claims.
- Safety: controlled fixtures are the only valid live targets for input, pointer, overlay, screenshot, app-state, and AT-SPI evidence.
- Scope: X11/Cinnamon baseline only; Wayland and portal-required runtime paths remain out of scope.

## Decisions Evaluated

- **Unset `NO_AT_BRIDGE` for controlled GTK fixture subprocesses** — accepted. It avoids presence-based bridge suppression and does not mutate global user environment.
- **Set `NO_AT_BRIDGE=0` for fixtures** — rejected. Existing evidence and historical GTK/ATK bridge behavior make this ambiguous and likely wrong.
- **Add a specific `atspi_gtk_bridge_disabled_by_environment` diagnostic state** — accepted as an additive refinement of tree-extraction-unavailable diagnostics.
- **Treat bridge-disabled AT-SPI as an X11 baseline blocker** — rejected. ADR 0009 allows optional semantic accessibility degradation while preserving X11 window/input readiness.
- **Use real user windows to prove tree extraction when fixtures are missing** — rejected for safety and privacy.
- **Change global session environment from the harness** — rejected; remediation is documented operator action and child fixture env sanitation only.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None required. The current architecture snapshot already states the X11-only baseline, degraded diagnostics, controlled fixture safety posture, and relevant ADR relationships.

## No ADR Needed

No durable ADR is needed because this change does not alter backend identity, coordinate model, provider takeover architecture, lifecycle policy, or supported runtime scope. It repairs diagnostic specificity and fixture environment handling within the already accepted Cinnamon/X11 baseline.
