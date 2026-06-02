## 1. CLI Safety Gates and Report Shape

- [x] 1.1 Add RED CLI test proving targeted `click` refuses out-of-bounds coordinates before focus or pointer input.
- [x] 1.2 Expose shared target resolution from the existing targeted-input module without changing keyboard report compatibility.
- [x] 1.3 Add `src/pointer.rs` report types, point/bounds validation, missing-target/global mode safety, and stable `success` / `input_sent` / `targeted` / `verification_mode` / `error_code` fields.
- [x] 1.4 Wire `click` CLI parsing with target/global selectors, coordinates, button/count validation, `--json`, and unsupported-usage errors.

## 2. Verified-Focus Pointer Backend

- [x] 2.1 Add RED/GREEN tests proving targeted click runs active-context `xdotool` only after bounds and exact focus verification.
- [x] 2.2 Add RED/GREEN test proving focus mismatch blocks pointer backend invocation with `FocusNotVerified`.
- [x] 2.3 Implement active-context `xdotool` click backend with no `--window` direct events and `InputBackendUnavailable` safe failure.
- [x] 2.4 Add RED/GREEN scroll tests and implementation for direction-to-wheel mapping, finite amount clamping, and bounds/focus gates.
- [x] 2.5 Add RED/GREEN drag tests and implementation for finite down/move/up sequence, endpoint bounds validation, and huge-drag refusal.
- [x] 2.6 Add RED/GREEN global mode test and implementation proving `--global` is explicitly `global_unverified` and no-target without `--global` is refused.

## 3. MCP Tool Surface

- [x] 3.1 Update MCP `tools/list` to expose `x11_click`, `x11_scroll`, and `x11_drag` after keyboard tools in deterministic project-owned order.
- [x] 3.2 Add MCP schemas and runtime validation for pointer coordinates, target selectors, `global`, button, direction, count, and amount.
- [x] 3.3 Add MCP tests proving missing target returns `isError: true` with JSON `MissingTarget`, and pointer failures remain tool errors.

## 4. Evidence, Docs, and Verification

- [x] 4.1 Record RED/GREEN evidence in `test-plan.md` after each implemented slice.
- [x] 4.2 Run fake-command focused tests: `cargo test --test pointer_actions_cli` and `cargo test --test mcp_server`.
- [x] 4.3 Run `openspec validate add-x11-pointer-actions --strict`, `make fmt`, `make check`, and `make test`.
- [x] 4.4 Run safe live/degraded Cinnamon/X11 smoke for click, scroll, drag, and at least one refusal case, recording exact evidence or blocker.
- [x] 4.5 Confirm project git status is clean before archive and the target checkout was not modified.
