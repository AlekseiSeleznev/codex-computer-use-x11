## Context Read

- `CONSTITUTION.md` — required technologies, verification rules, source-overlay target path, secret-handling, and no direct `/opt/codex-desktop` mutation requirements.
- `CONTEXT.md` — glossary for `x11-ewmh`, source overlay, app state, layer-degraded app state, target window, overlay drift, X11 root coordinates, and newly resolved `E2E harness` / `Capability matrix evidence` terms.
- `ARCHITECTURE.md` and `adr/README.md` — intent-driven lifecycle, OpenSpec artifact gates, Claude review controls, source-overlay boundaries, and in-force ADR expectations.
- `adr/0008-adopt-x11-root-coordinate-model.md` — root/global coordinates and source-overlay screenshot-provider boundary.
- `backlog/00-research-reuse-map.md` and `backlog/11-codex-e2e-test-harness.md` — v1 scope, no-copy license posture, and stage-specific acceptance constraints.
- `openspec/changes/add-codex-x11-e2e-test-harness/proposal.md` and `specs/codex-x11-e2e-test-harness/spec.md`.
- Existing code/tests/docs: `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/codex-source-overlay.py`, `scripts/status-codex-source-overlay.sh`, `tests/mcp_server.rs`, `tests/plugin_installer.rs`, `tests/source_overlay_scripts.rs`, `docs/integration-contract.md`, `src/mcp.rs`, `src/doctor.rs`, `src/list_windows.rs`, `src/focus.rs`, `src/input.rs`, `src/pointer.rs`, `src/app_state.rs`, and `src/accessibility.rs`.
- Target checkout research: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` at `1a6f343...`, current `computer-use-linux/src/server.rs`, `diagnostics.rs`, `atspi_tree.rs`, `screenshot.rs`, `remote_desktop.rs`, `abs_pointer.rs`, and `windowing/**`.
- External references: `tak-uukti/linux-computer-use`, `BeckhamLabsLLC/linux-desktop-mcp`, `iFurySt/open-codex-computer-use`, plus broader current prior art listed in the proposal research refresh. No external code is copied.

## Plan Summary

- Add two public smoke entrypoints under `scripts/e2e/`: one for standalone plugin metadata/MCP stdio and one for source-overlay status/apply/uninstall/stock-target coverage.
- Fake mode is the deterministic archive gate: it uses isolated temp fixtures, fake `CODEX_HOME`, fake X11 commands, no GUI, no sudo, and logs under `target/e2e-logs/`.
- Live mode is additive evidence for the current Cinnamon/X11 machine and real target checkout; source-overlay live mode must uninstall and leave the target clean even on failure.
- Capability-matrix evidence is the common output: every v1 group must be `pass` or `degraded` with a reason for standalone and source-overlay paths; missing evidence fails.
- The harness validates current target stock vocabulary (`activate_window`, `get_app_state`, click/scroll/drag/type_text/press_key) and must not require absent stock `focus_window`, `mousemove`, or competing stock `x11_get_app_state`.

## Question Loop

### Q1: Should automated e2e primarily drive the Codex Desktop UI or the installed plugin MCP stdio boundary?

Recommended answer: Use installed plugin metadata plus MCP stdio as the primary automated boundary; document manual Codex Desktop UI steps only when no stable stock runner exists.

Rationale: Existing tests already exercise the standalone MCP server over stdio, and current external references such as `open-codex-computer-use` expose direct call-style smoke as a stable pattern. Driving the Desktop UI would be flaky, environment-specific, and less suitable for no-GUI fake mode.

Resolution: Answered from repository context and research. Specs already require MCP stdio startup from `.mcp.json`, fake-mode no-GUI checks, and manual documentation for missing direct stock runner.

### Q2: Is degraded screenshot/AT-SPI evidence acceptable in fake mode?

Recommended answer: Yes, but only when the evidence is explicit, attributed to the tool/check that produced it, and does not hide missing matrix entries.

Rationale: The project already defines layer-degraded app state: one layer can fail while window context or diagnostics remain useful. Fake mode cannot depend on a live screenshot provider or AT-SPI bus, so requiring `pass` for those layers would make CI non-deterministic.

Resolution: Specs require each capability group to be `pass` or `degraded` with reason and fail on missing evidence.

### Q3: Should source-overlay smoke require target stock `focus_window` or `mousemove` tools?

Recommended answer: No. The current target uses stock `activate_window` for focus and stock `click`/`scroll`/`drag` for pointer input; absence of stock `focus_window` or `mousemove` must be recorded as a non-blocking fact.

Rationale: Fresh target research found `activate_window`, `get_app_state`, `click`, `scroll`, `drag`, `press_key`, and `type_text`, but not stock `focus_window` or `mousemove`. Requiring absent names would contradict the actual target API and backlog guidance.

Resolution: Specs explicitly map focus to `activate_window` and reject failing solely for absent `focus_window`/`mousemove`.

### Q4: May live source-overlay smoke temporarily mutate the real target checkout?

Recommended answer: Yes, only during a reversible status/install/check/uninstall interval, with a clean starting target and final clean target status.

Rationale: Existing source-overlay architecture and accepted specs require real target smoke for compatibility, but the constitution and integration contract prohibit permanent target forks or unowned direct app mutations.

Resolution: Specs require no `/opt/codex-desktop` mutation, no sudo, uninstall on failure, and final clean target status.

## Resolved Terms

- `E2E harness` — added to `CONTEXT.md` as the repeatable fake/live evidence boundary for Codex-facing installation or target-source delivery paths.
- `Capability matrix evidence` — added to `CONTEXT.md` as pass/degraded per-capability evidence where missing entries fail but explicit degraded reasons are allowed.

## Document Updates Applied

- Updated `CONTEXT.md` with `E2E harness` and `Capability matrix evidence` glossary entries.
- Proposal already includes research refresh and target-tool vocabulary findings.
- Specs already require fake/live modes, capability-matrix missing-evidence failure, source-overlay reversibility, log retention, no `/opt` mutation, and stock target vocabulary.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No durable ADR candidate is required at this gate. The change adds a verification harness around existing delivery paths rather than changing backend identity, coordinate model, source-overlay ownership, stock-vs-standalone tool naming, or project architecture.

## Open Questions

None.
