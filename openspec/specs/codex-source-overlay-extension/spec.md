# codex-source-overlay-extension Specification

## Purpose
Defines the reversible source-overlay contract for applying project-owned X11/EWMH integration changes to a local Codex Desktop Linux checkout, validating drift, and uninstalling owned changes without creating a long-lived fork.
## Requirements
### Requirement: Source overlay target preflight
The project MUST provide a source-overlay installer that validates a Codex Desktop Linux target checkout before mutating it. The installer MUST accept `--target <path>`, MUST default to `CODEX_DESKTOP_LINUX_FULL_PATH` or the documented local target path when no target is passed, MUST verify the expected `computer-use-linux` structure, and MUST fail clearly without partial mutation when required anchors are missing.

#### Scenario: Refuse a missing target structure
- **GIVEN** a temporary directory that does not contain `computer-use-linux/src/windowing/registry.rs`
- **WHEN** a developer runs `scripts/install-codex-source-overlay.sh --target <dir>`
- **THEN** the command exits with a non-zero status code
- **AND** stderr explains the missing target structure
- **AND** no overlay marker files or generated backend files are created in that directory

#### Scenario: Resolve the documented default target
- **GIVEN** `CODEX_DESKTOP_LINUX_FULL_PATH` points to a valid target checkout
- **WHEN** a developer runs `scripts/status-codex-source-overlay.sh` without `--target`
- **THEN** the script inspects that target checkout
- **AND** the report identifies the target path and current target commit when the target is a git checkout

### Requirement: Install applies an owned X11 source overlay
Installing the source overlay MUST copy or generate an `x11_ewmh.rs` backend with backend id `x11-ewmh` and MUST patch target source files only inside owned `BEGIN codex-computer-use-x11` / `END codex-computer-use-x11` marker blocks. The overlay MUST be idempotent: repeated installs MUST NOT duplicate markers, backend registration, capability-map entries, or generated files.

#### Scenario: Install writes backend and marker blocks
- **GIVEN** a fake target checkout with the expected `computer-use-linux` files and anchors
- **WHEN** a developer runs `scripts/install-codex-source-overlay.sh --target <fake-target>`
- **THEN** the command exits with status code 0
- **AND** `<fake-target>/computer-use-linux/src/windowing/backends/x11_ewmh.rs` exists
- **AND** patched target files contain owned begin/end markers
- **AND** `windowing/backends/mod.rs` declares the generated backend module
- **AND** `windowing/registry.rs` registers `x11-ewmh` after desktop-specific backends

#### Scenario: Install is idempotent
- **GIVEN** a fake target checkout where the source overlay is already installed
- **WHEN** a developer runs `scripts/install-codex-source-overlay.sh --target <fake-target>` again
- **THEN** the command exits with status code 0
- **AND** each owned marker pair still appears exactly once per patched anchor
- **AND** the generated backend file remains a single owned file
- **AND** the target source remains compilable according to the overlay's fake-target checks

#### Scenario: Preserve native X11 backend ownership
- **GIVEN** a target checkout already contains an unowned native X11 backend file or registration
- **WHEN** the installer evaluates the target
- **THEN** it MUST NOT overwrite or delete the unowned native backend
- **AND** it MUST report compatibility/adaptor mode or a clear refusal instead of silently replacing upstream code

### Requirement: Status detects clean, applied, and drifted targets
The status script MUST report whether the source overlay is cleanly absent, cleanly applied, or drifted. Drift MUST include missing generated files, missing marker blocks, changed generated backend content, or a target commit/structure that no longer matches the overlay metadata.

#### Scenario: Report clean target
- **GIVEN** a valid target checkout with no owned overlay markers and no generated `x11_ewmh.rs`
- **WHEN** a developer runs `scripts/status-codex-source-overlay.sh --target <target>`
- **THEN** the command exits with status code 0
- **AND** stdout reports `state=clean`
- **AND** stdout includes the target path and target git commit when available

#### Scenario: Report applied target
- **GIVEN** a valid target checkout after a successful overlay install
- **WHEN** a developer runs `scripts/status-codex-source-overlay.sh --target <target>`
- **THEN** the command exits with status code 0
- **AND** stdout reports `state=applied`
- **AND** stdout identifies the generated backend file and owned marker count

