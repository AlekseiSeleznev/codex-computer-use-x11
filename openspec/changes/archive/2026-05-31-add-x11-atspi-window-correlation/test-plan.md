## TDD Strategy

Use the project-local `tdd` skill with vertical public-interface slices. Each behavior starts with one CLI or MCP integration test against the compiled binary/server, confirms the expected RED failure, implements the smallest GREEN path, and refactors only while the focused test and relevant surrounding tests are green. AT-SPI is treated as a system boundary: fake `wmctrl`, `xprop`, and `python3` executables in a temporary `PATH` provide deterministic fixtures without depending on the live desktop. Live Cinnamon/X11 smoke runs after fake tests pass and may record either a subtree or a precise degraded reason.

## Vertical TDD Slices

| Slice | Public interface / behavior | RED command and expected failure | GREEN command and expected pass | Refactor criteria |
| --- | --- | --- | --- | --- |
| 1 | CLI `accessibility-tree --window-id` returns high-confidence subtree when reliable PID, title/name, and bounds match | `cargo test --test accessibility_tree_cli accessibility_tree_returns_high_confidence_subtree_for_reliable_pid` fails because `accessibility-tree` is unsupported | Same command passes; JSON has `success=true`, `correlation.status=matched`, `confidence=high`, non-empty `tree`, and fake `python3` was invoked after listing resolved | Report types are stable; no MCP wiring yet |
| 2 | CLI missing X11 window returns JSON `WindowNotFound` without AT-SPI collection | `cargo test --test accessibility_tree_cli accessibility_tree_refuses_missing_window_before_atspi_collection` fails until window-id resolution is wired | Same command passes; `success=false`, `error_code=WindowNotFound`, `tree=[]`, and collector log is empty | Window lookup is shared with existing X11 id normalization |
| 3 | Unreliable PID is discounted and title/class/bounds can produce medium confidence | `cargo test --test accessibility_tree_cli accessibility_tree_uses_non_pid_evidence_when_pid_unreliable` fails until `WindowMetadata.pid_reliability` is joined into matcher input | Same command passes; candidate with matching title/class/bounds but different pid is selected with `confidence=medium`, and reasons mention unreliable PID | Scoring helpers remain deterministic and small |
| 4 | Ambiguous AT-SPI candidates return no subtree | `cargo test --test accessibility_tree_cli accessibility_tree_refuses_ambiguous_candidates` fails until ambiguity detection exists | Same command passes; `success=false`, `correlation.status=ambiguous`, `error_code=AmbiguousAccessibilityMatch`, `tree=[]`, candidate diagnostics include both object refs | Candidate summary formatting stays automation-friendly |
| 5 | Browser/terminal PID mismatch is not over-trusted | `cargo test --test accessibility_tree_cli accessibility_tree_matches_browser_by_title_class_bounds_without_pid` and `accessibility_tree_does_not_match_terminal_child_pid_alone` fail until non-PID corroboration and child-pid avoidance are implemented | Both tests pass; browser fixture matches with medium confidence without claiming PID, terminal child-only fixture returns `NoAccessibilityMatch` or requires title/class/bounds corroboration | Matcher reasons make accepted/rejected signals visible |
| 6 | AT-SPI unavailable degrades structurally | `cargo test --test accessibility_tree_cli accessibility_tree_degrades_when_atspi_collector_unavailable` fails until collector exit/parse failures become report JSON | Same command passes; `success=false`, `correlation.status=degraded`, `error_code=AtspiUnavailable`, `tree=[]`, diagnostics include collector stderr/exit detail | CLI stdout remains one JSON object even when collector warns on stderr |
| 7 | MCP `x11_accessibility_tree` is listed and delegates to CLI report behavior | `cargo test --test mcp_server mcp_server_lists_x11_tools` fails because `x11_accessibility_tree` is absent | `cargo test --test mcp_server mcp_server_lists_x11_tools mcp_accessibility_tree_requires_window_id` passes; tool order includes `x11_accessibility_tree`, missing argument is a tool error | Existing MCP tool behavior remains unchanged |
| 8 | Full project verification and live/degraded smoke | `make test` or live smoke fails before all wiring/evidence is complete | `openspec validate add-x11-atspi-window-correlation --strict`, `make fmt`, `make check`, `make test`, focused CLI/MCP tests, and live/degraded smoke pass or record blocker | Project and target checkout status are clean; no secrets staged |

## Mocking / Boundary Policy

- Use fake executable scripts in a temporary `PATH` for `wmctrl`, `xprop`, and `python3` to verify public CLI behavior without relying on real AT-SPI or moving the desktop.
- Do not mock internal Rust collaborators. Tests run the compiled binary or MCP stdio server.
- The fake `python3` collector emits the same JSON shape expected from the live collector and can log invocations for ordering assertions.
- Live smoke may use real `python3`/GI/AT-SPI, `wmctrl`, and `xprop` after fake tests are green. If the live desktop exposes no safe GTK/browser target or GI/AT-SPI fails, record `AtspiUnavailable` or exact degraded reason.
- Pure helper unit tests may supplement but not replace public CLI/MCP evidence.

## Required Checks

- `openspec validate add-x11-atspi-window-correlation --strict`
- `cargo test --test accessibility_tree_cli`
- `cargo test --test mcp_server`
- `make fmt`
- `make check`
- `make test`
- Live/degraded Cinnamon/X11 smoke:
  - Identify a safe listed GTK app and/or browser window.
  - Run `target/debug/codex-computer-use-x11 accessibility-tree --window-id <id> --json`.
  - Record either `success=true` with a matched subtree or `success=false` with precise degraded/ambiguous reason.
