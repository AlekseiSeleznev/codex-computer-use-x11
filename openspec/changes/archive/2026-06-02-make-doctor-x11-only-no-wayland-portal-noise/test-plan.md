## TDD Strategy

Use project-local `tdd` discipline with vertical behavior slices through the public doctor report surface. Start with tests that exercise `src::doctor::report_from_probe` because existing unit tests already encode the stale readiness taxonomy and fail fast without live desktop dependencies. Then add a public CLI fake-desktop regression so serialized `doctor --json` readiness fields omit the forbidden RemoteDesktop/Wayland strings. Production changes must follow RED evidence for each behavior, then GREEN, then refactor while green.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. X11 baseline ready when RemoteDesktop absent | `doctor::report_from_probe(base_facts with empty RemoteDesktop introspection)` returns ready X11 readiness with no RemoteDesktop degraded reason or optional enrichment | Update `src/doctor.rs` unit test `doctor_remote_desktop_gap_is_report_only_when_x11_input_path_works` (or replacement) to expect `readiness.ok=true`, empty blockers, empty degraded reasons, no `remote_desktop_portal_unavailable`, and no RemoteDesktop in `recommended_next_step`; run `cargo test doctor_remote_desktop_gap_is_neutral_when_x11_input_path_works -- --nocapture`. Expected RED: current code includes `RemoteDesktop portal unavailable or incomplete` and `remote_desktop_portal_unavailable`. | Same command passes after removing RemoteDesktop portal absence from readiness aggregation. | Keep compatibility `portals.remote_desktop` facts if retained; assertions focus on readiness fields. |
| 2. `WAYLAND_DISPLAY` beside X11 is neutral | `doctor::report_from_probe(base_facts with XDG_SESSION_TYPE=x11 and WAYLAND_DISPLAY)` does not emit Wayland readiness issue | Update `doctor_readiness_exposes_additive_x11_taxonomy` into `doctor_x11_taxonomy_ignores_remote_desktop_and_wayland_noise`; run `cargo test doctor_x11_taxonomy_ignores_remote_desktop_and_wayland_noise -- --nocapture`. Expected RED: current code includes `wayland_runtime_out_of_scope` and RemoteDesktop optional enrichment. | Same command passes with `readiness.ok=true`, no blockers, no degraded reasons, and no `wayland_runtime_out_of_scope`. | Do not remove static product-scope docs/ADR; only readiness runtime noise is removed. |
| 3. Development input readiness ignores portal | `doctor::report_from_probe` derives `can_send_development_input` from `/dev/uinput` or ydotool, not RemoteDesktop | Add/update a unit test such as `doctor_development_input_uses_local_x11_backends_not_remote_desktop`; run `cargo test doctor_development_input_uses_local_x11_backends_not_remote_desktop -- --nocapture`. Expected RED: portal-only input currently can satisfy readiness or no-input recommendation mentions RemoteDesktop. | Same command passes with portal-only not enough, local ydotool/uinput enough, and no-input recommendation mentioning only supported local X11 input backends. | Keep the test at report boundary; no internal mocking beyond `ProbeFacts`. |
| 4. Serialized CLI omits forbidden readiness strings | `codex-computer-use-x11 doctor --json` with fake X11 desktop, AT-SPI tree ok, ydotool/uinput ok, RemoteDesktop absent, and `WAYLAND_DISPLAY` present serializes clean readiness | Add `tests/doctor_cli.rs` fake desktop test; run `cargo test --test doctor_cli doctor_cli_x11_baseline_ignores_absent_remote_desktop_and_wayland_display -- --nocapture`. Expected RED: current serialized readiness contains `RemoteDesktop portal unavailable or incomplete`, `remote_desktop_portal_unavailable`, or `wayland_runtime_out_of_scope`. | Same command passes with `readiness.ok=true`, `blockers=[]`, `degraded_reasons=[]`, and no forbidden strings in readiness fields or `recommended_next_step`. | Assert forbidden strings only in readiness subtree/next-step unless full JSON cleanup removes compatibility fields. |
| 5. Docs no longer describe portal/Wayland as current doctor readiness remediation | User-facing docs/specs avoid current standalone doctor RemoteDesktop/Wayland readiness guidance | Add/update text checks if existing docs tests cover snippets, otherwise run `rg -n "RemoteDesktop|portal readiness|wayland_runtime_out_of_scope|remote_desktop_portal_unavailable" README.md INSTALL_CODEX.md docs openspec/specs` and inspect allowed scope notes. Expected RED is manual/textual: docs currently mention portal readiness and RemoteDesktop diagnostics as current troubleshooting. | Docs are rewritten so static scope notes remain but current doctor readiness troubleshooting no longer tells users to fix RemoteDesktop/Wayland; strict OpenSpec validation and `make test` pass. | Avoid deleting target/source-overlay wording that is not current standalone doctor readiness unless tests or wording make it ambiguous. |

## Mocking / Boundary Policy

- Use `ProbeFacts` unit tests for readiness aggregation; this is the public report model boundary, not an internal collaborator mock.
- Use fake command `PATH` fixtures for CLI tests (`wmctrl`, `xprop`, `xdotool`, `ydotool`, `busctl`, `gdbus`, `python3`) as already established in `tests/doctor_cli.rs`.
- Do not mock internal Rust functions.
- Do not access `.secrets.local.env`, screenshots, input injection, external services, or target checkout writes.
- Keep live smoke non-invasive: `doctor --json` only when applicable.

## Required Checks

