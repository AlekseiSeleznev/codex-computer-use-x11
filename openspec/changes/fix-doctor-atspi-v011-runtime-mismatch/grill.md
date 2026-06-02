## Context Read

- `AGENTS.md` — required OpenSpec workflow and git/secret safety.
- `CONSTITUTION.md` — Rust/Makefile verification, non-invasive doctor JSON verification, no secrets.
- `CONTEXT.md` — existing terms: AT-SPI window correlation, Accessibility tree, Layer-degraded app state, Controlled fixture.
- `ARCHITECTURE.md`, `adr/README.md`, and `adr/0001` through `adr/0011` — in-force OpenSpec, TDD, X11, AT-SPI degraded, provider, and rollback decisions.
- `openspec/changes/fix-doctor-atspi-v011-runtime-mismatch/proposal.md`.
- `openspec/changes/fix-doctor-atspi-v011-runtime-mismatch/specs/doctor-cli/spec.md`.
- `openspec/changes/fix-doctor-atspi-v011-runtime-mismatch/specs/x11-atspi-window-correlation/spec.md`.
- Canonical specs: `openspec/specs/doctor-cli/spec.md`, `openspec/specs/x11-atspi-window-correlation/spec.md`, `openspec/specs/standalone-codex-mcp-plugin/spec.md`, and `openspec/specs/codex-x11-e2e-test-harness/spec.md`.
- Prior archived fix: `openspec/changes/archive/2026-06-02-fix-live-doctor-atspi-probe-mismatch/`.
- Relevant code/tests/docs grep: `src/doctor.rs`, `src/accessibility.rs`, `tests/doctor_cli.rs`, `tests/accessibility_tree_cli.rs`, `docs/troubleshooting.md`, e2e harness tests, and installer tests.

## Plan Summary

- The current canonical doctor specs and `tests/doctor_cli.rs` still encode a presence-based `NO_AT_BRIDGE=1` branch that can prevent the doctor AT-SPI tree probe from proving success.
- The observed v0.1.1 runtime contradicts the desired readiness semantics: `accessibility-tree` succeeds for the focused Codex window while `doctor` reports `collector_unavailable` or bridge-disabled degradation.
- The change narrows bridge-disabled behavior: `NO_AT_BRIDGE=1` remains a sanitized diagnostic/setup-risk fact, but it no longer short-circuits a valid collector result.
- True degraded states remain: collector unavailable, invalid output, no usable tree/candidates, timeout, or target-specific ambiguity when a target match is required.
- The implementation must use TDD through public CLI behavior and preserve non-invasive doctor operation.

## Question Loop

1. **Should `NO_AT_BRIDGE=1` still force degraded doctor state even if the actual bounded collector returns valid candidates/tree?**
   - **Recommended answer**: No. `NO_AT_BRIDGE=1` should remain a diagnostic fact, but proven collector success should win.
   - **Rationale**: The user-provided v0.1.1 evidence shows the runtime can expose a usable tree despite inherited `NO_AT_BRIDGE=1`; the readiness surface should report actual collector availability rather than an environment-only prediction. ADR 0011 requires recording/neutralizing bridge env for install safety, but it does not require doctor to ignore successful runtime evidence.
   - **Resolution**: Specs already updated so `NO_AT_BRIDGE=1` cannot override successful collector output. Degraded bridge-disabled state applies only when no usable tree/candidates can be obtained.

2. **Should doctor require a target window selector to prove tree availability?**
   - **Recommended answer**: No. Doctor should remain an ambient, bounded, non-invasive readiness probe; live verification may compare it with `accessibility-tree --window-id <focused>`.
   - **Rationale**: The doctor contract is a smoke/readiness report without target selection. Requiring a target would break existing automation and the MCP `x11_doctor` zero-argument tool shape. The collector can prove tree availability with candidate/tree facts; target-specific confidence remains the job of `accessibility-tree`.
   - **Resolution**: Keep doctor targetless. Use candidate/tree availability as the doctor success signal and use live-safe comparison as verification evidence.

3. **Does this change require changing the glossary?**
   - **Recommended answer**: No.
   - **Rationale**: Existing terms already cover the behavior: `AT-SPI window correlation`, `Accessibility tree`, `Layer-degraded app state`, and `Controlled fixture`. The change clarifies precedence between environment facts and collector proof; it does not introduce a new domain term.
   - **Resolution**: No `CONTEXT.md` update.

4. **Does this change require a new durable ADR?**
   - **Recommended answer**: No durable ADR.
   - **Rationale**: This is a corrective bugfix within the existing architecture: doctor remains non-invasive, AT-SPI remains degraded on genuine absence/ambiguity, and environment facts remain sanitized. It is not hard to reverse and does not introduce a new architectural boundary.
   - **Resolution**: Record the no-new-ADR decision in change-local `adr.md` later.

5. **What existing tests/docs are likely to conflict with the new behavior?**
   - **Recommended answer**: Update the public CLI regression that currently asserts doctor must not run the AT-SPI tree probe when `NO_AT_BRIDGE=1`; preserve docs/installer/e2e statements that controlled GTK fixtures should avoid inheriting `NO_AT_BRIDGE`.
   - **Rationale**: `tests/doctor_cli.rs` contains an explicit old assertion that `NO_AT_BRIDGE=1` prevents probing. That is the behavior being changed. Installer and troubleshooting docs still correctly advise removing `NO_AT_BRIDGE` as remediation when tree extraction actually fails.
   - **Resolution**: Design and test plan must target `tests/doctor_cli.rs` first and avoid unnecessary docs churn unless behavior text becomes false.

## Resolved Terms

- No new terms introduced.
- `NO_AT_BRIDGE` is treated as a sanitized bridge-environment fact/setup-risk indicator, not as conclusive proof that tree extraction failed.
- No `CONTEXT.md` update required.

## Document Updates Applied

- Delta specs explicitly changed the old bridge-disabled semantics so proven collector success wins over `NO_AT_BRIDGE=1`.
- Delta specs added `env -u NO_AT_BRIDGE` and true degraded collector scenarios.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- None. The change is a corrective implementation/spec alignment under ADR 0009 and ADR 0011 rather than a new durable architecture decision.

## Open Questions

None.
