## Context Read

- `openspec/changes/add-x11-window-listing/proposal.md`
- `openspec/changes/add-x11-window-listing/specs/x11-window-listing/spec.md`
- `openspec/changes/add-x11-window-listing/grill.md`
- `openspec/changes/add-x11-window-listing/design.md`
- Root project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`
- Current canonical specs: `openspec/specs/doctor-cli/spec.md`, `openspec/specs/project-bootstrap/spec.md`, `openspec/specs/x11-integration-contract/spec.md`
- Standalone code and tests context in `Cargo.toml`, `Makefile`, `README.md`, `src/lib.rs`, `src/main.rs`, `src/doctor.rs`, and `src/x11_id.rs`
- Target repo read-only source context: `computer-use-linux/src/windowing/types.rs`, `registry.rs`, `target.rs`, `server.rs`, `diagnostics.rs`, and existing backend parsers for i3/KWin/Hyprland
- Local safe X11 row-shape probe and active-window probe without recording live window titles

## Design Summary

- The design adds a standalone `list-windows --json` command backed by a new X11 listing module.
- `wmctrl -lpGx` is the MVP listing source, parsed through pure fixtures and a command seam.
- `_NET_ACTIVE_WINDOW` from `xprop -root` marks focus when available.
- Primary `windows[]` objects stay compatible with target `WindowInfo`; raw X11/provenance/reliability facts live in diagnostics.
- Per-window type/hidden enrichment is not unbounded in the MVP; uncertainty is explicit.

## Question Loop

### Question 1: Is the designed `WM_CLASS` mapping deterministic enough for implementation?

- **Recommended answer:** It needed one clarification: no-dot class values must have a deterministic mapping.
- **Rationale:** The original design text said to map no-dot class values to both `wm_class` and `app_id` “when clearer,” which could lead to inconsistent implementations and tests.
- **Resolution:** Updated `design.md` before this review: no-dot class values map to `wm_class`, and `app_id` falls back to the same value only when no better app identifier is available, with diagnostics recording the fallback. No user question required.

### Question 2: Does the command-runner seam conflict with the source-overlay command-style rule?

- **Recommended answer:** No, because this change is standalone-only. The source-overlay contract still says a future target repo overlay should prefer thin `Command::new(...)` wrappers plus pure parser tests unless a later design/ADR accepts an exception.
- **Rationale:** `openspec/specs/x11-integration-contract/spec.md` distinguishes standalone test seams from source-overlay command style. The standalone crate already uses testable probes in `doctor.rs`; a seam here supports TDD without imposing architecture on the target checkout.
- **Resolution:** Resolved by existing canonical spec and design wording.

### Question 3: Is `hidden: false` acceptable when `_NET_WM_STATE_HIDDEN` is unknown?

- **Recommended answer:** Yes only if diagnostics explicitly say hidden state is unknown or not enriched; future implementations may add bounded enrichment.
- **Rationale:** The target `WindowInfo` field is boolean, but `wmctrl` alone does not prove hidden state. The spec and design both require conservative defaults plus diagnostics rather than fabricated certainty.
- **Resolution:** Resolved by spec/design; implementation tasks must include a test or diagnostic check for unknown enrichment.

### Question 4: Should live Cinnamon/X11 smoke output be copied into artifacts?

- **Recommended answer:** No. Record command success and high-level counts only; avoid live titles or other user-sensitive local window contents.
- **Rationale:** The constitution prohibits leaking secrets, and local window titles can contain private data even when not formal secrets.
- **Resolution:** Resolved as a verification/task constraint.

## Design Findings

- **Finding 1 — fixed:** no-dot `WM_CLASS` mapping was ambiguous. `design.md` was updated and checkpointed before this review consumed it.
- **Finding 2 — handled:** `wmctrl -lpGx` title parsing must use a max-split strategy that preserves the title remainder. This is present in design and must be represented in tests.
- **Finding 3 — handled:** the design must not create source-overlay pressure. It explicitly limits implementation to the standalone crate and preserves the target repo command-style rule.
- **Finding 4 — handled:** unbounded per-window `xprop -id` is a performance risk. The design prefers no enrichment in MVP and requires bounded/optional enrichment if added.
- **Finding 5 — handled:** live smoke evidence can be useful but must not leak window titles. This is captured as a task/test-plan constraint.

## Document Updates Applied

- Updated `openspec/changes/add-x11-window-listing/design.md` to clarify deterministic no-dot `WM_CLASS` mapping.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No durable ADR candidate. The design follows existing project architecture and canonical specs rather than introducing a hard-to-reverse, surprising, or cross-project architecture decision.

## Open Questions

None.
