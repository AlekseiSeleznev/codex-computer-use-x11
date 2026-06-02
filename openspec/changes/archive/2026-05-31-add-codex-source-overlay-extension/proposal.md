## Why

The standalone X11/EWMH crate now proves the core windowing, input, accessibility, screenshot, app-state, and targeting behaviors, but the integration target still has no reversible way to evaluate those behaviors inside the stock `codex-computer-use-linux` source tree. A source overlay lets this project apply, inspect, test, and remove an `x11-ewmh` backend against the moving Codex Desktop Linux checkout without maintaining a long-lived fork.

## What Changes

- Add reversible source-overlay scripts: `install-codex-source-overlay.sh`, `uninstall-codex-source-overlay.sh`, and `status-codex-source-overlay.sh` with explicit `--target` handling and documented default target resolution.
- Generate or copy an `x11_ewmh.rs` backend into the target repo and patch target registry/module/diagnostic files only inside owned `BEGIN codex-computer-use-x11` / `END codex-computer-use-x11` marker blocks.
- Make install/uninstall/status idempotent and drift-aware so repeated runs do not duplicate markers, uninstall removes only owned overlay content, and status distinguishes clean, applied, and drifted target states.
- Preserve upstream `WindowInfo` compatibility: the source overlay maps `x11-ewmh` windows to the target repo's existing `WindowInfo` shape and keeps X11-specific diagnostics out of the primary window model.
- Register `x11-ewmh` as a late fallback backend after desktop-specific backends and integrate activation through the existing stock `activate_window` path rather than adding duplicate bundled tools such as `focus_window` or `mousemove`.
- Patch readiness/capability diagnostics only where the overlay touches them, including strict portal method detection so an empty `RemoteDesktop` introspection table does not count as available.
- Add fixture-backed tests for overlay scripts and record live target status/apply/test/uninstall evidence after fake-target tests are green.

## Capabilities

- New capability: `codex-source-overlay-extension` — reversible source overlay installation, removal, status/drift detection, target compatibility checks, `x11-ewmh` backend generation, late fallback registration, diagnostic patch markers, and target smoke verification.
- Existing capabilities consumed: `doctor-cli`, `x11-window-listing`, `x11-active-window-focus`, `x11-targeted-input-safety`, `x11-pointer-actions`, `x11-atspi-window-correlation`, `x11-screenshot-coordinate-model`, `x11-get-app-state-integration`, and `x11-target-window-groups-overlays`.

## Research refresh

Date: 2026-05-31.

Sources checked:

