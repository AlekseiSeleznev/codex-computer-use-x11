## ADR Review

## Existing In-Force ADRs

- `adr/README.md` and `ARCHITECTURE.md` list the following in-force durable decisions as current project context:
  - `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — in force according to the snapshot; body is absent from this checkout, so only the README/snapshot summary was available.
  - `adr/0003-formalize-project-context-entrypoints.md` — in force according to the snapshot; body is absent from this checkout, so only the README/snapshot summary was available.
  - `adr/0005-adopt-matt-grill-and-tdd-gates.md` — in force according to the snapshot; body is absent from this checkout, so only the README/snapshot summary was available.
  - `adr/0006-adopt-claude-artifact-review.md` — in force according to the snapshot; body is absent from this checkout, so only the README/snapshot summary was available. Session Claude review is disabled by user request.
  - `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — in force according to the snapshot; body is absent from this checkout, so only the README/snapshot summary was available.
- Superseded according to `adr/README.md`/`ARCHITECTURE.md`: ADR 0002 and ADR 0004.

## Constitution / Architecture Rules Considered

- `CONSTITUTION.md`: Rust 2021/root Cargo package remains the implementation stack; root Makefile checks (`make fmt`, `make check`, `make test`) are required for Rust changes; OpenSpec validation is required for changed artifacts; no secrets or `.secrets.local.env` access are needed.
- `CONSTITUTION.md`: the local integration target is machine-specific via `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented default and must not be mutated unless an accepted task explicitly targets overlay changes.
- `ARCHITECTURE.md`: OpenSpec lifecycle remains `proposal -> specs -> grill -> design -> design-review -> adr -> test-plan -> tasks -> apply -> verify -> archive`.
- `ARCHITECTURE.md`: `grill.md` and `design-review.md` must reach `Open Questions: None`; both do.
- `ARCHITECTURE.md`: safe checkpoint commits are automatic in session `auto` mode; archive and push still require explicit approval, which the user provided in the session request.
- `CONTEXT.md`: `x11-ewmh`, `Active window`, `Focus verification`, and `FocusNotVerified` are the glossary terms used by the change.

## Decisions Evaluated

- **Use verified active-window identity as the standalone focus safety boundary.** Accepted in the change spec/design because it follows EWMH behavior and target repo activation semantics. Not promoted to a durable ADR because it is a stage-specific behavioral requirement under the existing `x11-ewmh` architecture, not a new project-wide architecture style.
- **Use `wmctrl -ia` before `xdotool windowactivate --sync` fallback.** Accepted in design as an MVP implementation choice with diagnostics and final verification. Not durable: it can be changed in a future implementation detail without superseding project architecture.
- **Do not mutate the target checkout in this stage.** Accepted as a scope boundary from the constitution/backlog. Not a new ADR: it preserves existing standalone-before-source-overlay architecture.
- **Do not bless direct `xdotool --window` input as safe.** Accepted as a safety boundary for this change. Not a durable ADR now because later input stages will produce their own specs/designs before adding input behavior.
- **Create a new durable ADR for focus verification.** Rejected by grill/design-review: the decision is important but expected from existing architecture and not hard to reverse at project architecture level.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. The current architecture snapshot already covers the standalone `x11-ewmh` path, OpenSpec lifecycle gates, automatic checkpoints, and local-secret boundaries. This change adds a new standalone behavior capability but does not alter the project architecture snapshot.

## No ADR Needed

- No durable ADR is needed because the change implements the next standalone behavior slice within the existing `x11-ewmh` direction. The choices are captured normatively in the change spec and design, do not mutate target architecture, do not supersede existing ADRs, and can be revisited by later input/source-overlay changes if new evidence appears.
