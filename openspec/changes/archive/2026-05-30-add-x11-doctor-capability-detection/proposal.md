## Why

The bootstrap `doctor --json` currently proves only package identity, but the next Cinnamon/X11 milestone needs a machine-readable readiness report that tells Codex which desktop-control capabilities are actually available before listing windows, focusing targets, or sending input.

This change upgrades doctor capability detection for the standalone `codex-computer-use-x11` path and keeps the source-overlay contract aligned with current Computer Use Linux diagnostics vocabulary instead of inventing unsupported upstream fields.

## Research refresh

Date: 2026-05-30.

- Baseline context read: `backlog/00-research-reuse-map.md`, `backlog/02-doctor-capability-detection.md`, root `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, and existing specs `doctor-cli`, `x11-integration-contract`, and `project-bootstrap`.
- Current project state: `/home/as/ai-projects/codex-computer-use-x11` is on `main` at `5b2b8ab`, with a clean working tree before this artifact; existing Rust code still exposes bootstrap `doctor --json` plus `x11_id` normalization.
- Current target repo state: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` is on `main` at `1a6f343`, with no dirty status reported. Reviewed `computer-use-linux/src/diagnostics.rs`, `server.rs`, `atspi_tree.rs`, `screenshot.rs`, `abs_pointer.rs`, `remote_desktop.rs`, and `windowing/{types,registry,target}.rs`.
- Target diagnostics vocabulary confirmed: upstream-shaped readiness uses `ReadinessReport.can_query_windows`, `can_focus_apps`, `can_focus_windows`, and `can_send_development_input`; `WindowTarget::requires_exact_focus()` is behavior, not a serialized JSON readiness field.
- Target/source-overlay gap confirmed: `PortalReport.remote_desktop` and `screenshot` currently use a generic `portal_interface_check()` that treats `busctl introspect` success as enough, while Cinnamon/X11 can return success with an empty method table for `RemoteDesktop`/`ScreenCast`. `capability_map()` also treats `gnome-shell --version` as the `gnome_shell` screenshot signal, which misses Cinnamon's `org.gnome.Shell.Screenshot` DBus provider.
- Local Cinnamon/X11 probe: session type is X11, current desktop is Cinnamon, and an X11 display is present; `wmctrl`, `xprop`, `xdotool`, and `ydotool` are installed; `ydotoold` is running; `/dev/uinput` opened read/write; the socket path named by `YDOTOOL_SOCKET` is stale/missing, `$XDG_RUNTIME_DIR/.ydotool_socket` is missing, and `/tmp/.ydotool_socket` is connectable.
- Local portal/DBus probe: `org.freedesktop.portal.Screenshot` exposes `Screenshot`, `PickColor`, and `version=2`; `org.freedesktop.portal.RemoteDesktop` and `org.freedesktop.portal.ScreenCast` introspection return empty tables; `org.gnome.Shell.Screenshot` is owned by the `cinnamon` process and exposes `Screenshot`, `ScreenshotArea`, and `ScreenshotWindow`.
- External source refresh:
  - `agent-sh/computer-use-linux` was checked on GitHub/docs.rs (MIT; updated 2026-05-30 by GitHub API). Ideas to keep: same readiness vocabulary, capability map layers, portal/ydotool/uinput/windowing report separation, and existing MCP tool names such as `activate_window` rather than a new `focus_window` tool. Local target differs from upstream in several files, so source-overlay planning must use the real local checkout plus upstream as a reference, not assume byte-for-byte equality.
  - `BeckhamLabsLLC/linux-desktop-mcp` was rechecked (MIT). Ideas to keep: explicit capability reporting and AT-SPI-first degraded UX; reject adding its broader target-window/overlay feature set to this doctor-only stage.
  - `joe223/sootie` was rechecked. GitHub API still reports `NOASSERTION`, while the current `LICENSE` file presents MIT/Apache-2.0 options; use only as reference/ideas for X11 helpers unless a later task records exact attribution and copy policy.
  - Additional backlog-listed repos were checked through the GitHub API for freshness/license (`tak-uukti/linux-computer-use`, `wimi321/linux-computer-use-skill`, `Touchpoint-Labs/Touchpoint`, `MONTBRAIN/vadgr-computer-use`). No stronger replacement for the current Codex-first/X11-EWMH doctor scope was found.
  - XDG Desktop Portal docs were rechecked: Screenshot version 3 documents `AvailableTargets` as a version-3 addition, so Cinnamon's version-2 Screenshot method should still count as basic screenshot availability; RemoteDesktop readiness should require real methods/properties such as `CreateSession`, `SelectDevices`, `Start`, `AvailableDeviceTypes`, and Notify* input methods rather than an empty successful introspection.
