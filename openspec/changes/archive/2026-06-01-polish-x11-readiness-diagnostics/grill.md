## Context Read

- `CONSTITUTION.md` — required technologies, no-secret policy, verification rules, automatic safe checkpoint guidance.
- `CONTEXT.md` — project glossary, especially `x11-ewmh`, `App state`, `Capability matrix evidence`, `E2E harness`, `Final DoD`, and newly resolved `Reason category` / `Controlled fixture` terms.
- `ARCHITECTURE.md` — current Cinnamon/X11 v1 baseline, lifecycle gates, ADR relationships, and X11-only scope.
- `adr/README.md` plus in-force ADRs 0005, 0007, 0008, 0009, and 0010.
- Existing canonical specs: `doctor-cli`, `x11-atspi-window-correlation`, `codex-x11-e2e-test-harness`, `x11-screenshot-coordinate-model`, `x11-get-app-state-integration`, `x11-target-window-groups-overlays`, and `x11-packaging-docs-upstreaming`.
- Change proposal and delta specs under `openspec/changes/polish-x11-readiness-diagnostics/`.

## Plan Summary

- The change is a planning-only X11/Cinnamon production-polish change; it does not implement runtime behavior yet.
- The supported scope is ADR 0009's Cinnamon/X11 `x11-ewmh` baseline; Wayland and portal-required runtime paths remain out of scope.
- Doctor readiness will distinguish blockers from acceptable degraded optional enrichments and unsupported paths while preserving bootstrap-compatible JSON fields.
- E2E evidence will use stable reason categories and controlled fixtures so production claims are based on safe, reproducible evidence.
- Documentation will make PASS/DEGRADED/FAIL, safe retest commands, and production-claim evidence readable without chat context.

## Question Loop

### Question 1: Should Wayland or RemoteDesktop portal become a required runtime fallback in this polish change?

- **Recommended answer:** No. Keep Wayland out of scope and keep RemoteDesktop portal only as an optional diagnostic signal.
- **Rationale:** The user request explicitly excludes Wayland support and portal-required runtime paths. ADR 0009 already defines the supported v1 claim as Cinnamon/X11 with unsupported Wayland and unsafe unverified input out of scope.
- **Resolution:** Resolved from user request and ADR 0009; no user question needed. Specs classify Wayland/portal-required paths as unsupported/out-of-scope diagnostics, not blockers for the X11 baseline.

### Question 2: Should live input/pointer/overlay checks ever fall back to real user applications when controlled fixtures are missing?

- **Recommended answer:** No. Missing fixtures must be classified as `missing_fixture_setup`; real user apps must not be used as fallback targets.
- **Rationale:** The user request and existing safety model require live input/pointer/overlay only against controlled fixtures. Falling back to ambient apps would violate targeted-input safety and make evidence unsafe.
- **Resolution:** Resolved from user request, existing target-window/input specs, and ADR 0009. Specs require fixture uniqueness proof and classify missing fixtures separately.

### Question 3: Should fake-mode screenshot degraded evidence be allowed forever?

- **Recommended answer:** Allow either a fake screenshot provider pass or an explicitly documented expected fake-fixture limitation, but keep real screenshot crop integrity strict.
- **Rationale:** The retest showed fake screenshot degradation because fake `gdbus` was unavailable. The production risk is ambiguity, not necessarily the lack of fake screenshot bytes. The real crop path must still validate output files and dimensions.
- **Resolution:** Resolved in specs: fake smoke may pass with a fake provider fixture, or degrade with an expected fake-fixture reason category; provider success without a valid output file remains a code failure.

## Resolved Terms

- `Reason category` — added to `CONTEXT.md` as the stable machine-readable label explaining degraded/fail evidence.
- `Controlled fixture` — added to `CONTEXT.md` as the test-owned target boundary for safe live validation.

## Document Updates Applied

- Added `Reason category` and `Controlled fixture` glossary entries to `CONTEXT.md`.
- Delta specs explicitly preserve X11-only scope and classify Wayland/portal-required runtime paths as unsupported/out of scope.
- Delta specs require controlled fixture uniqueness, cleanup evidence, and `missing_fixture_setup` classification when fixture setup is absent.
- Delta specs preserve strict screenshot-crop output integrity while allowing fake screenshot limitation evidence to be explicit.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

- No new durable ADR candidate is required for this change. The X11-only scope, root-coordinate screenshot model, fixture safety posture, and standalone provider identity are already covered by ADRs 0008, 0009, and 0010. This change refines diagnostics/evidence within those accepted decisions.

## Open Questions

None.
