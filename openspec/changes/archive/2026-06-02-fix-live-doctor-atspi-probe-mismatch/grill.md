## Context Read

- `openspec/changes/fix-live-doctor-atspi-probe-mismatch/proposal.md`
- `openspec/changes/fix-live-doctor-atspi-probe-mismatch/specs/doctor-cli/spec.md`
- `openspec/changes/fix-live-doctor-atspi-probe-mismatch/specs/x11-atspi-window-correlation/spec.md`
- `CONSTITUTION.md` — Rust/Makefile verification, non-invasive doctor JSON, no secrets.
- `CONTEXT.md` — AT-SPI window correlation, accessibility tree, app state, layer-degraded app state, controlled fixture.
- `ARCHITECTURE.md` — standalone Rust runtime, thin AT-SPI boundary, degraded diagnostics, ADR 0009/0011 constraints.
- `adr/README.md`, `adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md`, `adr/0011-adopt-rollback-first-install-manifest.md`.
- Existing specs: `openspec/specs/doctor-cli/spec.md` and `openspec/specs/x11-atspi-window-correlation/spec.md`.
- Relevant code locations identified from verification: `src/doctor.rs` around doctor fact gathering and `src/accessibility.rs` around `atspi_probe_from_system()` / collector execution.

## Plan Summary

- The change is a targeted corrective follow-up for a live doctor false-negative found after the rollback-first install change was archived.
- The behavior boundary is `doctor --json` AT-SPI facts, not installer rollback, provider takeover, screenshots, input, or source overlay.
- `accessibility-tree --window-id ... --json` remains the canonical proof that the collector can return a usable subtree for a resolved target; doctor must not contradict that when it uses the same effective environment.
- The fix must remain read-only and additive/corrective in JSON shape.
- Tests must reproduce the mismatch without relying only on the developer's live session, then final verification should include a live-safe comparison when X11 is available.

## Question Loop

1. **Should doctor require a target window to prove AT-SPI availability?**
   - **Recommended answer**: No. Doctor remains a non-invasive readiness probe without target selection, but it should interpret successful ambient collector enumeration consistently with the accessibility-tree collector path.
   - **Rationale**: Existing specs require a lightweight probe without a target selector; adding mandatory target selection would change the doctor contract and make smoke tests harder.
   - **Resolution**: Use collector availability/candidate facts for doctor; reserve target-specific confidence checks for `accessibility-tree` and live-safe verification evidence.

2. **Should `NO_AT_BRIDGE=1` still force a degraded doctor state even if another already-running app exposes AT-SPI nodes?**
   - **Recommended answer**: Yes for the doctor process environment. Presence of `NO_AT_BRIDGE=1` remains a bridge-disabled diagnostic because new GTK fixture/application processes inheriting that environment may suppress bridge loading.
   - **Rationale**: Existing specs and ADR 0011 use presence-based `NO_AT_BRIDGE` handling for setup/rollback safety. The observed local shell had `NO_AT_BRIDGE=1`, but clearing it still reproduced the mismatch; the fix should address both the bridge-disabled branch and the cleared-env collector branch.
   - **Resolution**: Preserve bridge-disabled degradation; fix the collector-success branch when bridge is not disabled.

3. **Is this an architecture decision requiring a durable ADR?**
   - **Recommended answer**: No separate durable ADR. This is a bugfix aligning implementation with existing ADR 0009 and existing AT-SPI/doctor specs.
   - **Rationale**: It does not introduce a new hard-to-reverse architecture boundary; it corrects a divergence between two existing paths.
   - **Resolution**: Record ADR review in `adr.md` as “no new durable ADR required.”

## Resolved Terms

- Existing terms are sufficient: `AT-SPI window correlation`, `Accessibility tree`, `App state`, `Layer-degraded app state`, and `Controlled fixture`.
- No `CONTEXT.md` glossary update is needed because no new project term was introduced.

## Document Updates Applied

- Delta specs require `doctor --json` and the lightweight AT-SPI probe to share the collector success contract used by accessibility-tree.
- Delta specs preserve bridge-disabled degradation and safe ambiguous/no-match behavior.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- None. This is a corrective implementation change under in-force ADR 0009 and existing AT-SPI/doctor contracts.

## Open Questions

None.
