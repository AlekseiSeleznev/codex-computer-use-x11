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
fn write_basic_wmctrl(dir: &std::path::Path) {
    write_executable(
        &dir.join("wmctrl"),
        "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 1024 768 app.App testhost Second Window\nOUT\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n",
    );
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
#[test]
fn focused_window_cli_reports_matched_active_window() {
    let dir = temp_dir("focused-window-match");
    write_basic_wmctrl(&dir);
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );

    let output = run_cli_with_path(&["focused-window", "--json"], path_with_fake_commands(&dir));
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["backend"], "x11-ewmh");
    assert_eq!(report["focused_window"]["window_id"], 2);
    assert_eq!(report["focused_window"]["focused"], true);
    assert_eq!(report["diagnostics"]["active_window"], 2);
    assert_eq!(
        report["diagnostics"]["activation_attempts"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
}

#[cfg(unix)]
#[test]
fn focused_window_cli_degrades_when_no_active_window() {
    let dir = temp_dir("focused-window-no-active");
    write_basic_wmctrl(&dir);
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x0'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );

    let output = run_cli_with_path(&["focused-window", "--json"], path_with_fake_commands(&dir));
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert!(report["focused_window"].is_null());
    assert!(report["diagnostics"]["active_window"].is_null());
    assert!(report["diagnostics"]["degraded_reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item
            .as_str()
            .unwrap_or_default()
            .contains("no active X11 window")));
}

#[cfg(unix)]
#[test]
fn focused_window_cli_degrades_when_active_window_is_not_listed() {
    let dir = temp_dir("focused-window-unmatched");
    write_basic_wmctrl(&dir);
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x3'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );

    let output = run_cli_with_path(&["focused-window", "--json"], path_with_fake_commands(&dir));
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert!(report["focused_window"].is_null());
    assert_eq!(report["diagnostics"]["active_window"], 3);
    assert!(report["diagnostics"]["degraded_reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item
            .as_str()
            .unwrap_or_default()
            .contains("could not be matched")));
}

#[cfg(unix)]
fn write_focus_resolution_commands(dir: &std::path::Path, activation_log: &std::path::Path) {
    let log = activation_log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 1024 768 app.App testhost Second Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x1'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );
    write_executable(
        &dir.join("xdotool"),
        &format!("#!/bin/sh\necho \"xdotool $*\" >> '{log}'\nexit 0\n"),
    );
}

#[cfg(unix)]
#[test]
fn focus_window_cli_rejects_invalid_id_before_activation() {
    let dir = temp_dir("focus-invalid-id");
    let log = dir.join("activation.log");
    write_focus_resolution_commands(&dir, &log);

    let output = run_cli_with_path(
        &["focus-window", "--window-id", "not-a-window", "--json"],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "status should fail for invalid id"
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid X11 window id"));
    assert_eq!(log_contents, "", "activation should not be attempted");
}

#[cfg(unix)]
#[test]
fn focus_window_cli_reports_window_not_found_without_activation() {
    let dir = temp_dir("focus-window-not-found");
    let log = dir.join("activation.log");
    write_focus_resolution_commands(&dir, &log);

    let output = run_cli_with_path(
        &["focus-window", "--window-id", "0x99", "--json"],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "status should fail for missing window"
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert!(report["requested_window"].is_null());
    assert_eq!(report["error_code"], "WindowNotFound");
    assert!(report["note"]
        .as_str()
        .unwrap_or_default()
        .contains("No window matched"));
    assert_eq!(log_contents, "", "activation should not be attempted");
}

#[cfg(unix)]
fn write_stateful_focus_commands(
    dir: &std::path::Path,
    initial_active: &str,
) -> std::path::PathBuf {
    let state = dir.join("active-window.txt");
    let log = dir.join("activation.log");
    std::fs::write(&state, initial_active).expect("write active state");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x0000000a 0 110 30 40 1024 768 app.App testhost Tenth Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  echo \"$2\" > '{state_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  id=$(cat '{state_path}')\n  echo \"_NET_ACTIVE_WINDOW(WINDOW): window id # $id\"\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xdotool"),
        &format!("#!/bin/sh\necho \"xdotool $*\" >> '{log_path}'\nexit 0\n"),
    );
    log
}

#[cfg(unix)]
#[test]
fn focus_window_cli_verifies_wmctrl_activation() {
    let dir = temp_dir("focus-wmctrl-success");
    let log = write_stateful_focus_commands(&dir, "0x1");

    let output = run_cli_with_path(
        &["focus-window", "--window-id", "10", "--json"],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["exact_window_focused"], true);
    assert_eq!(report["requested_window"]["window_id"], 10);
    assert_eq!(report["focused_window"]["window_id"], 10);
    assert_eq!(report["focused_window"]["focused"], true);
    assert!(report["error_code"].is_null());
    assert_eq!(
        report["diagnostics"]["activation_attempts"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        report["diagnostics"]["activation_attempts"][0]["command"],
        "wmctrl"
    );
    assert!(log_contents.contains("wmctrl -ia 0xa"));
    assert!(!log_contents.contains("xdotool"));
}

#[cfg(unix)]
fn write_wmctrl_no_update_commands(
    dir: &std::path::Path,
    initial_active: &str,
) -> std::path::PathBuf {
    let state = dir.join("active-window.txt");
    let log = dir.join("activation.log");
    std::fs::write(&state, initial_active).expect("write active state");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x0000000a 0 110 30 40 1024 768 app.App testhost Tenth Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  id=$(cat '{state_path}')\n  echo \"_NET_ACTIVE_WINDOW(WINDOW): window id # $id\"\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xdotool"),
        &format!("#!/bin/sh\necho \"xdotool $*\" >> '{log_path}'\nexit 0\n"),
    );
    log
}

