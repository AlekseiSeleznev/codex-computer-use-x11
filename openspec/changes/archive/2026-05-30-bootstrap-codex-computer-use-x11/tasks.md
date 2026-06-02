<!-- Implementation checklist for `bootstrap-codex-computer-use-x11`.
Keep real secret values out of tracked files. Do not read `.secrets.local.env`.
Do not modify `${CODEX_DESKTOP_LINUX_FULL_PATH}` or the documented target checkout.
Apply must follow `test-plan.md`: one RED -> GREEN -> REFACTOR slice at a time, with evidence recorded before checking tasks complete. -->

## 1. Apply preflight and TDD evidence discipline

- [x] 1.1 Run project preflight before implementation: confirm `git status --short` is clean, read `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and this change's proposal/specs/grill/design/design-review/adr/test-plan/tasks.
- [x] 1.2 Confirm no external credentials are needed and do not read `.secrets.local.env`; verify implementation scope is repository-local and excludes `${CODEX_DESKTOP_LINUX_FULL_PATH}` writes.
- [x] 1.3 Prepare to record RED/GREEN/REFACTOR evidence in `openspec/changes/bootstrap-codex-computer-use-x11/test-plan.md` for each slice before marking the related task complete.
- [x] 1.4 If any setup beyond minimal harness/scaffold is needed before Slice 1 RED, record an explicit TDD exception in `test-plan.md` before writing production behavior.

## 2. Slice 1 — root Rust package identity

- [x] 2.1 RED: run `cargo metadata --no-deps --format-version 1` from the repository root, or an equivalent shell assertion, and record the expected failure because root package metadata is absent.
- [x] 2.2 GREEN: add minimal root `Cargo.toml` declaring package `codex-computer-use-x11`, version `0.1.0`, edition `2021`, and add minimal `src/lib.rs` without extra behavior.
- [x] 2.3 GREEN evidence: rerun `cargo metadata --no-deps --format-version 1` and assert package name, version, and edition match the design/test-plan.
- [x] 2.4 REFACTOR guard: keep a single root package, no subcrates/workspace complexity, no target checkout writes, and record Slice 1 evidence in `test-plan.md`.

## 3. Slices 2–3 — X11 window-id normalizer

- [x] 3.1 RED Slice 2: add one inline `#[cfg(test)]` unit test in `src/x11_id.rs` proving `parse_x11_window_id("0x5624b36") == parse_x11_window_id("0x05624b36")`; run `cargo test x11_id --lib` and record the expected failure.
- [x] 3.2 GREEN Slice 2: implement pure `parse_x11_window_id(input: &str) -> Result<u64, ParseX11WindowIdError>` and expose `pub mod x11_id;` from `src/lib.rs`; rerun `cargo test x11_id --lib` and record pass.
- [x] 3.3 RED Slice 3: add focused inline unit tests for empty input and invalid hex returning `ParseX11WindowIdError::{Empty, InvalidHex}`; run `cargo test x11_id --lib` and record expected failure.
- [x] 3.4 GREEN Slice 3: extend the parser and small error enum minimally; rerun `cargo test x11_id --lib` and record pass.
- [x] 3.5 REFACTOR guard: keep parser numeric-only and independent from command formatting for `wmctrl`, `xprop`, `xdotool`, or live X11 behavior.

## 4. Slices 4–5 — doctor CLI JSON and argument behavior

- [x] 4.1 RED Slice 4: add `tests/doctor_cli.rs` success test invoking `Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11")).args(["doctor", "--json"])`; assert exit `0`, empty stderr, parseable compact JSON, `project = "codex-computer-use-x11"`, `version = env!("CARGO_PKG_VERSION")`, `backend = "x11-ewmh"`, `readiness.ok = true`, empty blockers, `implemented` contains `doctor-json`, `planned == ["x11-ewmh-windowing"]`, and checks `bootstrap-project`, `backend-identity`, `no-live-x11-probes` are present with `ok = true` and the no-live detail includes `stage 01 performs no live X11 probes or external command execution`; run `cargo test doctor_cli_success --test doctor_cli` and record expected failure.
- [x] 4.2 GREEN Slice 4: add minimal `serde` with `derive` and `serde_json` dependencies; implement `src/doctor.rs` with owned-string report structs and bootstrap report values; implement `src/main.rs` success path for `doctor --json` using `serde_json::to_string` plus one newline.
- [x] 4.3 GREEN evidence Slice 4: rerun `cargo test doctor_cli_success --test doctor_cli` and record pass; verify no `.secrets.local.env`, live X11 tools, or target checkout paths are accessed.
- [x] 4.4 RED Slice 5: extend `tests/doctor_cli.rs` for `--help` and `-h` success usage output plus unknown command and `doctor` without `--json` non-zero stderr; run `cargo test doctor_cli_arguments --test doctor_cli` and record expected failure.
- [x] 4.5 GREEN Slice 5: extend `src/main.rs` argument handling with simple `std::env::args()` logic only; rerun `cargo test doctor_cli_arguments --test doctor_cli` and record pass.
- [x] 4.6 REFACTOR guard: keep CLI surface limited to `doctor --json`, `--help`, and error handling; do not add `clap`, `anyhow`, `thiserror`, `rmcp`, `tokio`, `x11rb`, command execution, or broader behavior.

