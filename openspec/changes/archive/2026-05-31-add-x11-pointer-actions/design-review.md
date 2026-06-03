## Context Read

- `proposal.md`, `specs/x11-pointer-actions/spec.md`, `specs/standalone-codex-mcp-plugin/spec.md`, `grill.md`, and `design.md` for this change.
- Root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/README.md`; referenced durable ADR bodies are absent in this checkout, so the architecture snapshot and README are the available in-force decision context.
- Existing standalone code: `src/cli.rs`, `src/input.rs`, `src/focus.rs`, `src/list_windows.rs`, `src/mcp.rs`, `src/doctor.rs`, and current integration tests.
- Target checkout code: `computer-use-linux/src/server.rs` pointer tool implementations, `abs_pointer.rs`, `remote_desktop.rs`, `windowing/target.rs`, `diagnostics.rs`, `atspi_tree.rs`, and `screenshot.rs`.
- Local tool behavior: `xdotool --version` is available and the local session has X11/Cinnamon, but live pointer smoke must use a disposable test window or be recorded as degraded.

## Design Summary

- The design adds a new standalone `src/pointer.rs` module and reuses `WindowTarget`/target resolution rather than duplicating selector semantics.
- Targeted pointer actions validate target uniqueness, bounds, points, drag distance, focus verification, and backend availability before invoking `xdotool`.
- Explicit global mode skips target/focus only when marked `global_unverified` and keeps finite coordinate/amount/distance constraints.
- MCP tool order expands from six to nine project-owned tools and continues to avoid unprefixed stock names.
- Source-overlay pointer backend selection remains deferred; this change is standalone only.

## Question Loop

### Q1: Can the design safely reuse `WindowTarget` from `src/input.rs` without confusing keyboard and pointer domains?

- Recommended answer: yes, if target resolution helpers are exposed as generic selector utilities while report/action types remain separate.
- Rationale: target selectors are already part of the public CLI/MCP vocabulary and apply equally to keyboard and pointer targeting. Keeping pointer reports separate avoids changing the keyboard contract.
- Resolution: no user question required. Implementation should expose `ResolveTargetError`/`resolve_target` carefully and leave keyboard report fields unchanged.

### Q2: Does global pointer mode weaken the existing targeted-input safety invariant?

- Recommended answer: no, if global mode is opt-in and never reported as targeted input.
- Rationale: the existing invariant is for targeted input: verify before injecting into a specific window. Backlog/07b separately asks for global/unverified reporting. The design preserves targeted safety and creates an explicit diagnostic mode for no-target actions.
- Resolution: no user question required. Tests must prove missing target without `--global` is refused.

### Q3: Are the finite limits sufficient and testable?

- Recommended answer: yes for this stage.
- Rationale: fixed constants for click count, scroll amount, and drag delta are deterministic, visible in tests, and reversible later. They prevent infinite or accidentally massive pointer sequences without requiring new configuration.
- Resolution: implementation and test-plan should record exact constants and edge behavior.

### Q4: Does the design need to update `CONTEXT.md` or `ARCHITECTURE.md`?

- Recommended answer: no.
- Rationale: the pointer terms are capability/report terms, not new project-wide glossary or architecture decisions. The architecture snapshot already captures verify-before-inject, standalone plugin, and source-overlay boundaries.
- Resolution: no document update required.

### Q5: Is a durable ADR required because global mode allows pointer side effects?

- Recommended answer: no durable ADR.
- Rationale: global/unverified pointer action is explicit, reversible, and confined to the standalone plugin. It applies the existing global-injector honesty rule rather than changing project architecture.
- Resolution: per-change `adr.md` will record no new durable ADR.

## Design Findings

- **No material conflict with constitution or architecture.** Required Rust/OpenSpec/secret/verification constraints are preserved.
- **No target checkout mutation.** The target repo's `abs_pointer`/portal/ydotool ordering is researched but not copied or patched.
- **Report shape must be automation-friendly.** Implementation should expose stable `success`, `input_sent`, `targeted`, `verification_mode`, `error_code`, `target`, `focus`, `pointer`, and diagnostics fields.
- **Bounds validation must precede focus.** This prevents focus side effects for requests that are known unsafe by geometry alone.
- **MCP runtime validation remains necessary.** JSON schemas can describe fields but the code must enforce target-or-global, button/direction enums, integer ranges, and safety gates.
- **Live smoke must be conservative.** A disposable X11 test window is preferred; if unavailable, fake-command evidence plus documented live limitation is acceptable.

## Document Updates Applied

- None. The proposal, specs, grill, and design already reflect the design-review findings.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- None. No hard-to-reverse, surprising, project-wide architecture decision is introduced.

## Open Questions

None
