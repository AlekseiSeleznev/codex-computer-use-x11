## ADR Review

## Existing In-Force ADRs

- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted and in force. The generated backend preserves signed X11 root/global coordinates in target `WindowBounds` and does not redefine coordinate semantics.
- `ARCHITECTURE.md` and `adr/README.md` summarize earlier in-force project decisions for the Codex/OpenSpec overlay, project context entrypoints, Matt grill/TDD gates, Claude review controls, automatic checkpoints, and source-overlay boundaries. Those rules remain project context for this change.

## Constitution / Architecture Rules Considered

- Rust 2021/Cargo remains the implementation stack for the standalone crate; root `make fmt`, `make check`, and `make test` are required before completion.
- The local target checkout is machine-specific and referenced by `CODEX_DESKTOP_LINUX_FULL_PATH` or `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` on this machine.
- The target checkout can be modified only for the explicit source-overlay task and must be returned to clean state after verification.
- OpenSpec remains source of truth; planning artifacts must be completed and checkpointed before apply.
- No secrets are needed; `.secrets.local.env` is not read, printed, staged, archived, or copied.
- External source reuse must respect license boundaries. MIT/Apache references may inform design, but GPL/AGPL/no-license/unknown code is not copied.

## Decisions Evaluated

- **Marker-owned source overlay vs long-lived target fork**: choose marker-owned overlay scripts and one generated backend file. Durable ADR rejected because the durable source-overlay boundary already exists in architecture context and this change implements a reversible local mechanism.
- **Python overlay engine vs Bash-only patching**: choose Python engine with shell wrappers. Durable ADR rejected because it is an implementation detail that can be replaced without changing architecture.
- **Install repairs drift vs refuses drift**: choose refusal by default. Durable ADR rejected because the safety behavior is local to the script contract and captured in specs/design.
- **Shell-command X11 backend vs native `x11rb` target dependency**: choose shell-command backend for this reversible overlay. Durable ADR rejected because native dependency selection is deferred and would need its own future decision if adopted.
- **Extend target diagnostics schema vs narrow strict portal patch**: choose narrow patch preserving existing report vocabulary. Durable ADR rejected because it intentionally avoids a durable public API/schema change.
- **Permanent target checkout mutation vs reversible smoke**: choose status/apply/test/uninstall and final clean target. Durable ADR rejected because the permanent rule already follows project constitution/update-safety boundaries.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. This change implements an already-recognized source-overlay integration path and does not alter the durable architecture snapshot. `ARCHITECTURE.md` already names source overlays as the future integration boundary and ADR 0008 already covers root-coordinate semantics.

## No ADR Needed

No durable ADR is needed because the decisions are reversible, local to installer/status/uninstall mechanics, and bounded by existing architecture rules. A future change should create or supersede a durable ADR if it introduces a permanent target branch/fork, a new public target API/schema, a native X11 dependency, or mandatory source-overlay installation as the only supported delivery model.
