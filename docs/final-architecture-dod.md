# Final Architecture and DoD for Cinnamon/X11 Computer Use

This report is the final tracked Definition-of-Done gate for `codex-computer-use-x11` v1. It answers whether the project is a complete Computer Use baseline for the documented scope, and it keeps the evidence in tracked files instead of chat history.

## Final answer

**Yes for Cinnamon/X11 v1 baseline** (yes for Cinnamon/X11): the project has a documented and machine-checkable `x11-ewmh` Computer Use baseline for Linux Mint Cinnamon on X11 with doctor/capabilities, EWMH window listing/focus, verified targeted keyboard and pointer input, app-state composition, screenshot/AT-SPI degraded diagnostics, standalone MCP plugin evidence, reversible source-overlay evidence, e2e smoke, and uninstall/rollback evidence.

**Degraded or unsupported outside that baseline**: Cinnamon Wayland, unstable Cinnamon/Muffin extension work, unavailable AT-SPI/screenshot/input layers, terminal context enrichment beyond the target stock path, and unsafe targeted input without verification are not claimed as complete v1 capabilities.

## Research refresh

Date: 2026-05-31.

Local project research:

- Project root: `/home/as/ai-projects/codex-computer-use-x11`.
- Current branch/status before this change: `main`, clean.
- Current planning state: the retired planning notes have been removed; canonical behavior now lives in `openspec/specs/`, archived lifecycle evidence, this DoD report, and the in-force ADRs.
- Reviewed project files: `src/doctor.rs`, `src/list_windows.rs`, `src/focus.rs`, `src/input.rs`, `src/pointer.rs`, `src/accessibility.rs`, `src/coordinates.rs`, `src/app_state.rs`, `src/target_window.rs`, `src/mcp.rs`, `scripts/e2e/codex-x11-e2e.py`, `README.md`, `docs/integration-contract.md`, `docs/e2e-harness.md`, `docs/upstreaming.md`, `docs/license-attribution.md`, and `docs/release-checklist.md`.

Current target checkout research:

- Target checkout used for the 2026-05-31 research refresh: `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` (referenced in portable docs as `CODEX_DESKTOP_LINUX_FULL_PATH`). Fresh 2026-06-01 reinstall/takeover validation moved the active local target to `/home/as/Документы/AI_PROJECTS/codex-desktop-linux`; see `openspec/changes/archive/2026-06-01-replace-bundled-computer-use-with-x11-provider/reports/fresh-target-install-takeover-20260601T102256Z.json`.
- Current target branch/status during refresh: `main`, clean.
- Target files reviewed: `computer-use-linux/src/windowing/`, `computer-use-linux/src/server.rs`, `computer-use-linux/src/diagnostics.rs`, `computer-use-linux/src/atspi_tree.rs`, `computer-use-linux/src/screenshot.rs`, and `computer-use-linux/src/remote_desktop.rs`.
- Target stock vocabulary observed: `activate_window`, `get_app_state`, `type_text`, `press_key`, `click`, `scroll`, and `drag` are present. v1 does not require inventing stock `focus_window` or stock `mousemove` unless future target research changes the contract.

External references refreshed:

- Freedesktop EWMH specification for `_NET_CLIENT_LIST`, `_NET_CLIENT_LIST_STACKING`, and `_NET_ACTIVE_WINDOW` behavior.
- XDG Desktop Portal Screenshot and RemoteDesktop documentation: screenshot and input readiness must be checked by actual interface methods/properties, not by a successful empty introspection response.
- GNOME Shell `Shell.Screenshot` documentation for screenshot/screenshot area/window behavior.
- AT-SPI D-Bus documentation for accessibility tree/action/value boundaries.
- GitHub/license metadata checks for `psychon/x11rb`, `jordansissel/xdotool`, `ReimuNotMoe/ydotool`, and current public `wmctrl` mirrors.

Ideas kept:

- Keep `x11-ewmh` as the backend id and register it late as a generic fallback after more specific backends.
- Keep shell-out for v1 with strict degraded diagnostics and parser tests.
- Keep `x11rb` as the future native X11 threshold option.
- Keep source-overlay integration under existing target stock tools instead of parallel bundled stock X11 tools.
- Keep runtime command invocation separate from source copying/vendoring.

Ideas rejected for v1:

- Cinnamon Wayland support.
- A Cinnamon/Muffin extension.
- Direct `xdotool --window`/XSendEvent as a targeted-safety boundary.
- A source-overlay stock `x11_get_app_state` tool that competes with target `get_app_state`.
- Copying GPL/AGPL/unclear external source code into this MIT-targeted project.

