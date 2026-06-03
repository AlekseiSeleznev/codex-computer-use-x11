## TDD Strategy

Use the project-local `tdd` skill with vertical RED -> GREEN -> REFACTOR slices. Each behavior-changing task starts with one public-interface failing test/check, then the minimal production/harness change to make it pass, then refactor only while GREEN. Public interfaces are CLI JSON reports, MCP tool JSON content, e2e harness evidence files, and documented readiness summaries. Fake command fixtures are allowed for X11 boundary commands; internal Rust collaborators should not be mocked just to fit an imagined implementation shape.

No implementation is approved by this test plan; it defines the evidence required when `/opsx:apply` starts.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1. Keyboard aliases/stderr | `press-key` / `type-text` CLI JSON through fake `xdotool` fixtures | Add one test in `tests/targeted_input_cli.rs`; run `cargo test --test targeted_input_cli keyboard_aliases_and_semantic_stderr -- --nocapture`; expect failure because `Enter`/`Backspace` are not normalized or `xdotool` stderr exit-0 is treated as success | Same command passes: aliases become `Return`/`BackSpace`; stderr containing `No such key name` / `Ignoring it` yields `success=false`, `input_sent=false`, `InputBackendFailed` | Keep normalization pure; keep stderr semantic checks reusable by CLI and MCP; do not add speculative Unicode fallback in this slice |
| 2. Unicode keysyms + fallback decision | `type-text --text Привет --json` CLI/MCP JSON and fake backend args | Add one fake-backend test; run `cargo test --test targeted_input_cli unicode_text_uses_keysyms_before_clipboard -- --nocapture`; expect failure because current route uses `xdotool type` literal text | Same command passes: route is `xdotool-unicode-keysyms`, args include `U041F...`; no `--window`; fallback reports `route=clipboard-paste` only when keysym route is configured to fail | Keep focus verification before both routes; clipboard helper boundary restores previous content or emits restoration warning; no `ydotool` primary path |
| 3. AT-SPI token matching/enrichment | `accessibility-tree --window-id <id> --json` matcher/report behavior via fake X11/AT-SPI fixtures | Add tests in `tests/accessibility_tree_cli.rs`; run `cargo test --test accessibility_tree_cli atspi_token_matching_and_target_xprop -- --nocapture`; expect failure because `tk` may match `gtk3` and no target xprop enrichment/missing_signals exist | Same command passes: `Tk` does not match `gtk3`; one target-scoped `xprop -id` call is recorded; diagnostics include candidate reasons and missing_signals; bounds-only no-match remains no subtree | Keep normal `list-windows` fan-out disabled; score logic stays threshold-based and explainable |
| 4. GTK fixture | Live/fake e2e harness AT-SPI-positive fixture behavior | Add a fake GTK fixture/unit path first; run `python3 scripts/e2e/codex-x11-e2e.py plugin --fake --log-dir target/e2e-logs/tdd-gtk-fixture`; expect failure because harness has no GTK AT-SPI positive fixture row | Fake pass records GTK AT-SPI evidence; live pass/degraded records dependency details and does not use Tk no-match as pass | If PyGObject/GTK is unavailable, record explicit degraded dependency evidence; do not lower matcher thresholds |
| 5. Overlay provider | `target-window --overlay`, `list-windows`, `release-window` CLI JSON | Add tests in `tests/target_window_cli.rs`; run `cargo test --test target_window_cli overlay_provider_shows_excludes_and_hides -- --nocapture`; expect failure because provider is `no-overlay` and `overlay.shown=false` | Same command passes with fake/provider seam: `overlay.shown=true`, overlay windows filtered from listing/target resolution, release hides overlay | Keep provider boundary narrow; real X11 provider can be swapped for fake in tests; overlay failure remains warning path covered by existing tests |
| 6. Evidence/app-state cleanup | `x11_get_app_state` report summary and e2e evidence sanitizer | Add tests in `tests/get_app_state_cli.rs` or `tests/e2e_harness_scripts.rs`; run `cargo test --test get_app_state_cli app_state_summary_uses_diagnostics_layers -- --nocapture` and/or `python3 scripts/e2e/codex-x11-e2e.py validate-matrix <fixture>`; expect failure for `.layers` or base64 summary behavior | Tests pass: summary reads `diagnostics.layers`, no-screenshot-data summary omits `data_url`, portal absence is optional/report-only when X11 path works | Keep raw app-state JSON shape compatible; sanitizer only affects summary/evidence mode unless CLI flag explicitly requests it |
| 7. Live e2e harness | `scripts/e2e/codex-x11-e2e.py` live mode matrix | First run targeted safe live check after local code is GREEN: `python3 scripts/e2e/codex-x11-e2e.py plugin --live --log-dir target/e2e-logs/live-functional-hardening`; expected initial failure/degraded rows identify exact Cyrillic, GTK AT-SPI, overlay, or evidence gaps | Final live run records exact Cyrillic value pass or explicit degraded reason, GTK AT-SPI pass/degraded with dependency evidence, overlay lifecycle pass/degraded, and complete pass/degraded matrix | Keep live checks safe-window-only; logs sanitized; no destructive desktop actions; manual UI STOP-gate remains separate if needed |