## 5. Slice 6 — Makefile verification wrappers

- [x] 5.1 RED: run `make fmt`, `make check`, and `make test` before adding root Makefile targets; record expected missing-target/Makefile failure as wiring verification, not behavioral coverage.
- [x] 5.2 GREEN: add root `Makefile` targets mapping `fmt` to `cargo fmt -- --check`, `check` to `cargo check`, and `test` to bare `cargo test` with no default `-- --nocapture`; run `cargo fmt` in reformatting mode before using `make fmt` as a check.
- [x] 5.3 GREEN evidence: after `cargo fmt`, run actual `make fmt`, `make check`, and `make test`; record pass. Use `make -n` only as optional target-wiring smoke evidence.
- [x] 5.4 REFACTOR guard: keep `make fmt` checking format without rewriting files in place, and keep wrappers thin over Cargo.

## 6. Slices 7–8 — README and integration contract documentation

- [x] 6.1 RED Slice 7: run the deterministic README text-check snippet from `test-plan.md`; record expected failure before `README.md` exists or before required posture text is present.
- [x] 6.2 GREEN Slice 7: add root `README.md` summarizing Codex-first, Cinnamon/X11-first, generic X11/EWMH, `x11-ewmh`, standalone plugin, future source overlay, root commands, no live backend in stage 01, no formal MSRV beyond stable Rust 2021 support, and link to `docs/integration-contract.md`.
- [x] 6.3 GREEN evidence Slice 7: rerun README text-check snippet and record pass.
- [x] 6.4 RED Slice 8: run the deterministic integration-contract text-check snippet from `test-plan.md`; record expected failure before `docs/integration-contract.md` exists or before required contract text is present.
- [x] 6.5 GREEN Slice 8: add `docs/integration-contract.md` as the normative future integration document with `x11-ewmh`, upstream `WindowInfo` primary model, sidecar/report default, non-implemented `WindowObservationMeta` sketch, late fallback order after GNOME extension/GNOME introspect/COSMIC/KWin/Hyprland/i3, `CODEX_DESKTOP_LINUX_FULL_PATH`, and reference-first/license policy.
- [x] 6.6 GREEN evidence Slice 8: rerun integration-contract text-check snippet and record pass.
- [x] 6.7 REFACTOR guard: keep docs non-secret, avoid claiming live backend support, avoid duplicating normative contract content in README, and do not copy external project code or license text beyond reference-first documentation.

## 7. Slice 9 — aggregate verification and apply readiness

- [x] 7.1 Run and record aggregate code checks: `cargo metadata --no-deps --format-version 1`, `cargo test`, `cargo test x11_id --lib`, `cargo test --test doctor_cli`, `make fmt`, `make check`, and `make test`.
- [x] 7.2 Run and record `codex-computer-use-x11 doctor --json` as a built binary or Cargo-run equivalent; parse stdout as JSON and confirm it satisfies the `doctor-cli` spec and exact `capabilities.planned == ["x11-ewmh-windowing"]` assertion.
- [x] 7.3 Rerun and record README and integration-contract documentation checks from `test-plan.md`; treat them as gate-weight equal to cargo/make checks.
- [x] 7.4 Run and record `openspec validate bootstrap-codex-computer-use-x11 --type change --json` (this invocation has been used successfully during planning in this repository; if the local CLI changes, run `openspec validate --help` and report the exact blocker rather than silently substituting an unrecorded command).
- [x] 7.5 Run and record `git status --short` with manual visual inspection for unrelated dirty work, plus `! git ls-files --error-unmatch .secrets.local.env` or equivalent deterministic check proving the local secret file is not tracked.
- [x] 7.6 Update `test-plan.md` Evidence Log with final RED/GREEN/REFACTOR evidence references for all slices; do not mark tasks complete without evidence or an explicit approved TDD exception.
- [x] 7.7 Confirm no files under `${CODEX_DESKTOP_LINUX_FULL_PATH}` or its documented development-machine default were modified by this change.


## Claude Tasks Review Disposition

The Claude review for this `tasks` stage returned `pass`, no `mustFix`, two actionable `shouldFix` items, and one context-answerable question. No user participation is required.

- Rust integration test invocation is now explicit as `.args(["doctor", "--json"])`, so apply should not pass a shell-style single string.
- Makefile GREEN steps now require running `cargo fmt` before `make fmt`, because `make fmt` is a check-only wrapper.
- The OpenSpec validation command `openspec validate bootstrap-codex-computer-use-x11 --type change --json` has been used successfully during planning; if a future local CLI changes, apply must inspect `openspec validate --help` and report the blocker rather than silently changing the required check.

## 8. Checkpoint and handoff

- [x] 8.1 Show `git status --short` before checkpointing the completed apply task group.
- [x] 8.2 Checkpoint the coherent implementation group with changed source/docs/tests and updated `test-plan.md` evidence; do not include unrelated dirty work or local secret files.
- [x] 8.3 Report verification results, any limitations, commit hash, and whether the change is ready for `/opsx:verify`.