- Ideas accepted for this stage: strict portal method/property detection; Cinnamon-aware screenshot facts based on DBus methods; ydotool socket candidate iteration that continues after stale env paths; explicit installed-tool, AT-SPI, uinput, X11/EWMH, portal, input-backend, and upstream-vocabulary readiness sections; degraded reasons that never over-promise targeted input before focus verification exists.
- Ideas rejected or deferred: Cinnamon Wayland support, Cinnamon/Muffin extension work, replacing existing GNOME/COSMIC/KWin/Hyprland/i3 backends, adding an upstream-required `can_send_targeted_input` JSON field, broad semantic targeting/window overlay features, and copying external code without a dedicated license/attribution decision.
- Risks/unknowns: exact source-overlay patch shape depends on later design; live `doctor --json` must stay safe/non-mutating; terminal/window-focused targeted input still depends on a later focus-verification stage; `busctl`/`gdbus` output format parsing needs fixture coverage before implementation.

## What Changes

- Expand standalone `codex-computer-use-x11 doctor --json` from a bootstrap smoke report into a structured capability/readiness report for Cinnamon/X11 and generic X11/EWMH environments.
- Treat the expanded standalone doctor schema as additive/backward-compatible for current bootstrap consumers: preserve the existing top-level `project`, `version`, `backend`, `readiness`, `capabilities`, and `checks` paths and the existing `readiness.ok`, `readiness.blockers`, `capabilities.implemented`, `capabilities.planned`, and check-entry `name`/`ok`/`detail` fields while adding richer nested facts.
- Detect session/environment facts, installed command-line tools, AT-SPI availability, X11/EWMH readiness signals, portal screenshot/remote-desktop facts, `ydotool`/`ydotoold` socket status, `/dev/uinput` availability, and input-backend candidates.
- Define no-display/headless behavior: `doctor --json` should still exit successfully with a structured JSON report whenever it can inspect the host safely, mark X11/EWMH-dependent facts unavailable or blocked when `DISPLAY` is unset/inaccessible, and reserve non-zero exits for unsupported CLI usage or internal serialization/runtime failures that prevent any JSON report.
- Report readiness using real upstream-compatible concepts: `can_query_windows`, `can_focus_apps`, `can_focus_windows`, `can_send_development_input`, blockers, degraded reasons, and recommended next step.
- Keep any targeted-input readiness explanation derived/report-only; do not introduce an upstream-required `can_send_targeted_input` field.
- Do not reserve a new stable targeted-input boolean in this stage. Until the later focus-verification stage defines one, downstream consumers should derive targeted-input readiness from `can_query_windows`, `can_focus_windows`, `can_send_development_input`, selected input backend facts, and blockers/recommended-next-step text. Any future derived field must be additive, explicitly report-only, nested under a diagnostics/capability-facts section rather than top-level `readiness`, and must not become a gate that upstream consumers are required to honor.
- Make `ydotoold` socket detection continue across `YDOTOOL_SOCKET`, `$XDG_RUNTIME_DIR/.ydotool_socket`, `/tmp/.ydotool_socket`, and useful service/process hints instead of failing on the first stale path.
- Make portal checks strict: empty introspection tables are unavailable, Screenshot version 2 with the `Screenshot` method is available, and RemoteDesktop requires concrete session/input methods or properties.
- Require fixture-backed parsing coverage for `busctl`/`gdbus` outputs before implementation relies on those probes, including empty successful introspection tables and Cinnamon's `org.gnome.Shell.Screenshot` method table.
- Record source-overlay acceptance for fixing the corresponding Computer Use Linux doctor/report gaps in the local integration target without changing unrelated desktop backends.
- Preserve safe degraded behavior: missing X11 tools or unavailable portals explain blockers instead of panicking or requiring secrets/external systems.

## Capabilities

