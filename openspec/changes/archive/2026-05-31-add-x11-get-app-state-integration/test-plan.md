## TDD Strategy

Apply the project-local `tdd` skill with vertical slices only: one public behavior test/check goes RED, minimal production code makes it GREEN, then refactor while GREEN. Tests must use public CLI or MCP surfaces for behavior-changing work. Pure parser/helper tests are allowed only for external boundary parsing such as PNG dimensions or DBus introspection text.

Primary fake boundaries:

- Fake `PATH` commands for `wmctrl`, `xprop`, `gdbus`, `busctl`, `python3`, and command availability.
- Tiny fake PNG files for screenshot data-url tests.
- JSON-RPC stdio interactions for MCP tests.

No production code may be written for a behavior before its RED evidence is recorded here.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | CLI `get-app-state --window-id 0x2 --no-screenshot --json` resolves `window_context` and no `window_error` | Add one test in new `tests/get_app_state_cli.rs`, then run `cargo test --test get_app_state_cli resolves_window_context_by_window_id`; expected failure: unsupported command or missing app-state fields | Same command passes with minimal `app_state` module, CLI parser, target resolution, and no-screenshot report | Keep composition shallow; no duplicate wmctrl parser |
| 2 | CLI ambiguous title returns `window_error` and no arbitrary context | Add focused ambiguous title test, run `cargo test --test get_app_state_cli refuses_ambiguous_title_without_random_context`; expected failure: no command or wrong target behavior | Same command passes after app-state maps `input::ResolveError` into `window_error` and candidate diagnostics | Preserve exit 0 for serializable layer-degraded report |
| 3 | CLI missing target window still returns screenshot when provider succeeds | Add fake `gdbus` test writing tiny PNG, run `cargo test --test get_app_state_cli keeps_screenshot_when_window_target_missing`; expected failure: no screenshot/data_url or unsupported command | Same command passes with screenshot provider, PNG dimensions, data URL, cleanup, and `window_error` | Screenshot temp paths are removed on success/failure |
| 4 | CLI `--no-screenshot` / screenshot failure layer semantics | Add test for `--no-screenshot` and provider unavailable/failure, run focused tests; expected failure: screenshot layer not controllable | Focused tests pass with `screenshot`/`screenshot_error` semantics | Avoid full-report nonzero for layer errors |
| 5 | CLI app-state includes matched AT-SPI tree through existing correlation | Add fake `python3` collector test, run `cargo test --test get_app_state_cli includes_matched_accessibility_tree`; expected failure: no accessibility composition | Same command passes by reusing `accessibility_tree_report_from_system` and copying tree/correlation diagnostics | Do not duplicate matcher logic |
| 6 | CLI app-state keeps window/screenshot when AT-SPI is ambiguous/unavailable | Add fake `python3` ambiguous/unavailable tests, run focused tests; expected failure: whole report fails or tree is arbitrary | Focused tests pass with empty tree and `accessibility_error` | Preserve layer-degraded app-state vocabulary |
| 7 | Doctor live probes gather strict RemoteDesktop, screenshot provider, and AT-SPI bus facts | Add fake `PATH` doctor test(s), run `cargo test doctor_live_probe_gathers_portal_screenshot_and_atspi_facts`; expected failure: `gather_system_facts` leaves facts empty | Test passes with safe live command gathering and parser reuse | Do not serialize DBus addresses or secrets |
| 8 | MCP `tools/list` includes `x11_get_app_state` in deterministic order | Add/update `tests/mcp_server.rs`, run `cargo test --test mcp_server lists_x11_tools`; expected failure: tool absent | Test passes after MCP tool definition added | Preserve existing order and existing tools |
| 9 | MCP `x11_get_app_state` wraps app-state and malformed args are tool errors | Add MCP call tests for `include_screenshot=false` and malformed `window_id`, run focused MCP tests; expected failure: unsupported tool | Focused tests pass with schema, argument parsing, and report wrapping | `isError=false` for layer-degraded report, true for malformed args |
| 10 | Docs and command list expose app-state guidance | Add/update docs checks if needed, run `grep`/focused test or `cargo test` doc-facing assertions if present; expected failure: docs omit commands | Docs include CLI/MCP and source-overlay guidance | Keep target checkout read-only |

## Mocking / Boundary Policy

- Use fake executable scripts in temporary directories prepended to `PATH` for external commands. This matches existing project tests and avoids live desktop dependence for RED/GREEN slices.
- Do not mock internal modules such as `input::resolve_target` or `accessibility` correlation; app-state should exercise them through public CLI/MCP behavior.
- Tiny PNG fixtures may be generated as raw bytes by fake `gdbus` scripts to verify dimensions and data-url prefix.
- Live Cinnamon/X11 smoke is required after automated tests but does not replace RED/GREEN evidence.

## Required Checks

Before marking apply complete:

- Focused tests per slice, especially `cargo test --test get_app_state_cli`, `cargo test --test mcp_server`, and doctor unit tests.
- `openspec validate add-x11-get-app-state-integration --strict --no-interactive`.
- `make fmt`.
- `make check`.
- `make test`.
- Live/degraded smoke:
  - `cargo run -- get-app-state --no-screenshot --json`.
  - `cargo run -- get-app-state --window-id <safe-listed-window> --no-screenshot --json` or exact degraded reason.
  - `cargo run -- get-app-state --window-id <missing> --json` to prove screenshot/diagnostics still return or exact provider-degraded reason.
