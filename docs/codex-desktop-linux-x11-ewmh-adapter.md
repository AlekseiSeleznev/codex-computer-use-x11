# codex-desktop-linux X11/EWMH adapter contract

Status: prepared adapter contract only. This repository has not merged an upstream integration into `codex-desktop-linux`, and this document does not enable the feature by default.

## Source of truth

This repository remains the source of truth for the standalone `codex-computer-use-x11` runtime, release artifact, checksum, plugin manifest, and `x11_*` MCP tool behavior. The upstream adapter should be a thin Linux Feature under:

```text
linux-features/x11-ewmh-computer-use/
```

The adapter should stage this plugin; it should not reinterpret this repository's runtime behavior or fork the plugin implementation.

## Upstream path guidance

The default upstream-ready path remains the thin Linux Feature adapter described here: keep `codex-computer-use-x11` as the source of truth, stage the release artifact or local source build, and expose the separate namespaced plugin only when the feature is explicitly enabled.

GitHub issue #389 also identifies `agent-sh/computer-use-linux` selectable backend/flavor work as a possible future evaluation path. Treat that as a separate change: first prove that the X11/EWMH behavior fits that backend/flavor model, then propose any small downstream adapter or generic Computer Use hook needed for selection. A backend flavor route must not change default `codex-desktop-linux` Computer Use behavior for users who do not opt in.

## Required upstream constraints

A future upstream adapter:

- must be disabled by default;
- must be fully opt-in through git-ignored `linux-features/features.json`;
- must not modify core Computer Use behavior;
- must not replace the bundled `computer-use` plugin;
- must not change global doctor behavior;
- must expose the existing namespaced `x11_*` plugin as a separate plugin named `codex-computer-use-x11`;
- must not use submodules — No submodules;
- must keep X11/EWMH readiness inside this plugin's `x11_doctor` / `doctor --json` rather than upstream global doctor checks.

Prepared adapter contract for optional linux-features/x11-ewmh-computer-use integration in codex-desktop-linux.

## Staging modes

### 1. Pinned release artifact + checksum

Preferred upstream release mode uses a published tarball from this repository and a pinned SHA256 value:

```bash
CODEX_X11_COMPUTER_USE_DOWNLOAD_URL=https://github.com/AlekseiSeleznev/codex-computer-use-x11/releases/download/v<VERSION>/codex-computer-use-x11-v<VERSION>-x86_64-unknown-linux-gnu.tar.gz
CODEX_X11_COMPUTER_USE_RELEASE_SHA256=<expected-sha256>
```

The adapter must verify sha256 before staging downloaded bytes. A local tarball may also be supplied for tests:

```bash
CODEX_X11_COMPUTER_USE_RELEASE_TARBALL=/path/to/codex-computer-use-x11-v<VERSION>-x86_64-unknown-linux-gnu.tar.gz
CODEX_X11_COMPUTER_USE_RELEASE_SHA256=<expected-sha256>
```

### 2. Local source checkout build

Development mode builds from a local checkout of this source-of-truth repository:

```bash
CODEX_X11_COMPUTER_USE_SOURCE=/path/to/codex-computer-use-x11
```

The adapter should verify `Cargo.toml` exists, run `cargo build --release` in that checkout, and stage `target/release/codex-computer-use-x11`.

### 3. Direct binary override

Tests and local development may use a direct executable:

```bash
CODEX_X11_COMPUTER_USE_BINARY=/path/to/codex-computer-use-x11
```

The adapter should fail clearly if no pinned artifact, local source checkout, download URL, or direct binary mode can produce a plugin tree.

## Baseline and non-goals

The supported baseline is Linux Mint Cinnamon on X11 using backend `x11-ewmh`. RemoteDesktop/Wayland facts remain debug-only or out of scope for this standalone X11 baseline. A future upstream adapter must not turn missing RemoteDesktop or Wayland capability into a global readiness blocker for this plugin.

Non-goals:

- no core Computer Use replacement;
- no Wayland/RemoteDesktop baseline;
- no default enablement;
- no submodule;
- no global doctor changes;
- no direct mutation of user home from `stage.sh`.

## Future upstream PR checklist

A later upstream PR to `ilysenko/codex-desktop-linux` should add only the disabled-by-default feature directory, tests modeled after `linux-features/read-aloud-mcp`, and any tiny generic Linux Feature hook support if upstream lacks one. It should not modify bundled `computer-use` plugin files or core Computer Use behavior.
## Local Codex Desktop Linux feature install helper

For manual verification before a published release, this repository provides a local opt-in installer that applies the prepared Linux Feature adapter to a selected `codex-desktop-linux` checkout/install directory:

```bash
scripts/install-codex-desktop-linux-x11-feature.sh \
  --target "$CODEX_DESKTOP_LINUX_FULL_PATH" \
  --install-dir /opt/codex-desktop \
  --source "$PWD" \
  --patch-mode auto \
  --report-json /tmp/codex-x11-feature-install.json
```

For a source checkout that is not writable as the current user, run the command with appropriate permissions or choose a writable fixture/install directory. The script does not silently escalate privileges.

Useful local modes:

```bash
# Show the plan only, with no writes.
scripts/install-codex-desktop-linux-x11-feature.sh --dry-run --report-json -

# Stage from an already built binary.
scripts/install-codex-desktop-linux-x11-feature.sh \
  --target /path/to/codex-desktop-linux \
  --install-dir /path/to/codex-app \
  --binary /path/to/codex-computer-use-x11 \
  --patch-mode skip \
  --report-json -

# Fixture tests only: writes a fake marker instead of a real app.asar patch.
scripts/install-codex-desktop-linux-x11-feature.sh \
  --target /tmp/fake-codex-desktop-linux \
  --install-dir /tmp/fake-codex-app \
  --binary /tmp/codex-computer-use-x11 \
  --patch-mode fake \
  --report-json -
```

`--patch-mode auto` patches `resources/app.asar` only when the selected install directory has app assets and the target checkout has the expected patcher/tooling. `--patch-mode skip` stages the feature/plugin without touching app assets. `--patch-mode fake` is test-only evidence for temporary fixtures and must not be used as real live UI patch evidence.

The installer writes a non-secret rollback manifest by default at:

```text
<install-dir>/.codex-x11-feature/install-manifest.json
```

Rollback/uninstall uses that manifest and refuses to overwrite drifted files:

```bash
scripts/uninstall-codex-desktop-linux-x11-feature.sh \
  --install-dir /opt/codex-desktop \
  --report-json /tmp/codex-x11-feature-uninstall.json

# Preview rollback only.
scripts/uninstall-codex-desktop-linux-x11-feature.sh \
  --install-dir /opt/codex-desktop \
  --dry-run \
  --report-json -
```

The helper preserves the bundled `computer-use` plugin and marketplace entry. It only stages the separate `codex-computer-use-x11` plugin and local `x11-ewmh-computer-use` feature state.

