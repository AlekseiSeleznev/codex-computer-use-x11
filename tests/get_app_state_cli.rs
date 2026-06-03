use std::process::Command;

#[cfg(unix)]
fn temp_dir(name: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-{name}-{}-{nanos}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
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
fn path_with_fake_commands(dir: &std::path::Path) -> String {
    format!(
        "{}:{}",
        dir.display(),
        std::env::var("PATH").unwrap_or_default()
    )
}

#[cfg(unix)]
fn run_cli_with_path(args: &[&str], path: String) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(args)
        .env("DISPLAY", ":99")
        .env("HOSTNAME", "testhost")
        .env("PATH", path)
        .output()
        .expect("run codex-computer-use-x11")
}

#[cfg(unix)]
fn write_window_commands(dir: &std::path::Path, wmctrl_output: &str, active: &str) {
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n{wmctrl_output}\nOUT\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # {active}'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
}

#[cfg(unix)]
fn write_unavailable_atspi(dir: &std::path::Path) {
    write_executable(
        &dir.join("python3"),
        "#!/bin/sh\nprintf '{\"ok\": false, \"candidates\": [], \"diagnostics\": {\"detail\": \"fake AT-SPI unavailable\"}}\\n'\n",
    );
}

#[cfg(unix)]
fn write_matched_atspi(dir: &std::path::Path) {
    write_executable(
        &dir.join("python3"),
        r#"#!/bin/sh
cat <<'JSON'
{"ok": true, "candidates": [{"object_ref": "pid:1234:/app:0", "name": "Editor Window", "role": "application", "pid": 1234, "bounds": {"x": 10, "y": 20, "width": 800, "height": 600}, "focused": true, "states": ["active"], "nodes": [{"index": 0, "parent_index": null, "depth": 0, "object_ref": "pid:1234:/app:0", "role": "application", "name": "Editor Window", "description": null, "child_count": 1, "bounds": {"x": 10, "y": 20, "width": 800, "height": 600}, "states": ["active"], "actions": [], "supports_editable_text": false}, {"index": 1, "parent_index": 0, "depth": 1, "object_ref": "pid:1234:/button:1", "role": "push button", "name": "Save", "description": null, "child_count": 0, "bounds": {"x": 20, "y": 30, "width": 90, "height": 30}, "states": ["enabled"], "actions": [{"index": 0, "name": "click", "description": "", "keybinding": ""}], "supports_editable_text": false}]}], "diagnostics": {"detail": "fake AT-SPI matched", "truncated": false}}
JSON
"#,
    );
}

#[cfg(unix)]
fn write_ambiguous_atspi(dir: &std::path::Path) {
    write_executable(
        &dir.join("python3"),
        r#"#!/bin/sh
cat <<'JSON'
{"ok": true, "candidates": [{"object_ref": "pid:1234:/app:0", "name": "Editor Window", "role": "application", "pid": 1234, "bounds": {"x": 10, "y": 20, "width": 800, "height": 600}, "focused": true, "states": ["active"], "nodes": []}, {"object_ref": "pid:1234:/app:1", "name": "Editor Window", "role": "application", "pid": 1234, "bounds": {"x": 10, "y": 20, "width": 800, "height": 600}, "focused": true, "states": ["active"], "nodes": []}], "diagnostics": {"detail": "fake AT-SPI ambiguous", "truncated": false}}
JSON
"#,
    );
}

#[cfg(unix)]
fn write_tiny_png(path: &std::path::Path) {
    const PNG: &[u8] = &[
        0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1a, b'\n', 0x00, 0x00, 0x00, 0x0d, b'I', b'H',
        b'D', b'R', 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00,
        0x1f, 0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x0a, b'I', b'D', b'A', b'T', 0x78, 0x9c, 0x63,
        0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0d, 0x0a, 0x2d, 0xb4, 0x00, 0x00, 0x00, 0x00,
        b'I', b'E', b'N', b'D', 0xae, 0x42, 0x60, 0x82,
    ];
    std::fs::write(path, PNG).expect("write tiny png");
}

#[cfg(unix)]
fn write_fake_gdbus_screenshot(dir: &std::path::Path) {
    let fixture = dir.join("fixture.png");
    write_tiny_png(&fixture);
    write_executable(
        &dir.join("gdbus"),
        &format!(
            "#!/bin/sh\nargs=\"$*\"\nif echo \"$args\" | grep -q 'org.gnome.Shell.Screenshot.Screenshot'; then\n  for last do :; done\n  cp '{}' \"$last\"\n  printf \"(true, '%s')\\n\" \"$last\"\n  exit 0\nfi\nif echo \"$args\" | grep -q 'org.gnome.Shell.Screenshot'; then\n  echo 'method Screenshot'\n  echo 'method ScreenshotArea'\n  exit 0\nfi\nif echo \"$args\" | grep -q 'org.freedesktop.portal.Desktop'; then\n  echo 'method Screenshot'\n  exit 0\nfi\nif echo \"$args\" | grep -q 'org.a11y.Bus.GetAddress'; then\n  echo \"('unix:path=/tmp/fake-atspi',)\"\n  exit 0\nfi\nexit 0\n",
            fixture.display()
        ),
    );
}

