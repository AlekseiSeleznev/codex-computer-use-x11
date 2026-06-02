## Context Read

- Change artifacts: `openspec/changes/add-x11-target-window-groups-overlays/proposal.md` and all delta specs under `openspec/changes/add-x11-target-window-groups-overlays/specs/`.
- Project rules/context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`, `backlog/00-research-reuse-map.md`, and `backlog/10-window-targeting-groups-overlays.md`.
- Current project code/docs: `src/cli.rs`, `src/mcp.rs`, `src/list_windows.rs`, `src/input.rs`, `src/pointer.rs`, `src/app_state.rs`, `src/coordinates.rs`, `README.md`, `docs/integration-contract.md`, and existing tests.
- Target checkout read-only context: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/computer-use-linux/src/server.rs`, `windowing/types.rs`, `windowing/target.rs`, `windowing/registry.rs`, `screenshot.rs`, `remote_desktop.rs`, and `diagnostics.rs` at commit `1a6f343ee7ce597019a4c573259c2a9838376874`.
- External/standards context: `BeckhamLabsLLC/linux-desktop-mcp` at `eaf67ca`, freedesktop EWMH specification, X11 SHAPE/input-shape references, GTK/GDK input-shape documentation, and live Cinnamon/X11 probes recorded in `proposal.md`.

## Plan Summary

- Add a target-window/group layer as session context over existing X11/EWMH `WindowInfo` and `WindowTarget` semantics.
- Keep target resolution strict: ambiguous, missing, or stale selectors do not become active saved targets.
- Keep overlays optional: target save/release correctness does not depend on drawing a border; unsupported or failed overlays are reported as warnings.
- Exclude or mark project-owned overlay/helper windows so the listing, target manager, and input paths never treat them as normal application windows.
- Keep this stage standalone and target-checkout read-only; future source-overlay work should adapt through existing stock target-resolution concepts.

## Question Loop

### Q1: Should the first standalone implementation make real visual overlays mandatory now, since the local Cinnamon/X11 session has GTK3 and Cairo available?

Recommended answer: No. Implement the overlay as a provider boundary with a no-overlay/unsupported production provider and testable fake provider first; treat successful real border drawing as optional live-smoke evidence only if a safe provider exists.

Rationale: Backlog 10 explicitly says target/window groups are state-first and overlay is optional. Real X11 overlay windows are risky because focus/input pass-through depends on EWMH/window manager behavior and input-shape support. A mandatory GUI dependency would also make headless/fake-command tests brittle. The specs already require overlay failure to be a warning and target save to remain successful.

Resolution: Use state-first design with an overlay provider seam. No user question needed because repository context and backlog answer it.

### Q2: Should CLI target state be globally persistent by default or test/session scoped?

Recommended answer: Use explicit local file-backed state for one-shot CLI commands, defaulting to an ignored session/cache path and overridable by an environment variable for tests; keep MCP state in-memory per stdio server process.

Rationale: The CLI is process-per-command, so save/release/context needs persistence to be observable across invocations. The MCP server is long-lived and should not leak state across processes. `CONSTITUTION.md` allows local non-secret session state and forbids secrets; target ids/titles are local UI context, not credentials, but state must still be validated against a fresh listing to avoid stale reuse.

Resolution: Design must specify file-backed CLI state plus in-memory MCP state, both validated before reporting targets as current.

### Q3: Should a saved target become an implicit selector for existing input or app-state commands?

Recommended answer: Not in this change. Existing input/app-state commands should continue requiring explicit selectors; target-context provides discoverability and future integration can decide whether a saved active target becomes a default selector.

Rationale: Existing targeted input safety is based on explicit target selectors and focus/bounds verification. Automatically reusing an active target would be a behavior and safety change across input paths and could surprise users. Backlog 10 asks for target-window/group UX and stale detection, not implicit defaults.

Resolution: Keep target-window state as explicit context only. Do not change `type-text`, `press-key`, `click`, `scroll`, `drag`, or `get-app-state` to read the saved active target by default.

## Resolved Terms

Updated `CONTEXT.md` with these glossary terms:

- `Target window` — application window explicitly selected for the current automation task; session context, not proof of presence/focus.
- `Window group` — named collection of target windows with one active target at a time.
- `Overlay window` — project-owned visual indicator that is not an application target.
- `Stale target` — saved target whose underlying desktop window can no longer be found.

## Document Updates Applied

- Updated `CONTEXT.md` with the resolved glossary terms above.
- No proposal/spec behavior changes were needed: the existing specs already encode state-first target groups, optional overlay warnings, stale detection, and owned-overlay listing safety.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No new durable ADR is required for this change. The state-first target group layer is useful UX but reversible, and the hard architecture decisions it depends on are already covered by existing `x11-ewmh` target-resolution, input-safety, app-state, and ADR 0008 coordinate-model decisions. If a later change makes real overlay rendering a default dependency or makes saved targets implicit input defaults, that later change should reconsider a durable ADR.

## Open Questions

None.