## Mocking / Boundary Policy

- Fake only external desktop command/process boundaries: `wmctrl`, `xprop`, `xdotool`, `xclip`/`xsel`, AT-SPI collector subprocess, screenshot provider, and overlay provider/X11 connection.
- Do not mock internal Rust functions such as target resolution, focus report composition, scoring, or JSON serialization when a public CLI/MCP path can exercise them.
- Fake `PATH` command fixtures must log invoked args so tests can prove route ordering and absence of `--window`.
- Live tests must use safe fixtures only and must not mutate external systems, secrets, target repositories, or `/opt/codex-desktop`.

## Required Checks

Before marking apply complete:

- `openspec validate --all --strict --json`
- `make fmt`
- `make check`
- `make test`
- Targeted slice commands listed above with RED/GREEN evidence recorded during apply.
- `python3 scripts/e2e/codex-x11-e2e.py ... --fake` matrix validation for affected harness paths.
- Live hardening run when a live X11 desktop is available; if unavailable, record blocker/degraded limitation rather than fabricating pass.
- `git status --short` and confirmation that `.secrets.local.env` or any local secret file is not staged/tracked.

Before archive (future step, not this planning change):

- Re-run strict OpenSpec validation and project checks.
- Confirm implementation state is merged to `main` and verify-ready under OpenSpec/git discipline.
- Manual UI STOP-gate evidence if the active Codex Desktop UI is part of the final readiness claim.

## Evidence Log

Fill during apply/verify. Initial planning evidence:

