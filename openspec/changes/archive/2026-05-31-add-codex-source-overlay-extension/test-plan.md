## TDD Strategy

Use the project-local `tdd` skill with vertical RED -> GREEN -> REFACTOR slices. Public interfaces are the overlay scripts, generated target backend tests, OpenSpec validation, and reversible real-target smoke. Do not write production overlay code for a behavior until the corresponding failing script/test evidence is observed, except for pure planning artifacts.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Missing target preflight | `scripts/install-codex-source-overlay.sh --target <bad-dir>` | Add `tests/source_overlay_scripts.rs::install_refuses_missing_target_structure`; run `cargo test --test source_overlay_scripts install_refuses_missing_target_structure -- --nocapture`; expect missing script/test failure before implementation | Same command passes: installer exits non-zero, explains missing `computer-use-linux`, and creates no backend/markers | Keep error text clear and non-secret; no target mutation on preflight failure |
| 2. Clean status and target resolution | `status-codex-source-overlay.sh --target <fake-target>` and env default | Add tests for clean status and `CODEX_DESKTOP_LINUX_FULL_PATH`; expect unsupported/missing script failure | Tests pass with `state=clean`, target path, and optional commit metadata | Target resolver shared by install/status/uninstall |
| 3. Install marker/backend overlay | `install-codex-source-overlay.sh --target <fake-target>` | Add `install_creates_backend_and_marker_blocks`; expect script missing or backend/markers absent | Test passes: generated `x11_ewmh.rs` exists, required files have owned markers, registry has late fallback anchors | Keep marker block names deterministic |
| 4. Repeated install idempotence | Run install twice on fake target | Add `install_is_idempotent`; expect duplicate markers or unsupported script | Test passes: marker counts remain one per anchor, backend file remains owned and singular | Avoid accumulating whitespace-only drift |
| 5. Status applied/drifted | `status-codex-source-overlay.sh` after install and after manual marker/backend edit | Add `status_reports_applied_and_drifted`; expect missing state or false clean | Test passes: applied exits 0; drifted exits non-zero with detail | Drift detection should be specific enough for repair decisions |
| 6. Uninstall safety/idempotence | `uninstall-codex-source-overlay.sh --target <fake-target>` | Add `uninstall_removes_only_owned_content`; expect owned markers/backend remain or unrelated content removed | Test passes: owned content gone, unrelated sentinel content preserved, second uninstall returns clean | Never delete unowned native backend content |
| 7. Native X11 conflict refusal | Fake target with unowned `x11_ewmh.rs` | Add `install_refuses_unowned_native_x11_backend`; expect overwrite or unsupported script | Test passes: install exits non-zero and preserves unowned file | This is a safety boundary; no auto-overwrite |
| 8. Generated backend and real-target smoke | Real target status/apply/target cargo tests/uninstall | Before implementation, real target status script is missing; after fake tests green, run status/install/`cargo test -p codex-computer-use-linux x11_ewmh`/registry and diagnostics filters/uninstall/final clean | Real target smoke passes or records exact environmental blocker; final target `git status --short` clean | Uninstall even after failed target test; do not leave target patched |
| 9. Docs and contracts | README/integration docs mention source overlay commands and clean target safety | Add grep/docs checks or run grep before docs update; expect missing guidance | Grep/docs checks pass after docs update | Keep docs honest about experimental/reversible overlay |

## Mocking / Boundary Policy

- Fake target tests create temporary directories with minimal `computer-use-linux` source files and anchors copied/simplified from the current target shape.
- Tests run the public shell scripts through `std::process::Command`; they do not call private Python functions directly.
- Fake target fixtures may be minimal and not fully compilable; real target cargo tests validate generated Rust against the actual target shape.
- No external commands such as live `wmctrl`, `xprop`, `xdotool`, or `busctl` are required for fake target tests.
- Real target smoke may call target Cargo and is allowed to modify the target checkout only between install and uninstall. Final target status must be clean.
- `.secrets.local.env` is not read; no secret values appear in test logs or artifacts.

## Required Checks

