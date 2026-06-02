use std::{io::Write, process::Command};

#[cfg(unix)]
fn temp_dir(name: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-source-overlay-{name}-{}-{nanos}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[cfg(unix)]
fn script(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("scripts")
        .join(name)
}

#[cfg(unix)]
fn write_file(path: &std::path::Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dir");
    }
    std::fs::write(path, content).expect("write fixture file");
}

#[cfg(unix)]
fn fake_target(name: &str) -> std::path::PathBuf {
    let dir = temp_dir(name);
    write_file(
        &dir.join("computer-use-linux/Cargo.toml"),
        "[package]\nname = \"codex-computer-use-linux\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    );
    write_file(
        &dir.join("computer-use-linux/src/windowing/backends/mod.rs"),
        "pub mod cosmic;\npub mod gnome;\npub mod hyprland;\npub mod i3;\npub mod kwin;\n",
    );
    write_file(
        &dir.join("computer-use-linux/src/windowing/registry.rs"),
        r#"use crate::windowing::backends::{cosmic, gnome, hyprland, i3, kwin};
pub use i3::I3_BACKEND;
enum BackendKind {
    GnomeExtension,
    GnomeIntrospect,
    Cosmic,
    Kwin,
    Hyprland,
    I3,
}
const BACKEND_ORDER: &[BackendKind] = &[
    BackendKind::GnomeExtension,
    BackendKind::GnomeIntrospect,
    BackendKind::Cosmic,
    BackendKind::Kwin,
    BackendKind::Hyprland,
    BackendKind::I3,
];
fn list_windows_for(backend: BackendKind) {
    match backend {
        BackendKind::I3 => i3::list_windows(),
    }
}
fn activate_window(window: &WindowInfo) {
    match window.backend.as_str() {
        I3_BACKEND => i3::activate_window(window.window_id),
        _ => (),
    }
}
fn probe_backends() {
    vec![
        i3::probe(),
    ];
}
impl BackendKind {
    fn id(self) -> &'static str {
        match self {
            BackendKind::I3 => I3_BACKEND,
            _ => "other",
        }
    }
}
"#,
    );
    write_file(
        &dir.join("computer-use-linux/src/windowing/mod.rs"),
        "pub mod backends;\npub mod registry;\npub mod target;\npub mod types;\n",
    );
    write_file(
        &dir.join("computer-use-linux/src/diagnostics.rs"),
        "fn portal_interface_check(interface: &str) -> Check {\n    command_check_with_session_bus(\n        \"busctl\",\n        &[\n            \"--user\",\n            \"introspect\",\n            \"org.freedesktop.portal.Desktop\",\n            \"/org/freedesktop/portal/desktop\",\n            interface,\n        ],\n    )\n}\n",
    );
    dir
}

