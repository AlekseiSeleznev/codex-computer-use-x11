## Context Read

- Change artifacts: `proposal.md`, delta specs, `grill.md`, and `design.md` for `make-doctor-x11-only-no-wayland-portal-noise`.
- Root context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`.
- In-force ADRs considered: 0008, 0009, 0010, 0011 plus the ADR list in `adr/README.md`.
- Canonical specs: `openspec/specs/doctor-cli/spec.md`, `openspec/specs/x11-packaging-docs-upstreaming/spec.md`, `openspec/specs/codex-source-overlay-extension/spec.md`.
- Implementation/test context: `src/doctor.rs` readiness aggregation and unit tests, `tests/doctor_cli.rs` public CLI tests.
- Documentation search results for RemoteDesktop/portal/Wayland in `README.md`, `INSTALL_CODEX.md`, `docs/troubleshooting.md`, `docs/integration-contract.md`, `docs/e2e-harness.md`, `docs/upstreaming.md`, `docs/release-checklist.md`, and `docs/final-architecture-dod.md`.

## Design Summary

- Keep portal/Wayland facts serialized as compatibility/debug data, but remove them from all readiness outputs.
- Compute `can_send_development_input` from local X11-supported input paths only (`/dev/uinput` and ydotool).
- Remove the old `remote_desktop_portal_unavailable` and `wayland_runtime_out_of_scope` readiness issue creation branches.
- Update user-facing docs so current standalone doctor troubleshooting is X11-scoped and no longer recommends RemoteDesktop/Wayland fixes.
- Verify through public doctor JSON tests before production changes, then full Make/OpenSpec/live-smoke checks.

## Question Loop

1. **Could retained `input.remote_desktop.available=true` still confuse readiness if code leaves it visible in JSON?**
   - **Recommended answer**: It is acceptable only if tests prove readiness ignores it and docs describe it as compatibility/debug-only or avoid presenting it as current readiness.
   - **Rationale**: Removing JSON fields would be more disruptive. The design can satisfy the user requirement by proving no readiness field, blocker, degraded reason, optional enrichment, unsupported issue, or next step depends on it.
   - **Resolution**: Test plan must include a regression where RemoteDesktop is absent and readiness is clean; implementation should optionally add a unit test that portal-only input does not satisfy `can_send_development_input` if practical.

2. **Could deleting `wayland_runtime_out_of_scope` make the unsupported Wayland product scope invisible?**
   - **Recommended answer**: No if docs keep a static scope statement and doctor still reports neutral `environment.wayland_display_present` when that compatibility field exists.
   - **Rationale**: The user wants doctor not to warn or degrade due to Wayland-related capabilities. Product scope belongs in README/troubleshooting/ADR, not per-run readiness noise when X11 passes.
   - **Resolution**: Docs update should keep static “Wayland unsupported/out of scope” wording while avoiding doctor-remediation guidance or runtime degraded-row wording.

3. **Does the design conflict with ADR 0009's line that diagnostics must use strict RemoteDesktop method/property checks?**
   - **Recommended answer**: No. Strict parsing may remain for compatibility facts; the change removes RemoteDesktop from standalone readiness influence.
   - **Rationale**: ADR 0009 was written when target/upstream-compatible portal diagnostics were still part of the report vocabulary. The same ADR also says Cinnamon Wayland and portal-required runtime paths are outside v1. Neutral debug-only parsing preserves strictness without readiness noise.
   - **Resolution**: No durable ADR required; change-local ADR should record that ADR 0009 remains in force and this change narrows doctor readiness interpretation under it.

4. **Which tests should be RED before production changes?**
   - **Recommended answer**: Update existing `src/doctor.rs` unit tests first because they currently assert the exact stale behavior: RemoteDesktop degraded reason, optional enrichment code, Wayland unsupported code, and RemoteDesktop wording expectations.
   - **Rationale**: They exercise the public serialized report model through `report_from_probe` without requiring live desktop state and will fail immediately on current code.
   - **Resolution**: Test plan should start with a vertical unit-report slice, then a CLI fake-desktop slice for the serialized `doctor --json` forbidden strings.

## Design Findings

- **Finding 1 — Keep vs remove fields is resolved**: Compatibility/debug retention is safer than field removal for this change. Implementation should not delete `PortalReport`/`InputReport.remote_desktop` unless a green refactor proves it is unused and non-breaking.
- **Finding 2 — Existing tests are stale by design**: `doctor_readiness_exposes_additive_x11_taxonomy` and `doctor_remote_desktop_gap_is_report_only_when_x11_input_path_works` must be rewritten as RED tests for the new behavior.
- **Finding 3 — Recommended next step must drop RemoteDesktop from input remediation**: Even when no local input backend is available, the recommendation should name supported local X11 input backends, not RemoteDesktop portal input.
- **Finding 4 — Docs need careful scope wording**: Static unsupported-scope wording can remain, but docs must not say doctor reports Wayland/portal degraded diagnostics or tells users to inspect/fix portal readiness for the standalone X11 plugin.
- **Finding 5 — Source-overlay target specs may remain scoped**: `codex-source-overlay-extension` can still discuss strict portal checks for target diagnostics as long as standalone docs/doctor no longer present RemoteDesktop portal as current plugin readiness.

## Document Updates Applied

None after design. Proposal/specs/grill/design already encode the resolved behavior and boundaries.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. ADR 0009 already carries the durable X11-only scope. The change-local ADR should document no new durable ADR.

## Open Questions

None.
