## Context Read

- Change artifacts: `proposal.md`, `specs/doctor-cli/spec.md`, and `specs/x11-packaging-docs-upstreaming/spec.md` for `make-doctor-x11-only-no-wayland-portal-noise`.
- Root project context: `AGENTS.md`, `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/README.md`.
- Relevant in-force ADRs: ADR 0008 (X11 root coordinate model), ADR 0009 (final Cinnamon/X11 v1 DoD baseline), ADR 0010 (X11 provider takeover shim), and ADR 0011 (rollback-first install manifest).
- Canonical specs: `openspec/specs/doctor-cli/spec.md`, `openspec/specs/x11-packaging-docs-upstreaming/spec.md`, and `openspec/specs/codex-source-overlay-extension/spec.md`.
- Relevant docs: `README.md`, `INSTALL_CODEX.md`, `docs/troubleshooting.md`, `docs/upstreaming.md`, `docs/integration-contract.md`, `docs/e2e-harness.md`, `docs/release-checklist.md`, and `docs/final-architecture-dod.md` search hits for RemoteDesktop/portal/Wayland/doctor readiness wording.
- Relevant code/tests: `src/doctor.rs`, `tests/doctor_cli.rs`, and existing doctor unit tests around `remote_desktop_portal_unavailable`, `wayland_runtime_out_of_scope`, and recommended next-step priority.

## Plan Summary

- The change narrows standalone doctor readiness to the explicit `x11-ewmh` baseline: X11 display/EWMH tooling, local input backend, and optional X11 enrichments such as AT-SPI/screenshot remain relevant.
- RemoteDesktop portal and Wayland facts may remain serialized for compatibility/debug visibility, but they must not enter readiness blockers, degraded reasons, optional enrichments, unsupported-out-of-scope readiness entries, or next-step recommendations.
- Existing tests intentionally prove the current bug: `doctor_readiness_exposes_additive_x11_taxonomy` expects `remote_desktop_portal_unavailable` and `wayland_runtime_out_of_scope`; `doctor_remote_desktop_gap_is_report_only_when_x11_input_path_works` expects the RemoteDesktop degraded string. These become RED candidates.
- Documentation currently contains several stale statements that tell users to inspect portal readiness or treat RemoteDesktop facts as diagnostics for current Cinnamon/X11 doctor behavior; those should be removed or reframed as non-readiness scope notes.
- Source-overlay target diagnostics may still have historical/target vocabulary about strict portal checks, but standalone plugin docs and doctor readiness must not present RemoteDesktop portal as a current plugin readiness layer.

## Question Loop

1. **Should compatibility JSON fields for `environment.wayland_display_present`, `portals.remote_desktop`, and `input.remote_desktop` be removed entirely or kept neutral/debug-only?**
   - **Recommended answer**: Keep them for now as neutral/debug-only compatibility fields, and remove them from readiness computation and documentation guidance.
   - **Rationale**: `doctor-cli` already promises additive bootstrap compatibility for top-level shape and richer facts. Removing public JSON fields would be a broader breaking/API cleanup not required to stop readiness noise. Neutralizing readiness usage satisfies the user requirement while minimizing downstream churn.
   - **Resolution**: Use the compatibility-field path. Design/test-plan must assert that retained fields do not affect `readiness.*` or `recommended_next_step`.

2. **Should RemoteDesktop portal ever count as `can_send_development_input` for this plugin after the X11-only baseline clarification?**
   - **Recommended answer**: No. For this standalone X11/EWMH-only plugin, `can_send_development_input` should be derived from supported local input backends (`/dev/uinput`/abs pointer and ydotool). RemoteDesktop may remain a serialized fact but not a readiness backend.
   - **Rationale**: The user explicitly says the project does not plan to support Wayland/RemoteDesktop portal in this plugin. Letting portal success or failure change input readiness keeps an unsupported backend coupled to the readiness result.
   - **Resolution**: Specs already encode this. Design must update `readiness_report` so `input.remote_desktop` is not part of `can_send_development_input` or input-blocker remediation.

3. **Does `WAYLAND_DISPLAY` present beside `DISPLAY` indicate unsupported runtime state that should be reported in doctor readiness?**
   - **Recommended answer**: No. In the X11-only baseline, a usable X11 session with `XDG_SESSION_TYPE=x11` and `DISPLAY` set should stay ready even if `WAYLAND_DISPLAY` exists in the inherited environment.
   - **Rationale**: Many desktop environments can expose mixed or inherited environment variables. Doctor readiness should answer whether the X11/EWMH baseline is usable, not warn about unsupported scope when the supported path passes.
   - **Resolution**: Specs include the mixed X11 + `WAYLAND_DISPLAY` scenario. Design/test-plan must add a regression proving no `wayland_runtime_out_of_scope` readiness issue or Wayland-specific recommendation.

4. **Does this require a new durable ADR or architecture snapshot update?**
   - **Recommended answer**: No new durable ADR; update the change-local `adr.md` to say ADR 0009 already decides X11-only scope, and this change only corrects doctor readiness semantics under that accepted baseline.
   - **Rationale**: The hard-to-reverse architecture decision already exists in ADR 0009: `x11-ewmh` baseline is supported and Cinnamon Wayland/portal-required runtime paths are outside v1. This change removes noisy implementation/reporting artifacts rather than introducing a new boundary.
   - **Resolution**: No durable ADR file and no `ARCHITECTURE.md` update are required unless implementation discovers a broader architecture contradiction.

## Resolved Terms

- `x11-ewmh` remains the canonical backend label and readiness baseline term from `CONTEXT.md`.
- `RemoteDesktop portal` is treated as unsupported compatibility/debug context for this standalone plugin, not as an X11-baseline readiness layer.
- `WAYLAND_DISPLAY` is an environment fact only; it is not a doctor readiness signal when the X11 baseline passes.
- No new glossary term was introduced, so `CONTEXT.md` did not require an update.

## Document Updates Applied

- Delta specs were written to require neutral/debug-only RemoteDesktop/Wayland fields if retained.
- Delta specs explicitly forbid `RemoteDesktop portal unavailable or incomplete`, `remote_desktop_portal_unavailable`, `wayland_runtime_out_of_scope`, and RemoteDesktop/Wayland remediation in readiness fields for an otherwise-ready X11 baseline.
- Delta docs specs were written to remove RemoteDesktop/Wayland troubleshooting from the standalone X11 plugin readiness path.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. ADR 0009 already records the durable X11-only scope; this change is a corrective readiness/reporting alignment.

## Open Questions

None.
