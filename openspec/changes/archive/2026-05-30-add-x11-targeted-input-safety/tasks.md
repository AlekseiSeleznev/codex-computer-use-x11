## 1. CLI Target Resolution and Safe Reports

- [x] 1.1 Add public integration tests for missing, stale, and ambiguous targets proving no activation/input commands run.
- [x] 1.2 Add standalone target resolution by `window_id`, title substring, `wm_class`, and pid with candidate diagnostics.
- [x] 1.3 Add `TargetedInputReport`, diagnostics, keyboard attempt, and stable `success` / `input_sent` / `error_code` fields.
- [x] 1.4 Wire `type-text` and `press-key` CLI parsing with `--json`, required payload validation, and unsupported usage errors.

## 2. Verified-Focus Keyboard Backend

- [x] 2.1 Add RED test for focus mismatch blocking `type-text` without invoking the keyboard backend.
- [x] 2.2 Add focus-gated `type-text` implementation using active-context `xdotool type --clearmodifiers` only after exact focus verification.
- [x] 2.3 Add RED/GREEN test and implementation for `press-key` using active-context `xdotool key --clearmodifiers` after exact focus verification.
- [x] 2.4 Add missing-backend safe failure with `InputBackendUnavailable` and no `input_sent` claim.

## 3. MCP Tool Surface

- [x] 3.1 Update MCP `tools/list` to expose `x11_type_text` and `x11_press_key` in deterministic project-owned order.
- [x] 3.2 Add MCP schemas and runtime validation for text/key plus target selectors.
- [x] 3.3 Add MCP tests proving missing target returns `isError: true` with JSON report and focus failures remain tool errors.

## 4. Evidence, Docs, and Verification

- [x] 4.1 Record RED/GREEN evidence in `test-plan.md` after each implemented slice.
- [x] 4.2 Record Cyrillic and non-BMP/emoji fake-backend and live/degraded behavior evidence.
- [x] 4.3 Run `openspec validate add-x11-targeted-input-safety --strict`, `make fmt`, `make check`, and `make test`.
- [x] 4.4 Confirm project git status is clean before archive and that the target checkout was not modified.