#[cfg(unix)]
#[test]
fn resolves_window_context_by_window_id() {
    let dir = temp_dir("get-app-state-window-id");
    write_window_commands(
        &dir,
        "0x00000002 0 1234 10 20 800 600 app.App testhost Editor Window",
        "0x2",
    );
    write_unavailable_atspi(&dir);

    let output = run_cli_with_path(
        &[
            "get-app-state",
            "--window-id",
            "0x2",
            "--no-screenshot",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "status: {:?}\nstderr: {}\nstdout: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["backend"], "x11-ewmh");
    assert_eq!(report["window_context"]["window_id"], 2);
    assert_eq!(report["window_context"]["backend"], "x11-ewmh");
    assert_eq!(report["window_context"]["bounds"]["x"], 10);
    assert_eq!(report["window_error"], serde_json::Value::Null);
    assert_eq!(report["screenshot"], serde_json::Value::Null);
    assert_eq!(report["screenshot_error"], serde_json::Value::Null);
    assert!(report["message"]
        .as_str()
        .unwrap_or_default()
        .contains("Window target resolved"));
}

#[cfg(unix)]
#[test]
fn refuses_ambiguous_title_without_random_context() {
    let dir = temp_dir("get-app-state-ambiguous-title");
    write_window_commands(
        &dir,
        "0x00000001 0 1234 10 20 800 600 app.App testhost Editor Alpha\n0x00000002 0 1235 30 40 800 600 app.App testhost Editor Beta",
        "0x1",
    );
    write_unavailable_atspi(&dir);

    let output = run_cli_with_path(
        &[
            "get-app-state",
            "--title",
            "Editor",
            "--no-screenshot",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "layer-degraded app-state should still emit successful JSON"
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["window_context"], serde_json::Value::Null);
    assert!(report["window_error"]
        .as_str()
        .unwrap_or_default()
        .contains("AmbiguousTarget"));
    assert_eq!(
        report["diagnostics"]["target_candidates"]
            .as_array()
            .expect("target candidates")
            .len(),
        2
    );
}

#[cfg(unix)]
#[test]
fn keeps_screenshot_when_window_target_missing() {
    let dir = temp_dir("get-app-state-missing-target-screenshot");
    write_window_commands(
        &dir,
        "0x00000002 0 1234 10 20 800 600 app.App testhost Editor Window",
        "0x2",
    );
    write_unavailable_atspi(&dir);
    write_fake_gdbus_screenshot(&dir);

    let output = run_cli_with_path(
        &["get-app-state", "--window-id", "0x99", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["window_context"], serde_json::Value::Null);
    assert!(report["window_error"]
        .as_str()
        .unwrap_or_default()
        .contains("WindowNotFound"));
    assert_eq!(report["screenshot"]["mime_type"], "image/png");
    assert_eq!(report["screenshot"]["width"], 1);
    assert_eq!(report["screenshot"]["height"], 1);
    assert_eq!(report["screenshot"]["data_url"], serde_json::Value::Null);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("data:image"),
        "default JSON must not contain inline image data"
    );
    assert!(
        !stdout.contains(";base64,"),
        "default JSON must not contain base64 screenshot payloads"
    );
    let screenshot_path = std::path::PathBuf::from(
        report["screenshot"]["path"]
            .as_str()
            .expect("screenshot path"),
    );
    assert!(
        screenshot_path.is_file(),
        "reported screenshot path should exist: {screenshot_path:?}"
    );
    assert!(std::fs::metadata(&screenshot_path).unwrap().len() > 0);
    assert_eq!(report["screenshot_error"], serde_json::Value::Null);
    let _ = std::fs::remove_file(screenshot_path);
}

#[cfg(unix)]
#[test]
fn no_screenshot_and_provider_failure_are_layer_degraded() {
    let dir = temp_dir("get-app-state-screenshot-flags");
    write_window_commands(
        &dir,
        "0x00000002 0 1234 10 20 800 600 app.App testhost Editor Window",
        "0x2",
    );
    write_unavailable_atspi(&dir);

    let no_screenshot = run_cli_with_path(
        &["get-app-state", "--no-screenshot", "--json"],
        path_with_fake_commands(&dir),
    );
    assert!(no_screenshot.status.success());
    let no_screenshot_report: serde_json::Value =
        serde_json::from_slice(&no_screenshot.stdout).expect("valid json");
    assert_eq!(no_screenshot_report["screenshot"], serde_json::Value::Null);
    assert_eq!(
        no_screenshot_report["screenshot_error"],
        serde_json::Value::Null
    );

    write_executable(
        &dir.join("gdbus"),
        "#!/bin/sh\necho 'screenshot failed' >&2\nexit 42\n",
    );
    let failed = run_cli_with_path(&["get-app-state", "--json"], path_with_fake_commands(&dir));
    let _ = std::fs::remove_dir_all(&dir);

    assert!(failed.status.success());
    let failed_report: serde_json::Value = serde_json::from_slice(&failed.stdout).expect("json");
    assert_eq!(failed_report["screenshot"], serde_json::Value::Null);
    assert!(failed_report["screenshot_error"]
        .as_str()
        .unwrap_or_default()
        .contains("screenshot failed"));
}

#[cfg(unix)]
#[test]
fn screenshot_output_path_is_caller_controlled_and_invalid_path_degrades_layer() {
    let dir = temp_dir("get-app-state-screenshot-output-path");
    write_window_commands(
        &dir,
        "0x00000002 0 1234 10 20 800 600 app.App testhost Editor Window",
        "0x2",
    );
    write_unavailable_atspi(&dir);
    write_fake_gdbus_screenshot(&dir);
    let requested = dir.join("app-state.png");

    let output = run_cli_with_path(
        &[
            "get-app-state",
            "--window-id",
            "0x2",
            "--screenshot-output",
            requested.to_str().unwrap(),
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert_eq!(report["screenshot"]["path"], requested.to_str().unwrap());
    assert!(requested.is_file());
    assert_eq!(report["screenshot"]["data_url"], serde_json::Value::Null);
    assert!(!String::from_utf8_lossy(&output.stdout).contains("data:image"));

    let invalid = dir.join("missing-parent/app-state.png");
    let degraded = run_cli_with_path(
        &[
            "get-app-state",
            "--window-id",
            "0x2",
            "--screenshot-output",
            invalid.to_str().unwrap(),
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);
    assert!(degraded.status.success());
    let degraded_report: serde_json::Value =
        serde_json::from_slice(&degraded.stdout).expect("json");
    assert_eq!(degraded_report["window_context"]["window_id"], 2);
    assert_eq!(degraded_report["screenshot"], serde_json::Value::Null);
    assert!(degraded_report["screenshot_error"]
        .as_str()
        .unwrap_or_default()
        .contains("screenshot output parent"));
}

#[cfg(unix)]
#[test]
fn inline_screenshot_requires_explicit_opt_in() {
    let dir = temp_dir("get-app-state-inline-opt-in");
    write_window_commands(
        &dir,
        "0x00000002 0 1234 10 20 800 600 app.App testhost Editor Window",
        "0x2",
    );
    write_unavailable_atspi(&dir);
    write_fake_gdbus_screenshot(&dir);

    let output = run_cli_with_path(
        &[
            "get-app-state",
            "--window-id",
            "0x2",
            "--inline-screenshot",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert!(report["screenshot"]["data_url"]
        .as_str()
        .unwrap_or_default()
        .starts_with("data:image/png;base64,"));
    let screenshot_path = std::path::PathBuf::from(
        report["screenshot"]["path"]
            .as_str()
            .expect("screenshot path"),
    );
    let _ = std::fs::remove_file(screenshot_path);
}

#[cfg(unix)]
#[test]
fn includes_matched_accessibility_tree() {
    let dir = temp_dir("get-app-state-atspi-match");
    write_window_commands(
        &dir,
        "0x00000002 0 1234 10 20 800 600 app.App testhost Editor Window",
        "0x2",
    );
    write_matched_atspi(&dir);

    let output = run_cli_with_path(
        &[
            "get-app-state",
            "--window-id",
            "0x2",
            "--no-screenshot",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert_eq!(report["window_context"]["window_id"], 2);
    assert_eq!(report["accessibility_error"], serde_json::Value::Null);
    assert_eq!(
        report["accessibility_tree"].as_array().expect("tree").len(),
        2
    );
    assert_eq!(
        report["diagnostics"]["accessibility_correlation"]["status"],
        "matched"
    );
}

#[cfg(unix)]
#[test]
fn keeps_context_when_accessibility_is_ambiguous() {
    let dir = temp_dir("get-app-state-atspi-ambiguous");
    write_window_commands(
        &dir,
        "0x00000002 0 1234 10 20 800 600 app.App testhost Editor Window",
        "0x2",
    );
    write_ambiguous_atspi(&dir);
    write_fake_gdbus_screenshot(&dir);

    let output = run_cli_with_path(
        &["get-app-state", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert_eq!(report["window_context"]["window_id"], 2);
    assert_eq!(
        report["accessibility_tree"].as_array().expect("tree").len(),
        0
    );
    assert!(report["accessibility_error"]
        .as_str()
        .unwrap_or_default()
        .contains("AmbiguousAccessibilityMatch"));
    assert_eq!(report["screenshot"]["mime_type"], "image/png");
    assert_eq!(report["screenshot"]["data_url"], serde_json::Value::Null);
    assert!(!String::from_utf8_lossy(&output.stdout).contains("data:image"));
    let screenshot_path = std::path::PathBuf::from(
        report["screenshot"]["path"]
            .as_str()
            .expect("screenshot path"),
    );
    assert!(screenshot_path.is_file());
    let _ = std::fs::remove_file(screenshot_path);
}