- 2026-06-01 live evidence source: `target/e2e-logs/live-functional/acceptance-summary.md`.
- Current planning artifacts explicitly state no implementation yet.
- Slice 1 RED: `cargo test --test targeted_input_cli -- --nocapture` failed as expected with missing `Enter`/`Backspace` normalization and semantic `xdotool` stderr exit-0 handling.
- Slice 1 GREEN: `cargo test --test targeted_input_cli -- --nocapture` passed after key alias normalization and semantic stderr failure detection.
- Slice 2 RED: `cargo test --test targeted_input_cli -- --nocapture` failed as expected because non-ASCII type-text still reported `xdotool-type` and did not use Unicode keysyms/clipboard fallback.
- Slice 2 GREEN: `cargo test --test targeted_input_cli -- --nocapture` passed after active-context Unicode keysyms and explicit `clipboard-paste` fallback with restore diagnostics.
- Slice 3 RED: `cargo test --test accessibility_tree_cli -- --nocapture` failed as expected because `Tk` still substring-matched `gtk3` and target-scoped `xprop -id`/`missing_signals` were absent.
- Slice 3 GREEN: `cargo test --test accessibility_tree_cli -- --nocapture` passed after token-boundary class matching, target-scoped xprop enrichment, score reasons, and missing signal diagnostics.
- Slice 5 RED: `cargo test --test target_window_cli overlay_provider_shows_excludes_and_hides -- --nocapture` failed as expected because overlay provider still returned `shown=false`.
- Slice 5 GREEN: `cargo test --test target_window_cli -- --nocapture` and `cargo test --lib list_windows -- --nocapture` passed after overlay provider boundary, overlay window exclusion, and release hide diagnostics.
- Slice 4 RED: `cargo test --test e2e_harness_scripts plugin_smoke_fake_records_app_state_and_input_matrix -- --nocapture` failed as expected because fake plugin evidence had no `gtk_atspi_fixture` row.
- Slice 4 GREEN: `cargo test --test e2e_harness_scripts plugin_smoke_fake_records_app_state_and_input_matrix -- --nocapture` passed after fake `python3`/`xprop` GTK AT-SPI fixture support. `python3 scripts/e2e/codex-x11-e2e.py plugin --fake --binary target/debug/codex-computer-use-x11 --log-dir target/e2e-logs/tdd-gtk-fixture` wrote `target/e2e-logs/tdd-gtk-fixture/standalone_plugin-fake-20260601T115109Z-391788/evidence.json` with `gtk_atspi_fixture=pass`; `docs/e2e-harness.md` now records Tk/Tkinter AT-SPI limitations and GTK dependency degradation rules.
- Slice 6 RED: `cargo test --test e2e_harness_scripts app_state_summary_requires_diagnostics_layers_and_sanitizes_screenshot -- --nocapture` failed as expected because `summarize-app-state` did not exist and no path reported missing `diagnostics.layers`.
- Slice 6 GREEN: `cargo test --test e2e_harness_scripts -- --nocapture` passed after adding `summarize-app-state`, evidence-safe screenshot `data_url` stripping, app-state summaries sourced from `diagnostics.layers`, and a doctor regression proving RemoteDesktop portal gaps remain report-only when the X11/local-input path works. `cargo test --lib doctor_remote_desktop_gap_is_report_only_when_x11_input_path_works -- --nocapture` passed. `python3 scripts/e2e/codex-x11-e2e.py plugin --fake --binary target/debug/codex-computer-use-x11 --log-dir target/e2e-logs/tdd-app-state-summary` wrote `target/e2e-logs/tdd-app-state-summary/standalone_plugin-fake-20260601T115506Z-399483/evidence.json` with sanitized `app_state_summary.layers` from `diagnostics.layers`.
- Slice 7 RED: `cargo test --test e2e_harness_scripts plugin_smoke_fake_records_app_state_and_input_matrix -- --nocapture` failed as expected because fake harness evidence lacked `keyboard_unicode_value` and `overlay_lifecycle` rows. `matrix_validator_rejects_missing_evidence` was extended to reject pass rows without evidence.
- Slice 7 GREEN: `cargo test --test e2e_harness_scripts -- --nocapture` passed after fake harness checks for exact Cyrillic value/route, GTK fixture evidence, overlay shown/listing-excluded/release-hide, live report-only degraded checks for unsafe fixture absence, and stricter matrix pass evidence validation. `python3 scripts/e2e/codex-x11-e2e.py plugin --fake --binary target/debug/codex-computer-use-x11 --log-dir target/e2e-logs/tdd-live-harness-fake` wrote `target/e2e-logs/tdd-live-harness-fake/standalone_plugin-fake-20260601T115826Z-405274/evidence.json`; `python3 scripts/e2e/codex-x11-e2e.py validate-matrix --evidence <that evidence>` passed.
- Final verification: `openspec validate --all --strict --json` passed 19/19 (with existing INFO-only long-requirement notes for unrelated specs). First `make fmt && make check && make test` failed at `cargo fmt -- --check`; after running `cargo fmt`, `make fmt && make check && make test` passed. `python3 scripts/e2e/codex-x11-e2e.py plugin --fake --binary target/debug/codex-computer-use-x11 --log-dir target/e2e-logs/final-fake-matrix` wrote `target/e2e-logs/final-fake-matrix/standalone_plugin-fake-20260601T120013Z-412070/evidence.json`, and `validate-matrix` passed.
- Live/report-only verification: environment had `DISPLAY=:0` and `XDG_SESSION_TYPE=x11`; `python3 scripts/e2e/codex-x11-e2e.py plugin --live --binary target/debug/codex-computer-use-x11 --log-dir target/e2e-logs/final-live-report-only` wrote `target/e2e-logs/final-live-report-only/standalone_plugin-live-20260601T120029Z-413037/evidence.json`, and `validate-matrix` passed. Because no explicit safe text/GTK/overlay fixture was configured, live `keyboard_unicode_value`, `gtk_atspi_fixture`, and `overlay_lifecycle` rows are intentionally `degraded` rather than fabricated passes.

## TDD Exceptions

None.
