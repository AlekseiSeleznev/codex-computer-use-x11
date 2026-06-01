# Codex X11 E2E harness

The E2E harness produces repeatable evidence for both `codex-computer-use-x11`
delivery paths:

- standalone Codex plugin marketplace/cache + MCP stdio;
- reversible source overlay against the Codex Desktop Linux target checkout.

Logs and JSON evidence are written under `target/e2e-logs` by default. Use
`--log-dir <dir>` to choose a different local directory. The harness never reads
`.secrets.local.env`; use variable names only in logs and docs.

## Fake mode

Fake mode is the deterministic no-GUI path for CI and local checks. It uses
isolated fixtures and does not require sudo or a real desktop session.

```bash
scripts/e2e/codex-plugin-smoke.sh --fake
scripts/e2e/codex-source-overlay-smoke.sh --fake
```

Useful options:

```bash
scripts/e2e/codex-plugin-smoke.sh --fake --codex-home /tmp/fake-codex-home
scripts/e2e/codex-plugin-smoke.sh --fake --log-dir target/e2e-logs/manual-plugin-fake
scripts/e2e/codex-source-overlay-smoke.sh --fake --log-dir target/e2e-logs/manual-source-overlay-fake
scripts/e2e/codex-x11-e2e.py validate-matrix --evidence target/e2e-logs/<run>/evidence.json
```

When `--codex-home` is omitted, plugin fake mode installs into an isolated temp
`CODEX_HOME`. When `--codex-home` is supplied, the harness validates exactly that
state and fails clearly if the plugin is missing.

Fake plugin mode injects fake `wmctrl`, `xprop`, `xdotool`, `python3`,
`busctl`, and `gdbus` commands ahead of `PATH`. The fake `python3` returns an
AT-SPI-positive GTK fixture with a stable `GTK Fixture` application and `Apply`
button, so the harness has a positive accessibility row without relying on
Tk/Tkinter. Tk/Tkinter remains useful for keyboard/pointer safe windows in live
runs, but a Tk no-match is reported as a Tk accessibility limitation or degraded
evidence rather than as an AT-SPI pass. The fake `busctl` returns a header-only
RemoteDesktop introspection response so the strict portal false-positive case is
covered without contacting the real desktop. Fake `xdotool` calls are written to
`fake-xdotool.log` in the run directory.

## Live mode

Live mode records current-machine evidence and can be environment-dependent.

```bash
scripts/e2e/codex-source-overlay-smoke.sh --live \
  --target "$CODEX_DESKTOP_LINUX_FULL_PATH" \
  --log-dir target/e2e-logs/live-source-overlay
```

If `--target` is omitted, the source-overlay smoke resolves
`CODEX_DESKTOP_LINUX_FULL_PATH` first and then the documented local default. The
real target checkout must start clean, and the script always attempts uninstall
before exit after an overlay install.

Plugin live mode validates the user-local `CODEX_HOME` plugin state. It should
not send real keyboard or pointer input unless a later explicit safe target is
added. When GTK/PyGObject is unavailable for a positive AT-SPI fixture, live
evidence must record the dependency as `degraded` with the missing module/tool
name instead of silently passing the AT-SPI row. For now, use fake mode for
machine-checkable input routing.

## Capability matrix groups

Every evidence file contains a `capability_matrix` with these groups for both
`standalone_plugin` and `source_overlay` paths:

| Group | Fake standalone plugin evidence | Source overlay evidence |
| --- | --- | --- |
| doctor/capabilities | `x11_doctor` over MCP | source integration or degraded reason |
| window listing/focus | `x11_list_windows`, `x11_focused_window`, `x11_focus_window` | stock target coverage maps focus to `activate_window` |
| get_app_state | `x11_get_app_state` | stock `get_app_state` manual/live evidence or degraded reason |
| keyboard input | `x11_type_text`, `x11_press_key` via fake `xdotool` | stock `type_text`, `press_key` manual/live evidence or degraded reason |
| pointer input | `x11_click`, `x11_scroll`, `x11_drag` via fake `xdotool` | stock `click`, `scroll`, `drag` manual/live evidence or degraded reason |
| screenshot | `x11_get_app_state` screenshot layer pass/degraded | stock screenshot path evidence or degraded reason |
| AT-SPI | `x11_accessibility_tree` pass/degraded | stock accessibility tree/action/value evidence or degraded reason |
| install/rollback | marketplace/cache metadata | source-overlay status/install/uninstall/final clean |

