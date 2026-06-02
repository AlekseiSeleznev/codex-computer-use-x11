## Why

The previous side-by-side X11 settings row can be present in the patched asset while the live Codex Desktop `Settings -> Computer use` page still shows only the Google Chrome fallback. We need a diagnosable baseline for the bundled `Any App` provider and a safe takeover mode where the local X11 provider intentionally replaces the bundled Computer Use row in the settings UI without globally masquerading the standalone plugin as `computer-use`.

## What Changes

- Add runtime diagnostics that record the real Computer Use settings payload: `availablePlugins`, `installedPlugins`, `computerUseAvailability`, provider row decisions, and feature-gate inputs relevant to the bundled `Any App` row.
- Introduce an X11 provider takeover mode for Codex Desktop Linux settings in which the bundled Computer Use row is hidden or disabled and `codex-computer-use-x11` is rendered as the active Computer Use provider row.
- Keep the standalone marketplace/plugin identity (`codex-computer-use-x11@codex-computer-use-x11`) intact; any compatibility alias or shim is localized to the settings/provider resolver layer.
- Extend the project installer path so this repository can apply the takeover overlay to `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` with `--provider x11 --mode takeover`, including live-asset backups, rollback/restore, a patch report, and restart guidance.
- Add rollback behavior that restores bundled Computer Use mode and removes only owned takeover/live-asset mutations.
- No implementation of target code or live asset mutation occurs until the mandatory planning artifacts are complete.

## Capabilities

- New capability: `codex-computer-use-provider-takeover` — diagnostic and provider-resolver behavior for baseline bundled Computer Use and X11 takeover in Codex Desktop Linux settings.
- Modified capability: `codex-computer-use-settings-ui` — supersedes the previous side-by-side row assumption with a takeover-mode row decision that can hide the bundled row and render the X11 provider as the active Computer Use control.
- Modified capability: `codex-source-overlay-extension` — extends the overlay/installer contract to apply, report, back up live assets for, and roll back a target-repo settings/provider takeover overlay.
- Modified capability: `standalone-codex-mcp-plugin` — continues to provide the owned X11 provider identity and metadata without renaming the global plugin id or rewriting bundled marketplace ownership.

## Impact

- Source OpenSpec/project repo: `/home/as/ai-projects/codex-computer-use-x11`, especially installer/overlay scripts, tests, and this change under `openspec/changes/replace-bundled-computer-use-with-x11-provider/`.
- Target checkout: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`, especially Computer Use settings webview patchers, provider-resolution glue, patcher tests, smoke tests, and any local live-asset application path.
- Live app assets: `/opt/codex-desktop/content/webview/assets/computer-use-settings-*.js` may be backed up and patched only by explicit installer tasks after planning; backup and rollback reports are required.
- Architecture constraints: OpenSpec remains the source of truth; ADR 0009 keeps standalone `x11_*` tool identity; any takeover shim must be local to settings/provider UI rather than a global marketplace/plugin-id masquerade; ADR 0007 permits safe local checkpoint commits but not push/archive without explicit approval.
- Verification: OpenSpec validation, target fixture tests for baseline and takeover rows, installer/rollback tests, patch-report checks, git-status checks for both repos, and recorded live/degraded UI verification. No secrets or external credentials are needed.
