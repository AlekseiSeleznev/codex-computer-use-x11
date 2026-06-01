## Context Read

- `proposal.md`, `specs/x11-atspi-window-correlation/spec.md`, `specs/standalone-codex-mcp-plugin/spec.md`, `grill.md`, and `design.md` for this change.
- Root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/README.md`; referenced durable ADR bodies are absent in this checkout, so the snapshot/README are the available in-force decision context.
- Existing canonical specs for standalone MCP, X11 listing, active/focus, targeted keyboard input, and pointer actions.
- Existing standalone code: `src/cli.rs`, `src/list_windows.rs`, `src/focus.rs`, `src/input.rs`, `src/pointer.rs`, `src/mcp.rs`, `src/doctor.rs`, and integration tests.
- Target checkout code: `computer-use-linux/src/atspi_tree.rs`, `server.rs`, `terminal.rs`, `diagnostics.rs`, `windowing/types.rs`, and `windowing/target.rs`.
- Live environment probes: Cinnamon/X11 session, `toolkit-accessibility=true`, `org.a11y.Bus` visible, Python GI `Atspi` import works, and `Atspi.Accessible` exposes `get_process_id`, `get_child_count`, `get_role_name`, `get_name`, and Component `get_extents`.

## Design Summary

- The design adds `src/accessibility.rs` with a stable report builder, pure multi-signal matcher, and bounded external AT-SPI collector boundary.
- CLI `accessibility-tree --window-id <id> --json` resolves the X11 window first and avoids AT-SPI collection for missing/invalid windows.
- MCP `x11_accessibility_tree` delegates to the same report builder and treats report failures as tool errors with JSON content.
- Reliable PID, title/name, wm_class/app name, bounds overlap, and focus state contribute to deterministic scores; ambiguous top candidates produce no subtree.
- Live collection uses Python GI/AT-SPI for the standalone stage but is explicitly replaceable by target-style Rust `atspi` later.

## Question Loop

### Q1: Does using Python GI violate the Rust-first implementation constraint?

- Recommended answer: no, if Python GI is treated like the existing `wmctrl`/`xprop`/`xdotool` command boundary and all failures degrade structurally.
- Rationale: the constitution says Rust/Cargo is the implementation stack for the crate, but the project already shells out to desktop tools as backend probes/actions. The Rust code owns parsing, scoring, report shape, CLI/MCP protocol, and tests. Python GI is a live OS capability boundary, not a replacement application stack.
- Resolution: no user question needed. Keep the collector boundary explicit in design and diagnostics.

### Q2: Can tests prove AT-SPI behavior without a real accessibility bus?

- Recommended answer: yes for matcher and CLI/MCP contracts, by faking `python3` stdout/stderr/exit code via `PATH`; live smoke separately proves the current desktop can produce a tree or a degraded reason.
- Rationale: public-interface fixture tests are already the project pattern for `wmctrl`, `xprop`, and `xdotool`. They verify observable JSON and command ordering without desktop side effects.
- Resolution: no user question needed. Test plan should make fake collector fixtures primary and live smoke secondary.

### Q3: Are the scoring constants over-specified for a first implementation?

- Recommended answer: they are acceptable if recorded as local/reversible defaults and exposed via reasons/scores.
- Rationale: deterministic thresholds are necessary for tests and safe ambiguity. They can be tuned in later changes without changing project architecture.
- Resolution: no artifact update required; test-plan should test outcomes rather than every numeric constant.

### Q4: Should `accessibility-tree` also accept selectors like title/class/pid?

- Recommended answer: no for this stage; accept only `--window-id`.
- Rationale: listing/focus/input already have target selector complexity. Correlation is safest and most deterministic when a caller first chooses a concrete current window id. Later `get_app_state` integration can add richer target resolution after this primitive is proven.
- Resolution: specs and design already restrict to `window_id`; no update required.

### Q5: Is a durable ADR needed for the Python collector boundary?

- Recommended answer: no durable ADR.
- Rationale: the boundary is local, reversible, and deliberately a standalone-stage implementation detail. The future target integration may replace it with Rust `atspi` without superseding project architecture.
- Resolution: per-change `adr.md` should record the trade-off but create no durable ADR.

## Design Findings

- **No constitution conflict found.** Rust owns the CLI/MCP/report/matcher implementation; Python GI is a desktop capability probe analogous to existing external commands.
- **No secret or external-system access.** The workflow does not need `.secrets.local.env`; AT-SPI is a local desktop bus capability.
- **Window-id-only interface is safer.** It avoids duplicating target-resolution ambiguity in a semantic read command.
- **Collector stderr must never break stdout JSON.** Implementation must capture Python/AT-SPI warnings into diagnostics and serialize one report object to stdout.
- **PID reliability must be joined from sidecar diagnostics.** `WindowInfo.pid` alone is insufficient because listing intentionally drops unreliable PIDs.
- **Live evidence should permit degraded success criteria.** Acceptance is satisfied when GTK/browser smoke returns a subtree or a precise degraded reason; fixture tests validate core behavior deterministically.
- **No target checkout mutation.** Source overlay and target Rust `atspi` port remain future work.

## Document Updates Applied

None. The proposal, specs, grill, and design already reflect the review findings.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

None. The Python collector boundary and score thresholds are local and reversible; no hard-to-reverse project-wide architecture decision is introduced.

## Open Questions

None
