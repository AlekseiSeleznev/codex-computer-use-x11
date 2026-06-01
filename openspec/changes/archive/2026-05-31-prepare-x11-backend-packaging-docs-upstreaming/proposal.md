## Why

The X11/EWMH standalone plugin, source overlay, and e2e evidence now exist, but the public documentation is still a running implementation summary rather than an installable, attributable, upstream-ready handoff. This change prepares the project for users and future upstream reviewers by making packaging, rollback, troubleshooting, license boundaries, and upstream target ownership explicit.

## What Changes

- Add user-facing documentation for the two supported v1 delivery paths: standalone user-local Codex MCP plugin and reversible Codex Desktop Linux source overlay.
- Add command-snippet and dry-run coverage so README/docs examples stay aligned with actual scripts and CLIs.
- Add troubleshooting guidance for Cinnamon/X11 readiness, plugin installation, source-overlay drift, e2e evidence, and safe rollback.
- Add architecture/upstreaming documentation that keeps backend code ownership (`agent-sh/computer-use-linux`) separate from packaging/integration ownership (`codex-desktop-linux-full`).
- Add license/attribution notes that distinguish invoking external runtime commands from copying/vendoring source code, and classify copy-safe versus copy-unsafe references.
- Add a release checklist for v1 handoff evidence and user-safe rollback.
- No **BREAKING** changes: runtime CLI/MCP behavior and source-overlay semantics remain unchanged except for documentation/test coverage around them.

## Research Refresh — 2026-05-31

### Repositories, files, and docs checked

- Local project state: `git status --short` on `main` was clean before scaffolding this change; current docs/scripts reviewed include `README.md`, `docs/e2e-harness.md`, `docs/integration-contract.md`, `scripts/install-codex-plugin.sh`, `scripts/uninstall-codex-plugin.sh`, `scripts/status-codex-source-overlay.sh`, `scripts/install-codex-source-overlay.sh`, `scripts/uninstall-codex-source-overlay.sh`, and `scripts/e2e/codex-x11-e2e.py`.
- Target checkout state: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is on `main` with clean `git status --short`; reviewed `computer-use-linux/src/windowing/{types.rs,registry.rs,target.rs}`, `computer-use-linux/src/server.rs`, `computer-use-linux/src/diagnostics.rs`, `computer-use-linux/src/atspi_tree.rs`, `computer-use-linux/src/screenshot.rs`, target `README.md`, `CHANGELOG.md`, `AGENTS.md`, and `linux-features/README.md`.
- External license/source refresh through `gh api repos/<owner>/<repo>/license --jq .license.spdx_id` and `gh repo view`: `agent-sh/computer-use-linux` MIT, `tak-uukti/linux-computer-use` MIT, `wimi321/linux-computer-use-skill` MIT, `BeckhamLabsLLC/linux-desktop-mcp` MIT, `Touchpoint-Labs/Touchpoint` MIT, `MONTBRAIN/vadgr-computer-use` Apache-2.0, `go-vgo/robotgo` Apache-2.0, `joe223/sootie` NOASSERTION, `hightemp/go_computer_use_mcp_server` no license endpoint, `linuxmint/cinnamon` GPL-2.0, `linuxmint/muffin` GPL-2.0, `linuxmint/wayland` no license endpoint, `linuxmint/cinnamon-spices-extensions` GPL-2.0, `Conservatory/wmctrl` GPL-2.0, `jordansissel/xdotool` BSD-3-Clause, `ReimuNotMoe/ydotool` AGPL-3.0, `psychon/x11rb` Apache-2.0, `github/github-mcp-server` MIT.
- Web refresh checked current public docs/search results for GitHub MCP server configuration and token/security guidance, Linux desktop MCP references, and current package/license metadata for `wmctrl`, `xdotool`, `ydotool`, and `x11rb`.

### Ideas used

- Keep README as a quick-start and posture document, with deeper install/uninstall/troubleshooting/release/upstreaming content in `docs/`.
- Mirror the target repo's documentation style: explicit feature status, supported platforms/scope, quick install commands, rollback/cleanup guidance, and changelog/release-oriented checklists.
- Treat `agent-sh/computer-use-linux` and the local `codex-desktop-linux-full` checkout as the primary compatible integration lineage; keep third-party projects as reference/ideas unless copied code has explicit compatible license review.
- Document runtime command invocation (`wmctrl`, `xdotool`, `ydotool`) separately from source copying or vendoring.

### Ideas rejected or deferred

- Do not add native `.deb`/`.rpm`/AppImage packaging in this stage; the standalone user-local plugin and source overlay remain the delivery paths for this repo.
- Do not copy code from NOASSERTION, no-license, GPL, or AGPL sources; they remain ideas-only unless a later explicit license decision changes scope.
- Do not merge backend-upstream and packaging/wrapper-upstream targets; backend work belongs toward `agent-sh/computer-use-linux`, while Codex Desktop packaging and feature wiring belong in `codex-desktop-linux-full`.
- Do not require Cinnamon Wayland support or a Cinnamon/Muffin extension for v1 docs; the v1 support statement stays Cinnamon/X11-first with generic X11/EWMH internals.

### Risks and unclear areas

- Current `gh search repos` queries did not reveal a clearly superior new Linux/X11 computer-use MCP project beyond the already-tracked references; future release work should repeat the search before copying code or claiming ecosystem coverage.
- Upstream ownership may shift if `agent-sh/computer-use-linux` and `codex-desktop-linux-full` reorganize their plugin/package boundaries; docs should describe the current target matrix and the rule for revalidating it.
- Some live commands (`cargo run -- doctor --json`, source-overlay live smoke, plugin live install) are machine-dependent; docs and tests must distinguish deterministic fake/dry-run checks from optional live evidence.

## Capabilities

- `x11-packaging-docs-upstreaming` — New capability covering installation/rollback documentation, command-snippet verification, troubleshooting, license/attribution policy, upstream target matrix, and release checklist behavior for the X11/EWMH v1 handoff.

## Impact

- Documentation: `README.md` and new or updated files under `docs/` for install/uninstall, troubleshooting, architecture/upstreaming, license/attribution, and release checklist.
- Tests/checks: add public-interface documentation checks, command-snippet checks, attribution policy checks, upstream target matrix checks, and source-overlay rollback doc checks, likely under `tests/` or a docs-check helper invoked by `cargo test`/`make test`.
- OpenSpec: new delta spec under `openspec/changes/prepare-x11-backend-packaging-docs-upstreaming/specs/x11-packaging-docs-upstreaming/spec.md`.
- Architecture constraints: keep `x11-ewmh` generic X11/EWMH naming, source-overlay fallback after desktop-specific backends, explicit X11 root coordinate model from ADR 0008, and no secret values in tracked docs.
- External systems/secrets: no real secrets needed; GitHub license/source refresh uses public repo metadata and local GitHub auth only through `gh` without copying token values into artifacts.
- Verification: run OpenSpec strict validation plus project checks (`make fmt`, `make check`, `make test`) and documentation-specific checks before archive.
