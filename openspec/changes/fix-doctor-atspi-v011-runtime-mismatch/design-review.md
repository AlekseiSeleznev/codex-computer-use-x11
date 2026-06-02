## Context Read

- `openspec/changes/fix-doctor-atspi-v011-runtime-mismatch/proposal.md`.
- `openspec/changes/fix-doctor-atspi-v011-runtime-mismatch/specs/doctor-cli/spec.md`.
- `openspec/changes/fix-doctor-atspi-v011-runtime-mismatch/specs/x11-atspi-window-correlation/spec.md`.
- `openspec/changes/fix-doctor-atspi-v011-runtime-mismatch/grill.md`.
- `openspec/changes/fix-doctor-atspi-v011-runtime-mismatch/design.md`.
- Root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and in-force ADRs 0001, 0003, 0005, 0006, 0007, 0008, 0009, 0010, and 0011.
- Canonical specs for `doctor-cli`, `x11-atspi-window-correlation`, `standalone-codex-mcp-plugin`, and `codex-x11-e2e-test-harness`.
- Relevant implementation and tests: `src/doctor.rs`, `src/accessibility.rs`, `tests/doctor_cli.rs`, `tests/accessibility_tree_cli.rs`, `docs/troubleshooting.md`, and e2e/installer tests mentioning `NO_AT_BRIDGE`.

## Design Summary

- The design removes the system-probe precondition that prevents `accessibility::atspi_probe_from_system()` from running when `NO_AT_BRIDGE=1` is present.
- The classifier remains evidence-based: bridge-disabled degradation applies only when bus is reachable, no tree is available, and bridge env indicates `NO_AT_BRIDGE=1`.
- Public CLI fake-command tests remain the primary TDD seam; no internal Rust function mocking is introduced.
- Existing bridge-env metadata and remediation remain for true degraded outcomes.
- Live verification compares doctor with `accessibility-tree` but does not make doctor target-scoped.

## Question Loop

1. **Could running the collector despite `NO_AT_BRIDGE=1` make doctor hang or regress the prior pipe-deadlock fix?**
   - **Recommended answer**: No if the implementation reuses `accessibility::atspi_probe_from_system()` unchanged.
   - **Rationale**: That path already uses the bounded collector timeout and concurrent stdout/stderr draining. The design changes only whether doctor calls it, not the collector execution model.
   - **Resolution**: Test plan must include the existing hung-command/large-output doctor CLI regression after the new `NO_AT_BRIDGE=1` success test.

2. **Does treating ambient collector candidates as doctor success violate AT-SPI correlation safety?**
   - **Recommended answer**: No, because doctor success means tree extraction availability, not target-specific subtree selection.
   - **Rationale**: `accessibility-tree` still owns target matching and confidence thresholds. Doctor reports readiness facts and does not return arbitrary target subtrees or send input.
   - **Resolution**: Keep wording and tests focused on `tree_available` / `match_outcome=tree_available`, not on target-specific `matched_subtree` unless a controlled fixture proves it.

3. **Are docs/installer/e2e statements about removing `NO_AT_BRIDGE` now wrong?**
   - **Recommended answer**: No, unless they claim doctor must never probe under `NO_AT_BRIDGE=1`.
   - **Rationale**: It remains correct to remove inherited `NO_AT_BRIDGE` for controlled GTK fixtures and future application processes. The changed rule is narrower: successful runtime collector output is authoritative for doctor availability.
   - **Resolution**: Implementation should update tests first. Docs only need changes if a test exposes text that contradicts the new precedence rule.

4. **Does this design require a durable ADR or architecture snapshot update?**
   - **Recommended answer**: No.
   - **Rationale**: It is a bugfix to an implementation/spec mismatch, not a new architecture boundary or hard-to-reverse decision. Existing ADRs already allow safe degraded AT-SPI and sanitized environment facts.
   - **Resolution**: Record no durable ADR in change-local `adr.md`; no `ARCHITECTURE.md` update required.

## Design Findings

- **Finding 1 — Existing old test is intentionally stale**: `tests/doctor_cli.rs::doctor_atspi_probe_preserves_bridge_disabled_state` currently asserts the collector must not run when `NO_AT_BRIDGE=1`. This must become a RED test for the new expected success behavior, not be preserved as-is.
- **Finding 2 — Success precedence is already supported by classifier**: `accessibility_report()` checks bridge-disabled only under `!tree`; once system facts set `atspi_tree_available=true`, diagnostic state naturally becomes `tree_extraction_available` for `match_outcome=tree_available`.
- **Finding 3 — True degraded tests must be explicit**: because the collector will run under `NO_AT_BRIDGE=1`, regression coverage must distinguish successful collector output from invalid/unavailable/timed-out output.
- **Finding 4 — Live evidence is optional but useful**: live comparison should be attempted if X11/focused window is available, but fake CLI regressions are the authoritative non-live proof required by the change.

## Document Updates Applied

None. The proposal, specs, grill, and design already encode the corrected behavior and implementation boundary.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. No durable ADR required.

## Open Questions

None.