- `openspec validate make-doctor-x11-only-no-wayland-portal-noise --type change --strict`
- Slice 1 command: `cargo test doctor_remote_desktop_gap_is_neutral_when_x11_input_path_works -- --nocapture`
- Slice 2 command: `cargo test doctor_x11_taxonomy_ignores_remote_desktop_and_wayland_noise -- --nocapture`
- Slice 3 command: `cargo test doctor_development_input_uses_local_x11_backends_not_remote_desktop -- --nocapture`
- Slice 4 command: `cargo test --test doctor_cli doctor_cli_x11_baseline_ignores_absent_remote_desktop_and_wayland_display -- --nocapture`
- Focused doctor suites: `cargo test doctor_ -- --nocapture` and `cargo test --test doctor_cli -- --nocapture`
- `make fmt`
- `make check`
- `make test`
- Strict OpenSpec validation
- Live installed/local doctor smoke if applicable: `cargo run -- doctor --json` or `./target/debug/codex-computer-use-x11 doctor --json`, parse as JSON, and confirm readiness fields are free of forbidden RemoteDesktop/Wayland strings.
- Final `git status --short --untracked-files=all`; ensure no local secret files were read, printed, staged, or committed.

## Evidence Log

- Slice 1 — RemoteDesktop absence is neutral for ready X11
  - RED command: `cargo test doctor_remote_desktop_gap_is_neutral_when_x11_input_path_works -- --nocapture`.
  - RED result: failed as expected before production change because `readiness.degraded_reasons` contained `RemoteDesktop portal unavailable or incomplete` and `optional_enrichments` contained `remote_desktop_portal_unavailable`.
  - GREEN change: removed RemoteDesktop portal absence from `readiness_report` degraded/optional readiness aggregation while preserving compatibility facts.
  - GREEN command/result: same command passed.

- Slice 2 — `WAYLAND_DISPLAY` beside X11 is neutral
  - RED command: `cargo test doctor_x11_taxonomy_ignores_remote_desktop_and_wayland_noise -- --nocapture`.
  - RED result: failed as expected because `readiness.unsupported_out_of_scope` contained `wayland_runtime_out_of_scope`.
  - GREEN change: removed Wayland/`WAYLAND_DISPLAY` runtime readiness issue creation while preserving neutral environment facts.
  - GREEN command/result: same command passed.

- Slice 3 — development input uses local X11 backends
  - RED command: `cargo test doctor_development_input_uses_local_x11_backends_not_remote_desktop -- --nocapture`.
  - RED result: failed as expected because the no-input recommendation still mentioned `RemoteDesktop`, and portal input still participated in development-input readiness.
  - GREEN change: `can_send_development_input`, no-input blocker, and recommendation now use local X11 input backends only: `/dev/uinput` and ydotool.
  - GREEN command/result: same command passed.

- Slice 4 — serialized CLI readiness omits forbidden strings
  - Added `tests/doctor_cli.rs::doctor_cli_x11_baseline_ignores_absent_remote_desktop_and_wayland_display` with fake X11/EWMH, valid AT-SPI tree, connectable fake ydotool socket, empty RemoteDesktop introspection, and present `WAYLAND_DISPLAY`.
  - First run exposed a test-fixture issue (`UnixListener` path too long), fixed by using a short `/tmp` runtime directory for the fake ydotool socket.
  - GREEN command/result: `cargo test --test doctor_cli doctor_cli_x11_baseline_ignores_absent_remote_desktop_and_wayland_display -- --nocapture` passed. The serialized readiness had `ok=true`, empty blockers, empty degraded reasons, empty optional enrichments, empty unsupported-out-of-scope, and no forbidden strings.

- Documentation cleanup
  - Updated README/troubleshooting/integration/e2e/upstreaming/final DoD wording and packaging docs tests so RemoteDesktop/Wayland are static out-of-scope or compatibility/debug context, not current standalone doctor readiness remediation.
  - `cargo test --test packaging_docs -- --nocapture`: passed.

- Focused regression checks
  - `cargo test doctor_remote_desktop_gap_is_neutral_when_x11_input_path_works -- --nocapture`: passed.
  - `cargo test doctor_x11_taxonomy_ignores_remote_desktop_and_wayland_noise -- --nocapture`: passed.
  - `cargo test doctor_development_input_uses_local_x11_backends_not_remote_desktop -- --nocapture`: passed.
  - `cargo test --test doctor_cli doctor_cli_x11_baseline_ignores_absent_remote_desktop_and_wayland_display -- --nocapture`: passed.
  - `cargo test doctor_ -- --nocapture`: passed, 30/30 doctor-filtered unit tests.
  - `cargo test --test doctor_cli -- --nocapture`: passed, 11/11 tests.
  - `cargo test --test packaging_docs -- --nocapture`: passed, 9/9 tests.



- Full verification
  - `openspec validate make-doctor-x11-only-no-wayland-portal-noise --type change --strict`: passed.
  - `openspec validate --all --strict`: passed, 20/20 items.
  - First `make fmt`: failed on formatting in `src/doctor.rs`; ran `cargo fmt`.
  - Final `make fmt`: passed.
  - `make check`: passed.
  - `make test`: passed.
  - Local live doctor smoke: `./target/debug/codex-computer-use-x11 doctor --json` emitted valid JSON with `backend=x11-ewmh`, `readiness.ok=true`, `blockers=[]`, `degraded_reasons=[]`, `recommended_next_step=Ready for fixture-backed X11 doctor follow-up checks`, and no forbidden RemoteDesktop/Wayland readiness strings.
  - Secret safety: `.secrets.local.env` was not read or printed; no secret values were staged.

## TDD Exceptions

None.
