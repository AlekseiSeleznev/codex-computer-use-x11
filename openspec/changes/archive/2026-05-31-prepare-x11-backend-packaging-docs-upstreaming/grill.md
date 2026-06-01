## Context Read

- OpenSpec artifacts: `openspec/changes/prepare-x11-backend-packaging-docs-upstreaming/proposal.md`; `openspec/changes/prepare-x11-backend-packaging-docs-upstreaming/specs/x11-packaging-docs-upstreaming/spec.md`.
- Project rules/context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`.
- Backlog and research baseline: `backlog/00-research-reuse-map.md`, `backlog/12-packaging-docs-upstreaming.md`.
- Project docs/scripts: `README.md`, `docs/integration-contract.md`, `docs/e2e-harness.md`, `docs/intent-driven-lifecycle.md`, `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/status-codex-source-overlay.sh`, `scripts/install-codex-source-overlay.sh`, `scripts/uninstall-codex-source-overlay.sh`, `scripts/e2e/codex-plugin-smoke.sh`, `scripts/e2e/codex-source-overlay-smoke.sh`, `scripts/e2e/codex-x11-e2e.py`.
- Target checkout: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` on clean `main`; reviewed target `README.md`, `CHANGELOG.md`, `AGENTS.md`, `linux-features/README.md`, and `computer-use-linux/src/windowing/{types.rs,registry.rs,target.rs}`, `server.rs`, `diagnostics.rs`, `atspi_tree.rs`, `screenshot.rs`.
- External refresh: GitHub license API/repo metadata for the reference projects and runtime command repos listed in the proposal research refresh; web search results for GitHub MCP server documentation and current license/package metadata for `wmctrl`, `xdotool`, `ydotool`, and `x11rb`.

## Plan Summary

- The change is documentation and documentation-check focused: it prepares README/deep docs/release guidance, not new native packaging formats or new runtime X11 behavior.
- Docs must preserve the two delivery paths: standalone user-local Codex MCP plugin and reversible source overlay against the Codex Desktop Linux target checkout.
- The license posture is reference-first: external projects may be ideas/references unless copying has compatible-license review and attribution/NOTICE handling.
- Upstreaming guidance must separate backend/windowing contributions from Codex Desktop wrapper/packaging integration.
- Verification must use public interfaces: docs checks, script existence/help/dry-run checks, fake e2e evidence, optional live evidence, OpenSpec validation, and project `make` checks.

## Question Loop

### Q1 — Is this stage allowed to add native package artifacts?

- **Recommended answer:** No. Keep this stage to documentation, docs-check tests, and release/upstreaming guidance; defer `.deb`/`.rpm`/AppImage or target packaging changes to a separate design.
- **Rationale:** The backlog goal says "packaging docs" and upstream-ready integration, while current supported paths are standalone plugin and source overlay. Introducing native package artifacts would cross a larger boundary and conflict with the existing source-overlay contract.
- **Resolution:** Answered from backlog and current architecture. No user question needed. The proposal/spec already mark native packaging as rejected/deferred.

### Q2 — Which upstream target owns future backend code versus wrapper integration?

- **Recommended answer:** Backend/windowing/diagnostics/input-safety work should target the Computer Use Linux backend lineage (`agent-sh/computer-use-linux` and the target's `computer-use-linux/` subtree); Codex Desktop packaging, Linux feature toggles, launcher/update-manager wiring, and bundled plugin staging belong to `codex-desktop-linux-full`.
- **Rationale:** The target repo's `AGENTS.md` and README keep Computer Use backend code under `computer-use-linux/`, while packaging/update-manager/feature toggles are wrapper concerns. Mixing them would make review and rollback harder.
- **Resolution:** Answered from target repo docs/source. The spec requires an upstream target matrix with this split.

### Q3 — How should license/attribution docs handle runtime commands?

- **Recommended answer:** Treat invocation of installed runtime commands separately from source copying/vendoring. Document `wmctrl`, `xdotool`, and `ydotool` as runtime command dependencies when invoked, and mark source copying/vendoring according to SPDX/license status.
- **Rationale:** The project invokes tools like `wmctrl`/`xdotool` in standalone flows but has not copied their source. GPL/AGPL risk is materially different for vendoring/adapting code than for invoking a user's installed command.
- **Resolution:** Answered from proposal research refresh and existing integration contract. Added glossary term `Runtime command dependency` to `CONTEXT.md`.

### Q4 — How strict should docs checks be?

- **Recommended answer:** Check public, stable facts: required headings/sections, existence of referenced project scripts, `--help`/`--dry-run` snippets where scripts support them, license table classifications, upstream matrix rows, and source-overlay rollback command names. Do not write brittle prose snapshot tests for every sentence.
- **Rationale:** Public-interface docs checks prevent stale commands while leaving maintainers room to improve prose. This aligns with the TDD skill's behavior-focused testing rule.
- **Resolution:** Answered from project TDD guidance and existing tests. Test-plan should define small RED/GREEN docs-check slices rather than a single broad snapshot.

### Q5 — Should source overlay be presented as a long-lived fork?

- **Recommended answer:** No. Present it as reversible staging/evidence that must install, test, uninstall, and leave the target clean.
- **Rationale:** Existing source-overlay spec, docs, and target-safety rules require owned marker blocks and rollback. A long-lived fork would undermine upstream-ready evidence and make target drift unsafe.
- **Resolution:** Answered from `docs/integration-contract.md`, `openspec/specs/codex-source-overlay-extension/spec.md`, and e2e harness docs.

## Resolved Terms

- `Upstream target matrix` — added to `CONTEXT.md` as the project term for the backend-vs-wrapper handoff map.
- `Runtime command dependency` — added to `CONTEXT.md` to keep invocation distinct from source copying/vendoring.
- `Release checklist` — added to `CONTEXT.md` as the project-owned v1 handoff checklist term.

## Document Updates Applied

- Updated `CONTEXT.md` with glossary entries for `Upstream target matrix`, `Runtime command dependency`, and `Release checklist`.
- No proposal/spec updates were required; the current proposal and spec already reflect the grill resolutions.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. This change applies existing architecture and license/reuse policy to documentation and checks. It does not make a hard-to-reverse architectural decision beyond ADR 0008 or existing source-overlay/plugin contracts.

## Open Questions

None.
