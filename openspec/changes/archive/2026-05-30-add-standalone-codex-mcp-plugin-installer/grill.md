## Context Read

- Root project rules and context: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`.
- Change artifacts: `openspec/changes/add-standalone-codex-mcp-plugin-installer/proposal.md`, `openspec/changes/add-standalone-codex-mcp-plugin-installer/specs/standalone-codex-mcp-plugin/spec.md`.
- Backlog/research context: `backlog/00-research-reuse-map.md`, `backlog/05-standalone-mcp-plugin-install.md`.
- Current implementation: `src/cli.rs`, `src/doctor.rs`, `src/list_windows.rs`, `src/focus.rs`, `src/x11_id.rs`, `tests/*_cli.rs`, `README.md`, `Cargo.toml`, `Makefile`.
- Target compatibility research, read-only: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/computer-use-linux/src/server.rs`, `windowing/{types,target,registry}.rs`, `diagnostics.rs`, `atspi_tree.rs`, `screenshot.rs`, bundled plugin manifests, `scripts/lib/bundled-plugins.sh`, and `launcher/start.sh.template`.
- Current local Codex plugin layout, read-only: `~/.codex/plugins/cache/openai-bundled/computer-use`, `~/.codex/.tmp/bundled-marketplaces/openai-bundled/.agents/plugins/marketplace.json`, `/opt/codex-desktop/resources/plugins/openai-bundled`, and non-secret plugin/marketplace sections in `~/.codex/config.toml`.
- External docs/projects checked 2026-05-30: official MCP stdio transport and tools specs, official `openai/plugins` repository README/manifests, and `BeckhamLabsLLC/linux-desktop-mcp` README/source overview.

## Plan Summary

- Add `codex-computer-use-x11 mcp` as a stdio MCP mode that exposes only `x11_doctor`, `x11_list_windows`, `x11_focused_window`, and `x11_focus_window`.
- Implement the MCP tools as thin wrappers over the existing standalone JSON capabilities, preserving `x11-ewmh`, focus verification, and degraded diagnostics rather than creating a second behavior model.
- Install the plugin into a user-local, owned `codex-computer-use-x11` namespace with its own cache, marketplace root, `.codex-plugin/plugin.json`, `.mcp.json`, and Codex config entries.
- Keep the plugin separate from `openai-bundled` and stock `computer-use`; never write `/opt` or the target checkout.
- Make install reversible and fixture-testable through `CODEX_HOME`/config overrides, dry-run mode, idempotent repeated install, and uninstall that removes only owned paths/sections.
- Provide direct MCP stdio smoke evidence so backend progress does not depend on whether the running Codex process hot-loads newly installed plugin tools.

## Question Loop

### Q1: Should this standalone plugin reuse the stock `computer-use` plugin name or MCP tool names?

- **Recommended answer:** No. Use the owned namespace `codex-computer-use-x11` and `x11_*` tool names.
- **Rationale:** The backlog explicitly requires non-conflicting `x11_*` tools; the installed bundled plugin is `computer-use@openai-bundled`; masking it would make rollback and diagnosis unsafe.
- **Resolution:** Captured in the spec as `x11_*` tools only and an owned namespace. No user question required.

### Q2: Is copying a plugin folder enough for current Codex to load it?

- **Recommended answer:** No. The installer should also create marketplace metadata and user-local Codex config entries.
- **Rationale:** Current local Codex config contains `[marketplaces.*]` and `[plugins."plugin@marketplace"]` entries; launcher code stages bundled plugins into cache plus a marketplace root. A random folder in the repo or cache would not be a reliable installed plugin.
- **Resolution:** Captured in the spec and design scope. No user question required.

### Q3: Where should the standalone marketplace live?

- **Recommended answer:** Use an owned marketplace root under `$CODEX_HOME/plugins/marketplaces/codex-computer-use-x11`, with cache under `$CODEX_HOME/plugins/cache/codex-computer-use-x11/codex-computer-use-x11/<version>`.
- **Rationale:** Observed bundled marketplace roots use `.agents/plugins/marketplace.json` plus `plugins/<plugin>` links, but the launcher specifically clears only bundled `openai-bundled` temp roots. An owned persistent root avoids `/opt`, avoids `openai-bundled`, and remains removable by namespace.
- **Resolution:** Use this layout in design/tests. No user question required.

### Q4: Should installer tests write into the real `HOME`?

- **Recommended answer:** No. Tests must use temporary `CODEX_HOME` and config-file overrides; only the final live install smoke may write the user's real `$CODEX_HOME`, and the user already requested live install/rollback capability.
- **Rationale:** The constitution requires secret/local safety; backlog acceptance explicitly says installer tests must not touch real HOME.
- **Resolution:** Captured in the spec. No user question required.

### Q5: Should the MCP server depend on an external Rust MCP framework now?

- **Recommended answer:** No for this stage. Implement a minimal stdio JSON-RPC server using existing `serde_json`, supporting only initialize, initialized notification, tools/list, and tools/call.
- **Rationale:** The current crate has no MCP dependency, the target repo already owns the full `rmcp` integration separately, and this stage needs a fast standalone smoke path. Adding a framework can be revisited if the minimal protocol surface becomes insufficient.
- **Resolution:** Design will use a minimal internal `mcp` module. No user question required.

### Q6: What if Codex cannot hot-load the newly installed plugin in the current process?

- **Recommended answer:** Do not block backend progress. Record direct `mcp` stdio smoke evidence and provide exact Codex refresh/restart/inspection instructions; live install still verifies filesystem/config state and rollback.
- **Rationale:** Plugin discovery may be lazy or process-scoped. The acceptance criteria allow exact verification instructions if tool visibility cannot be proven immediately.
- **Resolution:** Captured in the spec and planned verification. No user question required.

### Q7: Is a durable ADR required for this stage?

- **Recommended answer:** Not unless design review finds a hard-to-reverse architecture decision.
- **Rationale:** The standalone plugin path and user-local namespace are already part of the architecture/backlog posture; this change implements the next delivery slice without superseding existing ADRs.
- **Resolution:** Re-evaluate during `adr.md`; no durable ADR candidate at pre-design. No user question required.

## Resolved Terms

- `Standalone plugin` — existing glossary term for validating this project as its own CLI/MCP integration before source-overlay adaptation.
- `Owned plugin namespace` — the concrete `codex-computer-use-x11` marketplace/cache/config namespace used to keep the standalone plugin separate from bundled Codex plugins. This is implementation-specific enough to remain in artifacts rather than `CONTEXT.md`.
- `Direct MCP stdio smoke` — a verification path that starts `codex-computer-use-x11 mcp` directly and exercises MCP JSON-RPC without installing into real `$CODEX_HOME`.

## Document Updates Applied

None. Existing `CONTEXT.md` already defines `Standalone plugin`; the other resolved terms are change-specific implementation vocabulary rather than durable glossary language.

## Document Updates Required Before Next Gate

None.

## ADR Candidates

No durable ADR candidate at pre-design. Re-evaluate after design review if the installer/cache/config approach becomes a hard-to-reverse project architecture decision.

## Open Questions

None.
