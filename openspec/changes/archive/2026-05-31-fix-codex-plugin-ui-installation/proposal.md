## Why

The user-local Codex installation of `codex-computer-use-x11` is stale: Codex discovers the plugin as `X11 Computer Use`, but the installed cache still exposes only the early six-tool MCP surface, has stale repository metadata, and starts the MCP process without the desktop environment needed to query X11 windows. Before real Computer Use task testing, the plugin must display and run correctly inside the current Codex app.

## What Changes

- Refresh standalone plugin manifest and marketplace metadata so the Codex UI card shows `X11 Computer Use`, developer/author `AlekseiSeleznev`, and the actual GitHub repository URL.
- Add a project-owned plugin icon and install it into the user-local plugin bundle.
- Keep Privacy Policy and Terms links omitted until project-owned legal docs exist; do not point to unrelated policies.
- Ensure reinstalling the plugin refreshes the copied binary and exposes the full current fourteen-tool `x11_*` MCP surface.
- Add safe MCP desktop environment hydration so the plugin can recover required graphical session variables, such as `DISPLAY`, when Codex starts the plugin with a sparse environment.
- Extend tests and e2e smoke coverage for metadata, icon installation, tool discovery, and environment hydration.

## Capabilities

- Modified capability: `standalone-codex-mcp-plugin`.

## Impact

- Affected files: `scripts/install-codex-plugin.sh`, `tests/plugin_installer.rs`, `src/doctor.rs` or a small runtime env helper, `tests/mcp_server.rs`, e2e plugin smoke validation, and a project-owned asset under `assets/`.
- Public interface impact: no stock/bundled Computer Use tool names change; the standalone plugin remains namespaced as `x11_*` and remains separate from bundled `Computer Use`.
- Verification impact: Rust changes must pass `make fmt`, `make check`, and `make test`; plugin changes must pass fake/live plugin smoke where the environment permits it; OpenSpec validation must pass.
- Constitution/architecture constraints: keep Rust 2021/Cargo, user-local plugin writes only under owned `$CODEX_HOME` namespace, no `/opt` or `openai-bundled` writes, no secrets in tracked files or logs, and preserve the documented Cinnamon/X11 `x11-ewmh` baseline.
