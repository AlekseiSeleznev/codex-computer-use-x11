## ADR Review

## Existing In-Force ADRs

- `adr/0008-adopt-x11-root-coordinate-model.md` — Accepted and in force. This change preserves its X11 root/global coordinate model by using `WindowInfo.bounds` for overlay requests and stale/target context. No supersession needed.
- `ARCHITECTURE.md` and `adr/README.md` also describe earlier in-force project decisions for the Codex/OpenSpec overlay, project context entrypoints, Matt grill/TDD gates, Claude review controls, and automatic checkpoints. Their historical ADR files are not all present in the current `adr/` directory, but the rules they summarize remain relevant project context for this change.

## Constitution / Architecture Rules Considered

- Rust 2021/Cargo remains the implementation stack; root `make fmt`, `make check`, and `make test` are required before completion.
- The local Codex Desktop Linux target checkout is read-only for this change and is referenced only through `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented machine default.
- No secrets or `.secrets.local.env` access are needed; runtime target state must not contain or print credential values.
- OpenSpec remains the source of truth; planning artifacts must be completed and checkpointed before apply.
- Existing targeted-input safety remains in force: saved target-window state is context only and does not replace explicit selectors or focus/bounds verification.
- ADR 0008 root-coordinate semantics remain in force for bounds and optional overlay requests.

## Decisions Evaluated

- **State-first target groups vs mandatory real overlay provider**: choose state-first with a no-overlay production provider and fake-provider tests. Durable ADR rejected because the decision is intentionally reversible and local to this standalone UX layer.
- **File-backed CLI state vs in-memory-only CLI state**: choose file-backed local runtime state with `CODEX_X11_TARGET_STATE` test override because CLI commands are process-per-command. Durable ADR rejected because this does not set a project-wide storage architecture; it is a small local runtime persistence detail.
- **In-memory MCP state vs shared file-backed MCP state**: choose process-scoped in-memory MCP state to avoid cross-session leakage. Durable ADR rejected because it follows standard server process state and can be changed later if session persistence becomes a requirement.
- **One-owner target group membership vs multi-group membership**: choose one-owner membership to keep `release-window --window-id` deterministic. Durable ADR rejected because the API can be extended later with group-qualified release if multi-group membership becomes necessary.
- **Project-owned overlay filtering**: choose explicit owned/internal metadata and filtering from primary listing targets. Durable ADR rejected because it is a safety rule within the new capability and follows existing non-application-window listing policy.

## New Durable ADRs Created

- None.

## Superseded ADRs

- None.

## Architecture Snapshot Updates

- None. The current architecture snapshot already describes the existing lifecycle, target-checkout boundary, and X11 root-coordinate model. This change adds a standalone UX/context layer but does not change the durable architecture snapshot.

## No ADR Needed

- No durable ADR is needed because all decisions are local, reversible, and bounded to the standalone target-window UX implementation. The surprising or durable constraints are already recorded elsewhere: explicit targeted-input verification, `x11-ewmh` standalone/source-overlay boundary, and ADR 0008 root-coordinate semantics. A future change that makes real overlays mandatory, introduces a long-running overlay daemon, or changes input/app-state commands to implicitly consume saved active targets should create or supersede a durable ADR.