- Modify `doctor-cli` to specify the real capability-detection `doctor --json` contract, degraded behavior, ydotool socket probing, strict portal facts, and public CLI JSON assertions.
- Modify `x11-integration-contract` to specify source-overlay compatibility with Computer Use Linux `PortalReport`/`InputReport`/`WindowingReport`/`ReadinessReport`, strict portal detection, screenshot provider facts, and the rule that targeted input remains gated by verified focus behavior.
- Screenshot provider representation: specs should expose separate report-only screenshot backend/fact entries, such as GNOME Shell-compatible DBus provider availability and XDG Portal Screenshot availability, while mapping source-overlay compatibility back to upstream `PortalReport.screenshot`/capability-map semantics instead of collapsing provider provenance into one ambiguous boolean.
- No change to `project-bootstrap` is expected for this stage after checking its current requirements: this proposal preserves the existing bootstrap doctor JSON paths additively. The specs gate must re-check this and add a `project-bootstrap` delta if any later artifact proposes a breaking field removal, rename, or type change.
- Forward gate: the specs artifact MUST explicitly verify that no breaking bootstrap doctor field removal, rename, or type change is being introduced before closing `project-bootstrap` as out of scope.
- Proposal acceptance checklist for specs: include a pass/fail compatibility table for the preserved bootstrap fields, include no-display/headless behavior, include fixture-backed DBus parser cases, and place source-overlay acceptance in the `x11-integration-contract` spec delta with follow-up tasks rather than in a new durable ADR unless ADR review later finds a hard-to-reverse architecture decision.

## Impact

- Standalone Rust code under `/home/as/ai-projects/codex-computer-use-x11/src/`, with tests through public CLI/API seams and pure parser/fixture units where appropriate.
- Existing OpenSpec specs `openspec/specs/doctor-cli/spec.md` and `openspec/specs/x11-integration-contract/spec.md` will need deltas before design.
- Later source-overlay work may affect `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full/computer-use-linux/src/diagnostics.rs` and possibly screenshot/capability helper code, but this proposal does not patch the target checkout.
- Required project constraints: Rust 2021, root Cargo/Makefile verification (`make fmt`, `make check`, `make test`), TDD vertical slices, no secret values, and no `.secrets.local.env` access for this local doctor work.
- Compatibility constraint: specs should make the doctor JSON expansion additive unless a later artifact explicitly marks a breaking change; existing bootstrap smoke-test callers should continue to find the current top-level fields at the same JSON paths.
- Compatibility table required in specs: at minimum, `project` stays a string, `version` stays a string, `backend` stays `x11-ewmh`, `readiness.ok` stays a boolean, `readiness.blockers` stays an array of strings, `capabilities.implemented` and `capabilities.planned` stay arrays of strings, `checks` stays an array, and each check keeps string `name`, boolean `ok`, and string `detail`.
- Reuse constraint: `joe223/sootie` remains ideas-only in this change. Any later sootie-derived code must first add an explicit license/attribution tracking task or artifact note that resolves the GitHub API `NOASSERTION` versus repository `LICENSE` evidence before code is copied or adapted.
- Testability constraint: `busctl`/`gdbus` probe parsing must be covered by fixtures or fake command output before live Cinnamon/X11 smoke evidence is used to mark implementation tasks complete.
- Architecture constraints: keep `x11-ewmh` as the canonical generic X11/EWMH backend id; preserve existing desktop-specific backend priority; do not add durable architecture decisions unless later ADR review finds a hard-to-reverse trade-off.

## Claude Review Disposition

Final proposal review returned `warn` with no `mustFix`, no warnings, and no user-facing questions. The remaining `shouldFix` items are handled as concrete specs-stage obligations:

- `doctor-cli` specs must enumerate the exit-code contract at least by category: success when a JSON report is emitted, unsupported CLI usage, and internal/runtime failure where no JSON report can be produced.
- `doctor-cli` specs must state ydotool socket probe ordering: check ordered candidates, continue past stale/missing paths, select the first connectable socket for the primary ok status, and report checked candidates/details sufficiently for degraded diagnostics.
- The specs artifact must include a visible proposal-acceptance checklist, and the specs reviewer/Claude review must verify the bootstrap compatibility table is present before `project-bootstrap` is closed out of scope.
- The targeted-input derivation guidance in this proposal is non-normative until a specs delta defines a concrete derived/report-only contract; this stage must not introduce a top-level or upstream-required targeted-input boolean.
