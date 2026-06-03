## 1. TDD slice: RemoteDesktop absence is neutral for ready X11

- [x] 1.1 RED: update/replace `src/doctor.rs` unit coverage as `doctor_remote_desktop_gap_is_neutral_when_x11_input_path_works`, expecting X11-ready facts with empty RemoteDesktop introspection to produce `readiness.ok=true`, `blockers=[]`, `degraded_reasons=[]`, no `remote_desktop_portal_unavailable`, and no RemoteDesktop next step.
- [x] 1.2 RED: run `cargo test doctor_remote_desktop_gap_is_neutral_when_x11_input_path_works -- --nocapture` and record the expected failure in `test-plan.md`.
- [x] 1.3 GREEN: remove RemoteDesktop portal absence from `readiness_report` degraded/optional readiness aggregation while preserving neutral compatibility facts if retained.
- [x] 1.4 GREEN: rerun the slice 1 command and record passing evidence.
- [x] 1.5 REFACTOR: keep compatibility field wording neutral and avoid deleting JSON fields unless no compatibility tests require them.

## 2. TDD slice: Wayland env beside X11 is neutral

- [x] 2.1 RED: update `doctor_readiness_exposes_additive_x11_taxonomy` into `doctor_x11_taxonomy_ignores_remote_desktop_and_wayland_noise`, expecting X11-ready facts plus `WAYLAND_DISPLAY` to keep readiness clean and omit `wayland_runtime_out_of_scope`.
- [x] 2.2 RED: run `cargo test doctor_x11_taxonomy_ignores_remote_desktop_and_wayland_noise -- --nocapture` and record the expected failure in `test-plan.md`.
- [x] 2.3 GREEN: remove Wayland/`WAYLAND_DISPLAY` readiness issue creation from `readiness_report` while preserving neutral environment facts.
- [x] 2.4 GREEN: rerun the slice 2 command and record passing evidence.

## 3. TDD slice: development input uses local X11 backends

- [x] 3.1 RED: add `doctor_development_input_uses_local_x11_backends_not_remote_desktop`, proving `/dev/uinput` or ydotool satisfy development input readiness and RemoteDesktop portal alone does not.
- [x] 3.2 RED: run `cargo test doctor_development_input_uses_local_x11_backends_not_remote_desktop -- --nocapture` and record the expected failure or guard result in `test-plan.md`.
- [x] 3.3 GREEN: update `can_send_development_input`, the no-input blocker, and the no-input recommendation to use supported local X11 input backends only.
- [x] 3.4 GREEN: rerun the slice 3 command and record passing evidence.

## 4. TDD slice: serialized CLI readiness omits forbidden strings

- [x] 4.1 RED: add `tests/doctor_cli.rs::doctor_cli_x11_baseline_ignores_absent_remote_desktop_and_wayland_display` using fake X11/EWMH, AT-SPI tree, ydotool/uinput readiness, absent RemoteDesktop portal, and present `WAYLAND_DISPLAY`.
- [x] 4.2 RED: run `cargo test --test doctor_cli doctor_cli_x11_baseline_ignores_absent_remote_desktop_and_wayland_display -- --nocapture` and record the expected failure in `test-plan.md`.
- [x] 4.3 GREEN: make minimal code adjustments so serialized readiness has `ok=true`, empty blockers/degraded reasons, and no forbidden RemoteDesktop/Wayland strings in readiness fields or next step.
- [x] 4.4 GREEN: rerun the slice 4 command and record passing evidence.
- [x] 4.5 Regression: run `cargo test --test doctor_cli -- --nocapture` and `cargo test doctor_ -- --nocapture`.

## 5. Documentation/spec cleanup

- [x] 5.1 Update user-facing docs so current standalone `doctor --json` troubleshooting is X11-scoped and no longer describes RemoteDesktop portal or Wayland repair as readiness remediation.
- [x] 5.2 Inspect remaining `RemoteDesktop`, `portal`, `Wayland`, `remote_desktop_portal_unavailable`, and `wayland_runtime_out_of_scope` hits in README/docs/specs; preserve only static scope notes or target/source-overlay-specific wording that is not current standalone doctor readiness.
- [x] 5.3 Run strict OpenSpec validation after docs/spec updates and record the result.

## 6. Full verification and checkpoint

- [x] 6.1 Run `openspec validate make-doctor-x11-only-no-wayland-portal-noise --type change --strict` and `openspec validate --all --strict`.
- [x] 6.2 Run `make fmt`.
- [x] 6.3 Run `make check`.
- [x] 6.4 Run `make test`.
- [x] 6.5 Run live/local `doctor --json` smoke if applicable, parse JSON, and confirm readiness fields are free of forbidden RemoteDesktop/Wayland strings; record exact blocker if live smoke is not applicable.
- [x] 6.6 Confirm `.secrets.local.env` was not read, printed, staged, or committed.
- [x] 6.7 Show `git status --short --untracked-files=all`, checkpoint implementation/test evidence, push the branch to GitHub after successful verification, and stop before archive unless the user separately confirms archive.
