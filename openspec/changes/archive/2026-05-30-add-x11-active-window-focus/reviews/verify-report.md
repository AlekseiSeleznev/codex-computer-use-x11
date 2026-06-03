# Verify Report: add-x11-active-window-focus

Date: 2026-05-30

## Summary

Result: PASS — implementation is verify-ready and archive-eligible.

## Completeness

- Tasks: 12/12 complete in `tasks.md`.
- OpenSpec artifacts: proposal, specs, grill, design, design-review, adr, test-plan, and tasks are complete.
- Evidence log: `test-plan.md` records RED/GREEN evidence for all parser, CLI, focus verification, fallback, final checks, and live smoke slices.

## Correctness

Implemented requirements from `specs/x11-active-window-focus/spec.md`:

- `focused-window --json` emits one JSON object with `project`, `version`, `backend`, `focused_window`, and focus diagnostics.
- Focused-window matching uses `wmctrl -lpGx` plus `_NET_ACTIVE_WINDOW`; it returns a matched `WindowInfo` with `focused: true` when the active id is listed.
- Explicit no-active (`0x0`) and active-not-in-list cases degrade with machine-readable diagnostics while preserving JSON output.
- `focus-window --window-id <id> --json` parses shared X11 ids including decimal CLI ids and `0x`/padded hex ids.
- Invalid ids fail before activation with stderr; missing listed windows return JSON `WindowNotFound` and no activation attempt.
- Verified focus activation uses `wmctrl -ia 0x<id>` first, then a fresh `_NET_ACTIVE_WINDOW` lookup; success requires exact id equality.
- Activation success with active-id mismatch returns non-zero JSON `FocusNotVerified` and explains the observed active id vs requested id.
- `xdotool windowactivate --sync <decimal-id>` fallback is attempted after a failed/unverified primary activation and still requires fresh verification.
- `README.md` documents the new command surface and safety boundary.

## Coherence

- Design followed: standalone `src/focus.rs`, shared listing and id normalizer reuse, no target checkout mutation, and no direct targeted input behavior.
- Grill/design-review findings resolved: active-window source is `_NET_ACTIVE_WINDOW`, command exit status is advisory, fallback does not weaken verification, and returned `focused_window.focused` is normalized from fresh verification.
- ADR review followed: no durable ADR or `ARCHITECTURE.md` update needed.
- Constitution followed: Rust 2021 root crate, Cargo/Makefile checks, no secrets, no external credentialed systems, and target checkout read-only.

## Verification Commands

- `git status --short` before verify — clean.
- `openspec status --change add-x11-active-window-focus` — 8/8 artifacts complete.
- `openspec validate add-x11-active-window-focus --type change --strict` — PASS.
- `make fmt` — PASS.
- `make check` — PASS.
- `make test` — PASS: 40 library tests, 2 `doctor_cli` tests, 8 `focus_cli` tests, 3 `list_windows_cli` tests, and doc tests.
- `cargo run --quiet -- focused-window --json` — PASS, valid JSON; local smoke focused/current active id `65011716`, diagnostics ok `true`.
- `cargo run --quiet -- focus-window --window-id 65011716 --json` — PASS, valid JSON; `success=true`, `exact_window_focused=true`, requested/focused id `65011716`, activation attempts `['wmctrl']`.
- Target checkout status at `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` — clean after read-only research/smoke.
- Project `git status --short` after verify commands — clean before writing this verify report.

## Issues

- Critical: None.
- Warnings: None.
- Suggestions: None.
