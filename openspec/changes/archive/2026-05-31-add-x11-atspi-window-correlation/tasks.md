## 1. CLI Report and Correlation Core

- [x] 1.1 Add RED CLI integration test `accessibility_tree_returns_high_confidence_subtree_for_reliable_pid` using fake `wmctrl`, `xprop`, and `python3` collector fixtures.
- [x] 1.2 Implement `src/accessibility.rs` report types, fake/live collector JSON parsing, reliable-PID/title/bounds scoring, and `accessibility-tree --window-id <id> --json` CLI wiring for the high-confidence match.
- [x] 1.3 Add RED/GREEN CLI test proving missing `window_id` returns JSON `WindowNotFound` and skips AT-SPI collection.
- [x] 1.4 Add RED/GREEN CLI test and matcher behavior for unreliable PID discounted in favor of title/class/bounds non-PID evidence.

## 2. Ambiguity, Edge Cases, and Degraded AT-SPI

- [x] 2.1 Add RED/GREEN CLI test proving equivalent AT-SPI candidates return `AmbiguousAccessibilityMatch`, `success=false`, and an empty tree.
- [x] 2.2 Add RED/GREEN browser-style fixture proving a candidate can match by title/class/bounds when X11 pid differs from AT-SPI pid without claiming a PID match.
- [x] 2.3 Add RED/GREEN terminal child-pid fixture proving child PID alone does not create a match without title/class/bounds or terminal context corroboration.
- [x] 2.4 Add RED/GREEN CLI test proving collector exit/parse/import failures return `AtspiUnavailable` degraded JSON with stdout remaining one report object.
- [x] 2.5 Refactor matcher/report helpers only while focused CLI tests are green, keeping scores/reasons deterministic and candidate diagnostics automation-friendly.

## 3. MCP Tool Surface

- [x] 3.1 Add RED MCP test proving `tools/list` includes `x11_accessibility_tree` after `x11_drag` and excludes unprefixed `accessibility_tree`.
- [x] 3.2 Add MCP schema and `tools/call` wrapper for `x11_accessibility_tree`, including shared X11 id normalization and missing-argument tool errors.
- [x] 3.3 Add MCP regression tests proving missing `window_id` is a tool error and report failures are returned as JSON tool errors.

## 4. Evidence, Verification, and Lifecycle

- [x] 4.1 Record RED/GREEN evidence in `test-plan.md` after each implemented TDD slice before marking the related task complete.
- [x] 4.2 Run focused checks: `cargo test --test accessibility_tree_cli` and `cargo test --test mcp_server`.
- [x] 4.3 Run required project checks: `openspec validate add-x11-atspi-window-correlation --strict`, `make fmt`, `make check`, and `make test`.
- [x] 4.4 Run live/degraded Cinnamon/X11 smoke for `accessibility-tree --window-id <id> --json` against at least one safe listed GTK/browser window or record exact degraded/ambiguous reason.
- [x] 4.5 Confirm project git status is clean, target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` remains unmodified, and no local secret files are staged before archive.