- Confirm project git status is clean before archive and target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` remains unmodified.

## Evidence Log

- Slice 1 RED: `cargo test --test accessibility_tree_cli accessibility_tree_returns_high_confidence_subtree_for_reliable_pid` failed because `accessibility-tree` was unsupported and returned non-zero.
- Slice 1 GREEN: same command passed after adding `src/accessibility.rs`, CLI `accessibility-tree --window-id <id> --json` parsing, fake collector JSON parsing, reliable-PID/title/bounds/focus scoring, and high-confidence matched report output. Test fixture was corrected to use the real local hostname so sidecar PID reliability is `reliable`.
- Slice 2 RED: `cargo test --test accessibility_tree_cli accessibility_tree_refuses_missing_window_before_atspi_collection` failed because the initial implementation returned `NoAccessibilityMatch` after attempting the collector.
- Slice 2 GREEN: same command passed after `accessibility_tree_report_from_system()` checks the listing for the requested `window_id` before AT-SPI collection and returns JSON `WindowNotFound` with an empty tree and no collector log.
- Slice 3 RED: `cargo test --test accessibility_tree_cli accessibility_tree_uses_non_pid_evidence_when_pid_unreliable` failed because non-PID evidence initially produced no successful match.
- Slice 3 GREEN: same command passed after medium-confidence matching was allowed when at least two non-PID signals corroborate the candidate and unreliable PID evidence is explicitly recorded in reasons.
- Slice 4 RED: `cargo test --test accessibility_tree_cli accessibility_tree_refuses_ambiguous_candidates` failed because the first high-confidence candidate was returned even when a second equivalent candidate existed.
- Slice 4 GREEN: same command passed after adding ambiguity detection for multiple high/medium candidates within 10 score points, returning `AmbiguousAccessibilityMatch` and an empty tree.
- Slice 5a RED: `cargo test --test accessibility_tree_cli accessibility_tree_matches_browser_by_title_class_bounds_without_pid` failed because browser-style class/name evidence was not scored when PID differed.
- Slice 5a GREEN: same command passed after adding normalized wm_class/app-name matching as non-PID evidence and returning medium confidence without claiming `reliable PID matched`.
- Slice 5b RED: `cargo test --test accessibility_tree_cli accessibility_tree_does_not_match_terminal_child_pid_alone` failed because diagnostics did not explain why a terminal child PID candidate was rejected.
- Slice 5b GREEN: same command passed after recording `candidate PID did not match reliable window PID` when a reliable X11 window PID differs from an AT-SPI candidate PID; child PID alone still returns `NoAccessibilityMatch` with an empty tree.
- Slice 6 RED: `cargo test --test accessibility_tree_cli accessibility_tree_degrades_when_atspi_collector_unavailable` failed because collector diagnostics identified command `python3` rather than the actual `python3 -c` boundary.
- Slice 6 GREEN: same command passed after normalizing collector command diagnostics to `python3 -c`; failure returns JSON `AtspiUnavailable`, `correlation.status=degraded`, empty tree, and captured collector stderr while CLI stderr stays empty.
- CLI correlation group GREEN: `cargo test --test accessibility_tree_cli` passed (7 tests).
- Slice 7a RED: `cargo test --test mcp_server mcp_server_lists_x11_tools` failed because `tools/list` exposed only nine tools and omitted `x11_accessibility_tree`.
- Slice 7a GREEN: same command passed after adding `x11_accessibility_tree` to the deterministic tool list after `x11_drag` with a `window_id` input schema and unprefixed-name exclusion.
- Slice 7b RED: `cargo test --test mcp_server mcp_accessibility_tree_requires_window_id` failed because `x11_accessibility_tree` was listed but not handled and returned `unsupported tool` rather than a `window_id` argument error.
- Slice 7b GREEN: same command passed after adding the MCP call wrapper, shared X11 id normalization, and report delegation.
- MCP regression GREEN: `cargo test --test mcp_server mcp_accessibility_tree` passed (2 tests), proving missing `window_id` is a tool error and report-level `WindowNotFound` is returned as JSON with `isError=true`.
- MCP group GREEN: `cargo test --test mcp_server` passed (7 tests).
- Live collector implementation: replaced the placeholder collector with bounded Python GI/AT-SPI traversal after focused fake tests were green; regression `cargo test --test accessibility_tree_cli` and `cargo test --test mcp_server` passed.
- Focused checks GREEN: `cargo test --test accessibility_tree_cli` passed (7 tests) and `cargo test --test mcp_server` passed (7 tests).
- Verification GREEN: `openspec validate add-x11-atspi-window-correlation --strict`, `make fmt`, `make check`, and `make test` passed. Full `make test` included 40 unit tests plus integration tests: accessibility tree CLI (7), doctor CLI (2), focus CLI (8), list-windows CLI (3), MCP server (7), plugin installer (5), pointer actions CLI (7), targeted input CLI (6), and doc tests.
- Live smoke GREEN (browser): `target/debug/codex-computer-use-x11 accessibility-tree --window-id 98566147 --json` returned `success=true`, `status=matched`, `confidence=high`, `score=120`, `tree_len=396`, matched Firefox frame, collector detail `gi.repository.Atspi collector returned 56 candidates` with truncation reported.
- Live smoke GREEN (GTK/terminal): `target/debug/codex-computer-use-x11 accessibility-tree --window-id 134217734 --json` returned `success=true`, `status=matched`, `confidence=high`, `score=90`, `tree_len=49`, matched gnome-terminal frame, collector detail `gi.repository.Atspi collector returned 56 candidates` with truncation reported.
- Git/target safety GREEN: project `git status --short` was clean after implementation/evidence checkpoint; target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` had clean `git status --short`; no tracked `.secrets.local.env` or other local secret file was present.

## TDD Exceptions

None
