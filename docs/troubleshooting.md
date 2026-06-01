# Troubleshooting

Start with safe diagnostics. No secrets are needed for this project. No secrets from `.secrets.local.env`, Codex config, or private endpoints should be pasted into docs, logs, issues, or OpenSpec artifacts.

## Doctor and session layers

Run:

```bash
cargo run -- doctor --json
```

Inspect layers independently:

- Session: `XDG_SESSION_TYPE`, `XDG_CURRENT_DESKTOP`, `DISPLAY`, `XAUTHORITY`, `DBUS_SESSION_BUS_ADDRESS`.
- X11 window commands: `wmctrl` for EWMH window listing, `xprop` for `_NET_ACTIVE_WINDOW`, and `xdotool` for standalone activation/input smoke.
- Existing Codex input paths: `ydotool`, `/tmp/.ydotool_socket`, `/dev/uinput`, and any target-specific diagnostics.
- Screenshot providers: GNOME Shell-compatible `Screenshot`/`ScreenshotArea` and XDG portal Screenshot.
- AT-SPI: accessibility bus, toolkit accessibility, app PID/title correlation, and degraded tree evidence.

A layer can be degraded while other layers still provide useful evidence. For example, screenshot or AT-SPI may be degraded while X11 window context is valid.

## Portal false positives

Use strict RemoteDesktop checks. An empty RemoteDesktop introspection table is not enough to mark portal input available, even when the command exits successfully. The expected safe outcome is a degraded or unavailable RemoteDesktop input layer unless the interface actually exposes usable methods/properties.

Screenshot availability is separate from RemoteDesktop input availability. A working Screenshot method does not prove pointer or keyboard portal input is available.

For the supported Cinnamon/X11 `x11-ewmh` path, an incomplete RemoteDesktop
portal is report-only when X11 window queries, verified focus, local input
backends, and screenshot/app-state evidence are otherwise usable. Treat it as a
degraded diagnostic for future portal work, not as the main blocker or next step
for X11/EWMH readiness.

## Standalone plugin issues

If the standalone plugin is missing in Codex, first use fake/dry-run checks:

```bash
scripts/install-codex-plugin.sh --dry-run
scripts/e2e/codex-plugin-smoke.sh --fake
```

Check `CODEX_HOME` only by path/variable name; do not paste private config contents. The installer owns only the `codex-computer-use-x11` cache/marketplace/config sections. If rollback is needed:

```bash
scripts/uninstall-codex-plugin.sh --dry-run
scripts/uninstall-codex-plugin.sh
```

## Source-overlay drift

source-overlay drift means the target checkout's owned marker blocks, generated `x11_ewmh.rs`, anchors, or metadata no longer match the overlay expectations. The status script reports this as `state=drifted`.

```bash
scripts/status-codex-source-overlay.sh --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
git -C "$CODEX_DESKTOP_LINUX_FULL_PATH" status --short
```

If drift appears, stop before reinstall/uninstall. Inspect the target git status and marker blocks. Do not overwrite unowned target code or native X11 backend files blindly; in other words, do not overwrite unowned target code.

## E2E evidence and logs

fake mode is deterministic and safe for CI/no-GUI checks:

```bash
scripts/e2e/codex-plugin-smoke.sh --fake
scripts/e2e/codex-source-overlay-smoke.sh --fake
```

live mode is environment-dependent and should be used only with a clean target checkout and safe desktop context:

```bash
scripts/e2e/codex-source-overlay-smoke.sh --live --target "$CODEX_DESKTOP_LINUX_FULL_PATH"
```

Logs and JSON evidence are written under `target/e2e-logs` by default or the requested `--log-dir`. Preserve the log directory path and sanitized diagnostics when reporting failures; do not paste tokens, credentials, private local config, or raw secret values.

## Screenshot crop output integrity

`screenshot-crop` success requires both provider success and a verified output file. The output path is resolved before provider invocation, and the command must not report `success=true` unless the result exists, is readable, is non-empty, and begins with the PNG signature. Common structured failures:

- `OutputPathUnavailable` — output parent directory is missing or not a directory.
- `ScreenshotOutputMissing` — the provider returned false or no file was created.
- `ScreenshotOutputEmpty` — the provider created a zero-byte file.
- `ScreenshotOutputInvalidFormat` — the file exists but is not PNG data.
- `ScreenshotOutputUnreadable` — the file cannot be read as output evidence.


## App-state screenshot evidence safety

