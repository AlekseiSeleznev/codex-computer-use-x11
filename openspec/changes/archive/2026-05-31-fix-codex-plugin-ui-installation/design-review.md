## Context Read

- `proposal.md`, spec delta, `grill.md`, and `design.md` for this change.
- `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and `adr/README.md` for project rules and in-force decisions.
- `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/e2e/codex-x11-e2e.py`, `tests/plugin_installer.rs`, `tests/mcp_server.rs`.
- Installed plugin cache and bundled `computer-use` manifest for UI metadata shape comparison.
- Target checkout examples for desktop environment hydration allowlist and fallback strategy.

## Design Summary

- The design updates generated user-local plugin metadata and assets without touching bundled plugin namespaces.
- The design adds MCP startup-only desktop env hydration, preserving CLI no-display behavior and explicit caller env.
- The design extends existing installer and MCP public-interface tests rather than relying on manual UI inspection alone.
- The design keeps user-selected identity: developer/author `AlekseiSeleznev`, website GitHub repo, no privacy/terms links.

## Question Loop

- **Question:** Could metadata-only changes be enough without MCP env hydration?
  - **Recommended answer:** No.
  - **Rationale:** Read-only tool calls showed Codex can discover the plugin but `x11_doctor` inside MCP reports `DISPLAY` unset; UI correctness alone would still block real X11 tasks.
  - **Resolution:** Keep hydration in scope.

- **Question:** Should hydration run for every CLI command?
  - **Recommended answer:** No, run at MCP startup only.
  - **Rationale:** Existing CLI tests and headless behavior intentionally report degraded no-display state; the integration bug is Codex-spawned MCP env. Keeping CLI behavior unchanged reduces regression risk.
  - **Resolution:** Hydration boundary remains MCP startup.

- **Question:** Is copying a logo asset safe?
  - **Recommended answer:** Yes if project-owned and tracked in this repo.
  - **Rationale:** The design explicitly avoids copying bundled plugin assets and places the asset under the owned plugin namespace during install.
  - **Resolution:** Add project-owned icon and tests for installed path.

## Design Findings

- The installed user-local cache can be stale even when repo code is current; fake/live smokes must validate installed binary tool discovery, not just manifest files.
- `tests/mcp_server.rs` starts MCP with `DISPLAY` removed; new hydration tests must be deterministic and must not accidentally read the live developer session unless a fixture enables it.
- A config/env opt-out or fixture path is needed for tests that expect headless behavior.
- Installed metadata should be checked for absence of stale owner strings globally, not only the `homepage` field.
- The implementation should avoid logging or serializing environment variable values beyond existing doctor fields; test assertions should use booleans/status, not private values.

## Document Updates Applied

- No proposal/spec/design updates required after review; current artifacts already include the findings.

## Document Updates Required Before Next Gate

- None.

## ADR Candidates

- No durable ADR candidate. This is a reversible implementation detail within existing standalone plugin architecture and local secret/env handling constraints.

## Open Questions

None