## Architecture decision ledger

```json final-dod-decisions
[
  {
    "id": "backend_identity",
    "decision": "Use x11-ewmh as the canonical backend id and WindowInfo.backend value; avoid ambiguous x11; register as a late generic fallback unless upstream review requires an alias."
  },
  {
    "id": "window_model",
    "decision": "Use upstream-compatible WindowInfo as the primary model; keep X11 raw ids, source provenance, reliability, and warnings in diagnostics/sidecars unless a later ADR changes the model."
  },
  {
    "id": "command_execution_seam",
    "decision": "Standalone code may use command seams and fake PATH fixtures for TDD; source-overlay/upstream code should prefer thin Command wrappers plus pure parser/normalizer tests."
  },
  {
    "id": "shell_out_vs_native_x11",
    "decision": "Shell-out through wmctrl/xprop/xdotool is acceptable for v1; switch to native X11 such as x11rb if parsing, performance, reliability, or upstream review makes shell-out unsuitable."
  },
  {
    "id": "diagnostics_readiness",
    "decision": "Use real target readiness vocabulary and strict method/property checks; empty RemoteDesktop introspection is unavailable; screenshot readiness is based on screenshot methods; ydotool readiness requires a connectable socket."
  },
  {
    "id": "input_safety_invariant",
    "decision": "Targeted keyboard/pointer input requires verified target focus and bounds as appropriate; global injectors are not treated as isolated per-window channels."
  },
  {
    "id": "pointer_keyboard_backend_priority",
    "decision": "Standalone may use verified-focus xdotool routes; source overlay reuses existing target input dispatch and stock tools before adding any X11-specific backend route."
  },
  {
    "id": "atspi_correlation",
    "decision": "AT-SPI matching is confidence-scored and must degrade on absence or ambiguity rather than returning arbitrary application subtrees."
  },
  {
    "id": "screenshot_coordinate_model",
    "decision": "ADR 0008 remains in force: X11 root/global pixels are canonical for bounds, pointer points, screenshot crops, and app-state composition."
  },
  {
    "id": "get_app_state_integration",
    "decision": "Standalone x11_get_app_state is a validation surface; source overlay should improve stock get_app_state with window_context, screenshot, AT-SPI, diagnostics, and degraded reasons."
  },
  {
    "id": "plugin_source_overlay_strategy",
    "decision": "Standalone MCP plugin is the fast feedback loop; source overlay is reversible staging evidence and must not become a long-lived fork."
  },
  {
    "id": "licensing_upstream_policy",
    "decision": "Runtime command dependency invocation is distinct from copying source; GPL/AGPL/unclear code remains copy-unsafe without separate review; backend and wrapper upstream targets stay separated."
  },
  {
    "id": "cinnamon_extension_wayland_scope",
    "decision": "Cinnamon Wayland and a Cinnamon/Muffin extension are out of v1 scope and need future design/ADR work if pursued."
  }
]
```

## Final capability matrix

