## Why

The upstream `ilysenko/codex-desktop-linux` maintainer rejected a core Computer Use rewrite and requested a fully opt-in, disabled-by-default Linux Feature adapter instead. This repository must remain the source of truth while providing release artifacts, checksums, documentation, and a copyable thin adapter scaffold that make a later upstream PR small and safe.

## What Changes

- Add release packaging/checksum support for the current `VERSION` that emits a self-contained Codex plugin tarball with executable binary, `.mcp.json`, `.codex-plugin/plugin.json`, icon asset, and release metadata.
- Add an adapter contract document for `linux-features/x11-ewmh-computer-use/` that records maintainer constraints: disabled by default, no core Computer Use replacement, no bundled default plugin, no global doctor changes, no submodules, and two staging modes (pinned release + checksum or local checkout build via `CODEX_X11_COMPUTER_USE_SOURCE`).
- Add a copyable downstream adapter scaffold under this repository for a later upstream PR, including `feature.json`, `README.md`, `stage.sh`, conservative plugin gate patching, and self-contained Node tests modeled after upstream `read-aloud-mcp`.
- Update README, install, changelog, and docs cross-links to describe adapter readiness without claiming upstream integration is merged.
- Add tests proving release artifact integrity, checksum correctness, manifest consistency, forbidden-file exclusion, and adapter scaffold contract consistency.
- Do not publish a GitHub release, modify the upstream checkout, replace bundled `computer-use`, or archive this OpenSpec change in this change.

## Capabilities

- New capability: `x11-release-adapter-handoff` — release artifact, adapter contract documentation, and downstream Linux Feature scaffold for the optional `codex-desktop-linux` handoff.
- Modified capability: `x11-packaging-docs-upstreaming` — documentation and release checklist links must include adapter-ready packaging/docs without overstating upstream status.
- Modified capability: `standalone-codex-mcp-plugin` — packaged plugin bundle layout must remain consistent with the existing standalone installer-generated MCP/plugin metadata.

## Impact

- Code/scripts: new release packaging script, shared or reused plugin bundle generation path where practical, and adapter scaffold shell/Node files.
- Tests: Rust integration tests for packaging/docs/scaffold consistency and self-contained Node tests in the scaffold.
- Docs: `README.md`, `INSTALL_CODEX.md`, `CHANGELOG.md`, and new `docs/codex-desktop-linux-x11-ewmh-adapter.md`.
- OpenSpec: new change-local specs and full intent-driven artifacts through `tasks.md` before implementation.
- Architecture constraints: preserve standalone `codex-computer-use-x11` identity, `x11_*` namespacing, Cinnamon/X11 `x11-ewmh` baseline, X11-only doctor readiness semantics, rollback-first install safety, and source-overlay/upstream separation from ADRs 0008-0011.
- External systems/secrets: no credentials or `.secrets.local.env` are needed; upstream checkout is read-only research only. Publishing a release, pushing, opening PRs, merging, or archiving require separate explicit approval.
