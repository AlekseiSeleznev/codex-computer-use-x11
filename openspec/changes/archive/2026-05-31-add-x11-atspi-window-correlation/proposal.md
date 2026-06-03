## Why

The standalone X11/EWMH plugin can list and target windows, but Computer Use needs semantic context from AT-SPI to understand controls inside the selected window. A safe correlation layer is needed so `window_id`-targeted accessibility reads return a confident subtree or an explicit ambiguous/degraded result instead of guessing by PID alone.

## What Changes

- Add a standalone `accessibility-tree --window-id <id> --json` CLI command that resolves the X11 window, gathers AT-SPI application/window candidates, scores correlation signals, and returns a subtree only when the match is confident.
- Add an MCP tool `x11_accessibility_tree` that wraps the same behavior for the user-local standalone Codex plugin.
- Introduce an AT-SPI correlation matcher with confidence, score, and reasons based on reliable PID metadata, title/app/class text, bounds overlap, and focused-window signals.
- Preserve safe degraded behavior when AT-SPI is unavailable, candidates are ambiguous, PID is unreliable, or no candidate reaches the confidence threshold.
- Document research-refresh findings and use external projects as ideas/references only; no external source code is copied.

## Capabilities

- New capability: `x11-atspi-window-correlation` — correlate an X11/EWMH window with AT-SPI candidates and expose a safe accessibility subtree or structured degraded result.
- Modified capability: `standalone-codex-mcp-plugin` — add the project-owned `x11_accessibility_tree` MCP tool to the standalone tool surface.

## Research refresh

Date: 2026-05-31.

Sources checked:

- Project context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `backlog/00-research-reuse-map.md`, and `backlog/08-atspi-window-correlation.md`.
- Current target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: branch `main`, clean status; reviewed `computer-use-linux/src/atspi_tree.rs`, `server.rs`, `terminal.rs`, `diagnostics.rs`, `windowing/types.rs`, and `windowing/target.rs`.
- Current standalone project: branch `main`, clean before change scaffold; reviewed `src/cli.rs`, `src/list_windows.rs`, `src/input.rs`, `src/mcp.rs`, and existing CLI/MCP tests.
- Current docs and crates: docs.rs `atspi` 0.29/0.30 family documents `AccessibilityConnection`, `ObjectRef`, component extents, roles, and state APIs; Ubuntu AT-SPI DBus reference documents Accessible `Name`, `Parent`, `ChildCount`, `GetChildren`, `GetRole`, states, and Component `GetExtents` / `GetAccessibleAtPoint`.
- External references via web/GitHub metadata: `Touchpoint-Labs/Touchpoint` (MIT, updated 2026-05-30), `BeckhamLabsLLC/linux-desktop-mcp` (MIT), `tak-uukti/linux-computer-use` (MIT), `wimi321/linux-computer-use-skill` (MIT), `MONTBRAIN/vadgr-computer-use` (Apache-2.0), and `joe223/sootie` (license metadata `other`). GitHub repo search did not surface a more relevant current AT-SPI/MCP project.
- Local live probes: Cinnamon/X11 session is active (`XDG_CURRENT_DESKTOP=X-Cinnamon`, `XDG_SESSION_TYPE=x11`, `DISPLAY=:0`); `gsettings org.gnome.desktop.interface toolkit-accessibility` is `true`; `org.a11y.Bus` is present on the user bus; `gdbus`, `busctl`, and `python3` are installed.

Ideas accepted:

- Reuse the target repo's separation between AT-SPI tree extraction and MCP/server filtering, but keep this change standalone and command-testable.
- Treat D-Bus process PID as one signal, not the only truth; only reliable sidecar PID metadata may produce high confidence by itself.
- Prefer explicit `ambiguous` / `degraded` report states over returning an arbitrary subtree.
- Start with public CLI/MCP fixture tests for the matcher and a conservative live smoke/degraded path.

Ideas rejected or deferred:

- Do not patch the target checkout in this stage; source overlay remains backlog `06`/later integration work.
- Do not copy code from external projects; current work uses only ideas and public API behavior.
- Do not add a Cinnamon/Muffin extension or native X11 accessibility bridge; v1 stays generic X11/EWMH plus AT-SPI.
- Do not require browser CDP integration for this stage; browser multi-process behavior is represented by matching title/class/bounds when PID differs.

Risks / unknowns:

- AT-SPI availability and toolkit coverage vary by app; GTK apps are expected to expose better trees than some browser/electron surfaces.
- Window PID can be a wrapper, launcher, browser broker, or terminal child mismatch; confidence must account for PID reliability and non-PID signals.
- Standalone live AT-SPI collection should fail as a structured degraded result if Python GI/AT-SPI bindings are unavailable, while pure matcher tests still validate behavior.

## Impact

- Affected implementation: root Rust crate under `src/`, especially CLI dispatch, a new AT-SPI correlation module, MCP tool definitions/call handling, and tests.
- Affected OpenSpec specs: new `openspec/specs/x11-atspi-window-correlation/spec.md` after archive and modified `openspec/specs/standalone-codex-mcp-plugin/spec.md` tool list.
- Verification: OpenSpec validation plus `make fmt`, `make check`, `make test`; focused CLI/MCP fixture tests and live/degraded Cinnamon/X11 smoke.
- Architecture constraints: keep backend id `x11-ewmh`, keep source checkout read-only, preserve standalone plugin `x11_*` tool naming, and obey no-secrets policy. No external secret values or `.secrets.local.env` access are required.