#### Scenario: Report drifted target
- **GIVEN** a target checkout with one owned marker block removed or a generated backend file edited after install
- **WHEN** a developer runs `scripts/status-codex-source-overlay.sh --target <target>`
- **THEN** the command exits with a non-zero status code
- **AND** stdout or stderr reports `state=drifted`
- **AND** the report explains which owned artifact or marker is inconsistent

### Requirement: Uninstall removes only owned overlay content
Uninstalling the source overlay MUST remove the generated backend file and owned marker blocks while preserving unrelated target code, native upstream backends, user changes outside marker blocks, and target git metadata. Uninstall MUST be idempotent and MUST leave a clean target state when the overlay was previously applied.

#### Scenario: Uninstall removes owned markers and backend
- **GIVEN** a fake target checkout after a successful overlay install
- **WHEN** a developer runs `scripts/uninstall-codex-source-overlay.sh --target <fake-target>`
- **THEN** the command exits with status code 0
- **AND** `x11_ewmh.rs` is removed only if it is the owned generated file
- **AND** all owned marker blocks are removed from patched target files
- **AND** unrelated content outside marker blocks remains unchanged

#### Scenario: Uninstall is idempotent
- **GIVEN** a valid target checkout where the source overlay is absent
- **WHEN** a developer runs `scripts/uninstall-codex-source-overlay.sh --target <target>`
- **THEN** the command exits with status code 0
- **AND** the target remains in `state=clean`

### Requirement: Generated backend preserves target contracts
The generated `x11-ewmh` backend MUST map X11/EWMH observations into the target repo's existing `WindowInfo` shape, MUST preserve signed X11 root coordinates and positive dimensions, MUST expose exact focus activation through the existing registry `activate_window` path, and MUST keep X11-only diagnostics out of the primary `WindowInfo` fields.

#### Scenario: Backend parser maps wmctrl rows to WindowInfo
- **GIVEN** the generated backend receives a `wmctrl -lpGx` row with hex window id, workspace, pid, signed x/y, positive width/height, class, host, and title fields
- **WHEN** target tests exercise the parser
- **THEN** the produced window has numeric `window_id`, optional `title`, `app_id`, `wm_class`, `pid`, `bounds`, `workspace`, `focused`, `hidden`, `client_type`, and `backend` fields compatible with target `WindowInfo`
- **AND** `backend` equals `x11-ewmh`
- **AND** the primary object does not add X11-only sidecar fields

#### Scenario: X11 backend registers as late fallback
- **GIVEN** the overlay is installed in a target checkout
- **WHEN** target registry tests inspect descriptor order
- **THEN** `x11-ewmh` appears after GNOME extension, GNOME introspect, COSMIC, KWin, Hyprland, and i3 descriptors
- **AND** `x11-ewmh` reports exact focus support for the existing target-resolution safety gate

#### Scenario: Activation uses stock target focus path
- **GIVEN** the overlay is installed and a target window is resolved through stock `WindowTarget`
- **WHEN** the stock `activate_window` tool focuses that window
- **THEN** registry activation dispatches to the `x11-ewmh` backend
- **AND** focus success is still based on a fresh focused-window lookup
- **AND** the overlay does not add a duplicate stock `focus_window` tool

### Requirement: Diagnostics patch strict portal and X11 capability reporting
When the overlay patches diagnostics, it MUST preserve existing target report vocabulary while adding `x11-ewmh` facts and strict portal method checks. A successful-but-empty `RemoteDesktop` introspection table MUST be reported unavailable, while a Screenshot interface with the `Screenshot` method MAY satisfy screenshot availability without requiring GNOME Shell binary semantics.

#### Scenario: Empty RemoteDesktop introspection is unavailable
- **GIVEN** the diagnostic probe receives an empty `busctl introspect` table for `org.freedesktop.portal.RemoteDesktop`
- **WHEN** target diagnostics are evaluated after overlay install
- **THEN** the RemoteDesktop portal check is not ok
- **AND** capability-map input backends do not include `portal` solely because the command exited successfully

#### Scenario: X11 backend appears in windowing capability map
- **GIVEN** `wmctrl`, `xprop`, and `xdotool` are available in the target runtime environment
- **WHEN** target diagnostics are evaluated after overlay install
- **THEN** `WindowingReport` or equivalent target diagnostic output includes `x11-ewmh` availability facts
- **AND** the window-control capability map can list `x11-ewmh` as a fallback without removing existing desktop-specific backends