#[cfg(unix)]
#[test]
fn install_refuses_missing_target_structure() {
    let dir = temp_dir("missing-target-structure");

    let output = Command::new(script("install-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run install source overlay script");

    assert!(!output.status.success(), "missing target should fail");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("missing target structure"),
        "stderr should explain missing structure: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !dir.join("computer-use-linux/src/windowing/backends/x11_ewmh.rs")
            .exists(),
        "failed preflight must not create generated backend"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn status_reports_clean_target_and_env_default() {
    let dir = fake_target("status-clean");

    let explicit = Command::new(script("status-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run status source overlay script");
    assert!(explicit.status.success(), "clean status should succeed");
    let stdout = String::from_utf8_lossy(&explicit.stdout);
    assert!(stdout.contains("state=clean"), "stdout={stdout}");
    assert!(stdout.contains("target_commit="), "stdout={stdout}");

    let from_env = Command::new(script("status-codex-source-overlay.sh"))
        .env("CODEX_DESKTOP_LINUX_FULL_PATH", &dir)
        .output()
        .expect("run status with env target");
    assert!(
        from_env.status.success(),
        "env target status should succeed"
    );
    let env_stdout = String::from_utf8_lossy(&from_env.stdout);
    assert!(env_stdout.contains("state=clean"), "stdout={env_stdout}");
    assert!(
        env_stdout.contains(&format!("target={}", dir.canonicalize().unwrap().display())),
        "stdout={env_stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn install_creates_backend_and_marker_blocks() {
    let dir = fake_target("install-markers");

    let output = Command::new(script("install-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run install source overlay script");

    assert!(
        output.status.success(),
        "install should succeed\nstderr={}\nstdout={}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    let backend = dir.join("computer-use-linux/src/windowing/backends/x11_ewmh.rs");
    assert!(backend.exists(), "generated backend should exist");
    let backend_text = std::fs::read_to_string(&backend).expect("read generated backend");
    assert!(backend_text.contains("X11_EWMH_BACKEND"));
    assert!(backend_text.contains("x11-ewmh"));

    for rel in [
        "computer-use-linux/src/windowing/backends/mod.rs",
        "computer-use-linux/src/windowing/registry.rs",
        "computer-use-linux/src/windowing/mod.rs",
        "computer-use-linux/src/diagnostics.rs",
    ] {
        let text = std::fs::read_to_string(dir.join(rel)).expect("read patched file");
        assert!(
            text.contains("BEGIN codex-computer-use-x11"),
            "{rel} should contain owned begin marker\n{text}"
        );
        assert!(
            text.contains("END codex-computer-use-x11"),
            "{rel} should contain owned end marker\n{text}"
        );
    }

    let registry =
        std::fs::read_to_string(dir.join("computer-use-linux/src/windowing/registry.rs"))
            .expect("read registry");
    assert!(registry.contains("BackendKind::X11Ewmh"));
    assert!(registry.contains("x11_ewmh::list_windows()"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
fn marker_count_in_target(dir: &std::path::Path) -> usize {
    [
        "computer-use-linux/src/windowing/backends/mod.rs",
        "computer-use-linux/src/windowing/registry.rs",
        "computer-use-linux/src/windowing/mod.rs",
        "computer-use-linux/src/diagnostics.rs",
    ]
    .iter()
    .map(|rel| {
        std::fs::read_to_string(dir.join(rel))
            .expect("read patched file")
            .matches("BEGIN codex-computer-use-x11")
            .count()
    })
    .sum()
}

#[cfg(unix)]
#[test]
fn install_is_idempotent() {
    let dir = fake_target("install-idempotent");

    let first = Command::new(script("install-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run first install source overlay script");
    assert!(
        first.status.success(),
        "first install should succeed: stderr={}",
        String::from_utf8_lossy(&first.stderr)
    );
    let first_markers = marker_count_in_target(&dir);

    let second = Command::new(script("install-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run second install source overlay script");
    assert!(
        second.status.success(),
        "second install should succeed: stderr={}",
        String::from_utf8_lossy(&second.stderr)
    );

    let markers = marker_count_in_target(&dir);
    assert_eq!(
        markers, first_markers,
        "marker count should remain stable after repeated install"
    );
    let backend = dir.join("computer-use-linux/src/windowing/backends/x11_ewmh.rs");
    let backend_text = std::fs::read_to_string(&backend).expect("read generated backend");
    assert_eq!(
        backend_text
            .matches("Generated by codex-computer-use-x11 source overlay")
            .count(),
        1,
        "generated backend should not be duplicated"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn status_reports_applied_and_drifted() {
    let dir = fake_target("status-applied-drifted");
    let install = Command::new(script("install-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run install source overlay script");
    assert!(install.status.success(), "install should succeed");

    let applied = Command::new(script("status-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run status source overlay script");
    assert!(applied.status.success(), "applied status should succeed");
    let applied_stdout = String::from_utf8_lossy(&applied.stdout);
    assert!(
        applied_stdout.contains("state=applied"),
        "stdout={applied_stdout}"
    );
    assert!(
        applied_stdout.contains("backend=computer-use-linux/src/windowing/backends/x11_ewmh.rs")
    );

    let backend = dir.join("computer-use-linux/src/windowing/backends/x11_ewmh.rs");
    std::fs::OpenOptions::new()
        .append(true)
        .open(&backend)
        .expect("open backend for drift")
        .write_all(b"// local drift\n")
        .expect("append drift");

    let drifted = Command::new(script("status-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run drifted status source overlay script");
    assert!(!drifted.status.success(), "drifted status should fail");
    let drifted_stdout = String::from_utf8_lossy(&drifted.stdout);
    assert!(
        drifted_stdout.contains("state=drifted"),
        "stdout={drifted_stdout}"
    );
    assert!(
        drifted_stdout.contains("detail="),
        "stdout={drifted_stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn uninstall_removes_only_owned_content() {
    let dir = fake_target("uninstall-owned-content");
    let registry_path = dir.join("computer-use-linux/src/windowing/registry.rs");
    std::fs::OpenOptions::new()
        .append(true)
        .open(&registry_path)
        .expect("open registry")
        .write_all(b"\n// unrelated sentinel\n")
        .expect("write sentinel");

    let install = Command::new(script("install-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run install source overlay script");
    assert!(install.status.success(), "install should succeed");

    let uninstall = Command::new(script("uninstall-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run uninstall source overlay script");
    assert!(
        uninstall.status.success(),
        "uninstall should succeed: stderr={}",
        String::from_utf8_lossy(&uninstall.stderr)
    );

    assert!(
        !dir.join("computer-use-linux/src/windowing/backends/x11_ewmh.rs")
            .exists(),
        "owned generated backend should be removed"
    );
    for rel in [
        "computer-use-linux/src/windowing/backends/mod.rs",
        "computer-use-linux/src/windowing/registry.rs",
        "computer-use-linux/src/windowing/mod.rs",
        "computer-use-linux/src/diagnostics.rs",
    ] {
        let text = std::fs::read_to_string(dir.join(rel)).expect("read patched file");
        assert!(
            !text.contains("BEGIN codex-computer-use-x11"),
            "{rel} still has marker"
        );
        assert!(
            !text.contains("END codex-computer-use-x11"),
            "{rel} still has marker"
        );
    }
    let registry = std::fs::read_to_string(&registry_path).expect("read registry");
    assert!(
        registry.contains("unrelated sentinel"),
        "unrelated content must remain"
    );

    let second = Command::new(script("uninstall-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run second uninstall source overlay script");
    assert!(
        second.status.success(),
        "second uninstall should be idempotent"
    );

    let status = Command::new(script("status-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run status source overlay script");
    assert!(status.status.success(), "clean status should succeed");
    assert!(String::from_utf8_lossy(&status.stdout).contains("state=clean"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn install_refuses_unowned_native_x11_backend() {
    let dir = fake_target("unowned-native-x11");
    let backend = dir.join("computer-use-linux/src/windowing/backends/x11_ewmh.rs");
    write_file(
        &backend,
        "// upstream native backend\npub const X11_EWMH_BACKEND: &str = \"x11-ewmh\";\n",
    );

    let output = Command::new(script("install-codex-source-overlay.sh"))
        .args(["--target", dir.to_str().unwrap()])
        .output()
        .expect("run install source overlay script");

    assert!(
        !output.status.success(),
        "unowned native backend should be refused"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unowned native X11 backend"),
        "stderr={stderr}"
    );
    let backend_text = std::fs::read_to_string(&backend).expect("read backend");
    assert!(backend_text.contains("upstream native backend"));
    assert!(!backend_text.contains("Generated by codex-computer-use-x11"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
fn fake_provider_target(name: &str) -> std::path::PathBuf {
    let dir = temp_dir(name);
    write_file(
        &dir.join("scripts/patches/computer-use.js"),
        "module.exports = { applyX11ComputerUseSettingsRowPatch(source) { return source; } };\n",
    );
    write_file(
        &dir.join("scripts/patch-linux-window-ui.js"),
        "const { applyX11ComputerUseSettingsRowPatch } = require(\"./patches/computer-use.js\");\nmodule.exports = { applyX11ComputerUseSettingsRowPatch };\n",
    );
    write_file(
        &dir.join("scripts/patch-linux-window-ui.test.js"),
        "const test = require(\"node:test\");\nconst assert = require(\"node:assert/strict\");\ntest(\"baseline descriptor expectation\", () => assert.equal(1, 1));\n",
    );
    write_file(
        &dir.join("scripts/patches/core/all-linux/webview/computer-use-ui/patch.js"),
        "module.exports = [{ id: \"linux-x11-computer-use-settings-row\", pattern: /^computer-use-settings-.*\\\\.js$/ }];\n",
    );
    let init = Command::new("git")
        .args(["init", "-q", "--initial-branch", "main"])
        .current_dir(&dir)
        .output()
        .expect("git init fake provider target");
    assert!(init.status.success(), "git init failed");
    let add = Command::new("git")
        .args(["add", "."])
        .current_dir(&dir)
        .output()
        .expect("git add fake provider target");
    assert!(add.status.success(), "git add failed");
    let commit = Command::new("git")
        .env("GIT_AUTHOR_NAME", "Codex Test")
        .env("GIT_AUTHOR_EMAIL", "codex-test@example.invalid")
        .env("GIT_COMMITTER_NAME", "Codex Test")
        .env("GIT_COMMITTER_EMAIL", "codex-test@example.invalid")
        .args(["commit", "-q", "-m", "baseline"])
        .current_dir(&dir)
        .output()
        .expect("git commit fake provider target");
    assert!(commit.status.success(), "git commit failed");
    dir
}

#[cfg(unix)]
#[test]
fn provider_takeover_install_writes_report_and_restart_hint() {
    let dir = fake_provider_target("provider-install-report");
    let report = dir.join("takeover-report.json");

    let output = Command::new(script("install-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
            "--report-json",
            report.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover install");

    assert!(
        output.status.success(),
        "provider takeover install should succeed\nstderr={}\nstdout={}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("state=applied"), "stdout={stdout}");
    assert!(stdout.contains("restart_hint="), "stdout={stdout}");

    let computer_use = std::fs::read_to_string(dir.join("scripts/patches/computer-use.js"))
        .expect("read provider source");
    assert!(
        computer_use.contains("codex-computer-use-x11-provider-takeover:v1"),
        "provider source should contain takeover marker"
    );
    let descriptor = std::fs::read_to_string(
        dir.join("scripts/patches/core/all-linux/webview/computer-use-ui/patch.js"),
    )
    .expect("read descriptor");
    assert!(descriptor.contains("linux-x11-computer-use-provider-takeover"));
    let target_tests = std::fs::read_to_string(dir.join("scripts/patch-linux-window-ui.test.js"))
        .expect("read overlaid target tests");
    assert!(target_tests.contains("upgrades marketplace-filtered X11 provider lookup"));

    let report_json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&report).expect("read provider takeover report"),
    )
    .expect("parse provider takeover report");
    assert_eq!(report_json["provider"], "x11");
    assert_eq!(report_json["mode"], "takeover");
    assert_eq!(
        report_json["marker_version"],
        "codex-computer-use-x11-provider-takeover:v1"
    );
    assert_eq!(report_json["state"], "applied");
    assert!(report_json["changed_files"]
        .as_array()
        .unwrap()
        .iter()
        .any(|file| {
            file.as_str()
                .unwrap()
                .ends_with("scripts/patches/computer-use.js")
        }));
    assert!(report_json["restart_hint"]
        .as_str()
        .unwrap()
        .contains("restart"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn provider_takeover_dry_run_live_assets_uses_overlay_patcher() {
    let dir = fake_provider_target("provider-dry-run-live-unpatched");
    let live_dir = dir.join("live-assets");
    let asset = live_dir.join("computer-use-settings-test.js");
    let original_asset =
        "function rows(){let m=p,h;if(r.available&&m!=null){w.push(r)}if(g!=null){w.push(g)}}\n";
    write_file(&asset, original_asset);
    write_file(
        &dir.join("scripts/patches/computer-use.js"),
        "module.exports = { applyLinuxComputerUseFeaturePatch(source) { return source; } };\n",
    );

    let output = Command::new(script("install-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
            "--patch-live-assets",
            "--live-assets-dir",
            live_dir.to_str().unwrap(),
            "--dry-run",
        ])
        .output()
        .expect("run provider takeover dry-run with live assets");

    assert!(
        output.status.success(),
        "dry-run with live assets should succeed against an unpatched fresh target\nstderr={}\nstdout={}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(
        std::fs::read_to_string(&asset).expect("read live asset after dry-run"),
        original_asset,
        "dry-run must not mutate live assets"
    );
    let target_source = std::fs::read_to_string(dir.join("scripts/patches/computer-use.js"))
        .expect("read target source after dry-run");
    assert!(
        !target_source.contains("codex-computer-use-x11-provider-takeover:v1"),
        "dry-run must not mutate target source"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn provider_takeover_rejects_unsupported_provider_mode_before_mutation() {
    let dir = fake_provider_target("provider-unsupported");
    let before = std::fs::read_to_string(dir.join("scripts/patches/computer-use.js"))
        .expect("read provider source before");

    let output = Command::new(script("install-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "computer-use",
            "--mode",
            "takeover",
        ])
        .output()
        .expect("run unsupported provider takeover install");

    assert!(!output.status.success(), "unsupported provider should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unsupported provider/mode"),
        "stderr={stderr}"
    );
    let after = std::fs::read_to_string(dir.join("scripts/patches/computer-use.js"))
        .expect("read provider source after");
    assert_eq!(before, after, "unsupported args must not mutate target");

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn provider_takeover_rollback_restores_source_and_live_asset_backup() {
    let dir = fake_provider_target("provider-rollback-live");
    let live_dir = dir.join("live-assets");
    let asset = live_dir.join("computer-use-settings-test.js");
    let original_asset =
        "function rows(){let m=p,h;if(r.available&&m!=null){w.push(r)}if(g!=null){w.push(g)}}\n";
    write_file(&asset, original_asset);
    let install_report = dir.join("install-report.json");

    let install = Command::new(script("install-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
            "--patch-live-assets",
            "--live-assets-dir",
            live_dir.to_str().unwrap(),
            "--report-json",
            install_report.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover install with live assets");
    assert!(
        install.status.success(),
        "install with live assets should succeed\nstderr={}\nstdout={}",
        String::from_utf8_lossy(&install.stderr),
        String::from_utf8_lossy(&install.stdout)
    );
    let patched_asset = std::fs::read_to_string(&asset).expect("read patched live asset");
    assert!(patched_asset.contains("codex-computer-use-x11-provider-takeover:v1"));
    assert!(patched_asset
        .contains("codexLinuxComputerUseTakeoverProvider=((d.installedPlugins??[]).find"));
    assert!(!patched_asset.contains("codexLinuxComputerUseTakeoverProvider=X(d.installedPlugins"));
    assert!(
        !patched_asset.contains("push({id:`codex-computer-use-x11-unavailable`"),
        "unavailable X11 provider placeholder must not be inserted into plugin items"
    );
    let report_json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&install_report).expect("read install report"),
    )
    .expect("parse install report");
    assert_eq!(report_json["schema_version"], 1);
    let source_backups = report_json["source_backups"]
        .as_array()
        .expect("source_backups should be an array");
    assert!(
        !source_backups.is_empty(),
        "install report should include source backup metadata: {report_json}"
    );
    for backup in source_backups {
        for key in [
            "before_sha256",
            "before_size",
            "installed_sha256",
            "installed_size",
            "before_mode",
            "before_uid",
            "before_gid",
            "installed_mode",
            "installed_uid",
            "installed_gid",
            "before",
            "after",
            "installer_changed",
            "completed",
            "kind",
        ] {
            assert!(
                backup.get(key).is_some(),
                "source backup should include {key}: {backup}"
            );
        }
    }
    let live_backups = report_json["live_asset_backups"]
        .as_array()
        .expect("live_asset_backups should be an array");
    assert!(
        !live_backups.is_empty(),
        "install report should include live asset backup metadata: {report_json}"
    );
    for backup in live_backups {
        for key in [
            "before_sha256",
            "before_size",
            "before_mode",
            "before_uid",
            "before_gid",
            "installed_sha256",
            "installed_size",
            "installed_mode",
            "installed_uid",
            "installed_gid",
            "before",
            "after",
            "installer_changed",
            "completed",
            "kind",
        ] {
            assert!(
                backup.get(key).is_some(),
                "live asset backup should include {key}: {backup}"
            );
        }
    }

    let rollback = Command::new(script("uninstall-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
        ])
        .output()
        .expect("run provider takeover rollback");
    assert!(
        rollback.status.success(),
        "rollback should succeed\nstderr={}\nstdout={}",
        String::from_utf8_lossy(&rollback.stderr),
        String::from_utf8_lossy(&rollback.stdout)
    );
    let restored_source = std::fs::read_to_string(dir.join("scripts/patches/computer-use.js"))
        .expect("read restored provider source");
    assert!(!restored_source.contains("codex-computer-use-x11-provider-takeover:v1"));
    let restored_asset = std::fs::read_to_string(&asset).expect("read restored live asset");
    assert_eq!(restored_asset, original_asset);

    let second = Command::new(script("uninstall-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
        ])
        .output()
        .expect("run absent provider takeover rollback");
    assert!(second.status.success(), "absent rollback should no-op");
    assert!(String::from_utf8_lossy(&second.stdout).contains("state=clean"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn provider_takeover_rollback_refuses_live_asset_drift() {
    let dir = fake_provider_target("provider-rollback-drift");
    let live_dir = dir.join("live-assets");
    let asset = live_dir.join("computer-use-settings-test.js");
    write_file(
        &asset,
        "function rows(){let m=p,h;if(r.available&&m!=null){w.push(r)}if(g!=null){w.push(g)}}\n",
    );

    let install = Command::new(script("install-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
            "--patch-live-assets",
            "--live-assets-dir",
            live_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover install with live assets");
    assert!(install.status.success(), "install should succeed");
    std::fs::write(&asset, "// unknown user edit without owned marker\n").expect("write drift");

    let rollback = Command::new(script("uninstall-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
        ])
        .output()
        .expect("run provider takeover rollback with live drift");
    assert!(!rollback.status.success(), "drifted rollback should fail");
    let stderr = String::from_utf8_lossy(&rollback.stderr);
    assert!(stderr.contains("live asset drift"), "stderr={stderr}");
    assert_eq!(
        std::fs::read_to_string(&asset).expect("read drifted asset"),
        "// unknown user edit without owned marker\n"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn provider_takeover_rollback_refuses_live_marker_without_manifest() {
    let dir = fake_provider_target("provider-rollback-missing-manifest");
    let live_dir = dir.join("live-assets");
    let asset = live_dir.join("computer-use-settings-test.js");
    write_file(
        &asset,
        "/*codex-computer-use-x11-provider-takeover:v1*/ const codexLinuxComputerUseTakeoverProvider = {};\n",
    );

    let rollback = Command::new(script("uninstall-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
            "--live-assets-dir",
            live_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover rollback without manifest");

    assert!(
        !rollback.status.success(),
        "missing manifest rollback should fail"
    );
    let stderr = String::from_utf8_lossy(&rollback.stderr);
    assert!(
        stderr.contains("owned backup manifest"),
        "stderr should explain missing manifest: {stderr}"
    );
    assert!(
        std::fs::read_to_string(&asset)
            .expect("read live marker asset")
            .contains("codex-computer-use-x11-provider-takeover:v1"),
        "rollback without manifest must not blindly edit live assets"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn provider_takeover_uninstall_wrapper_restores_overlay_live_asset_and_plugin_state() {
    let dir = fake_provider_target("provider-wrapper-uninstall");
    let live_dir = dir.join("live-assets");
    let asset = live_dir.join("computer-use-settings-test.js");
    let original_asset =
        "function rows(){let m=p,h;if(r.available&&m!=null){w.push(r)}if(g!=null){w.push(g)}}\n";
    write_file(&asset, original_asset);
    let codex_home = dir.join("codex-home");
    write_file(
        &codex_home.join("config.toml"),
        "[plugins.\"keep@market\"]\nenabled = true\n\n[plugins.\"codex-computer-use-x11@codex-computer-use-x11\"]\nenabled = true\n\n[marketplaces.codex-computer-use-x11]\nsource_type = \"local\"\nsource = \"owned\"\n",
    );
    std::fs::create_dir_all(codex_home.join("plugins/cache/codex-computer-use-x11"))
        .expect("create fake plugin cache");
    std::fs::create_dir_all(codex_home.join("plugins/marketplaces/codex-computer-use-x11"))
        .expect("create fake marketplace");

    let install = Command::new(script("install-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
            "--patch-live-assets",
            "--live-assets-dir",
            live_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover install before wrapper uninstall");
    assert!(install.status.success(), "install should succeed");

    let report = dir.join("uninstall-wrapper-report.json");
    let uninstall = Command::new(script("uninstall-x11-provider-takeover.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--codex-home",
            codex_home.to_str().unwrap(),
            "--live-assets-dir",
            live_dir.to_str().unwrap(),
            "--report-json",
            report.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover uninstall wrapper");

    assert!(
        uninstall.status.success(),
        "wrapper uninstall should succeed\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&uninstall.stdout),
        String::from_utf8_lossy(&uninstall.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(&asset).expect("read restored asset"),
        original_asset
    );
    assert!(
        !std::fs::read_to_string(dir.join("scripts/patches/computer-use.js"))
            .expect("read restored provider source")
            .contains("codex-computer-use-x11-provider-takeover:v1")
    );
    assert!(!codex_home
        .join("plugins/cache/codex-computer-use-x11")
        .exists());
    assert!(!codex_home
        .join("plugins/marketplaces/codex-computer-use-x11")
        .exists());
    let config = std::fs::read_to_string(codex_home.join("config.toml")).expect("read config");
    assert!(config.contains("[plugins.\"keep@market\"]"));
    assert!(!config.contains("codex-computer-use-x11@codex-computer-use-x11"));
    assert!(!config.contains("marketplaces.codex-computer-use-x11"));
    let report_json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&report).expect("read wrapper report"))
            .expect("parse wrapper report");
    assert_eq!(report_json["operation"], "uninstall-x11-provider-takeover");
    assert_eq!(report_json["source_overlay"]["state"], "clean");
    assert_eq!(report_json["plugin"]["status"], "uninstalled");
    assert_eq!(report_json["live_assets"]["status"], "clean");

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn provider_takeover_uninstall_wrapper_dry_run_allows_pending_live_markers() {
    let dir = fake_provider_target("provider-wrapper-uninstall-dry-run-live-markers");
    let live_dir = dir.join("live-assets");
    let asset = live_dir.join("computer-use-settings-test.js");
    let original_asset =
        "function rows(){let m=p,h;if(r.available&&m!=null){w.push(r)}if(g!=null){w.push(g)}}\n";
    write_file(&asset, original_asset);

    let install = Command::new(script("install-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
            "--patch-live-assets",
            "--live-assets-dir",
            live_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover install before wrapper dry-run uninstall");
    assert!(install.status.success(), "install should succeed");
    let patched_asset = std::fs::read_to_string(&asset).expect("read patched live asset");
    assert!(patched_asset.contains("codex-computer-use-x11-provider-takeover:v1"));

    let report = dir.join("uninstall-wrapper-dry-run-report.json");
    let uninstall = Command::new(script("uninstall-x11-provider-takeover.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--live-assets-dir",
            live_dir.to_str().unwrap(),
            "--no-plugin",
            "--dry-run",
            "--report-json",
            report.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover uninstall wrapper dry-run");

    assert!(
        uninstall.status.success(),
        "dry-run wrapper uninstall should not require markers to be absent before mutation\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&uninstall.stdout),
        String::from_utf8_lossy(&uninstall.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(&asset).expect("read live asset after dry-run"),
        patched_asset,
        "dry-run must not mutate live assets"
    );
    let report_json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&report).expect("read wrapper report"))
            .expect("parse wrapper report");
    assert_eq!(report_json["operation"], "uninstall-x11-provider-takeover");
    assert_eq!(report_json["source_overlay"]["state"], "dry-run");
    assert_eq!(report_json["plugin"]["status"], "skipped");
    assert_eq!(report_json["live_assets"]["status"], "dry-run");

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn provider_takeover_install_failure_restores_current_transaction_source_writes() {
    let dir = fake_provider_target("provider-install-failure-rollback");
    let empty_live_dir = dir.join("empty-live-assets");
    std::fs::create_dir_all(&empty_live_dir).expect("create empty live dir");
    let original_source = std::fs::read_to_string(dir.join("scripts/patches/computer-use.js"))
        .expect("read original provider source");

    let output = Command::new(script("install-codex-source-overlay.sh"))
        .args([
            "--target",
            dir.to_str().unwrap(),
            "--provider",
            "x11",
            "--mode",
            "takeover",
            "--patch-live-assets",
            "--live-assets-dir",
            empty_live_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run provider takeover install with failing live asset phase");

    assert!(
        !output.status.success(),
        "install should fail when live assets are missing"
    );
    let restored_source = std::fs::read_to_string(dir.join("scripts/patches/computer-use.js"))
        .expect("read provider source after failed install");
    assert_eq!(
        restored_source, original_source,
        "failed install must restore source writes from the current transaction"
    );
    assert!(
        !dir.join(".codex-computer-use-x11-overlay/provider-takeover/manifest.json")
            .exists(),
        "failed install must not persist a success manifest"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