| Capability | Required for v1? | Status | Evidence | Degraded behavior |
| --- | --- | --- | --- | --- |
| doctor/capabilities | yes | pass | `src/doctor.rs`; `tests/doctor_cli.rs`; fake e2e `x11_doctor`; strict portal tests | Reports blockers/degraded reasons instead of fabricating readiness |
| list windows | yes | pass | `src/list_windows.rs`; `tests/list_windows_cli.rs`; e2e fake `x11_list_windows` | Missing `DISPLAY`/`wmctrl`/`xprop` returns structured degraded diagnostics |
| focused window | yes | pass | `src/focus.rs`; `tests/focus_cli.rs`; e2e fake `x11_focused_window` | No-active or active-not-listed is reported as degraded JSON |
| focus window with verification | yes | pass | `src/focus.rs`; `tests/focus_cli.rs`; e2e fake `x11_focus_window` | Refuses success when final `_NET_ACTIVE_WINDOW` does not match |
| resolve target by id/title/class/pid where safe | yes | pass | `src/input.rs`; `src/target_window.rs`; `tests/targeted_input_cli.rs`; `tests/target_window_cli.rs` | Missing/ambiguous/stale selectors fail safely with diagnostics |
| `get_app_state` with X11 target context | yes | pass | `src/app_state.rs`; `tests/get_app_state_cli.rs`; e2e fake `x11_get_app_state` | Window, screenshot, and AT-SPI layers degrade independently |
| keyboard `type_text` | yes | pass | `src/input.rs`; `tests/targeted_input_cli.rs`; e2e fake `x11_type_text` | No input sent without verified target focus |
| keyboard `press_key` | yes | pass | `src/input.rs`; `tests/targeted_input_cli.rs`; e2e fake `x11_press_key` | No input sent without verified target focus |
| pointer click | yes | pass | `src/pointer.rs`; `tests/pointer_actions_cli.rs`; e2e fake `x11_click` | No pointer injection without verified focus/bounds unless explicit global mode |
| pointer scroll | yes | pass | `src/pointer.rs`; `tests/pointer_actions_cli.rs`; e2e fake `x11_scroll` | No pointer injection without verified focus/bounds unless explicit global mode |
| pointer drag | should | pass | `src/pointer.rs`; `tests/pointer_actions_cli.rs`; e2e fake `x11_drag` | Reports backend failure and does not claim targeted success if verification fails |
| stock `activate_window` focus path | yes | pass | target `computer-use-linux/src/server.rs`; `scripts/e2e/codex-x11-e2e.py` stock vocabulary inspection; `docs/e2e-harness.md` | Standalone may expose namespaced `x11_focus_window`; target docs do not require stock `focus_window` |
| stock `mousemove` absence handled | yes | pass | target stock vocabulary inspection; `docs/e2e-harness.md`; `docs/upstreaming.md`; e2e source-overlay evidence | DoD does not fail solely because target has no stock `mousemove`; movement is covered through click/scroll/drag internals or documented unsupported |
| input backend works on Cinnamon X11 | yes | pass | `src/doctor.rs`; `src/input.rs`; `src/pointer.rs`; fake e2e xdotool log; target `remote_desktop.rs`/`abs_pointer.rs` inspection | If `ydotool`, `/dev/uinput`, RemoteDesktop, or xdotool route is unavailable, targeted input is refused or degraded with reason |
| screenshot/global via existing Codex path on Cinnamon | yes if available | pass | `src/app_state.rs`; `src/coordinates.rs`; `tests/get_app_state_cli.rs`; `tests/screenshot_coordinate_cli.rs`; GNOME Shell/portal research | If providers are unavailable, `screenshot_error` or crop provider diagnostics report degraded state |
| screenshot/window crop/bounds | should | pass | ADR 0008; `src/coordinates.rs`; `tests/screenshot_coordinate_cli.rs` | Crop provider is not invoked when target bounds/crop validation fails |
| AT-SPI tree for target window | yes if available | pass | `src/accessibility.rs`; `tests/accessibility_tree_cli.rs`; e2e fake `x11_accessibility_tree`; app-state tests | AT-SPI absence/ambiguity returns accessibility diagnostics instead of arbitrary tree |
| AT-SPI action/value set | should | degraded | `docs/e2e-harness.md`; target `atspi_tree.rs`/`server.rs` inspection; final source-overlay policy | Standalone v1 validates tree/correlation; stock action/value routing is supported only where target and live AT-SPI expose it |
| terminal context selectors | should if upstream target supports it | degraded | target `server.rs`/`terminal.rs` inspection; `docs/upstreaming.md`; target tool instructions | Standalone v1 does not add separate terminal enrichment; source overlay should reuse target terminal context when available |
| standalone Codex MCP plugin | yes | pass | `scripts/install-codex-plugin.sh`; `scripts/uninstall-codex-plugin.sh`; `tests/plugin_installer.rs`; `tests/mcp_server.rs`; e2e fake plugin smoke | Missing plugin metadata fails clearly and writes evidence |
| source overlay into Codex repo | should | pass | `scripts/codex-source-overlay.py`; source overlay install/status/uninstall scripts; `tests/source_overlay_scripts.rs`; e2e fake source-overlay smoke | Live target smoke is optional and requires clean target checkout; failures attempt uninstall |
| E2E from Codex | yes | pass | `scripts/e2e/codex-plugin-smoke.sh`; `scripts/e2e/codex-source-overlay-smoke.sh`; `scripts/e2e/codex-x11-e2e.py validate-matrix`; `tests/e2e_harness_scripts.rs` | Live/manual stock evidence can be degraded with a concrete reason |
| uninstall/rollback | yes | pass | plugin uninstall script; source-overlay uninstall script; tests and e2e install/rollback matrix | Drift or dirty target state blocks blind install/uninstall assumptions |

