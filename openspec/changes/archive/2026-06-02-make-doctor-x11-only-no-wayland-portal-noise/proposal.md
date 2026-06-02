## Why

`codex-computer-use-x11` is an explicit X11/EWMH-only plugin, but current doctor readiness still treats missing RemoteDesktop portal and Wayland-related facts as degraded or out-of-scope signals in X11 reports. This creates noisy false degradation for a ready X11 baseline and can steer users toward fixing portal/Wayland capabilities that this plugin does not support.

## What Changes

- Define the supported doctor baseline as `x11-ewmh` and make X11/EWMH readiness independent from RemoteDesktop portal and Wayland runtime availability.
- Remove RemoteDesktop portal absence and Wayland presence from `readiness.degraded_reasons`, `optional_enrichments`, `unsupported_out_of_scope`, `recommended_next_step`, and readiness blockers for the X11 baseline.
- Keep any remaining RemoteDesktop/Wayland JSON facts neutral/debug-only for compatibility, or remove stale fields/docs/tests if compatibility is not needed.
- Update documentation/specs so RemoteDesktop portal is no longer described as part of current X11 plugin readiness diagnostics.
- Add regression tests proving an otherwise-ready X11 session remains `readiness.ok=true` with no blockers or degraded reasons when RemoteDesktop portal is absent, including when `WAYLAND_DISPLAY` is also present.

## Capabilities

- Modified capability: `doctor-cli` — X11 baseline readiness taxonomy, RemoteDesktop/Wayland neutralization, recommended next-step behavior, and doctor JSON compatibility.
- Modified capability: `x11-packaging-docs-upstreaming` — documentation wording for X11-only production readiness and troubleshooting guidance.

## Impact

- Code/tests: likely `src/doctor.rs`, `tests/doctor_cli.rs`, and any docs/spec tests that assert portal/Wayland readiness wording.
- CLI/API: corrective doctor JSON semantics. Existing compatibility fields such as `environment.wayland_display_present`, `portals.remote_desktop`, and `input.remote_desktop` may remain if they are neutral/debug-only and do not feed readiness; no secret values are emitted.
- Documentation/specs: remove wording that tells X11 users to inspect or remediate RemoteDesktop portal/Wayland as part of the current plugin readiness path.
- Architecture/ADR constraints: preserves ADR 0009's `x11-ewmh` Cinnamon/X11 v1 baseline and out-of-scope Wayland decision, but changes doctor so unsupported scope is documented rather than emitted as per-run readiness noise.
- Verification: strict OpenSpec validation, TDD evidence, `make fmt`, `make check`, `make test`, machine-readable `doctor --json` smoke, and no archive without separate explicit confirmation.