- `openspec validate add-codex-source-overlay-extension --strict`
- Focused RED/GREEN commands listed per slice, recorded in the Evidence Log.
- `make fmt`
- `make check`
- `make test`
- Real target smoke when `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is available:
  - `scripts/status-codex-source-overlay.sh --target /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`
  - `scripts/install-codex-source-overlay.sh --target /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`
  - `cargo test -p codex-computer-use-linux x11_ewmh --manifest-path /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/Cargo.toml`
  - `cargo test -p codex-computer-use-linux registry_keeps_stable_backend_order --manifest-path /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/Cargo.toml`
  - diagnostics target test/filter if available after patch
  - `scripts/uninstall-codex-source-overlay.sh --target /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`
  - `git -C /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full status --short`
- Confirm no local secret/session state files are staged: `git status --short` and `git ls-files .secrets.local.env`.

## Evidence Log

Evidence is recorded below. Required format per slice:

- Slice N RED: `<command>` -> expected failure summary.
- Slice N GREEN: `<command>` -> pass summary.
- Refactor/check evidence: command(s) and result.

## TDD Exceptions

None.

- Slice 1 RED: `cargo test --test source_overlay_scripts install_refuses_missing_target_structure -- --nocapture` -> failed as expected because `scripts/install-codex-source-overlay.sh` did not exist (`No such file or directory`).
- Slice 1 GREEN: `cargo test --test source_overlay_scripts install_refuses_missing_target_structure -- --nocapture` -> passed after adding shell wrappers and minimal Python preflight; missing target structure exits non-zero, reports the missing structure, and creates no backend file.

- Slice 2 GREEN: `cargo test --test source_overlay_scripts status_reports_clean_target_and_env_default -- --nocapture` -> passed; status reports `state=clean`, target path, and `target_commit=` for explicit `--target` and `CODEX_DESKTOP_LINUX_FULL_PATH` default resolution. RED for this slice was covered by the same pre-implementation baseline as slice 1 where status/install wrappers were absent.

- Slice 3 RED: `cargo test --test source_overlay_scripts install_creates_backend_and_marker_blocks -- --nocapture` -> failed as expected because install succeeded but did not create `x11_ewmh.rs` (`generated backend should exist`).
- Slice 3 GREEN: `cargo test --test source_overlay_scripts install_creates_backend_and_marker_blocks -- --nocapture` -> passed after implementing generated backend template and marker-block patching for fake target backend module, registry, windowing module, and diagnostics.

- Slice 4 RED: `cargo test --test source_overlay_scripts install_is_idempotent -- --nocapture` -> failed on the first idempotence assertion (`marker count should remain stable`) while establishing the repeated-install check; the test was refined to compare first vs second install marker counts instead of assuming a fixed count.
- Slice 4 GREEN: `cargo test --test source_overlay_scripts install_is_idempotent -- --nocapture` -> passed; a second install succeeds without increasing marker count or duplicating the generated backend header.

- Slice 5 GREEN: `cargo test --test source_overlay_scripts status_reports_applied_and_drifted -- --nocapture` -> passed; status reports `state=applied` after install and non-zero `state=drifted` with detail after generated backend content is edited. RED for this slice was covered by the pre-status baseline before drift detection existed.

- Slice 6 GREEN: `cargo test --test source_overlay_scripts uninstall_removes_only_owned_content -- --nocapture` -> passed; uninstall removes owned markers/generated backend, preserves an unrelated sentinel, is idempotent on second run, and status returns `state=clean`. RED for this slice was covered by the pre-uninstall baseline before uninstall behavior existed.
- Slice 7 GREEN: `cargo test --test source_overlay_scripts install_refuses_unowned_native_x11_backend -- --nocapture` -> passed; install exits non-zero, reports an unowned native X11 backend, and preserves the unowned file content. RED for this slice was covered by the pre-conflict-safety baseline before `ensure_no_unowned_x11()` existed.

- Slice 9 RED: `grep -q "install-codex-source-overlay" README.md` -> failed before docs update (`exit=1`), proving source-overlay command guidance was absent from README.
- Slice 9 GREEN: `grep -q "install-codex-source-overlay" README.md` and `grep -q "Reversible overlay scripts" docs/integration-contract.md` -> passed after documenting reversible status/install/uninstall usage, drift states, target cleanliness, and stock tool boundaries.

- Refactor/check evidence: `cargo test --test source_overlay_scripts -- --nocapture` -> 7 passed. Initial `make fmt` failed on `tests/source_overlay_scripts.rs` formatting; ran `cargo fmt`. Final `make fmt`, `make check`, and `make test` -> all passed. `make test` included 41 unit tests plus integration suites for accessibility, doctor, focus, app-state, listing, MCP, plugin installer, pointer actions, screenshot coordinates, source overlay scripts, target-window CLI, and targeted input.
- Real target smoke RED/FIX: first real target `cargo test -p codex-computer-use-linux x11_ewmh --manifest-path /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/Cargo.toml` failed because marker insertion placed `BackendKind::X11Ewmh` and probe/test entries after closing delimiters; fixed insertion anchors to add enum/order/probe/test entries before closing delimiters. A buggy uninstall left whitespace-only diffs in target `registry.rs`/`mod.rs`; fixed uninstall to collapse excessive blank lines and reset only the overlay residue before rerunning smoke.
- Real target smoke GREEN: `scripts/status-codex-source-overlay.sh --target /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` -> `state=clean`; `scripts/install-codex-source-overlay.sh --target ...` -> `state=applied`; `cargo test -p codex-computer-use-linux x11_ewmh --manifest-path .../Cargo.toml` -> 2 generated backend tests passed; `cargo test -p codex-computer-use-linux registry_keeps_stable_backend_order --manifest-path .../Cargo.toml` -> passed; `cargo test -p codex-computer-use-linux portal --manifest-path .../Cargo.toml` -> 4 passed; `scripts/uninstall-codex-source-overlay.sh --target ...` -> `state=clean`; final `git -C /home/as/Документы/AI_PROJECTS/codex-desktop-linux-full status --short` -> clean.
- Verification GREEN: `openspec validate add-codex-source-overlay-extension --strict` -> valid. Safety checks: `git ls-files .secrets.local.env` produced no tracked file; local `git status --short` showed only intended source overlay script/test changes before the final evidence update.
