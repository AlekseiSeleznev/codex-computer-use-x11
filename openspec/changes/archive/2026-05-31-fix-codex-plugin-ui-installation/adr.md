## ADR Review

## Existing In-Force ADRs

- `adr/0001-adopt-codex-native-intent-driven-openspec-overlay.md` — in force; change continues OpenSpec/Codex overlay workflow.
- `adr/0003-formalize-project-context-entrypoints.md` — in force; root constitution/architecture and local-secret boundaries were read and preserved.
- `adr/0005-adopt-matt-grill-and-tdd-gates.md` — in force; grill/design-review gates and TDD plan are used.
- `adr/0006-adopt-claude-artifact-review.md` — in force but session review is disabled through local session state.
- `adr/0007-adopt-automatic-checkpoints-and-claude-session-controls.md` — in force; safe lifecycle checkpoints are automatic in this session.
- `adr/0008-adopt-x11-root-coordinate-model.md` — in force; no coordinate-model change.
- `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md` — in force; change supports the documented Cinnamon/X11 standalone plugin baseline.

## Constitution / Architecture Rules Considered

- Rust 2021/Cargo remains the implementation stack.
- Project checks are `make fmt`, `make check`, and `make test`.
- User-local standalone plugin writes must remain under the owned `codex-computer-use-x11` `$CODEX_HOME` namespace.
- No `/opt`, `openai-bundled`, bundled `computer-use`, or source-overlay target writes are introduced by the standalone installer path.
- Real secret values must not be printed, tracked, committed, or copied into logs/artifacts.
- Standalone plugin remains separate from bundled `Computer Use` and keeps project-owned `x11_*` tool names.

## Decisions Evaluated

- UI identity fields: use `AlekseiSeleznev` and the actual GitHub repo; reject stale/misspelled owner and project-id-only developer text for user-facing UI.
- Legal links: omit privacy/terms links; reject pointing at unrelated OpenAI/GitHub policies.
- Desktop env hydration: add reversible MCP startup hydration for missing graphical env; reject hard-coded `DISPLAY=:0` and reject replacing bundled Computer Use.
- Icon: add a project-owned asset; reject copying bundled plugin artwork.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. `ARCHITECTURE.md` already describes the standalone plugin/source-overlay split and local-secret boundary; this change implements that existing posture.

## No ADR Needed

- No durable ADR is needed because the change is reversible installer/runtime polish inside the accepted standalone plugin architecture. It does not change backend identity, coordinate model, input safety invariants, source-overlay strategy, or project-wide architecture.