### Requirement: Real target smoke is reversible
After fake-target tests pass, verification MUST run status/apply/test/uninstall smoke against the configured real target checkout when it is available. The real target checkout MUST be returned to its pre-smoke clean state before archive.

#### Scenario: Apply, test, and uninstall real target
- **GIVEN** the configured target checkout exists and starts clean
- **WHEN** verification runs source-overlay smoke
- **THEN** status initially reports clean or an explicitly documented state
- **AND** install applies the overlay successfully
- **AND** relevant target `cargo test` checks for windowing/diagnostics overlay behavior pass or report an exact environmental blocker
- **AND** uninstall removes the overlay
- **AND** final `git status --short` in the target checkout is clean

### Requirement: Installer applies X11 provider takeover overlay
The project installer MUST be able to apply an X11 provider takeover overlay over the configured Codex Desktop Linux target checkout. The command MUST accept `--provider x11 --mode takeover`, MUST resolve the target from `--target`, `CODEX_DESKTOP_LINUX_FULL_PATH`, or the documented local default, and MUST fail clearly before mutation when required target files or anchors are missing.

#### Scenario: Apply takeover overlay to target checkout
- **GIVEN** `/home/as/Документы/AI_PROJECTS/codex-desktop-linux-full` or `CODEX_DESKTOP_LINUX_FULL_PATH` points to a valid target checkout
- **WHEN** a developer runs the installer with `--provider x11 --mode takeover`
- **THEN** the installer applies only owned provider-takeover patch content to the target checkout
- **AND** the patch report records the target path, target git commit when available, provider `x11`, mode `takeover`, and every file changed
- **AND** the report includes a restart hint for Codex Desktop

#### Scenario: Refuse unsupported provider or mode
- **GIVEN** a valid target checkout
- **WHEN** a developer runs the installer with an unsupported provider or unsupported mode
- **THEN** the command exits with a non-zero status code
- **AND** stderr explains the supported values including `--provider x11 --mode takeover`
- **AND** no target source files or live assets are changed

### Requirement: Live asset backup and patch report
When the takeover installer mutates live Codex Desktop assets, it MUST create restorable backups before writing patched assets and MUST write a machine-readable patch report. The recorded backup metadata MUST be sufficient for uninstall to restore the original bytes, owner, group, mode, size, and checksum or to stop with a safe blocker before modifying the live asset.

#### Scenario: Backup live Computer Use settings asset before patch
- **GIVEN** the live asset directory contains `computer-use-settings-*.js`
- **WHEN** the takeover installer patches live assets
- **THEN** each live asset written by the installer is first copied to a timestamped backup path
- **AND** the backup path is recorded in the provider takeover manifest and patch report
- **AND** the backup metadata records original checksum, size, owner, group, and mode when available
- **AND** the patched asset contains an owned provider-takeover marker
- **AND** the report includes the restart hint required for the running Electron process to load the changed asset

#### Scenario: Dry-run reports target and live asset backup plan without mutation
- **GIVEN** a valid target checkout and live asset path
- **WHEN** a developer runs the takeover installer in dry-run or report-only mode
- **THEN** the command reports planned target and live-asset mutations
- **AND** it reports the backup paths and manifest path that would be used
- **AND** it does not write target source files, live assets, backups, or persistent settings

#### Scenario: Install failure rolls back completed takeover writes
- **GIVEN** the takeover installer has already written one owned source file or live asset during the current install transaction
- **AND** a later source file or live asset write fails
- **WHEN** the installer handles the failure
- **THEN** it attempts to restore every file changed in the current transaction from the transaction backup manifest
- **AND** it leaves a failure report that records which restores succeeded or failed
- **AND** it does not claim the provider takeover is installed unless all required writes and manifest updates succeeded

### Requirement: Rollback restores bundled mode
The installer MUST provide rollback or restore behavior that removes only owned X11 takeover mutations and returns the target and live settings assets to bundled Computer Use mode. Rollback MUST be available through a one-command provider-takeover uninstaller that mirrors the one-command installer.