Matrix statuses must be `pass` or `degraded`. Degraded entries need a concrete
reason. Missing evidence is a harness failure.

## Manual Codex Desktop fallback

When no stable direct Codex Desktop stock tool-call runner is available, collect
manual stock-tool evidence in the run notes while keeping the machine-checkable
MCP/source-overlay evidence as the archive gate.

Suggested manual sequence after source-overlay live smoke:

1. Run stock `doctor` and confirm diagnostics mention the expected X11/backend
   readiness or a clear degraded reason.
2. Run stock `list_windows` and `focused_window`.
3. Use stock `activate_window` for focus verification; do not assume a stock
   `focus_window` tool exists.
4. Run stock `get_app_state` and record screenshot, `window_context`, AT-SPI, and
   degraded diagnostics.
5. Run stock `type_text` and `press_key` only against a safe test target after
   verified focus.
6. Run stock `click`, `scroll`, and `drag` only in a safe test area. Do not fail
   solely because a stock `mousemove` tool is absent.
7. Confirm source-overlay rollback by running
   `scripts/status-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"`
   and checking final `git status --short` in the target checkout.

Do not paste secrets, private tokens, or raw local config contents into evidence
or tracked files.

## Industrial live verification

Metadata/tool-list live smoke is useful freshness evidence, but it is not the same as industrial live verification. Industrial live verification requires controlled fixture windows and machine-readable reason categories:

```bash
scripts/e2e/codex-plugin-smoke.sh --live --industrial --log-dir target/e2e-logs/<run-id>/plugin-live
scripts/e2e/codex-x11-e2e.py validate-matrix --industrial --evidence target/e2e-logs/<run-id>/plugin-live/<run>/evidence.json
```

Industrial evidence uses canonical JSON statuses `pass`, `degraded`, and `fail`. Degraded rows must include a reason and, for industrial fixture-backed rows, a `reason_category`:

- `fixture_pass` — controlled fixture-backed evidence passed.
- `environment_limitation` — the harness attempted fixture orchestration but the desktop/toolkit dependency was unavailable.
- `missing_fixture_setup` — a required controlled fixture was not started or not uniquely selected; this blocks industrial acceptance.
- `code_failure` — a controlled fixture was ready but the tool behavior failed.
- `unsafe_target_selection` — a tool would have targeted a non-fixture user application or project overlay/helper window.
- `malformed_evidence` / `not_evaluated` — evidence is incomplete for industrial acceptance.

Live input, pointer, screenshot, app-state, target-window, and overlay checks must target only a controlled fixture window with a unique neutral run-scoped title/class (for example `x11-safe-fixture-<role>-<run-id>`) and readiness record. There is no input fallback to currently focused real user apps, browsers, terminals, password managers, editors, Codex windows, or overlay helper windows. Missing or ambiguous fixture selection is `missing_fixture_setup` or `unsafe_target_selection`, not a pass.

Screenshot and app-state image evidence should be stored as files under `target/e2e-logs/<run-id>/` and referenced by path or metadata in summaries. Ordinary logs and chat-facing evidence must not dump huge screenshot data URLs.

## App-state screenshot evidence safety

`get-app-state --json` is path-only for screenshots by default. A successful screenshot layer reports `screenshot.path`, `mime_type`, `source`, dimensions, and file metadata; default JSON must not contain `data:image` or `;base64,` screenshot payloads.

Use `--screenshot-output <path>` when the retest needs a deterministic artifact location, for example:

```bash
codex-computer-use-x11 get-app-state --window-id <controlled-window-id> --screenshot-output target/e2e-logs/<run-id>/app-state.png --json
```

Use `--no-screenshot` when only window, accessibility, capability, and degraded-layer diagnostics are needed. If `--inline-screenshot` is available for local debugging, treat it as an unsafe opt-in and never use it for durable evidence logs, ordinary summaries, or industrial matrix evidence.

