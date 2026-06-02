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

- `CONSTITUTION.md`: Rust 2021 and root Cargo package remain the implementation stack; root `Makefile` checks (`make fmt`, `make check`, `make test`) are required for Rust changes.
- `CONSTITUTION.md`: OpenSpec validation is required; changed OpenSpec artifacts remain source of truth.
- `CONSTITUTION.md`: local integration target checkout is machine-specific and must not be mutated unless an accepted task explicitly targets overlay changes. This change keeps it read-only.
- `CONSTITUTION.md`: local secret files and real credential values must not be printed, staged, committed, archived, or copied. Installer/config handling must avoid full config dumps.
- `ARCHITECTURE.md`: lifecycle remains `proposal -> specs -> grill -> design -> design-review -> adr -> test-plan -> tasks -> apply -> verify -> archive`.
- `ARCHITECTURE.md`: `grill.md` and `design-review.md` must reach `Open Questions: None`; both do.
- `ARCHITECTURE.md`: safe checkpoint commits are automatic in session `auto` mode; archive and push require explicit approval, which the user provided in the session request.
- `CONTEXT.md`: `Standalone plugin`, `x11-ewmh`, `Active window`, `Focus verification`, and `FocusNotVerified` glossary terms are respected.

## Decisions Evaluated

- **Add a minimal internal MCP stdio server instead of a full MCP framework dependency.** Accepted as a reversible implementation decision for a small standalone feedback loop. Not promoted to a durable ADR because it can be replaced later without changing project architecture.
- **Use owned namespace `codex-computer-use-x11` and `x11_*` tool names.** Accepted to avoid collisions with `computer-use@openai-bundled`. Not promoted to a durable ADR because the standalone plugin path is already part of the project posture and the exact namespace is captured by the change spec/design.
- **Install into user-local cache/marketplace/config rather than `/opt` or `openai-bundled`.** Accepted for reversibility and safety. Not promoted to a durable ADR because it is an implementation of the existing local-plugin delivery path rather than a hard-to-reverse architecture change.
- **Use owned section replacement for Codex config.** Accepted to preserve unrelated config and avoid leaking secrets. Not durable: it can evolve if Codex plugin config format changes.
- **Create a new durable ADR for standalone plugin packaging.** Rejected by grill/design-review because the decision is reversible, local, and expected from existing architecture/backlog rather than surprising project-wide architecture.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` already includes the standalone plugin delivery posture, OpenSpec lifecycle gates, session Claude controls, and local-secret boundaries. This change adds concrete behavior under that posture but does not change the project architecture snapshot.

## No ADR Needed

- No durable ADR is needed because this change implements the next standalone plugin behavior slice within the existing Codex-first/X11-EWMH architecture. The implementation choices are user-local, reversible, and fully captured by the change spec/design; no existing ADR is superseded and no architecture snapshot update is required.
