## Why

The standalone `codex-computer-use-x11` plugin can install with correct marketplace metadata, but Codex Desktop Linux's `Settings -> Computer use` page only renders hardcoded rows for the bundled `computer-use` and Chrome plugins. Users need the project-owned X11 plugin to appear there as `X11 Computer Use` so real Computer Use testing starts from the same settings surface they already use.

## What Changes

- Add a Codex Desktop Linux webview patch that makes the Computer Use settings page recognize plugin id `codex-computer-use-x11`.
- Render the standalone plugin as a separate side-by-side `X11 Computer Use` control row instead of replacing or masquerading as `openai-bundled/computer-use`.
- Keep the existing bundled `Any App` and `Google Chrome` rows unchanged.
- Add patcher and smoke-test coverage proving the row injection is idempotent and only applies when the settings bundle contains the expected hardcoded Computer Use page shape.
- Document the cross-repo verification boundary and do not expose or store secrets.
- No breaking changes.

## Capabilities

- New capability: `codex-computer-use-settings-ui` — Codex Desktop Linux Computer Use settings page integration for the standalone `codex-computer-use-x11` plugin.
- Modified capability: `standalone-codex-mcp-plugin` — the installed standalone plugin's UI metadata is consumed by the settings row while preserving the owned plugin id and `x11_*` tool namespace.
- Consumed capability: `codex-source-overlay-extension` — the local integration target checkout is allowed to be mutated only by explicit OpenSpec tasks and must remain reversible/cleanly verifiable.

## Impact

- Primary implementation target: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` (`scripts/patches/computer-use.js`, webview patch descriptors/tests, and related smoke tests).
- Current OpenSpec source of truth: `/home/as/ai-projects/codex-computer-use-x11/openspec/changes/show-x11-plugin-in-computer-use-settings/`.
- Runtime behavior: Codex Desktop settings UI gains an extra plugin control row when the local marketplace exposes `codex-computer-use-x11`; MCP tool names and standalone plugin binary behavior do not change.
- Verification: OpenSpec validation in this repository, target patcher tests in `codex-desktop-linux-full`, targeted smoke checks over extracted/minified assets, and final git-status checks for both repositories.
- Architecture constraints: keep the side-by-side standalone plugin identity; do not overwrite `openai-bundled/computer-use`; do not copy secret values; keep target checkout writes limited to this change.
