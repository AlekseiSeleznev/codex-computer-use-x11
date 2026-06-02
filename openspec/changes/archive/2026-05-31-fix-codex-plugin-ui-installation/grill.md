## Context Read

- `CONSTITUTION.md` — required Rust/Cargo stack, user-local plugin safety, secret handling, verification rules.
- `CONTEXT.md` — project glossary for standalone plugin, source overlay, E2E harness, capability matrix evidence, final DoD language.
- `ARCHITECTURE.md` and `adr/README.md` — current v1 Cinnamon/X11 baseline, standalone plugin/source-overlay boundaries, no bundled Computer Use replacement.
- `openspec/specs/standalone-codex-mcp-plugin/spec.md` — existing MCP tool surface and installer contract.
- `openspec/specs/codex-x11-e2e-test-harness/spec.md` — fake/live plugin smoke and evidence expectations.
- `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/e2e/codex-x11-e2e.py` — current installer and smoke implementation.
- `tests/plugin_installer.rs`, `tests/mcp_server.rs` — public-interface tests for installer metadata and MCP tool discovery.
- Current installed user-local plugin under `$HOME/.codex/plugins/cache/codex-computer-use-x11/...` — read-only inspection showed stale metadata and six-tool binary.
- Codex Desktop Linux target checkout diagnostics environment hydration code in `computer-use-linux/src/diagnostics.rs` and launcher env import logic — reference for safe local desktop env recovery.

## Plan Summary

- The proposal fixes the standalone project-owned plugin path, not bundled OpenAI `Computer Use`.
- The UI contract is concrete: `X11 Computer Use`, developer/author `AlekseiSeleznev`, GitHub repo website, no invented privacy/terms links, project-owned icon.
- The runtime contract is concrete: reinstall must expose the current fourteen `x11_*` tools and reject stale six-tool installs.
- MCP desktop env hydration is needed because Codex currently starts the project plugin without `DISPLAY`, while the shell CLI can query X11.
- Secret policy is preserved: hydration may use local graphical env variables, but must not print secret values or write them to tracked files.

## Question Loop

- **Question:** Should this change replace bundled `Computer Use` or remain a separate plugin card?
  - **Recommended answer:** Keep it separate as `X11 Computer Use`.
  - **Rationale:** Existing architecture and README define standalone plugin as a validation path without replacing bundled Computer Use; replacing bundled cache would violate owned namespace and rollback rules.
  - **Resolution:** Separate plugin card retained.

- **Question:** What author/developer value should Codex UI show?
  - **Recommended answer:** `AlekseiSeleznev`.
  - **Rationale:** It matches the actual GitHub owner and the user's selected preference.
  - **Resolution:** Use `AlekseiSeleznev` in author and interface developer fields.

- **Question:** Should privacy/terms rows be added to mimic bundled plugins?
  - **Recommended answer:** Omit them until project-owned legal docs exist.
  - **Rationale:** Pointing to unrelated OpenAI/GitHub policies would be misleading; the user selected this option.
  - **Resolution:** Omit `privacyPolicyURL` and `termsOfServiceURL`.

- **Question:** Should MCP hydration hard-code this machine's `DISPLAY=:0`?
  - **Recommended answer:** No; recover from local systemd/process desktop env and preserve explicit caller env.
  - **Rationale:** Hard-coding would break other desktops/sessions and could violate portability; target code already uses local env discovery patterns.
  - **Resolution:** Hydrate from local non-secret env sources only, never overwrite explicit non-empty values.

## Resolved Terms

- No new glossary terms required. Existing glossary terms `Standalone plugin`, `E2E harness`, `Capability matrix evidence`, and `Final DoD` cover this change.

## Document Updates Applied

- Proposal created for `fix-codex-plugin-ui-installation`.
- Spec delta added under `standalone-codex-mcp-plugin` covering UI metadata, icon, current tool surface, MCP desktop env hydration, and smoke validation.

## Document Updates Required Before Next Gate

- None.

## ADR Candidates

- No durable ADR candidate. The change applies existing standalone plugin and local-secret/env-boundary decisions without changing architecture.

## Open Questions

None
