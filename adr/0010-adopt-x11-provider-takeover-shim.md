# 0010 — Adopt X11 provider takeover shim for Computer Use settings

## Status

Accepted

## Date

2026-06-01

## Context

The standalone `codex-computer-use-x11` plugin is installed under its owned marketplace and plugin identity, but Codex Desktop Linux's `Settings -> Computer use` page still does not reliably show the desired X11 provider row. A previous side-by-side settings-row patch added static markers and installed-first lookup, yet live UI feedback still showed only the Google Chrome fallback.

The obvious shortcut would be to make the standalone plugin masquerade globally as the bundled `computer-use` plugin. That would likely match more existing settings assumptions, but it would blur ownership between `openai-bundled/computer-use` and `codex-computer-use-x11`, make rollback dangerous, and conflict with the existing v1 baseline that keeps standalone `x11_*` tools and plugin identity distinct from stock Computer Use tools.

## Decision

Adopt an explicit **X11 provider takeover shim** for the Codex Desktop Linux Computer Use settings surface.

In takeover mode, the settings/provider resolver may select `codex-computer-use-x11` as the active Any App Computer Use provider and hide or disable the bundled `computer-use` settings row for that provider surface. Any compatibility alias needed by existing settings row components must be localized to the settings/provider resolver payload.

Do not globally rename `codex-computer-use-x11` to `computer-use`. Do not rewrite bundled marketplace metadata or bundled cache paths. Do not rename standalone MCP tools out of the `x11_*` namespace as part of settings takeover. Rollback must restore bundled mode without deleting unrelated standalone plugin files.

Baseline bundled Computer Use behavior must remain diagnosable: takeover work should record non-secret plugin payload facts, availability/gate facts, and row decisions before or while replacing the row.

## Considered Options

1. **Localized provider takeover shim** (chosen)
   - Lets the settings page present X11 as the active provider without changing global plugin ownership.
   - Preserves rollback to bundled mode.
   - Makes the takeover decision observable and testable at the row/provider boundary.

2. **Global plugin-id masquerade as `computer-use`**
   - Could satisfy hardcoded UI assumptions quickly.
   - Rejected because it risks overwriting or confusing `openai-bundled/computer-use`, breaks owned standalone identity, and makes rollback/upstream reasoning unsafe.

3. **Continue side-by-side X11 row only**
   - Minimal change and preserves bundled UI.
   - Rejected as insufficient because live evidence showed the side-by-side marker and installed-first lookup were not enough to make the desired provider visible or diagnosable.

4. **Patch live assets only**
   - Useful for local emergency verification.
   - Rejected as the durable design because live assets are generated/root-owned state and can be stale in a running Electron process. Live patching must be backed up, reported, and secondary to source/installer behavior.

## Consequences

- Provider takeover becomes an explicit architecture concept for this project rather than an accidental settings string patch.
- Future settings/provider work must preserve global plugin identity and localize compatibility aliases.
- Installer and rollback logic must distinguish source overlay state, live asset state, and standalone plugin install state.
- Diagnostics become part of the acceptance boundary for Computer Use settings takeover: static patch markers alone are not enough evidence.
- ADR 0009 remains in force; this decision extends its standalone/source-overlay separation to the Computer Use settings provider selection layer.

## Evidence

- OpenSpec change: `openspec/changes/replace-bundled-computer-use-with-x11-provider/`.
- Prior side-by-side attempt: `openspec/changes/show-x11-plugin-in-computer-use-settings/`.
- Live asset observation on 2026-06-01: `/opt/codex-desktop/content/webview/assets/computer-use-settings-aHZZtKP_.js` contains prior X11 patch markers and installed/available lookup literals, but the user still reported that the X11 row is absent in the live settings page.
