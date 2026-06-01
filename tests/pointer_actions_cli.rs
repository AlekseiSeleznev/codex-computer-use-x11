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
fn write_basic_window_commands(dir: &std::path::Path, log: &std::path::Path) {
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000002 0 222 10 20 100 100 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );
    write_executable(
        &dir.join("xdotool"),
        &format!("#!/bin/sh\necho \"xdotool $*\" >> '{log_path}'\nexit 0\n"),
    );
}

#[cfg(unix)]
#[test]
fn pointer_click_refuses_out_of_bounds_before_focus() {
    let dir = temp_dir("pointer-click-out-of-bounds");
    let log = dir.join("commands.log");
    write_basic_window_commands(&dir, &log);

    let output = run_cli_with_path(
        &[
            "click",
            "--window-id",
            "0x2",
            "--x",
            "999",
            "--y",
            "50",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "out-of-bounds click should fail");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["targeted"], true);
    assert_eq!(report["verification_mode"], "targeted_focus_required");
    assert_eq!(report["error_code"], "PointOutsideTargetBounds");
    assert_eq!(log_contents, "", "focus/input should not run");
}

#[cfg(unix)]
#[test]
fn pointer_click_invokes_xdotool_after_verified_focus() {
    let dir = temp_dir("pointer-click-verified-focus");
    let log = dir.join("commands.log");
    write_basic_window_commands(&dir, &log);

    let output = run_cli_with_path(
        &[
            "click",
            "--window-id",
            "0x2",
            "--x",
            "50",
            "--y",
            "60",
            "--button",
            "left",
            "--count",
            "2",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "verified click should succeed");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["input_sent"], true);
    assert_eq!(report["targeted"], true);
    assert_eq!(report["verification_mode"], "targeted_focus_verified");
    assert_eq!(report["target"]["window_id"], 2);
    assert_eq!(report["focus"]["exact_window_focused"], true);
    assert_eq!(report["pointer"]["command"], "xdotool");
    assert_eq!(report["pointer"]["active_context"], true);
    assert_eq!(report["pointer"]["used_direct_window"], false);
    assert!(log_contents.contains("wmctrl -ia 0x2"));
    assert!(log_contents.contains("xdotool mousemove --sync 50 60 click --repeat 2 1"));
    assert!(
        !log_contents.contains("--window"),
        "xdotool direct-window mode must not be used: {log_contents}"
    );
}

#[cfg(unix)]
#[test]
fn pointer_click_does_not_invoke_xdotool_when_focus_unverified() {
    let dir = temp_dir("pointer-click-focus-mismatch");
    let log = dir.join("commands.log");
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 0 0 100 100 app.App testhost Other Window\n0x00000002 0 222 10 20 100 100 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x1'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );
    write_executable(
        &dir.join("xdotool"),
        &format!("#!/bin/sh\necho \"xdotool $*\" >> '{log_path}'\nexit 0\n"),
    );

    let output = run_cli_with_path(
        &[
            "click",
            "--window-id",
            "0x2",
            "--x",
            "50",
            "--y",
            "60",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "focus mismatch should fail");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "FocusNotVerified");
    assert_eq!(report["focus"]["exact_window_focused"], false);
    assert!(log_contents.contains("wmctrl -ia 0x2"));
    assert!(
        !log_contents.contains("xdotool mousemove"),
        "pointer xdotool command must not run when focus is unverified: {log_contents}"
    );
}

#[cfg(unix)]
#[test]
fn pointer_scroll_maps_down_to_wheel_button_and_clamps_amount() {
    let dir = temp_dir("pointer-scroll-down");
    let log = dir.join("commands.log");
    write_basic_window_commands(&dir, &log);

    let output = run_cli_with_path(
        &[
            "scroll",
            "--window-id",
            "0x2",
            "--x",
            "50",
            "--y",
            "60",
            "--direction",
            "down",
            "--amount",
            "99",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "verified scroll should succeed");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["input_sent"], true);
    assert_eq!(report["action"], "scroll");
    assert_eq!(report["pointer"]["args"][7], "5");
    assert!(log_contents.contains("wmctrl -ia 0x2"));
    assert!(log_contents.contains("xdotool mousemove --sync 50 60 click --repeat 20 5"));
}

#[cfg(unix)]
#[test]
fn pointer_drag_refuses_huge_distance_without_xdotool() {
    let dir = temp_dir("pointer-drag-huge");
    let log = dir.join("commands.log");
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000002 0 222 0 0 10000 10000 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );
    write_executable(
        &dir.join("xdotool"),
        &format!("#!/bin/sh\necho \"xdotool $*\" >> '{log_path}'\nexit 0\n"),
    );

    let output = run_cli_with_path(
        &[
            "drag",
            "--window-id",
            "0x2",
            "--start-x",
            "0",
            "--start-y",
            "0",
            "--end-x",
            "5000",
            "--end-y",
            "0",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "huge drag should fail");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "DragDistanceTooLarge");
    assert_eq!(log_contents, "", "focus/input should not run");
}

#[cfg(unix)]
#[test]
fn pointer_drag_invokes_finite_down_move_up_after_verified_focus() {
    let dir = temp_dir("pointer-drag-verified-focus");
    let log = dir.join("commands.log");
    write_basic_window_commands(&dir, &log);

    let output = run_cli_with_path(
        &[
            "drag",
            "--window-id",
            "0x2",
            "--start-x",
            "50",
            "--start-y",
            "60",
            "--end-x",
            "70",
            "--end-y",
            "80",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "small verified drag should succeed"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["input_sent"], true);
    assert_eq!(report["action"], "drag");
    assert!(log_contents.contains("wmctrl -ia 0x2"));
    assert!(log_contents
        .contains("xdotool mousemove --sync 50 60 mousedown 1 mousemove --sync 70 80 mouseup 1"));
}

#[cfg(unix)]
#[test]
fn pointer_global_click_is_explicitly_unverified() {
    let dir = temp_dir("pointer-global-click");
    let log = dir.join("commands.log");
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        "#!/bin/sh\necho 'wmctrl should not matter for global mode' >&2\nexit 1\n",
    );
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\necho 'xprop should not matter for global mode' >&2\nexit 1\n",
    );
    write_executable(
        &dir.join("xdotool"),
        &format!("#!/bin/sh\necho \"xdotool $*\" >> '{log_path}'\nexit 0\n"),
    );

    let missing = run_cli_with_path(
        &["click", "--x", "50", "--y", "60", "--json"],
        path_with_fake_commands(&dir),
    );
    assert!(!missing.status.success(), "missing target should fail");
    let missing_report: serde_json::Value =
        serde_json::from_slice(&missing.stdout).expect("missing target json");
    assert_eq!(missing_report["error_code"], "MissingTarget");
    assert_eq!(missing_report["input_sent"], false);

    let output = run_cli_with_path(
        &[
            "click", "--global", "--x", "50", "--y", "60", "--count", "1", "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "explicit global click should succeed"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["input_sent"], true);
    assert_eq!(report["targeted"], false);
    assert_eq!(report["verification_mode"], "global_unverified");
    assert_eq!(report["focus"], serde_json::Value::Null);
    assert!(report["diagnostics"]["degraded_reasons"][0]
        .as_str()
        .unwrap()
        .contains("not window-isolated"));
    assert!(log_contents.contains("xdotool mousemove --sync 50 60 click --repeat 1 1"));
}
