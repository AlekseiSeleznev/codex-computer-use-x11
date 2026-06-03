## Context Read

- `AGENTS.md` and user instructions: OpenSpec lifecycle is mandatory; do not implement before proposal, specs, grill, design, design-review, adr, test-plan, and tasks are complete; no commit/push/archive without explicit permission.
- `CONSTITUTION.md`: Rust/Cargo/Makefile verification, no `.secrets.local.env`, no external credentials, OpenSpec strict validation, machine-readable `doctor --json` validation.
- `CONTEXT.md`: `x11-ewmh`, AT-SPI window correlation, accessibility tree, app state, layer-degraded app state, controlled fixture, reason category.
- `ARCHITECTURE.md`, `adr/README.md`, ADR 0008, ADR 0009, ADR 0010: X11 root coordinates, final Cinnamon/X11 baseline with explicit degraded diagnostics, and standalone provider identity/takeover boundaries remain in force.
- Existing specs: `doctor-cli`, `x11-atspi-window-correlation`, `codex-x11-e2e-test-harness`, and `x11-packaging-docs-upstreaming` already define AT-SPI diagnostics, controlled fixtures, reason categories, and documentation expectations.
- Relevant code/docs: `src/doctor.rs`, `src/accessibility.rs`, `scripts/e2e/codex-x11-e2e.py`, `scripts/e2e/fixtures/gtk_atspi_fixture.py`, `tests/e2e_harness_scripts.rs`, and `docs/troubleshooting.md` currently record or recommend `NO_AT_BRIDGE=0`, which is likely the wrong bridge-enable contract.

## Plan Summary

Create a focused repair change for AT-SPI readiness diagnostics that recognizes `NO_AT_BRIDGE=1` as a bridge-disable signal, keeps bus reachability separate from tree extraction, corrects GTK fixture launch to remove `NO_AT_BRIDGE` instead of setting it to `0`, records sanitized bridge-env metadata, and updates docs/tests/validator evidence. The change stays within Cinnamon/X11 and uses only controlled fixtures for live validation.

## Question Loop

### Question 1: Should the controlled GTK fixture set `NO_AT_BRIDGE=0` or remove `NO_AT_BRIDGE` entirely?

- **Recommended answer:** Remove/unset `NO_AT_BRIDGE` for the fixture subprocess.
- **Rationale:** `NO_AT_BRIDGE` is a bridge suppression flag. Historical GTK/ATK bridge integrations and GNOME review comments treat presence of the variable as the disabling signal and recommend unsetting it rather than assigning a false-like value. Setting `NO_AT_BRIDGE=0` is ambiguous and may still disable the bridge for presence-based checks.
- **Resolution:** Specs and design require fixture subprocesses to remove `NO_AT_BRIDGE`; evidence records `NO_AT_BRIDGE` as absent, not as `0`.

### Question 2: Should `NO_AT_BRIDGE=1` make `readiness.ok=false`?

- **Recommended answer:** No, not by itself for the Cinnamon/X11 window/input baseline.
- **Rationale:** ADR 0009 allows environment-dependent semantic accessibility enrichment to degrade while the X11/EWMH baseline remains usable. The AT-SPI row still needs explicit degraded evidence and cannot be claimed as a pass.
- **Resolution:** Doctor adds `atspi_gtk_bridge_disabled_by_environment` with `reason_category=environment_limitation`; X11 baseline blockers remain separate.

### Question 3: Should doctor try to prove tree extraction by inspecting real user windows when no controlled fixture is available?

- **Recommended answer:** No.
- **Rationale:** User instructions and existing glossary require controlled fixtures for live app-state, AT-SPI, screenshot, input, pointer, and overlay checks. Real user windows may expose private UI and must not be a fallback target.
- **Resolution:** Doctor may run non-invasive bus/env checks and report bridge-disabled/tree-unavailable diagnostics; fixture-backed proof belongs to the controlled GTK fixture path.

### Question 4: Should the harness mutate the global user/Codex environment to remove `NO_AT_BRIDGE`?

- **Recommended answer:** No. Only sanitize the environment passed to the controlled GTK fixture subprocess.
- **Rationale:** The project must not change global user state during tests, and the user explicitly asked not to change global environment. Parent/session remediation should be documentation and operator action, not hidden mutation.
- **Resolution:** Design uses a child-process env map that removes `NO_AT_BRIDGE`; docs recommend restarting affected sessions/processes after user-controlled env changes.

### Question 5: Is absence of real live GTK fixture code a code failure for this change?

- **Recommended answer:** No, not solely. It is a setup/environment limitation unless a controlled fixture was expected, started, and then the tool behavior failed.
- **Rationale:** Existing evidence taxonomy distinguishes `missing_fixture_setup`, `environment_limitation`, and `code_failure`. The validator should recognize bridge-disabled diagnosis and not convert safe non-targeting into an implementation failure.
- **Resolution:** Specs require fake tests for bridge-disabled diagnosis and allow live fixture absence to remain setup/degraded evidence while still forbidding real-window fallback.

## Resolved Terms and Context Updates

No `CONTEXT.md` update is required. Existing terms `Accessibility tree`, `AT-SPI window correlation`, `Controlled fixture`, and `Reason category` cover this change. The new code `atspi_gtk_bridge_disabled_by_environment` is a diagnostic state, not a glossary term.

## OpenSpec Artifact Updates Applied

- `proposal.md` created.
- Spec deltas created for `doctor-cli`, `x11-atspi-window-correlation`, `codex-x11-e2e-test-harness`, and `x11-packaging-docs-upstreaming`.
- Spec deltas correct the prior `NO_AT_BRIDGE=0` fixture contract to `NO_AT_BRIDGE` absent.

## OpenSpec Artifact Updates Required Before Next Gate

- `design.md` must define the doctor data-model changes, fixture subprocess env construction, evidence shape, docs updates, and validation boundaries.
- `design-review.md` must stress-test the design against bridge env ambiguity, privacy, and controlled-fixture safety.

## ADR Candidates

No durable ADR candidate. This change corrects diagnostics and fixture evidence inside ADR 0009’s accepted X11 degraded-readiness baseline and does not alter backend identity, coordinate model, provider takeover, or supported scope.

## Open Questions

None.