`get-app-state --json` no longer emits inline screenshot blobs by default. A successful screenshot layer should include a PNG artifact path such as `screenshot.path`, MIME/source metadata, dimensions, and file size. Use `--screenshot-output <path>` to choose the app-state screenshot artifact path, or `--no-screenshot` to keep window/accessibility/capability diagnostics without screenshot capture.

If app-state reports `screenshot_error` for an invalid output path, fix the `--screenshot-output` parent directory or permissions and rerun against a controlled fixture. Any `--inline-screenshot` mode is an unsafe opt-in for local debugging only; do not put inline screenshot data URLs into durable evidence logs.

## Industrial live fixture troubleshooting

Industrial live verification uses controlled fixtures and classifies failures separately from real environment limitations:

- `missing_fixture_setup` means the harness did not start or uniquely select the required fixture. This is a harness/setup blocker, not acceptable pass evidence.
- `environment_limitation` means fixture orchestration was attempted but a real desktop/toolkit dependency was unavailable.
- `code_failure` means the controlled fixture was ready and the tool behavior failed.

For semantic AT-SPI evidence, prefer the controlled GTK fixture path with
`GTK_MODULES=gail:atk-bridge` when that module hint is needed by the current
Cinnamon/X11 environment. The GTK fixture/application process must remove or
avoid inheriting `NO_AT_BRIDGE`; setting a false-looking value is not the
bridge-enable contract. Tk/Tkinter remains useful for keyboard and pointer
checks, but a Tk `NoAccessibilityMatch` is fixture-specific degraded evidence
and must not be converted into an AT-SPI pass by lowering matcher thresholds.

## AT-SPI bus reachable but tree extraction unavailable

`atspi_bus_available=true` with `tree_available=false` means the accessibility
bus can be reached but GTK/ATK application trees were not exposed. Doctor may
report `diagnostic_state=atspi_gtk_bridge_disabled_by_environment` when
`NO_AT_BRIDGE=1` is inherited by the Codex, fixture, or application process.
That state is an `environment_limitation` for semantic accessibility
enrichment; it is not by itself an X11 window/focus/input baseline failure.

Check the desktop prerequisites first:

- packages: `at-spi2-core`, `libatk-adaptor`, `libatk-bridge2.0-0t64`,
  `libatspi2.0-0t64`, or distribution-equivalent names;
- settings: toolkit accessibility such as
  `gsettings get org.gnome.desktop.interface toolkit-accessibility`;
- processes: `at-spi-bus-launcher`, the AT-SPI DBus daemon, and
  `at-spi2-registryd`;
- environment: inherited `NO_AT_BRIDGE=1` should be removed or not inherited by
  GTK fixture/application processes.
- remediation summary: remove or avoid inheriting `NO_AT_BRIDGE` before
  launching controlled GTK fixture/application processes; in shell terms,
  unset `NO_AT_BRIDGE` for that child process.
- quick check phrase: unset `NO_AT_BRIDGE` before launching the controlled GTK fixture.

After correcting bridge-related environment, restart the affected Cinnamon/Codex session or fixture process so the new environment is actually used. Then run the controlled GTK fixture self-test or live fixture smoke before claiming an AT-SPI PASS. The harness must not change the global user environment, and live checks must not target real user windows as fallback. It is not safe to test input against real user applications.

## PASS / DEGRADED / FAIL production evidence

Use the capability matrix status and `reason_category` fields when reading E2E
or doctor readiness output:

- **PASS** means a capability has concrete evidence for that delivery path and
  fixture mode. For live fixture-dependent capabilities, PASS requires a unique
  controlled fixture target and recorded evidence paths.
- **DEGRADED** means the row is explicitly limited, not silently successful.
  Common categories are `environment_limitation`, `missing_fixture_setup`,
  `expected_fake_fixture_limitation`, `unsupported_out_of_scope`, and
  `not_evaluated`.
- **FAIL** means the code, safety gate, cleanup, or output-integrity check failed
  and the row blocks production-readiness claims.

`expected_fake_fixture_limitation` is appropriate when fake mode cannot provide
fake `gdbus`/screenshot behavior; it does not prove real screenshot failure and
it does not weaken live screenshot-crop output integrity. `unsupported_out_of_scope`
should be used for Wayland or portal-required runtime paths in this X11-only
baseline.

Metadata-only live smoke must say `missing_fixture_setup` for fixture-dependent
rows. It is not safe to test input against real user applications; do not use a
browser, terminal, editor, password manager, Codex window, or overlay/helper
window as a fallback controlled target.

RemoteDesktop portal facts are diagnostics only for this Cinnamon/X11 baseline.
Wayland support is out of scope; use an X11 session for production-readiness
claims unless a future OpenSpec change and ADR explicitly add Wayland support.
