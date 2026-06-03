# X11/EWMH Computer Use Linux Feature scaffold

This is a copyable scaffold for a later `codex-desktop-linux` upstream PR. It is inert in the `codex-computer-use-x11` source repository until copied to:

```text
linux-features/x11-ewmh-computer-use/
```

## Enable

Enable through the git-ignored upstream file `linux-features/features.json`:

```json
{ "enabled": ["x11-ewmh-computer-use"] }
```

## Baseline

Supported baseline: Linux Mint Cinnamon on X11 / `x11-ewmh`.

## Tools exposed

The staged plugin exposes the standalone namespaced tool surface:

- `x11_doctor`
- `x11_list_windows`
- `x11_focused_window`
- `x11_focus_window`
- `x11_accessibility_tree`
- `x11_type_text`
- `x11_press_key`
- `x11_click`
- `x11_scroll`
- `x11_drag`
- `x11_get_app_state`
- `x11_target_window`
- `x11_target_context`
- `x11_release_window`

## Staging modes

Pinned artifact mode:

```bash
CODEX_X11_COMPUTER_USE_RELEASE_TARBALL=/path/to/codex-computer-use-x11-v<VERSION>-x86_64-unknown-linux-gnu.tar.gz
CODEX_X11_COMPUTER_USE_RELEASE_SHA256=<expected-sha256>
```

Download mode:

```bash
CODEX_X11_COMPUTER_USE_DOWNLOAD_URL=https://github.com/AlekseiSeleznev/codex-computer-use-x11/releases/download/v<VERSION>/codex-computer-use-x11-v<VERSION>-x86_64-unknown-linux-gnu.tar.gz
CODEX_X11_COMPUTER_USE_RELEASE_SHA256=<expected-sha256>
```

Local source mode:

```bash
CODEX_X11_COMPUTER_USE_SOURCE=/path/to/codex-computer-use-x11
```

Direct binary test mode:

```bash
CODEX_X11_COMPUTER_USE_BINARY=/path/to/codex-computer-use-x11
```

## Upstream alignment

This scaffold wires the separate `codex-computer-use-x11` plugin as an opt-in Linux Feature. It does not move X11/EWMH behavior into the core Computer Use backend and does not replace the bundled `computer-use` plugin.

`agent-sh/computer-use-linux` selectable backend/flavor integration is a separate future investigation. If that route proves a better fit, handle it in a separate change or pull request; no backend/flavor experiment may require enabling this feature by default or modifying core Computer Use behavior in this scaffold.

## Non-goals

- no core Computer Use replacement;
- no Wayland/RemoteDesktop baseline;
- no default enablement;
- no submodule;
- no global doctor changes;
- no writes to user home from `stage.sh`.
