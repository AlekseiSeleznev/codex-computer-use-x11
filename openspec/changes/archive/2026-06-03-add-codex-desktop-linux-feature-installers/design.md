## Context

The source-of-truth repository already contains a copyable `codex-desktop-linux` Linux Feature scaffold under `adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/`. The user manually verified the same architecture by copying the feature into a local `codex-desktop-linux` checkout/live install, enabling the feature through local feature config, staging the standalone plugin, and patching live app assets.

Constitution constraints require Rust/Cargo project checks for Rust changes, OpenSpec validation, no secrets in tracked files, and visible Git checkpoints. ADR 0009 keeps the X11 implementation as a standalone `x11_*` plugin and treats source-overlay/upstream integration as reversible staging evidence. ADR 0010 forbids global masquerade as bundled `computer-use`. ADR 0011 requires rollback-first manifests with drift blockers for installer-owned mutations.

## Goals / Non-Goals

**Goals:**

- Provide `scripts/install-codex-desktop-linux-x11-feature.sh` and `scripts/uninstall-codex-desktop-linux-x11-feature.sh` entrypoints.
- Share implementation in a testable helper so install/uninstall behavior can be verified with fake target/install directories.
- Install locally by copying the feature scaffold to the target checkout's ignored/local Linux Feature area, enabling `x11-ewmh-computer-use`, staging the plugin into the install directory, preserving `computer-use`, syncing update-builder feature state when present, and optionally patching app assets.
- Record a non-secret rollback manifest before mutation and after-state after mutation.
- Uninstall by restoring only completed installer-owned entries, blocking on drift, and supporting dry-run/report-json.

**Non-Goals:**

- No upstream PR, release publication, or default enablement.
- No core Computer Use rewrite, bundled `computer-use` replacement, global doctor changes, or plugin id masquerade.
- No automatic privileged escalation design; scripts can operate on writable fixtures/checkouts and report permission failures for root-owned installs.
- No support for Wayland/RemoteDesktop as readiness baselines.

## Decisions

1. **Implement thin shell entrypoints over a Python state engine.**
   - Shell scripts preserve the existing project style and user-facing installer naming.
   - Python handles JSON reports, directory checksums, backup/restore, feature config editing, and fake fixture tests more safely than ad hoc shell.
   - The shared engine exposes subcommands `install` and `uninstall`.

2. **Use explicit path options and safe defaults.**
   - `--target` selects a `codex-desktop-linux` checkout; absent value falls back to `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented local default.
   - `--install-dir` selects the app install directory; absent value falls back to `/opt/codex-desktop` when present, else `<target>/codex-app`.
   - `--source` defaults to this repository root; `--binary` can provide a direct local binary for tests/manual verification.
   - `--manifest` defaults under `<install-dir>/.codex-x11-feature/install-manifest.json`.

3. **Delegate plugin staging to the copyable adapter `stage.sh`.**
   - The installer first copies the scaffold into the target's local feature area, then invokes the scaffold stage hook with `INSTALL_DIR`, `WORK_DIR`, and `CODEX_X11_COMPUTER_USE_BINARY` or `CODEX_X11_COMPUTER_USE_SOURCE`.
   - This keeps the adapter scaffold and local installer aligned and avoids duplicating marketplace/plugin bundle rules.

4. **Treat every surface as a manifest entry.**
   - Entries include target local feature directory, target feature config, install plugin directory, install marketplace file, update-builder feature directory/config when present, and optional app/webview assets when patching.
   - Each entry records before existence/checksum, backup path, completed status, changed/already-acceptable classification, after checksum, mode, and path.
   - Directory checksums are content hashes over relative file names, modes, and file bytes.

5. **Use drift checks before rollback.**
   - Uninstall compares the current checksum for each completed changed entry with the recorded installer after checksum.
   - Matching entries are restored from backup or removed if absent before install.
   - Drifted entries are blocked and never overwritten automatically.

6. **Make app patching explicit and testable.**
   - Default patch mode is `auto`: patch only when `resources/app.asar` exists and required Node/asar tooling is available.
   - `--patch-mode skip` skips app patching while still staging feature/plugin state.
   - `--patch-mode fake` is reserved for deterministic tests/fixtures and writes a recognizable non-production marker into fixture assets.
   - Real patching uses the target checkout's Linux Features patcher path; if required hooks are missing, the report explains the blocker.

No Mermaid diagram is needed: the boundary is a linear local installer over file-system surfaces, and the manifest table in design/tasks is more precise for implementation.

## Risks / Trade-offs

- Root-owned live installs may require the user to run the script with appropriate permissions; automatic sudo would make tests, reports, and rollback provenance harder to reason about.
- Real `app.asar` patching depends on upstream Node/asar tooling and the current target patch script shape; the installer should fail/skip with a structured report rather than invent patches.
- Restoring entire marketplace/config files from backup is safer for rollback but can block on legitimate post-install edits; drift blockers are expected and intentional under ADR 0011.
- A fake patch mode must stay clearly documented as test-only and must not be used to claim real live app patch success.

## Migration Plan

1. Add failing fixture tests for installer dry-run/install/uninstall/drift behavior.
2. Implement the Python engine and shell wrappers.
3. Update documentation with local manual verification commands for binary/source, skip/fake/auto patch modes, report-json, and rollback.
4. Run OpenSpec validation and targeted tests. Run full project checks where feasible; if a root-owned live install or existing unrelated dirty state blocks full verification, record the exact limitation.
5. Do not archive or push unless explicitly requested.

Rollback for this change itself is normal Git revert. Rollback for local installs is provided by the new uninstaller and manifest.

## Open Questions

None
