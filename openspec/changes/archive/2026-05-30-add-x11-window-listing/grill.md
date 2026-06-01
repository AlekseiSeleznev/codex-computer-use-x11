## Context Read

- `openspec/changes/add-x11-window-listing/proposal.md`
- `openspec/changes/add-x11-window-listing/specs/x11-window-listing/spec.md`
- Root project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`
- Current canonical specs: `openspec/specs/doctor-cli/spec.md`, `openspec/specs/project-bootstrap/spec.md`, `openspec/specs/x11-integration-contract/spec.md`
- Backlog context: `backlog/00-research-reuse-map.md`, `backlog/03-ewmh-window-listing.md`
- Standalone code: `Cargo.toml`, `README.md`, `src/lib.rs`, `src/main.rs`, `src/doctor.rs`, `src/x11_id.rs`
- Target repo read-only context at `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: `computer-use-linux/src/windowing/types.rs`, `registry.rs`, `target.rs`, `backends/i3.rs`, `backends/kwin.rs`, `backends/hyprland.rs`, `server.rs`, `diagnostics.rs`, `atspi_tree.rs`, `screenshot.rs`
- Local safe probes: tool availability for `wmctrl`, `xprop`, `xdotool`, `ydotool`; sanitized `wmctrl -lpGx` row shape and active-window/window-type facts without recording window titles
- External research refreshed for ideas/licensing: `tak-uukti/linux-computer-use`, `joe223/sootie`, `wimi321/linux-computer-use-skill`, `BeckhamLabsLLC/linux-desktop-mcp`, and freedesktop EWMH/wmctrl references

## Plan Summary

- Add the first real window backend behavior to the standalone CLI: `codex-computer-use-x11 list-windows --json`.
- Use `wmctrl -lpGx` as the MVP X11/EWMH listing source because it is available locally, simple to fixture, and aligned with prior research.
- Keep primary windows compatible with the target repo `WindowInfo` shape and move raw/provenance/reliability fields to sidecar diagnostics.
- Use `_NET_ACTIVE_WINDOW` for focused-window marking when available; keep `_NET_WM_WINDOW_TYPE` / hidden-state enrichment bounded or optional.
- Preserve TDD discipline: fixture parser tests and CLI fake-command tests before live Cinnamon/X11 smoke.

## Question Loop

### Question 1: Should this change introduce native `x11rb` listing now instead of shelling out to `wmctrl`?

- **Recommended answer:** No. Use `wmctrl -lpGx` for this MVP and leave native `x11rb` as a later fallback only if design evidence shows shell-out is insufficient.
- **Rationale:** The constitution favors root Rust/Cargo simplicity unless evidence justifies more complexity; backlog stage 03 explicitly frames `wmctrl -lpGx` as the fastest stable MVP; local probes show `wmctrl` is installed and returns rows; target repo source-overlay style already tolerates thin command wrappers plus pure parser tests.
- **Resolution:** Resolved from repository/backlog/target context. No user question required.

### Question 2: Should `pid_reliable`, raw X11 ids, source command, and warnings be added directly to primary window objects?

- **Recommended answer:** No. Keep primary objects compatible with upstream `WindowInfo`; put X11-only metadata in `diagnostics` / sidecar report fields.
- **Rationale:** `openspec/specs/x11-integration-contract/spec.md` already requires upstream `WindowInfo` to remain the primary model and X11-only provenance to live in sidecar/report fields by default. Target `WindowInfo` currently has no `pid_reliable`, `raw_id`, `source`, or `warnings` fields.
- **Resolution:** Resolved by existing canonical spec and target code. Specs already require this.

### Question 3: Should the command perform `_NET_WM_WINDOW_TYPE` and `_NET_WM_STATE_HIDDEN` lookup for every window on every call?

- **Recommended answer:** Not unconditionally. Use a bounded, lazy, cached, or explicit optional enrichment strategy; MVP may ship with conservative unknown/default values plus diagnostics if full EWMH state is not fetched.
- **Rationale:** `wmctrl -lpGx` does not include reliable window type/hidden state. Per-window `xprop -id` for every row is an N+1 process-spawning risk. The spec now requires bounded/optional lookup and explicit uncertainty rather than fabricated certainty.
- **Resolution:** Resolved from backlog risk analysis and target backend patterns. No user question required.

### Question 4: Should `list-windows --json` fail non-zero when `DISPLAY` or `wmctrl` is missing?

- **Recommended answer:** No, not when a degraded JSON report can be produced. Reserve non-zero exits for unsupported CLI usage or failures that prevent JSON serialization.
- **Rationale:** This matches the existing `doctor --json` design, supports headless CI/fake-command testing, and gives automation a stable JSON diagnostic surface.
- **Resolution:** Resolved by `doctor-cli` precedent and proposal scope. Specs already require degraded JSON.

### Question 5: Does this stage need a durable top-level ADR?

- **Recommended answer:** No. A change-local ADR review is mandatory, but a durable ADR is not needed unless design discovers a hard-to-reverse architecture decision beyond the existing accepted contract.
- **Rationale:** The current choice (`wmctrl` MVP, `WindowInfo` primary, sidecar diagnostics, no native X11 dependency yet) is expected backlog sequencing and can be reversed or upgraded later. The hard architectural decisions are already captured in the canonical integration contract and architecture snapshot.
- **Resolution:** Resolved for pre-design. ADR review should record “no durable ADR” unless design-review finds a new hard-to-reverse decision.

## Resolved Terms

- `x11-ewmh` remains the canonical backend id; it is not renamed to `x11` or `cinnamon`.
- `WindowInfo-compatible primary object` means the standalone JSON primary window object follows the target repo `WindowInfo` field set and semantics.
- `sidecar diagnostics` means report-level metadata for X11-only provenance, reliability, degraded reasons, and raw command facts that must not expand upstream `WindowInfo` by default.
- No `CONTEXT.md` update was required: existing glossary already defines `x11-ewmh`, standalone plugin, and source overlay; the sidecar-diagnostics wording is artifact-local contract language rather than a new domain term.

## Document Updates Applied

- None after creating `proposal.md` and `specs/x11-window-listing/spec.md`; the current specs already encode the grill resolutions above.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No new durable ADR candidate at this gate.
- Change-local `adr.md` must still document that in-force architecture/context were considered and that no durable ADR is required unless design-review changes the decision profile.

## Open Questions

None.
