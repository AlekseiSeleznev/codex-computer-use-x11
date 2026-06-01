## Why

The project now has standalone X11 MCP tools and a reversible source overlay, but there is no repeatable end-to-end harness proving those paths still work from Codex-facing installation and tool-call boundaries. This change adds fake-mode CI evidence and live Cinnamon/X11 smoke evidence so future stages can prove the v1 Computer Use capability matrix instead of relying on ad-hoc manual checks.

## What Changes

- Add `scripts/e2e/codex-plugin-smoke.sh` for standalone plugin marketplace metadata, MCP stdio startup, tool discovery, safe fake input routing, `x11_get_app_state`, and capability-matrix evidence.
- Add `scripts/e2e/codex-source-overlay-smoke.sh` for source-overlay status/apply/test/uninstall evidence and target-stock capability coverage/degraded reasons.
- Add a small project-owned e2e runner/library under `scripts/e2e/` that supports `--fake` for no-GUI CI and `--live` for the current Cinnamon/X11 development machine.
- Write all smoke logs and machine-readable evidence to `target/e2e-logs/`, including failure diagnostics.
- Document live/manual smoke steps when Codex Desktop itself does not expose a stable direct stock tool-call runner.
- No **BREAKING** changes to existing CLI/MCP tool names, source-overlay ownership markers, or target checkout state.

## Capabilities

- New capability: `codex-x11-e2e-test-harness` — repeatable fake/live e2e smoke for standalone plugin and source-overlay delivery paths, including capability-matrix coverage and log/evidence retention.

## Impact

- Affected code: new scripts under `scripts/e2e/`, tests under `tests/`, and documentation under `docs/`.
- Affected systems: user-local Codex plugin state under `CODEX_HOME` in live mode; fake mode uses an isolated temp `CODEX_HOME` fixture. Source-overlay live mode may temporarily patch the configured Codex Desktop Linux target checkout only between install and uninstall and must leave it clean.
- Required technologies and verification: Rust 2021/Cargo tests, Bash/Python helper scripts, `make fmt`, `make check`, `make test`, `openspec validate --strict`, and explicit e2e smoke logs.
- Secret handling: no secrets are required; `.secrets.local.env` is not read. Git-tracked artifacts document only variable names such as `CODEX_HOME` and `CODEX_DESKTOP_LINUX_FULL_PATH`.
- Architecture constraints: preserve `x11-ewmh`, X11 root-coordinate semantics, standalone `x11_*` names, stock target `get_app_state`/`activate_window` names, source-overlay marker ownership, and no permanent target fork.

## Research Refresh

Date: 2026-05-31.

### Local project and target state

- Project repo `/home/as/ai-projects/codex-computer-use-x11`: on `main`, clean before scaffold; scaffold commit `3f04b1d` created `add-codex-x11-e2e-test-harness`.
- Target repo `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: on `main` at `1a6f343ee7ce597019a4c573259c2a9838376874`, clean.
- Target files inspected: `computer-use-linux/src/windowing/**`, `server.rs`, `diagnostics.rs`, `atspi_tree.rs`, `screenshot.rs`, `remote_desktop.rs`, `abs_pointer.rs`, `scripts/ci-local.sh`, and bundled plugin staging scripts.
- Current target stock tools include `doctor`, `list_windows`, `focused_window`, `activate_window`, `get_app_state`, `screenshot`, `click`, `scroll`, `drag`, `press_key`, `type_text`, `perform_action`, and `set_value`. No stock `focus_window` or `mousemove` tool was found; source-overlay smoke must use `activate_window` and record absent stock `mousemove` as non-blocking.
- Source overlay status against the real target currently reports `state=clean` and no owned markers.

### External references checked

- `tak-uukti/linux-computer-use` (`https://github.com/tak-uukti/linux-computer-use`) — confirms a compact X11 MCP smoke shape using AT-SPI, xdotool, wmctrl, and screenshot capture. Use as reference only; no code copied.
- `BeckhamLabsLLC/linux-desktop-mcp` (`https://github.com/BeckhamLabsLLC/linux-desktop-mcp`) — reinforces AT-SPI semantic snapshot and element reference ideas. Use as reference only; no code copied.
- `iFurySt/open-codex-computer-use` (`https://github.com/iFurySt/open-codex-computer-use`) — notable for a direct `call` command that runs one or more MCP-style Computer Use calls in one process; this supports adding a project-owned machine-checkable stdio runner. Use as reference only; no code copied.
- `domdomegg/computer-use-mcp`, `Touchpoint-Labs/Touchpoint`, `MONTBRAIN/vadgr-computer-use`, `hightemp/go_computer_use_mcp_server`, and `ezpzai/codex-computer-use-windows` were checked as broad current prior art. They do not replace the project-specific Codex plugin/source-overlay harness.

### Ideas adopted

- Prefer a direct MCP stdio smoke runner for deterministic fake-mode checks instead of requiring the Codex Desktop UI.
- Keep fake mode hermetic with temp `CODEX_HOME`, fake command binaries, and no GUI dependency.
- Treat live mode as additional evidence that can document pass/degraded outcomes per capability group.
- Store one JSON evidence file and command logs per run under `target/e2e-logs/`.

### Ideas rejected or deferred

- Do not drive the Codex Desktop UI as the primary automated harness; the current stable, project-owned boundary is plugin metadata plus MCP stdio.
- Do not modify `/opt/codex-desktop` or any installed app resources directly.
- Do not vendor or copy external project code without a separate license review; all external projects are references only in this change.
- Do not fail source-overlay smoke solely because stock `mousemove` is absent; the current target provides `click`, `scroll`, and `drag` instead.

### Risks and uncertainties

- The real Codex Desktop app may not expose a stable direct stock Computer Use tool-call runner; source-overlay live smoke must therefore combine reversible target tests with documented manual stock-tool evidence until a stable runner exists.
- Live desktop behavior depends on window focus, installed desktop tools, accessibility state, and input backend availability; fake-mode checks remain the archive gate for deterministic coverage, while live mode records explicit degraded reasons.
