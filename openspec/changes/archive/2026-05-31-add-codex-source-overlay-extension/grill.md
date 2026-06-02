## Context Read

- Change artifacts: `openspec/changes/add-codex-source-overlay-extension/proposal.md` and `openspec/changes/add-codex-source-overlay-extension/specs/codex-source-overlay-extension/spec.md`.
- Project rules/context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`, `backlog/00-research-reuse-map.md`, and `backlog/06-codex-source-overlay-extension.md`.
- Current project code/docs: `src/list_windows.rs`, `src/focus.rs`, `src/doctor.rs`, `src/input.rs`, `src/pointer.rs`, `src/app_state.rs`, `src/mcp.rs`, existing integration tests, plugin install scripts, `README.md`, and `docs/integration-contract.md`.
- Target checkout read-only context: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` at commit `1a6f343ee7ce597019a4c573259c2a9838376874`, especially `computer-use-linux/src/windowing/types.rs`, `registry.rs`, `mod.rs`, `backends/mod.rs`, `backends/i3.rs`, `target.rs`, `server.rs`, `diagnostics.rs`, `screenshot.rs`, `atspi_tree.rs`, and Cargo manifests.
- External/reuse refresh: GitHub CLI metadata and code search for `ilysenko/codex-desktop-linux`, `agent-sh/computer-use-linux`, `tak-uukti/linux-computer-use`, `BeckhamLabsLLC/linux-desktop-mcp`, and `joe223/sootie`.

## Plan Summary

The change will add a project-owned reversible source overlay that can patch the moving Codex Desktop Linux target with a generic `x11-ewmh` backend, marker-block registry/diagnostic integration, status/drift reporting, and uninstall cleanup. The overlay is evaluated by fake-target tests first, then by reversible real-target smoke, and must leave the real target checkout clean after verification.

## Question Loop

### Q1: Should this change permanently modify the real Codex Desktop Linux target checkout?

Recommended answer: No. The overlay should be applied to the real target only during verification smoke and then uninstalled before archive.

Rationale: `CONSTITUTION.md` treats the local integration target as machine-specific and read-only unless a task explicitly targets overlay changes. Backlog 06 explicitly targets reversible overlay scripts, not a long-lived fork. Keeping the real target clean avoids drift against fast-moving upstream and lets this repository remain the source of truth for the overlay engine.

Resolution: Answered from project context. The real target smoke is reversible: status/apply/test/uninstall, then clean target status is required.

### Q2: Should the overlay add public stock tools such as `focus_window` or `mousemove` to the bundled target?

Recommended answer: No. Use the existing target surfaces: `activate_window` for focus and current pointer/keyboard tools for input.

Rationale: Fresh target research shows `server.rs` exposes stock `activate_window` and uses internal ydotool `mousemove` command arguments but does not expose a public `mousemove` tool. Backlog 06 explicitly says not to assume separate `focus_window` and not to require stock `mousemove` without fresh evidence.

Resolution: Answered from target code and backlog. The overlay backend integrates through registry activation and existing stock target/input tools only.

### Q3: Is it safe to overwrite an upstream/native X11 backend if one appears in the target checkout?

Recommended answer: No. The installer must detect unowned native X11 files/registrations and either report compatibility/adaptor mode or refuse clearly; it must not overwrite unowned code.

Rationale: The target repo is moving. A future native backend may appear between runs. Overwriting it would violate update safety and make uninstall dangerous.

Resolution: Captured in spec and design: owned files/marker blocks only, unowned native X11 content is preserved.

### Q4: Where should X11-specific metadata live when patching target `WindowInfo`?

Recommended answer: Keep primary target `WindowInfo` unchanged and compatible; put extra X11 diagnostics in backend-local details, status output, or logs, not in new primary fields.

Rationale: Existing canonical specs and target `WindowInfo` define a compatibility boundary. Adding X11-only fields to target `WindowInfo` would turn a reversible overlay into an upstream API change.

Resolution: Captured in spec. Generated backend maps to existing fields only.

## Resolved Terms and Context Updates

- Added `Overlay drift` to `CONTEXT.md` as the safety status where marker blocks, generated files, target anchors, or baseline metadata no longer match installer expectations.
- Existing glossary term `Source overlay` remains the canonical term for adapting this project into the local Codex Desktop Linux target checkout.

## Document Updates Applied

- Updated `CONTEXT.md` with `Overlay drift`.
- Proposal already records the fresh target repo research, external GitHub refresh, accepted/rejected ideas, and risks.
- Spec already records reversible real-target smoke, native-backend preservation, marker ownership, late fallback registration, status states, and strict portal diagnostics.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No durable ADR is required at the pre-design gate. The durable architecture already recognizes `Source overlay` as the integration path, the X11 root-coordinate model is covered by ADR 0008, and this change's installer/status mechanics are reversible and local to the overlay scripts. Reconsider a durable ADR only if the design introduces a long-lived fork, a new public target API, or an upstream-level dependency/architecture change.

## Open Questions

None.