- Project context and backlog: `CONSTITUTION.md`, `CONTEXT.md`, `ARCHITECTURE.md`, `adr/README.md`, `adr/0008-adopt-x11-root-coordinate-model.md`, `backlog/00-research-reuse-map.md`, and `backlog/06-codex-source-overlay-extension.md`.
- Current project checkout `/home/as/ai-projects/codex-computer-use-x11`: branch `main`, scaffold checkpoint `dc3b584`, clean before proposal; reviewed `src/list_windows.rs`, `src/focus.rs`, `src/doctor.rs`, `src/input.rs`, `src/pointer.rs`, `src/app_state.rs`, `src/mcp.rs`, existing tests, plugin install scripts, and current canonical specs.
- Current target checkout `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`: branch `main`, commit `1a6f343ee7ce597019a4c573259c2a9838376874`, clean status; reviewed `computer-use-linux/src/windowing/types.rs`, `windowing/registry.rs`, `windowing/mod.rs`, `windowing/backends/mod.rs`, `windowing/backends/i3.rs`, `windowing/target.rs`, `server.rs`, `diagnostics.rs`, `screenshot.rs`, `atspi_tree.rs`, root `Cargo.toml`, and `computer-use-linux/Cargo.toml`.
- Target repo findings: stock tools include `list_windows`, `focused_window`, `activate_window`, `click`, `scroll`, `drag`, `press_key`, `type_text`, `get_app_state`, `perform_action`, and `set_value`; there is no separate stock `focus_window` tool. Internal ydotool command sequences use `mousemove`, but no public stock `mousemove` tool is exposed. `WindowInfo` already has the needed `window_id`, `title`, `app_id`, `wm_class`, `pid`, `bounds`, `workspace`, `focused`, `hidden`, `client_type`, `backend`, and optional `terminal` fields.
- Registry findings: current backend order is GNOME extension, GNOME introspect, COSMIC, KWin, Hyprland, and i3. The source overlay should add `x11-ewmh` after i3 as a generic fallback and set `can_exact_focus=true` because activation can be verified by fresh active-window lookup.
- Diagnostic findings: `diagnostics.rs` derives input/screenshot/window capability maps from `PortalReport`, `WindowingReport`, and `InputReport`; `portal_interface_check()` currently checks an interface by name and can be patched to require real methods for `RemoteDesktop` and `Screenshot`.
- External/reuse refresh via GitHub CLI: `ilysenko/codex-desktop-linux` is MIT and updated 2026-05-31; `agent-sh/computer-use-linux` is MIT and updated 2026-05-31; `tak-uukti/linux-computer-use` is MIT and provides X11/AT-SPI/xdotool reference ideas; `BeckhamLabsLLC/linux-desktop-mcp` is MIT and remains useful for capability-matrix ideas; `joe223/sootie` reports license key `other`, so it remains copy-unsafe/reference-only until manual license review. GitHub code search for `x11-ewmh computer-use-linux` and owner-scoped `x11 backend path:computer-use-linux/src/windowing` returned no ready-made native `x11-ewmh` backend to reuse directly.

Ideas accepted:

- Keep the source overlay project-owned and reversible, with all mutations inside marker blocks plus one owned generated backend file.
- Track target baseline commit in status metadata/reporting so drift is relative to a named target state, not an implicit current checkout.
- Use target-style Rust modules and public target tests where possible, but keep the overlay engine itself in this repository and test it against temporary fake target checkouts first.
- Register `x11-ewmh` as a late fallback and use stock `activate_window` / target-resolution behavior to validate focus; do not create parallel stock bundled tools.
- Patch strict portal checks in the overlay because Cinnamon/X11 can produce a successful but empty `RemoteDesktop` introspection table.

Ideas rejected or deferred:

- Do not keep the real target checkout permanently patched after smoke testing; install/test/uninstall must return it to clean state.
- Do not introduce a Cinnamon/Muffin extension, Wayland backend, or native `x11rb` dependency in this overlay step; shell command parity is enough for the reversible source overlay scaffold.
- Do not copy GPL/AGPL/no-license source code into this MIT project. External command invocation remains distinct from source-code reuse.
- Do not require a public stock `mousemove` tool or separate `focus_window` tool unless a future fresh target repo revision exposes them.

Risks / unknowns:

- The target repo is moving quickly; marker insertion and structure checks must fail clearly if expected anchors disappear.
- Applying generated Rust into the target can break target compilation if upstream `WindowInfo` or registry shapes change; status and install must detect incompatible structures before patching.
- Real X11 focus/input behavior still depends on desktop state and installed tools; automated fixture tests cover script behavior, while live target smoke remains best-effort and must be uninstalled afterward.

## Impact

- New overlay engine/templates and wrapper scripts under `/home/as/ai-projects/codex-computer-use-x11/scripts/`.
- New integration tests under `/home/as/ai-projects/codex-computer-use-x11/tests/` using temporary fake target repositories.
- README and integration-contract documentation for source overlay install/status/uninstall, target cleanliness, and upstream boundary.
- Temporary live smoke may apply overlay changes to `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full`; verification must uninstall afterward and confirm that checkout returns to clean status.
- Verification: `openspec validate add-codex-source-overlay-extension --strict`, `make fmt`, `make check`, `make test`, fake-target overlay tests, real-target status/apply/test/uninstall smoke, and git safety checks.
- No external secrets are needed; `.secrets.local.env` is not read. GitHub push uses configured Git credentials without printing secrets.
