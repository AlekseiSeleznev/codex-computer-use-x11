use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn temp_dir(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-feature-installer-{name}-{}-{nanos}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, content).unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
}

#[cfg(unix)]
fn write_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    write_file(
        path,
        "#!/bin/sh\nif [ \"$1\" = doctor ]; then echo '{\"project\":\"codex-computer-use-x11\",\"version\":\"test\",\"backend\":\"x11-ewmh\",\"readiness\":{\"ok\":true}}'; exit 0; fi\nif [ \"$1\" = mcp ]; then exit 0; fi\nexit 0\n",
    );
    let mut perms = std::fs::metadata(path).expect("metadata").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).expect("chmod");
}

fn fixture() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let root = temp_dir("fixture");
    let target = root.join("codex-desktop-linux");
    let install_dir = root.join("codex-app");
    let fake_binary = root.join("codex-computer-use-x11");
    std::fs::create_dir_all(target.join("linux-features")).expect("target linux-features");
    write_file(
        &target.join("linux-features/features.json"),
        "{\"enabled\":[]}\n",
    );
    let computer_use_dir =
        install_dir.join("resources/plugins/openai-bundled/plugins/computer-use");
    write_file(
        &computer_use_dir.join(".mcp.json"),
        "{\"mcpServers\":{\"computer-use\":{\"command\":\"./bin/computer-use\"}}}\n",
    );
    write_file(
        &install_dir.join("resources/plugins/openai-bundled/.agents/plugins/marketplace.json"),
        "{\"plugins\":[{\"name\":\"computer-use\",\"source\":{\"path\":\"./plugins/computer-use\"}}]}\n",
    );
    write_file(&install_dir.join("resources/app.asar"), "before-app-asar\n");
    write_file(
        &install_dir.join("content/webview/index.html"),
        "before-webview\n",
    );
    write_executable(&fake_binary);
    (root, target, install_dir, fake_binary)
}

fn run(script: &str, args: &[&str]) -> Output {
    Command::new(repo_root().join("scripts").join(script))
        .args(args)
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|err| panic!("run scripts/{script}: {err}"))
}

fn parse_stdout(output: &Output) -> Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|err| panic!("parse JSON: {err}\n{stdout}"))
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
#[test]
fn installer_dry_run_reports_plan_without_mutation() {
    let (_root, target, install_dir, fake_binary) = fixture();
    let output = run(
        "install-codex-desktop-linux-x11-feature.sh",
        &[
            "--target",
            target.to_str().unwrap(),
            "--install-dir",
            install_dir.to_str().unwrap(),
            "--binary",
            fake_binary.to_str().unwrap(),
            "--patch-mode",
            "fake",
            "--dry-run",
            "--report-json",
            "-",
        ],
    );
    assert_success(&output);
    let report = parse_stdout(&output);
    assert_eq!(
        report["operation"],
        "install-codex-desktop-linux-x11-feature"
    );
    assert_eq!(report["dry_run"], true);
    assert_eq!(report["feature_id"], "x11-ewmh-computer-use");
    assert!(report["planned_surfaces"].as_array().unwrap().len() >= 4);
    assert!(!target
        .join("linux-features/local/x11-ewmh-computer-use")
        .exists());
    assert!(!install_dir
        .join("resources/plugins/openai-bundled/plugins/codex-computer-use-x11")
        .exists());
    assert!(!install_dir
        .join(".codex-x11-feature/install-manifest.json")
        .exists());
}

