## TDD Strategy

Use the project-local `tdd` skill with small vertical slices. Each slice starts with one observable behavior test through the public CLI or MCP stdio interface, confirms RED, implements the minimal code for GREEN, then refactors only while green. External X11 commands are simulated with fake `PATH` executables for deterministic tests; live Cinnamon/X11 smoke runs only after unit/CLI/MCP tests pass.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | CLI rejects missing/ambiguous/stale targets without activation or input | `cargo test --test targeted_input_cli targeted_input_refuses_ambiguous_title_without_commands` fails because `type-text` and target resolution do not exist | Same test passes; JSON has `error_code=AmbiguousTarget`, `input_sent=false`, candidate ids, and empty activation/input log | Target resolution code is shared by type/key, no duplicated parser branches |
| 2 | CLI refuses to type when focus verification mismatches | `cargo test --test targeted_input_cli targeted_type_text_does_not_invoke_xdotool_when_focus_unverified` fails because no targeted input pipeline exists | Same test passes; focus attempts may be logged but `xdotool type` is absent and JSON has `FocusNotVerified` | Failure report shape remains stable and deterministic |
| 3 | CLI type-text sends active-context xdotool after verified focus | `cargo test --test targeted_input_cli targeted_type_text_invokes_active_context_xdotool_after_verified_focus` fails because `type-text` is unsupported | Same test passes; log order is focus command then `xdotool type --clearmodifiers`, with no `--window` | Keyboard backend wrapper keeps command construction isolated |
| 4 | CLI press-key sends active-context xdotool after verified focus | `cargo test --test targeted_input_cli targeted_press_key_invokes_active_context_xdotool_after_verified_focus` fails because `press-key` is unsupported | Same test passes; log contains `xdotool key --clearmodifiers Enter` after focus verification | Type and key share report/focus plumbing without hiding action-specific details |
| 5 | MCP tools list/call wraps targeted input reports | `cargo test --test mcp_server mcp_server_lists_and_refuses_targeted_input_tools` fails because MCP tools are absent | Same test passes; tools list includes six `x11_*` tools and missing target returns `isError=true` with JSON report | MCP schemas stay hand-written but validation is delegated to report builders |
| 6 | Unicode/layout evidence is recorded | `cargo test --test targeted_input_cli targeted_type_text_preserves_cyrillic_and_emoji_argument_after_verified_focus` fails until fake backend evidence covers literal argument flow | Same test passes; fake command log preserves Cyrillic and emoji argument; live smoke records actual behavior or degraded limitation | Do not claim live Unicode correctness unless live command proves it |

## Mocking / Boundary Policy

- Use fake executable scripts in a temporary `PATH` for `wmctrl`, `xprop`, and `xdotool` to verify observable CLI behavior without live X11 side effects.
- Do not mock internal Rust collaborators; tests run the compiled binary through its public CLI/MCP interfaces.
- Parser-only helper functions may have unit tests if introduced, but acceptance relies on CLI/MCP integration tests.
- Live smoke may use real `wmctrl`, `xprop`, and `xdotool` only after fake tests are green and only against a verified target window.

## Required Checks

- `openspec validate add-x11-targeted-input-safety --strict`
- `make fmt`
- `make check`
- `make test`
- Targeted fake-command tests:
  - `cargo test --test targeted_input_cli`
  - `cargo test --test mcp_server`
- Live/degraded smoke:
  - `cargo run -- type-text --window-id <verified-safe-window> --text <sample> --json` or a documented refusal/degraded reason if no safe live target is available.
  - Evidence must include Cyrillic and non-BMP/emoji behavior or an explicit limitation.
- Git status check to ensure `.secrets.local.env` and target checkout files are not staged or modified.

## Evidence Log

- Slice 1 RED: `cargo test --test targeted_input_cli targeted_input_refuses_ambiguous_title_without_commands` failed with unsupported `type-text` command before targeted input CLI existed.
- Slice 1 GREEN: same command passed after adding `WindowTarget`, target resolution, `TargetedInputReport`, and `type-text` JSON wiring; ambiguous title returns `AmbiguousTarget`, `input_sent: false`, and no activation/input log.
- Slice 2 RED: `cargo test --test targeted_input_cli targeted_type_text_does_not_invoke_xdotool_when_focus_unverified` failed with `InputBackendUnavailable` before focus verification was wired.
- Slice 2 GREEN: same command passed after calling `focus_window_report_from_listing()` and returning `FocusNotVerified` with no `xdotool type` invocation.
- Slice 3 RED: `cargo test --test targeted_input_cli targeted_type_text_invokes_active_context_xdotool_after_verified_focus` failed because verified focus still returned non-success before the keyboard backend existed.
- Slice 3 GREEN: same command passed after adding active-context `xdotool type --clearmodifiers` with `used_direct_window: false`.
- Slice 4 GREEN: `cargo test --test targeted_input_cli targeted_press_key_invokes_active_context_xdotool_after_verified_focus` passed through the shared focus-gated keyboard backend and verifies `xdotool key --clearmodifiers Enter`; the pre-implementation state for this public command was unsupported before the shared backend was added.
- Slice 5 RED: `cargo test --test mcp_server` failed because `tools/list` still exposed four tools and `x11_type_text` returned unsupported/plain text.
- Slice 5 GREEN: `cargo test --test mcp_server` passed after adding `x11_type_text` / `x11_press_key` schemas and report-builder calls; missing target returns MCP `isError: true` with JSON `MissingTarget`.
- Slice 6 GREEN: `cargo test --test targeted_input_cli targeted_type_text_preserves_cyrillic_and_emoji_argument_after_verified_focus` passed; fake backend log and JSON `keyboard.args[2]` preserve `Привет 🌍` after verified focus.
- Missing backend GREEN: `cargo test --test targeted_input_cli targeted_type_text_reports_missing_keyboard_backend_after_verified_focus` passed; with fake `PATH` lacking `xdotool`, focus verifies but input is not sent and `error_code` is `InputBackendUnavailable`.
- Full project GREEN: `make fmt && make check && make test` passed; unit + integration total includes new `targeted_input_cli` and updated `mcp_server` tests.
- Live/degraded smoke: disposable `xterm` target was resolved and focus-verified; `type-text` with `CodexSmoke-Привет-🌍\n` returned `success: true` and active-context xdotool args, but the xterm capture file was not produced. Treat live text delivery/Unicode correctness as degraded/unproven on this desktop even though fake tests prove command argument preservation; no target checkout files were modified.

## TDD Exceptions

None