#[cfg(unix)]
#[test]
fn focus_window_cli_reports_focus_not_verified_on_mismatch() {
    let dir = temp_dir("focus-not-verified");
    let log = write_wmctrl_no_update_commands(&dir, "0x1");

    let output = run_cli_with_path(
        &["focus-window", "--window-id", "10", "--json"],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "status should fail when focus is unverified"
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["exact_window_focused"], false);
    assert_eq!(report["error_code"], "FocusNotVerified");
    assert_eq!(report["focused_window"]["window_id"], 1);
    assert!(report["diagnostics"]["degraded_reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item
            .as_str()
            .unwrap_or_default()
            .contains("active window 1 did not match requested window 10")));
    assert!(log_contents.contains("wmctrl -ia 0xa"));
}

#[cfg(unix)]
fn write_wmctrl_fail_xdotool_success_commands(
    dir: &std::path::Path,
    initial_active: &str,
) -> std::path::PathBuf {
    let state = dir.join("active-window.txt");
    let log = dir.join("activation.log");
    std::fs::write(&state, initial_active).expect("write active state");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x0000000a 0 110 30 40 1024 768 app.App testhost Tenth Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  echo 'wmctrl refused activation' >&2\n  exit 7\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  id=$(cat '{state_path}')\n  echo \"_NET_ACTIVE_WINDOW(WINDOW): window id # $id\"\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xdotool"),
        &format!(
            "#!/bin/sh\necho \"xdotool $*\" >> '{log_path}'\nif [ \"$1\" = \"windowactivate\" ]; then\n  echo \"0x$(printf '%x' \"$3\")\" > '{state_path}'\n  exit 0\nfi\nexit 2\n"
        ),
    );
    log
}

#[cfg(unix)]
#[test]
fn focus_window_cli_falls_back_to_xdotool() {
    let dir = temp_dir("focus-xdotool-fallback");
    let log = write_wmctrl_fail_xdotool_success_commands(&dir, "0x1");

    let output = run_cli_with_path(
        &["focus-window", "--window-id", "10", "--json"],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["focused_window"]["window_id"], 10);
    assert_eq!(
        report["diagnostics"]["activation_attempts"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        report["diagnostics"]["activation_attempts"][0]["command"],
        "wmctrl"
    );
    assert_eq!(report["diagnostics"]["activation_attempts"][0]["ok"], false);
    assert_eq!(
        report["diagnostics"]["activation_attempts"][1]["command"],
        "xdotool"
    );
    assert_eq!(
        report["diagnostics"]["activation_attempts"][1]["args"][0],
        "windowactivate"
    );
    assert_eq!(
        report["diagnostics"]["activation_attempts"][1]["args"][1],
        "--sync"
    );
    assert_eq!(
        report["diagnostics"]["activation_attempts"][1]["args"][2],
        "10"
    );
    assert!(log_contents.contains("wmctrl -ia 0xa"));
    assert!(log_contents.contains("xdotool windowactivate --sync 10"));
}