#[cfg(unix)]
#[test]
fn installer_stages_feature_plugin_and_manifest() {
    let (_root, target, install_dir, fake_binary) = fixture();
    let computer_use_before = std::fs::read_to_string(
        install_dir.join("resources/plugins/openai-bundled/plugins/computer-use/.mcp.json"),
    )
    .expect("read computer-use before");
    let output = run(
        "install-codex-desktop-linux-x11-feature.sh",
        &[
            "--target",
            target.to_str().unwrap(),
            "--install-dir",
            install_dir.to_str().unwrap(),
            "--binary",
            fake_binary.to_str().unwrap(),
            "--patch-mode",
            "fake",
            "--report-json",
            "-",
        ],
    );
    assert_success(&output);
    let report = parse_stdout(&output);
    assert_eq!(report["dry_run"], false);

    assert!(target
        .join("linux-features/local/x11-ewmh-computer-use/feature.json")
        .exists());
    let features: Value = serde_json::from_str(
        &std::fs::read_to_string(target.join("linux-features/features.json")).unwrap(),
    )
    .unwrap();
    assert!(features["enabled"]
        .as_array()
        .unwrap()
        .iter()
        .any(|v| v == "x11-ewmh-computer-use"));

    let plugin =
        install_dir.join("resources/plugins/openai-bundled/plugins/codex-computer-use-x11");
    assert!(plugin.join(".mcp.json").exists());
    assert!(plugin.join("bin/codex-computer-use-x11").exists());
    assert_eq!(
        std::fs::read_to_string(
            install_dir.join("resources/plugins/openai-bundled/plugins/computer-use/.mcp.json")
        )
        .unwrap(),
        computer_use_before
    );
    let marketplace: Value = serde_json::from_str(
        &std::fs::read_to_string(
            install_dir.join("resources/plugins/openai-bundled/.agents/plugins/marketplace.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let plugins = marketplace["plugins"].as_array().unwrap();
    assert!(plugins.iter().any(|p| p["name"] == "computer-use"));
    assert!(plugins
        .iter()
        .any(|p| p["name"] == "codex-computer-use-x11"));

    let manifest_path = install_dir.join(".codex-x11-feature/install-manifest.json");
    assert!(manifest_path.exists());
    let manifest: Value =
        serde_json::from_str(&std::fs::read_to_string(manifest_path).unwrap()).unwrap();
    assert_eq!(
        manifest["operation"],
        "install-codex-desktop-linux-x11-feature"
    );
    assert!(manifest["entries"]
        .as_array()
        .unwrap()
        .iter()
        .any(|entry| entry["id"] == "plugin_dir" && entry["completed"] == true));
    assert!(
        std::fs::read_to_string(install_dir.join("resources/app.asar"))
            .unwrap()
            .contains("codex-computer-use-x11 fake patch")
    );
}

#[cfg(unix)]
#[test]
fn uninstaller_restores_clean_install_and_supports_dry_run() {
    let (_root, target, install_dir, fake_binary) = fixture();
    let marketplace_before = std::fs::read_to_string(
        install_dir.join("resources/plugins/openai-bundled/.agents/plugins/marketplace.json"),
    )
    .unwrap();
    let app_before = std::fs::read_to_string(install_dir.join("resources/app.asar")).unwrap();
    let webview_before =
        std::fs::read_to_string(install_dir.join("content/webview/index.html")).unwrap();

    assert_success(&run(
        "install-codex-desktop-linux-x11-feature.sh",
        &[
            "--target",
            target.to_str().unwrap(),
            "--install-dir",
            install_dir.to_str().unwrap(),
            "--binary",
            fake_binary.to_str().unwrap(),
            "--patch-mode",
            "fake",
        ],
    ));

    let dry = run(
        "uninstall-codex-desktop-linux-x11-feature.sh",
        &[
            "--install-dir",
            install_dir.to_str().unwrap(),
            "--dry-run",
            "--report-json",
            "-",
        ],
    );
    assert_success(&dry);
    assert!(install_dir
        .join("resources/plugins/openai-bundled/plugins/codex-computer-use-x11")
        .exists());

    let uninstall = run(
        "uninstall-codex-desktop-linux-x11-feature.sh",
        &[
            "--install-dir",
            install_dir.to_str().unwrap(),
            "--report-json",
            "-",
        ],
    );
    assert_success(&uninstall);
    let report = parse_stdout(&uninstall);
    assert_eq!(
        report["operation"],
        "uninstall-codex-desktop-linux-x11-feature"
    );
    assert!(!install_dir
        .join("resources/plugins/openai-bundled/plugins/codex-computer-use-x11")
        .exists());
    assert_eq!(
        std::fs::read_to_string(
            install_dir.join("resources/plugins/openai-bundled/.agents/plugins/marketplace.json")
        )
        .unwrap(),
        marketplace_before
    );
    assert_eq!(
        std::fs::read_to_string(install_dir.join("resources/app.asar")).unwrap(),
        app_before
    );
    assert_eq!(
        std::fs::read_to_string(install_dir.join("content/webview/index.html")).unwrap(),
        webview_before
    );
    assert!(report["restored"].as_array().unwrap().len() >= 3);
}

#[cfg(unix)]
#[test]
fn uninstaller_blocks_on_drift() {
    let (_root, target, install_dir, fake_binary) = fixture();
    assert_success(&run(
        "install-codex-desktop-linux-x11-feature.sh",
        &[
            "--target",
            target.to_str().unwrap(),
            "--install-dir",
            install_dir.to_str().unwrap(),
            "--binary",
            fake_binary.to_str().unwrap(),
            "--patch-mode",
            "fake",
        ],
    ));
    let app_path = install_dir.join("resources/app.asar");
    write_file(&app_path, "admin changed app after install\n");

    let output = run(
        "uninstall-codex-desktop-linux-x11-feature.sh",
        &[
            "--install-dir",
            install_dir.to_str().unwrap(),
            "--report-json",
            "-",
        ],
    );
    assert!(!output.status.success(), "drift should fail");
    let report = parse_stdout(&output);
    assert_eq!(report["success"], false);
    assert!(report["blockers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|b| b["id"] == "app_asar"));
    assert_eq!(
        std::fs::read_to_string(app_path).unwrap(),
        "admin changed app after install\n"
    );
}
