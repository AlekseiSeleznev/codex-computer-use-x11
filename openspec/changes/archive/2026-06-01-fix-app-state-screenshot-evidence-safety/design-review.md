## Context Read

- Proposal, delta specs, `grill.md`, and `design.md` for `fix-app-state-screenshot-evidence-safety`.
- Root context and architecture: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`.
- Durable ADRs: `adr/README.md`, ADR 0008, ADR 0009, ADR 0010, plus in-force ADR index.
- Existing implementation: `src/app_state.rs`, `src/cli.rs`, `src/mcp.rs`, `scripts/e2e/codex-x11-e2e.py`, fixture scripts, and relevant tests/docs.
- Retest evidence summaries under `target/e2e-logs/real-live-full-retest/20260601T162050Z/`, inspected only for keys/markers and not printed as screenshot payload.

## Design Summary

- Default app-state screenshot behavior changes from inline `data_url` serialization to path-oriented PNG artifact metadata.
- `--no-screenshot` and layer-degraded app-state semantics remain intact.
- CLI/MCP gain an explicit screenshot output path; optional inline compatibility is allowed only behind explicit unsafe opt-in if retained.
- The existing controlled fixture runner is reused but fixture titles/classes become neutral and evidence records controlled ownership through metadata.
- Docs/tests must enforce no inline screenshot blobs in app-state JSON and evidence logs.

## Question Loop

### Q1: Does writing a generated screenshot file by default conflict with the no-secret/no-uncontrolled-evidence rules?

**Recommended answer:** No, if generated files are stored under a documented evidence directory, JSON references the path/metadata only, and controlled real-live runs target only fixture windows.

**Rationale:** The current behavior already captures pixels and embeds them in JSON; path-oriented storage reduces evidence leakage in machine-readable logs. The screenshot still may contain pixels, so the harness must keep controlled fixture target rules and docs must warn operators.

**Resolution:** Design already requires controlled fixture targeting and generated paths under `target/e2e-logs/app-state/` or equivalent. No design change required.

### Q2: Does neutral fixture naming weaken the controlled-fixture safety proof?

**Recommended answer:** No. Controlled ownership should be proven by run-scoped metadata, PID/title/class/window-id matching, and allowlist selection, not by a `Codex` substring.

**Rationale:** A project-owned substring can collide with filters that exclude Codex/overlay windows. Metadata-driven ownership is more precise and still blocks uncontrolled user windows.

**Resolution:** Design and tasks must update fixture identity tests and docs. No glossary update required.

### Q3: Is a durable ADR needed for safe-by-default app-state screenshot paths?

**Recommended answer:** No durable ADR for this change. ADR 0008 already established path-oriented screenshot evidence and ADR 0009 already requires explicit safe evidence classification. This change repairs an implementation/spec gap within those decisions.

**Rationale:** The decision is important but not a new architecture direction. It does not supersede coordinate model, backend baseline, or provider takeover decisions.

**Resolution:** Record no durable ADR in `adr.md`; no `ARCHITECTURE.md` update required unless implementation changes durable architecture beyond the current design.

## Design Findings

- **Resolved:** Existing `src/app_state.rs` deletes the temporary screenshot after base64 encoding; implementation must stop deleting caller-visible/generated artifacts before JSON consumers can read them.
- **Resolved:** Existing e2e summarizer strips `data_url` when summarizing, but the raw app-state file from the real-live retest still contained the inline blob. Implementation/tests must cover raw CLI/MCP JSON, not only summarized evidence.
- **Resolved:** Existing fixture names include `codex`/`Codex`; implementation must use neutral names and update tests that assert current fixture identity.
- **Resolved:** Screenshot-crop already has path-only semantics from prior changes; this change must avoid modifying screenshot-crop provider behavior except regression tests.
- **No conflict found:** The design preserves Cinnamon/X11 scope, standalone `x11_*` MCP names, ADR 0008 root coordinates, ADR 0009 safety/degraded behavior, ADR 0010 provider identity boundaries, and secret-handling rules.

## Document Updates Applied

- `design.md` already incorporates path-oriented screenshot capture, CLI/MCP compatibility, layer degradation, neutral fixture naming, and docs/test migration plan.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No durable top-level ADR is required. This is a bugfix/hardening change within ADR 0008 and ADR 0009.

## Open Questions

None.
