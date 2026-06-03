## Why

GitHub issue #389 confirms the current thin `linux-features/x11-ewmh-computer-use/` adapter direction, but the latest maintainer discussion adds a second alignment path: if the X11 runtime fits `agent-sh/computer-use-linux`, it may become a selectable backend/flavor with only a small opt-in `codex-desktop-linux` adapter. The project needs to record that guidance so the existing adapter handoff remains accurate without implying a premature core rewrite.

## What Changes

- Update the X11 release adapter handoff contract to distinguish hard upstream constraints from the new optional backend/flavor evaluation path.
- Document that the current thin Linux Feature adapter remains the default upstream-ready path unless a later change proves `agent-sh/computer-use-linux` flavor integration is a better fit.
- Add copyable scaffold README guidance for maintainers explaining when to keep the separate plugin adapter and when to open a separate backend/flavor investigation.
- Add tests that fail if the contract/scaffold omit `agent-sh/computer-use-linux`, selectable backend/flavor, or the no-default-behavior-change boundary.
- No runtime behavior, release artifact layout, MCP tool names, or default enablement changes.

## Capabilities

- Modify `x11-release-adapter-handoff` to cover issue #389 backend/flavor guidance while preserving the existing adapter handoff requirements.

## Impact

- Affected docs/specs: `openspec/specs/x11-release-adapter-handoff/spec.md`, `docs/codex-desktop-linux-x11-ewmh-adapter.md`, and `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/README.md`.
- Affected tests: documentation/packaging tests under `tests/` that validate adapter handoff wording.
- Architecture constraints: ADR 0009 keeps standalone `x11_*` tools namespaced and separates backend vs wrapper PRs; ADR 0010 forbids global masquerading as bundled `computer-use`; the new guidance must not enable the feature by default or change core Computer Use behavior in this repository.
- External systems: GitHub issue #389 is used only as public non-secret requirement context; no secrets are required.
