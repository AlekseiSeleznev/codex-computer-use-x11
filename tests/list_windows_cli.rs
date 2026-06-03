use std::process::Command;

fn run_cli_without_display(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(args)
        .env_remove("DISPLAY")
        .output()
        .expect("run codex-computer-use-x11")
}

#[test]
fn list_windows_cli_degrades_without_display() {
    let output = run_cli_without_display(&["list-windows", "--json"]);
    assert!(output.status.success(), "status: {:?}", output.status);
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(report["backend"], "x11-ewmh");
    assert_eq!(report["windows"].as_array().unwrap().len(), 0);
    assert!(report["diagnostics"]["blockers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item.as_str().unwrap_or_default().contains("DISPLAY")));
}

#[cfg(unix)]
fn write_executable(path: &std::path::Path, content: &str) {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    let mut file = std::fs::File::create(path).expect("create fake command");
    file.write_all(content.as_bytes())
        .expect("write fake command");
    let mut permissions = file.metadata().unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).expect("chmod fake command");
}

#[cfg(unix)]
#[test]
fn list_windows_cli_outputs_windows_with_fake_commands() {
    let temp = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-list-windows-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp).expect("create temp dir");
    write_executable(
        &temp.join("wmctrl"),
        "#!/bin/sh\ncat <<'OUT'\n0x1 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 1024 768 app.App testhost Second Window\nOUT\n",
    );
    write_executable(
        &temp.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelse\n  echo 'unexpected xprop args' >&2\n  exit 1\nfi\n",
    );

    let path = format!(
        "{}:{}",
        temp.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let output = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(["list-windows", "--json"])
        .env("DISPLAY", ":99")
        .env("HOSTNAME", "testhost")
        .env("PATH", path)
        .output()
        .expect("run codex-computer-use-x11");

    let _ = std::fs::remove_dir_all(&temp);

    assert!(output.status.success(), "status: {:?}", output.status);
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["windows"].as_array().unwrap().len(), 2);
    assert_eq!(report["windows"][0]["title"], "First Window");
    assert_eq!(report["windows"][1]["focused"], true);
    assert_eq!(report["diagnostics"]["focused_window"], 2);
    assert_eq!(report["diagnostics"]["enrichment"]["xprop_id_calls"], 0);
    assert!(report["windows"][0].get("raw_id").is_none());
}

#[test]
fn list_windows_cli_rejects_unsupported_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(["list-windows"])
        .output()
        .expect("run codex-computer-use-x11");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unsupported"));
}

#[cfg(unix)]
#[test]
fn excludes_project_owned_overlay_windows() {
    let temp = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-overlay-list-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp).expect("create temp dir");
    write_executable(
        &temp.join("wmctrl"),
        "#!/bin/sh\ncat <<'OUT'\n0x00000002 0 222 10 20 800 600 app.App testhost Editor\n0x00000009 0 333 8 18 804 604 codex-computer-use-x11-overlay.CodexOverlay testhost codex-computer-use-x11-overlay border\nOUT\n",
    );
    write_executable(
        &temp.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelse\n  echo 'unexpected xprop args' >&2\n  exit 1\nfi\n",
    );

    let path = format!(
        "{}:{}",
        temp.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let output = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(["list-windows", "--json"])
        .env("DISPLAY", ":99")
        .env("HOSTNAME", "testhost")
        .env("PATH", path)
        .output()
        .expect("run codex-computer-use-x11");

    let _ = std::fs::remove_dir_all(&temp);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["windows"].as_array().unwrap().len(), 1);
    assert_eq!(report["windows"][0]["window_id"], 2);
    let overlay_metadata = report["diagnostics"]["window_metadata"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["window_id"] == 9)
        .expect("overlay metadata");
    assert_eq!(overlay_metadata["owned_by_project"], true);
    assert_eq!(overlay_metadata["internal"], true);
}