- Confirm target checkout remains clean and project has no staged/committed secret files.

## Evidence Log

Fill during apply:

- Slice 1 RED: `cargo test --test get_app_state_cli resolves_window_context_by_window_id` failed as expected with `unsupported command; try --help` before CLI/app-state implementation.
- Slice 1 GREEN: `cargo test --test get_app_state_cli resolves_window_context_by_window_id` passed after adding `src/app_state.rs` and CLI parsing.
- Slice 2 RED: `cargo test --test get_app_state_cli` failed for screenshot-layer tests while the ambiguous-title test passed from the initial target-resolution implementation; target ambiguity behavior remained covered by public CLI before task closure.
- Slice 2 GREEN: `cargo test --test get_app_state_cli` later passed with ambiguous title returning `window_error` and two target candidates.
- Slice 3 RED: `cargo test --test get_app_state_cli` failed `keeps_screenshot_when_window_target_missing` because `screenshot` was null before screenshot capture implementation.
- Slice 3 GREEN: `cargo test --test get_app_state_cli` passed after GNOME Shell-compatible fake `gdbus` screenshot capture produced `image/png` data URL and retained `WindowNotFound`.
- Slice 4 RED: `cargo test --test get_app_state_cli` failed `no_screenshot_and_provider_failure_are_layer_degraded` because provider failure text was not propagated before screenshot layer implementation.
- Slice 4 GREEN: `cargo test --test get_app_state_cli` passed with `--no-screenshot` null screenshot/error and provider failure in `screenshot_error` while exit stayed 0.
- Slice 5 RED: After adding fake collector coverage, initial app-state composition lacked screenshot but matched AT-SPI coverage passed once composition reused `accessibility_tree_report_from_system`; evidence is covered in the focused `get_app_state_cli` run.
- Slice 5 GREEN: `cargo test --test get_app_state_cli includes_matched_accessibility_tree` passed with matched tree length 2 and correlation status `matched`.
- Slice 6 RED: `cargo test --test get_app_state_cli` failed `keeps_context_when_accessibility_is_ambiguous` before screenshot capture, proving layer composition still needed to retain other layers.
- Slice 6 GREEN: `cargo test --test get_app_state_cli keeps_context_when_accessibility_is_ambiguous` passed with `window_context`, screenshot data, empty tree, and `AmbiguousAccessibilityMatch` error.
- Slice 7 RED: `cargo test --test doctor_cli doctor_live_probe_gathers_portal_screenshot_and_atspi_facts` failed because live portal screenshot / GNOME Shell screenshot / AT-SPI facts were false before `gather_system_facts()` probes.
- Slice 7 GREEN: same doctor focused test passed after safe `busctl`/`gdbus` probe gathering and parser support for busctl/gdbus introspection formats.
- Slice 8 RED: `cargo test --test mcp_server` failed `mcp_server_lists_x11_tools` because `x11_get_app_state` was absent.
- Slice 8 GREEN: `cargo test --test mcp_server` passed after adding `x11_get_app_state` tool definition/schema.
- Slice 9 RED: `cargo test --test mcp_server` failed `mcp_server_calls_x11_get_app_state_without_screenshot` and malformed window-id coverage because the tool was unsupported.
- Slice 9 GREEN: `cargo test --test mcp_server` passed with `include_screenshot=false` returning app-state JSON and malformed `window_id` returning tool error.
- Slice 10 RED/GREEN: README and `docs/integration-contract.md` were updated after implementation; coverage is via verification grep/manual review plus full test suite because docs have no separate renderer.
- Verification: Focused tests passed: `cargo test --test get_app_state_cli` (6 passed), `cargo test --test doctor_cli doctor_live_probe_gathers_portal_screenshot_and_atspi_facts` (1 passed), `cargo test --test mcp_server` (9 passed).
- Verification: `openspec validate add-x11-get-app-state-integration --strict --no-interactive` passed: change valid.
- Verification: `make fmt` passed (`cargo fmt -- --check`) with clean status.
- Verification: `make check` passed (`cargo check` finished successfully).
- Verification: `make test` passed: 41 lib tests plus integration suites for accessibility tree, doctor CLI, focus CLI, get-app-state CLI, list-windows CLI, MCP server, plugin installer, pointer actions, screenshot coordinates, targeted input, and doc-tests.
- Live smoke: `cargo run --quiet -- get-app-state --no-screenshot --json` returned valid app-state JSON with usable layers in the local Cinnamon/X11 session.
- Live smoke: `cargo run --quiet -- list-windows --json` found 15 windows; `cargo run --quiet -- get-app-state --window-id <safe-listed-window> --no-screenshot --json` returned matching `window_context`, no `window_error`, no screenshot by request, and usable accessibility context.
- Live smoke: `cargo run --quiet -- get-app-state --window-id 0x99999999 --json` returned valid JSON with `WindowNotFound`, retained a GNOME Shell-compatible screenshot (`image/png`, 5760x1547), and reported the accessibility layer as not attempted because no window context resolved.
- Verification: `git status --short` was clean before marking verification tasks; `.secrets.local.env` is not tracked and absent; only `.secrets.example.env` matched secret-like tracked paths; `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` status was clean/read-only.

## TDD Exceptions

None.
