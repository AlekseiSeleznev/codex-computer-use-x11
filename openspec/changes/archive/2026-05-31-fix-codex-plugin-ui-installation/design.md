## Context

The repository already implements the standalone `codex-computer-use-x11 mcp` server and installer. The canonical spec requires fourteen namespaced `x11_*` tools, user-local `$CODEX_HOME` installation, and no writes to `/opt`, `openai-bundled`, or bundled `computer-use`. Read-only inspection showed the installed local cache is stale: the installed manifest points at a misspelled GitHub owner, the description only covers early keyboard tools, the installed binary exposes six tools, and the MCP process launched by Codex lacks `DISPLAY` even though the shell CLI can query Cinnamon/X11 windows.

Relevant constraints: Rust 2021/Cargo remain the implementation stack; secret values must not be printed or tracked; local graphical environment variables may be read from local process/systemd state but must not be written into docs, manifests, or logs as secrets; the standalone plugin remains separate from bundled OpenAI `Computer Use`.

## Goals / Non-Goals

**Goals:**

- Make the generated plugin manifest and marketplace metadata render correctly in Codex UI as `X11 Computer Use` by `AlekseiSeleznev` with the actual GitHub website.
- Install a project-owned icon and point `interface.logo` at it.
- Ensure install/reinstall refreshes the copied binary and validates the current fourteen-tool MCP surface.
- Hydrate MCP desktop environment from safe local sources when Codex launches the plugin without required X11/session variables.
- Extend tests and fake/live smoke validation for metadata, icon, tool surface, and hydration behavior.

**Non-Goals:**

- Do not replace or patch bundled `computer-use@openai-bundled`.
- Do not add privacy or terms links without project-owned policy documents.
- Do not hard-code this machine's `DISPLAY`, private paths, tokens, or secrets into tracked files.
- Do not change the public stock Codex tool names; project tools remain `x11_*`.
- Do not archive or push this change without a later explicit user approval.

## Decisions

- **Manifest identity:** Generate `author.name = "AlekseiSeleznev"`, `author.url = "https://github.com/AlekseiSeleznev"`, `homepage` and `interface.websiteURL` as the project GitHub repository, and `interface.developerName = "AlekseiSeleznev"`. This matches user preference and current remote origin.
- **Legal links:** Omit `privacyPolicyURL` and `termsOfServiceURL`; tests will assert absence so they are not accidentally populated from unrelated bundled plugin examples.
- **Icon:** Add a tracked project-owned `assets/app-icon.png`, copy it into `assets/app-icon.png` in the installed bundle, and reference it via `interface.logo`. The icon is not a copied bundled plugin asset.
- **Installer validation:** Keep the existing owned cache/marketplace/config layout. Add generated metadata fields and extend installer tests rather than changing the install location or plugin id.
- **MCP env hydration boundary:** Add a small Rust module invoked at MCP startup before serving requests. It preserves caller-provided non-empty variables and fills only missing desktop variables from local non-secret sources: `systemctl --user show-environment`, parent/desktop process environments, and conventional session-bus fallback when appropriate. CLI one-shot commands keep current no-display degraded behavior unless they already inherit env normally.
- **Test seam:** Add test-only environment controls for deterministic hydration tests, such as fixture files/commands and an opt-out flag. Tests must not rely on the developer's live `systemctl` or `/proc` state.
- **Smoke validation:** Extend `scripts/e2e/codex-x11-e2e.py` metadata validation to fail stale installs with old owner, missing icon, missing website/developer fields, invented privacy/terms links, or missing current tools.

## Risks / Trade-offs

- Hydrating from local process/systemd state may vary by desktop/session manager. The implementation will degrade to existing `DISPLAY` blocker when no safe source is available.
- Reading `/proc/<pid>/environ` can contain many variables; code will only copy an allowlist of desktop variables and will not serialize values into plugin metadata or logs.
- Existing installed Codex processes may require restart/refresh after reinstall; fake and direct MCP smokes remain deterministic even when current UI lazy-loading cannot refresh tools in-place.
- A generated icon increases installer surface slightly, but it keeps UI display self-contained and rollback-safe under the owned plugin namespace.

## Migration Plan

1. Implement tests first for plugin metadata/icon, stale tool-surface detection, and deterministic desktop env hydration.
2. Update installer manifest generation and asset copying.
3. Add MCP startup hydration and diagnostics-safe tests.
4. Extend plugin e2e validation.
5. Run project checks and fake plugin smoke.
6. Reinstall the user-local plugin only after project checks pass, then verify Codex tool discovery/live doctor manually or with available tool calls.
7. Roll back with `scripts/uninstall-codex-plugin.sh` if the UI or MCP startup is not correct.

## Open Questions

None
