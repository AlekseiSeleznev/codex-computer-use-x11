## Context Read

- `openspec/changes/add-codex-x11-e2e-test-harness/proposal.md`, `specs/codex-x11-e2e-test-harness/spec.md`, `grill.md`, and `design.md`.
- `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and `adr/0008-adopt-x11-root-coordinate-model.md`.
- `docs/integration-contract.md`, especially source-overlay fallback, stock `get_app_state`, and target-window guidance.
- Existing implementation/test surfaces: `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/status-codex-source-overlay.sh`, `scripts/install-codex-source-overlay.sh`, `scripts/uninstall-codex-source-overlay.sh`, `tests/plugin_installer.rs`, `tests/source_overlay_scripts.rs`, `tests/mcp_server.rs`, `src/mcp.rs`, `src/doctor.rs`, `src/list_windows.rs`, `src/focus.rs`, `src/input.rs`, `src/pointer.rs`, `src/app_state.rs`, and `src/accessibility.rs`.
- Target checkout files: current `computer-use-linux/src/server.rs`, `diagnostics.rs`, `screenshot.rs`, `atspi_tree.rs`, `remote_desktop.rs`, `abs_pointer.rs`, `windowing/registry.rs`, and `scripts/ci-local.sh`.

## Design Summary

- The design adds a stdlib Python runner under `scripts/e2e/` with two public shell wrappers for standalone plugin and source-overlay smoke.
- Fake mode creates isolated `CODEX_HOME` or target fixtures by default, injects fake X11 commands, and validates MCP stdio/tool routing without a GUI.
- Live mode is additive, preserving user-local and target safety: no sudo, no `/opt/codex-desktop`, source-overlay uninstall in `finally`, and final clean target status.
- Evidence is written as logs plus JSON capability-matrix records under `target/e2e-logs/`; missing matrix entries fail, explicit degraded reasons are accepted.
- Source-overlay evidence maps current stock target focus to `activate_window` and does not require absent stock `focus_window`, `mousemove`, or target `x11_get_app_state`.

## Question Loop

### Q1: Does fake plugin auto-install hide the required missing-plugin failure path?

Recommended answer: No, because auto-install only happens when fake mode is run without `--codex-home`; when `--codex-home` is supplied, the script validates exactly that directory and fails if the plugin is absent.

Rationale: This preserves a convenient no-argument CI smoke while allowing a deterministic negative test for missing plugin metadata.

Resolution: Answered from design. Tests must cover both supplied-missing `--codex-home` and no-argument fake auto-install.

### Q2: Could fake input routes accidentally touch the real desktop?

Recommended answer: Keep fake command directory first on `PATH`, set fake `DISPLAY`, and assert the fake `xdotool` log receives the calls; never call global pointer/input against real commands in fake mode.

Rationale: Existing standalone input/pointer code shells out to `xdotool` after focus verification. A fake `PATH` fixture is the project-established way to test shell-out behavior without desktop mutation.

Resolution: Answered from code patterns and design. The test plan must include a fake `xdotool` log assertion for keyboard and pointer smoke.

### Q3: Does the source-overlay fake target need to compile?

Recommended answer: No for fake mode; it only needs enough anchor structure for source-overlay status/install/uninstall. Live mode remains responsible for target cargo tests against the real checkout.

Rationale: Existing source-overlay tests already separate fake anchor/marker behavior from real-target compile smoke. Requiring a compilable fake target would duplicate target fixtures and slow CI without increasing confidence in current target compatibility.

Resolution: Answered from existing `tests/source_overlay_scripts.rs` and source-overlay archive evidence.

### Q4: Is a durable ADR needed for the harness boundary?

Recommended answer: No. The harness applies existing architectural decisions rather than changing durable architecture: standalone namespaced tools, stock target names, source-overlay reversibility, and X11 root coordinates remain unchanged.

Rationale: The decision is reversible and local to verification mechanics; future maintainers can understand it from design/test-plan/docs without a durable ADR.

Resolution: Record no durable ADR in `adr.md` unless implementation uncovers an architecture change.

## Design Findings

- **Verification feasibility is good.** Existing Rust integration tests already spawn scripts and MCP stdio processes, so new tests can stay public-interface oriented.
- **Failure-path logging is a first-class behavior.** The implementation must write evidence in `finally`; otherwise the missing-plugin and MCP-startup negative scenarios will be unhelpful.
- **Matrix validation should be isolated and directly testable.** A `validate-matrix --evidence <file>` runner subcommand prevents matrix completeness from being tested only indirectly through large smoke runs.
- **Source-overlay live mode must be defensive.** If install succeeds and target cargo tests fail, uninstall/final clean checks still run before the script exits.
- **No glossary conflict found.** Added glossary terms are behavior-boundary terms, not implementation details.
- **No constitution/ADR conflict found.** The design avoids secrets, avoids direct installed-app mutation, keeps source-overlay reversibility, and preserves ADR 0008 coordinate semantics.

## Document Updates Applied

None. Proposal, specs, grill, and design already encode the review findings.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No durable ADR candidate. The e2e harness is a verification mechanism around already accepted delivery paths and can be changed or removed without superseding a project-level architectural decision.

## Open Questions

None.