```json final-dod-capability-matrix
[
  {
    "id": "doctor_capabilities",
    "capability": "doctor/capabilities",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/doctor.rs", "tests/doctor_cli.rs", "scripts/e2e/codex-x11-e2e.py x11_doctor"],
    "degraded_behavior": "Reports blockers and degraded reasons instead of fabricating readiness."
  },
  {
    "id": "list_windows",
    "capability": "list windows",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/list_windows.rs", "tests/list_windows_cli.rs", "x11_list_windows fake e2e evidence"],
    "degraded_behavior": "Missing DISPLAY, wmctrl, or xprop returns structured degraded diagnostics."
  },
  {
    "id": "focused_window",
    "capability": "focused window",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/focus.rs", "tests/focus_cli.rs", "x11_focused_window fake e2e evidence"],
    "degraded_behavior": "No-active or active-not-listed is reported as degraded JSON."
  },
  {
    "id": "focus_window_verification",
    "capability": "focus window with verification",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/focus.rs", "tests/focus_cli.rs", "x11_focus_window fake e2e evidence"],
    "degraded_behavior": "Refuses success when final active-window lookup does not match."
  },
  {
    "id": "safe_target_resolution",
    "capability": "resolve target by id/title/class/pid where safe",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/input.rs", "src/target_window.rs", "tests/targeted_input_cli.rs", "tests/target_window_cli.rs"],
    "degraded_behavior": "Missing, ambiguous, or stale selectors fail safely."
  },
  {
    "id": "get_app_state_x11_context",
    "capability": "get_app_state with X11 target context",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/app_state.rs", "tests/get_app_state_cli.rs", "x11_get_app_state fake e2e evidence"],
    "degraded_behavior": "Window, screenshot, and AT-SPI layers degrade independently."
  },
  {
    "id": "keyboard_type_text",
    "capability": "keyboard type_text",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/input.rs", "tests/targeted_input_cli.rs", "x11_type_text fake e2e evidence"],
    "degraded_behavior": "No input is sent without verified target focus."
  },
  {
    "id": "keyboard_press_key",
    "capability": "keyboard press_key",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/input.rs", "tests/targeted_input_cli.rs", "x11_press_key fake e2e evidence"],
    "degraded_behavior": "No input is sent without verified target focus."
  },
  {
    "id": "pointer_click",
    "capability": "pointer click",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/pointer.rs", "tests/pointer_actions_cli.rs", "x11_click fake e2e evidence"],
    "degraded_behavior": "No targeted pointer injection without verified focus and bounds."
  },
  {
    "id": "pointer_scroll",
    "capability": "pointer scroll",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/pointer.rs", "tests/pointer_actions_cli.rs", "x11_scroll fake e2e evidence"],
    "degraded_behavior": "No targeted pointer injection without verified focus and bounds."
  },
  {
    "id": "pointer_drag",
    "capability": "pointer drag",
    "required_for_v1": "should",
    "status": "pass",
    "evidence": ["src/pointer.rs", "tests/pointer_actions_cli.rs", "x11_drag fake e2e evidence"],
    "degraded_behavior": "Reports backend failure and does not claim targeted success if verification fails."
  },
  {
    "id": "stock_activate_window",
    "capability": "stock activate_window focus path",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["target computer-use-linux/src/server.rs research", "scripts/e2e/codex-x11-e2e.py stock vocabulary inspection", "docs/e2e-harness.md"],
    "degraded_behavior": "Standalone may expose x11_focus_window; target docs do not require stock focus_window."
  },
  {
    "id": "stock_mousemove_absence",
    "capability": "stock mousemove absence handled",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["target stock vocabulary inspection", "docs/e2e-harness.md", "docs/upstreaming.md"],
    "degraded_behavior": "DoD does not fail solely because stock mousemove is absent."
  },
  {
    "id": "cinnamon_x11_input_backend",
    "capability": "input backend works on Cinnamon X11",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["src/doctor.rs", "src/input.rs", "src/pointer.rs", "fake e2e xdotool log", "target remote_desktop.rs and abs_pointer.rs research"],
    "degraded_behavior": "Unavailable input backends produce degraded/refusal results rather than unsafe targeted injection."
  },
  {
    "id": "screenshot_global_provider",
    "capability": "screenshot/global via existing Codex path on Cinnamon",
    "required_for_v1": "yes if available",
    "status": "pass",
    "evidence": ["src/app_state.rs", "src/coordinates.rs", "tests/get_app_state_cli.rs", "tests/screenshot_coordinate_cli.rs"],
    "degraded_behavior": "If providers are unavailable, screenshot_error or provider diagnostics report degraded state."
  },
  {
    "id": "screenshot_window_crop_bounds",
    "capability": "screenshot/window crop/bounds",
    "required_for_v1": "should",
    "status": "pass",
    "evidence": ["adr/0008-adopt-x11-root-coordinate-model.md", "src/coordinates.rs", "tests/screenshot_coordinate_cli.rs"],
    "degraded_behavior": "Crop provider is not invoked when bounds/crop validation fails."
  },
  {
    "id": "atspi_tree",
    "capability": "AT-SPI tree for target window",
    "required_for_v1": "yes if available",
    "status": "pass",
    "evidence": ["src/accessibility.rs", "tests/accessibility_tree_cli.rs", "x11_accessibility_tree fake e2e evidence"],
    "degraded_behavior": "AT-SPI absence or ambiguity returns accessibility diagnostics."
  },
  {
    "id": "atspi_action_value_set",
    "capability": "AT-SPI action/value set",
    "required_for_v1": "should",
    "status": "degraded",
    "evidence": ["docs/e2e-harness.md", "target atspi_tree.rs/server.rs research", "docs/upstreaming.md"],
    "degraded_behavior": "Standalone v1 validates tree/correlation; action/value routing is supported only where target and live AT-SPI expose it."
  },
  {
    "id": "terminal_context_selectors",
    "capability": "terminal context selectors",
    "required_for_v1": "should",
    "status": "degraded",
    "evidence": ["target server.rs/terminal.rs research", "docs/upstreaming.md", "target tool instructions"],
    "degraded_behavior": "Standalone v1 does not add separate terminal enrichment; source overlay should reuse target terminal context when available."
  },
  {
    "id": "standalone_codex_mcp_plugin",
    "capability": "standalone Codex MCP plugin",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["scripts/install-codex-plugin.sh", "scripts/uninstall-codex-plugin.sh", "tests/plugin_installer.rs", "tests/mcp_server.rs", "e2e fake plugin smoke"],
    "degraded_behavior": "Missing plugin metadata fails clearly and writes evidence."
  },
  {
    "id": "source_overlay",
    "capability": "source overlay into Codex repo",
    "required_for_v1": "should",
    "status": "pass",
    "evidence": ["scripts/codex-source-overlay.py", "tests/source_overlay_scripts.rs", "e2e fake source-overlay smoke", "docs/integration-contract.md"],
    "degraded_behavior": "Live target smoke is optional and requires a clean target checkout; failures attempt uninstall."
  },
  {
    "id": "e2e_from_codex",
    "capability": "E2E from Codex",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["scripts/e2e/codex-plugin-smoke.sh", "scripts/e2e/codex-source-overlay-smoke.sh", "scripts/e2e/codex-x11-e2e.py validate-matrix", "tests/e2e_harness_scripts.rs"],
    "degraded_behavior": "Live/manual stock evidence can be degraded with a concrete reason."
  },
  {
    "id": "uninstall_rollback",
    "capability": "uninstall/rollback",
    "required_for_v1": "yes",
    "status": "pass",
    "evidence": ["scripts/uninstall-codex-plugin.sh", "scripts/uninstall-codex-source-overlay.sh", "tests/plugin_installer.rs", "tests/source_overlay_scripts.rs", "e2e install/rollback matrix"],
    "degraded_behavior": "Drift or dirty target state blocks blind install/uninstall assumptions."
  }
]
```

