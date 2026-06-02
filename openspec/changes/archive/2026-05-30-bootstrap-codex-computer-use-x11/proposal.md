## Why

`codex-computer-use-x11` needs a clean, testable bootstrap contract before any X11/EWMH backend code is written, so later work can integrate with Codex Computer Use Linux without inventing incompatible window models, tool surfaces, or license risk.

This change establishes the standalone project baseline, public contract, and first TDD tracer bullet for a Codex-first, Cinnamon/X11-first, generic X11/EWMH integration path.

## What Changes

- Create a minimal standalone Rust project for `codex-computer-use-x11` without implementing a real `wmctrl`, `xprop`, or `xdotool` backend yet. The initial Rust package/workspace lives at the repository root (`Cargo.toml` plus `src/`) so the standalone crate, CLI, and future MCP server share one obvious entry point; subcrates may be introduced later only if design requires them.
- Add a CLI skeleton with binary name `codex-computer-use-x11` and a public `doctor --json` command that returns valid JSON identifying the project and baseline readiness. The minimum report includes `project`, `version`, `backend`, `readiness`, `capabilities`, and `checks`; `checks` has a bootstrap array-of-objects shape, while exact future check names/extensions remain design-owned. This standalone bootstrap report is a separate smoke-test surface that loosely mirrors upstream readiness concepts; it is not a strict subset of the target repo `doctor_report()` unless a later design/ADR says so. It must not perform real X11 probes during this bootstrap stage.
- Add a shared X11 window-id normalizer so hex forms such as `0x5624b36` and `0x05624b36` map to the same canonical `u64` value.
- Add project-level `make test`, `make check`, and `make fmt` commands for repeatable local verification. These are thin wrappers around `cargo test`, `cargo check`, and `cargo fmt -- --check` so CI or developers may still call Cargo directly.
- Document the integration contract for future source-overlay work:
  - upstream `WindowInfo` is the primary window model;
  - future X11/EWMH windows use `WindowInfo.backend = "x11-ewmh"`;
  - diagnostics such as PID reliability, raw command source, warnings, and degraded observations live in a sidecar/report rather than extending upstream `WindowInfo` by default;
  - standalone crate testing may use command-runner seams or fake `PATH`, while source-overlay code should follow the target repo style of thin `Command::new(...)` wrappers plus pure parser/normalizer fixture tests.
- Update README/project documentation to state the delivery posture: Codex-first integration, Cinnamon/X11-first validation, generic X11/EWMH backend, standalone plugin path, and future source overlay path. The integration target checkout is machine-local and should be referenced through `CODEX_DESKTOP_LINUX_FULL_PATH`, with `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` documented only as the current development-machine default.
- No breaking changes.

## Capabilities

- New `project-bootstrap` capability: the repository provides a separate, initialized, root-level Rust package/workspace with repeatable `make test`, `make check`, and `make fmt` commands.
- New `doctor-cli` capability: `doctor --json` exposes a stable, machine-readable baseline report for Codex and smoke tests.
- New `x11-integration-contract` capability: the project documents the canonical `x11-ewmh` backend identity, upstream `WindowInfo` mapping, X11 window-id normalization, sidecar diagnostics policy, and testing seams for standalone versus source-overlay contexts.

## Research refresh

Date: 2026-05-30.

Fresh local repository checks:

- Current project `/home/as/ai-projects/codex-computer-use-x11` is now its own Git repository on `main`, clean against `origin/main`, with OpenSpec already initialized and this change scaffolded.
- Integration target `${CODEX_DESKTOP_LINUX_FULL_PATH}` was inspected using the current development-machine default `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`; that checkout is on branch `main` with clean status.
- Target files inspected: `computer-use-linux/Cargo.toml`, `src/main.rs`, `src/server.rs`, `src/diagnostics.rs`, `src/atspi_tree.rs`, `src/screenshot.rs`, `src/windowing/types.rs`, `src/windowing/registry.rs`, `src/windowing/target.rs`, and `src/windowing/backends/*`.
- Target reality confirmed:
  - `WindowInfo` fields are `window_id`, `title`, `app_id`, `wm_class`, `pid`, `bounds`, `workspace`, `focused`, `hidden`, `client_type`, `backend`, and optional `terminal`; no `raw_id`, `pid_reliable`, `source`, `warnings`, or `degraded` fields exist.
  - MCP window focus tool is `activate_window`; `focused_window`, `list_windows`, and `get_app_state` already exist.
  - Registry order is currently GNOME extension, GNOME introspect, COSMIC, KWin, Hyprland, i3; future `x11-ewmh` must be inserted after those existing desktop-specific backends as a late fallback rather than replacing them.
  - `doctor_report()` already models readiness and capability maps across screenshot, input, windowing, and accessibility.
  - Screenshot code uses GNOME Shell first and XDG Desktop Portal fallback; AT-SPI and input paths already exist in the target and should be reused by future overlay work where possible.
  - The target Rust crate is `codex-computer-use-linux` and uses `rmcp`, `serde`, `schemars`, `tokio`, `zbus`, `atspi`, and related Linux desktop dependencies.

Fresh external source checks:

