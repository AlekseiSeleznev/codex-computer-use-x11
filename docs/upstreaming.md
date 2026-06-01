# Upstreaming guide

This repository is a staging and evidence project for `x11-ewmh`. The source overlay is reversible staging evidence, not a long-lived fork. Before PR work, rerun fresh target research before PR work because the target repository boundaries can move.

## Upstream target matrix

The upstream target matrix is the handoff map for backend-upstream and wrapper-integration work.

| Area | Target class | Repository/path | Evidence before PR | Notes |
| --- | --- | --- | --- | --- |
| X11/EWMH window listing, focused-window lookup, focus verification, backend descriptors | backend-upstream | `agent-sh/computer-use-linux` and target `computer-use-linux/` | standalone tests, source-overlay fake/live smoke, target cargo tests | Keep backend id `x11-ewmh`; preserve late fallback order after desktop-specific backends |
| Diagnostics, strict RemoteDesktop method checks, capability-map `x11-ewmh` facts | backend-upstream | `agent-sh/computer-use-linux` and target `computer-use-linux/src/diagnostics.rs` | doctor JSON evidence, portal false-positive fixture evidence | Do not mark empty RemoteDesktop introspection as available |
| `get_app_state` window context, screenshot, AT-SPI correlation, degraded diagnostics | backend-upstream | target `computer-use-linux/src/server.rs`, `screenshot.rs`, `atspi_tree.rs`, `windowing/` | app-state fake/live evidence and degraded-layer notes | Reuse stock target surfaces; do not add a competing bundled `x11_get_app_state` tool |
| Targeted keyboard and pointer safety routed through existing stock tools | backend-upstream | target `computer-use-linux/src/server.rs`, `remote_desktop.rs`, `windowing/target.rs` | focus/bounds verification evidence | Preserve stock tool vocabulary: `activate_window`, `get_app_state`, `type_text`, `press_key`, `click`, `scroll`, `drag` |
| Codex Desktop install/package wiring, launcher behavior, update-manager integration, feature toggles | wrapper-integration | `CODEX_DESKTOP_LINUX_FULL_PATH` / current local `codex-desktop-linux` packaging, launcher, update-manager, linux-features, and bundled plugin staging | wrapper smoke, package/update-manager tests, user-local rollback evidence | Keep this separate from backend implementation PRs |
| Standalone plugin marketplace/cache installer | wrapper-integration or project-local release artifact | this repository first; later target wrapper only if accepted | plugin dry-run/install/uninstall and MCP smoke | Does not modify `/opt`, `openai-bundled`, or bundled `computer-use` cache |

Do not mix backend and wrapper changes in one pull request unless a later OpenSpec design explicitly accepts that coupling. A backend PR should be reviewable without Codex Desktop packaging context; a wrapper PR should be reviewable without changing backend semantics.

## Source-overlay handoff rule

The source overlay is reversible staging evidence, not a long-lived fork. A handoff candidate must show:

1. project `make fmt`, `make check`, and `make test` pass;
2. fake standalone plugin and source-overlay e2e smoke pass;
3. the real target checkout starts clean when live smoke is attempted;
4. source overlay status reports clean, install applies owned marker blocks, target checks pass or report an exact environmental blocker, uninstall removes owned content, and final target `git status --short` is clean;
5. stock target vocabulary remains intact: `activate_window`, `get_app_state`, `type_text`, `press_key`, `click`, `scroll`, `drag`.

## PR preparation checklist

- Re-run license review before copying source code.
- Re-run fresh target research before PR work and record the target commit/branch.
- Keep `x11-ewmh` generic X11/EWMH wording; do not narrow it to Cinnamon-only behavior.
- Keep Cinnamon Wayland and Cinnamon/Muffin extension work out of v1 backend PRs unless a separate design changes scope.
- Include rollback and degraded-mode evidence in PR descriptions.
