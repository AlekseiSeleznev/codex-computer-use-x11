## 1. Apply preflight and RED/GREEN setup

- [x] 1.1 Re-read `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, this change's artifacts, and `openspec instructions apply --change add-x11-active-window-focus --json` before production edits.
- [x] 1.2 Run `openspec validate add-x11-active-window-focus --type change --strict` and confirm planning artifacts are committed before implementation starts.

## 2. Active-window parsing and focused-window CLI

- [x] 2.1 RED/GREEN slice 1: add and satisfy the pure active-window parser test for active id, explicit no-active, missing property, and invalid text; record evidence in `test-plan.md`.
- [x] 2.2 RED/GREEN slice 2: add and satisfy the public CLI fake-command test for `focused-window --json` returning a matched focused `WindowInfo`; record evidence in `test-plan.md`.
- [x] 2.3 RED/GREEN slice 3: add and satisfy focused-window no-active and active-not-in-list degradation tests; record evidence in `test-plan.md`.

## 3. Focus-window target resolution and activation verification

- [x] 3.1 RED/GREEN slice 4: add and satisfy tests proving invalid ids and missing windows fail before activation attempts; record evidence in `test-plan.md`.
- [x] 3.2 RED/GREEN slice 5: add and satisfy the `wmctrl -ia` activation plus fresh `_NET_ACTIVE_WINDOW` verification success test; record evidence in `test-plan.md`.
- [x] 3.3 RED/GREEN slice 6: add and satisfy the `FocusNotVerified` mismatch test; record evidence in `test-plan.md`.
- [x] 3.4 RED/GREEN slice 7: add and satisfy the `xdotool windowactivate --sync` fallback test with ordered diagnostics; record evidence in `test-plan.md`.

## 4. Documentation, smoke checks, and archive readiness

- [x] 4.1 Update `README.md` command documentation and any relevant user-facing notes for focused/focus commands without changing secret or target-checkout rules.
- [x] 4.2 Run required final checks: `make fmt`, `make check`, `make test`, `openspec validate add-x11-active-window-focus --type change --strict`, and JSON smoke commands; record evidence in `test-plan.md`.
- [x] 4.3 Verify the target checkout remains unmodified, update this task list to checked only after evidence exists, and leave git status ready for verify/archive.