## License refresh

The license refresh records runtime command invocation distinct from source copying. This is engineering handoff guidance, not legal advice.

- `wmctrl`: GPL lineage in public mirrors; allowed as an installed runtime command dependency, copy-unsafe for MIT upstream code without separate review.
- `xdotool`: BSD-3-Clause; runtime invocation is allowed, source copying requires attribution/license compliance.
- `ydotool`: AGPL-3.0; existing Codex paths may invoke it when a socket is verified, but copying/vendoring source is copy-unsafe without separate AGPL review.
- `x11rb`: Apache-2.0/MIT on current public crate metadata; acceptable future dependency/reference only with license obligations satisfied.
- Linux Mint Cinnamon/Muffin sources: GPL behavior reference only for this MIT-targeted project.

No external source code is copied or vendored by this final DoD stage.

## Required validation commands

Run these before claiming final v1 handoff or archiving this change:

```bash
scripts/validate-final-dod.py
cargo test --test final_dod
cargo test --test packaging_docs
make fmt
make check
make test
scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/final-dod-plugin-fake
scripts/e2e/codex-source-overlay-smoke.sh --fake --log-dir target/e2e-logs/final-dod-source-overlay-fake
scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/final-dod-plugin-fake/<run>/evidence.json
scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/final-dod-source-overlay-fake/<run>/evidence.json
openspec validate finalize-x11-computer-use-architecture-dod --type change --strict
openspec validate --all --strict
git status --short
```

Generated e2e evidence remains local under `target/e2e-logs` and must not be committed. Do not read, print, commit, archive, or copy `.secrets.local.env`.