- `tak-uukti/linux-computer-use` — MIT; useful ideas for X11 MVP scope, `wmctrl`/`xdotool`/AT-SPI/scrot composition, concise MCP tools, and E2E evidence. Treat as reference unless code-copy attribution is explicitly planned.
- `BeckhamLabsLLC/linux-desktop-mcp` — MIT; useful ideas for capability reporting, degraded mode, semantic AT-SPI refs, target windows, and input backend matrix.
- `wimi321/linux-computer-use-skill` — MIT by GitHub metadata; useful as an ideas-only reference for standalone Linux computer-use packaging.
- `Touchpoint-Labs/Touchpoint` — MIT by GitHub metadata; useful confirmation that Linux X11 can pair AT-SPI2 with `xdotool`, while pure Wayland input remains separate.
- `MONTBRAIN/vadgr-computer-use` — Apache-2.0 with NOTICE; useful for cross-platform MCP primitive framing, but any copy would require Apache/NOTICE compliance.
- `joe223/sootie` — GitHub reports license as `other`/view license; useful as a Rust/MCP runtime reference, but copy-unsafe until manual license terms are accepted for the specific files.
- `hightemp/go_computer_use_mcp_server` — no GitHub license metadata; copy-unsafe. GitHub licensing guidance says unlicensed public code remains under default copyright restrictions, so use ideas only.
- `Conservatory/wmctrl` — GPL-2.0; external command invocation is acceptable as a system dependency, but code must not be copied or vendored into this MIT-oriented project without a separate decision.
- `jordansissel/xdotool` — BSD-3-Clause; acceptable as an invoked system command candidate, while copying `libxdo` code would require BSD attribution.
- `ReimuNotMoe/ydotool` — AGPL-3.0; external invocation can remain a runtime dependency path where already used, but code must not be copied or vendored without AGPL review.
- `psychon/x11rb` / docs.rs `x11rb` — Apache-2.0/MIT dual license on docs.rs; remains a plausible future Rust native X11 dependency if shelling out to `wmctrl`/`xprop` is insufficient.
- `linuxmint/cinnamon`, `linuxmint/muffin`, and `linuxmint/cinnamon-spices-extensions` — GPL-2.0; useful only for behavior/API study for possible future extension paths, not code copy into this project.
- GitHub repository searches for `linux computer use x11 mcp` and `cinnamon x11 computer use` found no newer, more directly suitable Cinnamon/X11 Codex integration project than the known references.

Ideas taken for this change:

- Use Rust for the baseline because the target Codex Computer Use Linux integration is a Rust crate; keep the initial package/workspace at repository root for the smallest standalone layout.
- Keep stage 01 limited to bootstrap, public contract, `doctor --json`, window-id normalization, and verification commands; expose verification primarily through Makefile wrappers over Cargo.
- Require standalone tests that exercise external command behavior to use a command-runner seam or fake `PATH` fixture, while leaving the exact abstraction style to design.
- Preserve two delivery paths: standalone CLI/MCP plugin for fast Codex validation, and source overlay for future integration into `codex-desktop-linux-full`.
- Keep `x11-ewmh` as the backend id, not `x11` or `cinnamon`.
- Treat external projects as references first; do not copy code in this stage.

Rejected for this change:

- Do not implement `wmctrl`, `xprop`, or `xdotool` probing yet.
- Do not patch `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` yet.
- Do not create a Cinnamon/Muffin extension in v1 bootstrap.
- Do not expand upstream `WindowInfo` with X11-only diagnostics before a design/ADR decision.
- Do not make `xdotool` or `wmctrl` the only source-overlay input path before comparing with existing `abs_pointer`, `ydotool`, portal, screenshot, and AT-SPI paths.

Risks and open uncertainties:

- Exact command-runner seam style, CLI/MCP packaging, source-overlay file layout, and future `doctor --json.checks` extensions still require design before implementation. The bootstrap report shape is defined by specs, and the seam abstraction style remains design-owned as long as standalone tests can run without live X11 command dependencies. The local source-overlay target path should remain configurable with `CODEX_DESKTOP_LINUX_FULL_PATH` rather than hard-coded.
- A durable ADR may be required if design finds that standalone plugin delivery and source-overlay delivery need materially different architecture, data models, or command-execution seams.
- Future live Cinnamon/X11 smoke tests may reveal differences from the target repo assumptions; implementation must adapt to the real target code at apply time.
- License status must be rechecked before any code copy, especially for `sootie`, unlicensed repositories, GPL projects, and AGPL projects.
- The standalone project should not diverge from upstream `WindowInfo` semantics; sidecar diagnostics must be designed so it remains transferable into the target repo.

## Impact

- Affected project areas: new Rust workspace/crate files, CLI entry point, tests, README/project contract documentation, and local build/check scripts or Makefile/justfile.
- Affected OpenSpec capabilities: new `project-bootstrap`, `doctor-cli`, and `x11-integration-contract` specs.
- Integration target impact: read-only in this change; no patch to `${CODEX_DESKTOP_LINUX_FULL_PATH}` or its documented current development-machine default.
- External systems: no external credentials or `.secrets.local.env` access are needed for this change.
- Required project constraints:
  - implementation remains blocked until the required intent-driven gates are complete: specs, grill, design, design-review, adr, test-plan, and tasks;
  - follow root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, and in-force ADRs;
  - use OpenSpec as source of truth;
  - keep secrets out of tracked artifacts;
  - checkpoint artifacts before dependent workflow phases consume them;
  - use TDD for behavior-changing apply work.
- Verification impact: later apply must record RED/GREEN evidence for the X11 id normalizer, `doctor --json`, and project test/check/fmt commands before marking tasks complete.