#### Scenario: One-command rollback restores all owned takeover surfaces
- **GIVEN** `scripts/install-x11-provider-takeover.sh` applied the standalone plugin, provider source overlay, and live asset patch
- **WHEN** a developer runs `scripts/uninstall-x11-provider-takeover.sh` for the same target and Codex home
- **THEN** the standalone plugin cache, marketplace, and owned config sections are removed
- **AND** target source files are restored from the provider takeover manifest backups
- **AND** live assets are restored from recorded backups when those backups exist
- **AND** owned provider-takeover metadata is removed only after source and live asset restore succeeds
- **AND** the rollback report records the restored files and the resulting bundled-mode state

#### Scenario: Rollback refuses unsafe live asset drift
- **GIVEN** a live asset backup is recorded in the provider takeover manifest
- **AND** the current live asset no longer contains the owned provider-takeover marker or its current checksum does not match the expected installed checksum
- **WHEN** a developer runs the provider-takeover uninstaller
- **THEN** the uninstaller stops before overwriting that live asset
- **AND** the report identifies the drifted asset and required manual action
- **AND** unrelated plugin caches, bundled Computer Use files, and unrelated live assets remain unchanged

#### Scenario: Rollback is safe when takeover is absent
- **GIVEN** the target checkout, standalone plugin state, and live assets do not contain owned takeover markers
- **WHEN** a developer runs provider-takeover rollback
- **THEN** the command exits successfully with a no-op clean report
- **AND** it does not remove bundled Computer Use files, Chrome files, unrelated plugins, unrelated backups, or unrelated target changes

#### Scenario: Rollback reports missing manifest instead of blind deletion
- **GIVEN** a target checkout or live asset still contains owned provider-takeover markers
- **AND** the provider takeover manifest is missing or lacks a backup for a marked file
- **WHEN** a developer runs provider-takeover rollback
- **THEN** the command fails with a safe blocker that names the missing manifest or backup
- **AND** it does not blindly delete marked code or modify live assets without a restorable backup

### Requirement: Backup manifest covers source overlay and live asset writes
The source-overlay and provider-takeover installers MUST write a rollback manifest before each mutation that records before-state and after-state for source target files, live webview assets, ownership, mode, and sha256. The manifest MUST distinguish installer-changed state from state that was already present.

#### Scenario: Source overlay records file metadata before patching
- **GIVEN** a valid Codex Desktop Linux target checkout
- **WHEN** the source-overlay installer plans a marker-block or generated-file write
- **THEN** it records the target path, existence, ownership, mode, and sha256 before mutation
- **AND** it records the intended owner, mode, and sha256 after mutation
- **AND** the manifest entry identifies whether the installer changed the file or found it already applied

#### Scenario: Live asset patch records root-owned metadata
- **GIVEN** a live webview asset is selected for provider takeover patching
- **WHEN** live asset patching is authorized and available
- **THEN** the installer backs up the asset before patching
- **AND** it records ownership, mode, and sha256 for the live asset and backup
- **AND** it reports the live asset marker used to prove takeover patching

#### Scenario: Partial install manifest supports rollback
- **GIVEN** installation fails after completing one source or live-asset mutation
- **WHEN** rollback reads the manifest
- **THEN** it can identify completed writes and their before-state
- **AND** it does not assume later planned writes occurred

### Requirement: Rollback reports drift and blockers instead of blind restoration
Rollback and uninstall for source overlay, provider takeover, and live assets MUST compare current state with manifest after-state before restoring before-state. They MUST report drift or blockers when current bytes, ownership, or mode do not match the installer-owned state.

#### Scenario: Rollback restores unchanged installer-owned live asset
- **GIVEN** a manifest records a live asset before-state and installer after-state
- **AND** the current live asset still matches the recorded installer after-state
- **WHEN** rollback runs in write mode
- **THEN** it restores the recorded before-state bytes, ownership, and mode
- **AND** it records the restoration in the rollback report

#### Scenario: Rollback blocks on live asset drift
- **GIVEN** a manifest records a live asset after-state
- **AND** the current live asset sha256 differs from the recorded installer after-state
- **WHEN** rollback evaluates the asset
- **THEN** it refuses blind restoration for that asset
- **AND** it reports the drift as a blocker with the affected path and expected metadata

#### Scenario: Rollback absent overlay is idempotent
- **GIVEN** no source overlay or provider takeover manifest is present
- **WHEN** rollback runs with `--dry-run --report-json`
- **THEN** it emits a JSON report stating that no owned takeover state was found
- **AND** it does not delete unowned target files or live assets

