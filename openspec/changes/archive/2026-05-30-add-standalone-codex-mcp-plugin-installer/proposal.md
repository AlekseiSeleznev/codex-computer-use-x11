## Why

The standalone X11/EWMH CLI now has doctor, window listing, focused-window, and verified focus commands, but Codex cannot exercise them as first-class tools without a local MCP plugin. This change adds a user-local Codex plugin and MCP server mode so `x11_*` tools can be smoke-tested from Codex without replacing the bundled `computer-use` plugin or mutating the target checkout.

## What Changes

- Add `codex-computer-use-x11 mcp` stdio server mode with deterministic `x11_doctor`, `x11_list_windows`, `x11_focused_window`, and `x11_focus_window` tools.
- Add a user-local Codex plugin bundle installer at `scripts/install-codex-plugin.sh`, including `--dry-run`, owned marketplace metadata, plugin cache entries, `.codex-plugin/plugin.json`, `.mcp.json`, and config enablement for a separate namespace.
- Add `scripts/uninstall-codex-plugin.sh` that removes only the owned namespace/cache/metadata/config entries and leaves bundled or unrelated plugins untouched.
- Use a standalone namespace `codex-computer-use-x11` with tool names prefixed by `x11_` so this plugin cannot mask `computer-use@openai-bundled` or its MCP server.
- Extend tests, README guidance, and smoke evidence for MCP introspection/calls, installer dry-run behavior, idempotent install, safe uninstall, and live user-local plugin install verification.
- No breaking change: existing CLI commands and bundled Codex Computer Use plugin behavior remain unchanged.

## Capabilities

- New capability: `standalone-codex-mcp-plugin` covering the standalone MCP server mode, `x11_*` tool registry/call behavior, user-local plugin installation, marketplace/cache metadata, idempotence, dry-run safety, uninstall safety, and verification instructions.
- Existing capabilities consumed: `doctor-cli`, `x11-window-listing`, and `x11-active-window-focus` provide the underlying JSON reports used by the MCP tools.

## Impact

- Affected code: root Rust CLI/library under `src/`, integration tests under `tests/`, user-facing scripts under `scripts/`, README documentation, and OpenSpec artifacts/specs.
- Affected runtime surface: new `codex-computer-use-x11 mcp` stdio command and user-local Codex plugin files under `$CODEX_HOME` during live install; no `/opt` writes and no target-checkout writes.
- Required technologies and verification: Rust 2021, root Cargo/Makefile checks, shell scripts, OpenSpec validation, fixture-backed TDD, and no secret access.
- Architecture/ADR constraints: preserve `x11-ewmh` backend vocabulary, standalone-before-source-overlay delivery, OpenSpec as source of truth, and local-secret boundaries; session Claude review is disabled by user request.
- External systems/secrets: no credentialed external systems are required. The target checkout and installed Codex plugin layout are inspected read-only for compatibility research.

## Research refresh (2026-05-30)

- Project state: `/home/as/ai-projects/codex-computer-use-x11` was on `main`; `openspec list --json` showed no active changes before this scaffold; session Claude review was already disabled.
- Target repo state: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` was on `main` with clean status. Files inspected read-only included `computer-use-linux/src/server.rs`, `windowing/{types,target,registry}.rs`, `diagnostics.rs`, `atspi_tree.rs`, `screenshot.rs`, `plugins/openai-bundled/plugins/computer-use/.codex-plugin/plugin.json`, `.mcp.json`, `scripts/lib/bundled-plugins.sh`, and `launcher/start.sh.template`.
- Current Codex plugin layout inspected read-only: `$HOME/.codex/plugins/cache/openai-bundled/computer-use/...`, `$HOME/.codex/.tmp/bundled-marketplaces/openai-bundled/.agents/plugins/marketplace.json`, `/opt/codex-desktop/resources/plugins/openai-bundled/...`, and non-secret plugin/marketplace sections in `$HOME/.codex/config.toml`.
- Fresh external sources checked: official MCP transport and tools specs for stdio JSON-RPC and `tools/list`/`tools/call`; official `openai/plugins` GitHub README/manifests for `.codex-plugin/plugin.json` and optional `.mcp.json`; `BeckhamLabsLLC/linux-desktop-mcp` README/source overview for Linux desktop MCP tool naming/UX patterns and MIT license posture.
- Ideas used: stdio MCP over line-delimited JSON-RPC; deterministic tool list ordering; plugin manifests with `.codex-plugin/plugin.json` plus `.mcp.json`; local marketplace root containing `.agents/plugins/marketplace.json` and `plugins/<name>` links; separate namespace and `x11_` tool names for collision avoidance.
- Ideas rejected: writing into `/opt/codex-desktop`, using `openai-bundled` as the namespace, replacing or modifying bundled `computer-use`, copying external project code, and treating uncertain marketplace/cache details as a blocker for CLI/MCP smoke tests.
- Risks/unknowns: Codex plugin cache/marketplace internals may change; therefore tests model only the observed local format, installer writes are reversible and owned, and README will include a fallback direct MCP smoke path (`codex-computer-use-x11 mcp`) plus exact refresh/restart guidance when plugin UI loading cannot be proven in-process.
