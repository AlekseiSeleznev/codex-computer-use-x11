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
fn run_cli_with_exact_path(args: &[&str], path: String) -> std::process::Output {
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
fn targeted_input_refuses_ambiguous_title_without_commands() {
    let dir = temp_dir("targeted-input-ambiguous-title");
    let log = dir.join("commands.log");
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost Editor Alpha\n0x00000002 0 112 30 40 800 600 app.App testhost Editor Beta\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
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
            "type-text",
            "--title",
            "Editor",
            "--text",
            "hello",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "ambiguous target should fail");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "AmbiguousTarget");
    assert_eq!(
        report["diagnostics"]["candidates"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(log_contents, "", "activation/input should not run");
}

#[cfg(unix)]
#[test]
fn targeted_type_text_does_not_invoke_xdotool_when_focus_unverified() {
    let dir = temp_dir("targeted-type-focus-mismatch");
    let log = dir.join("commands.log");
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 800 600 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
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
            "type-text",
            "--window-id",
            "0x2",
            "--text",
            "do-not-send",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "focus mismatch should block targeted input"
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "FocusNotVerified");
    assert_eq!(report["focus"]["exact_window_focused"], false);
    assert!(log_contents.contains("wmctrl -ia 0x2"));
    assert!(
        !log_contents.contains("type --clearmodifiers"),
        "xdotool type must not run when focus is unverified: {log_contents}"
    );
}

#[cfg(unix)]
#[test]
fn targeted_type_text_invokes_active_context_xdotool_after_verified_focus() {
    let dir = temp_dir("targeted-type-verified-focus");
    let state = dir.join("active-window.txt");
    std::fs::write(&state, "0x1").expect("write active state");
    let log = dir.join("commands.log");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 800 600 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  echo \"$2\" > '{state_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
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

    let output = run_cli_with_path(
        &[
            "type-text",
            "--window-id",
            "0x2",
            "--text",
            "hello",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status should succeed");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["input_sent"], true);
    assert_eq!(report["error_code"], serde_json::Value::Null);
    assert_eq!(report["target"]["window_id"], 2);
    assert_eq!(report["focus"]["exact_window_focused"], true);
    assert_eq!(report["keyboard"]["command"], "xdotool");
    assert_eq!(report["keyboard"]["active_context"], true);
    assert_eq!(report["keyboard"]["used_direct_window"], false);
    assert!(log_contents.contains("wmctrl -ia 0x2"));
    assert!(log_contents.contains("xdotool type --clearmodifiers hello"));
    assert!(
        !log_contents.contains("--window"),
        "xdotool direct-window mode must not be used: {log_contents}"
    );
}

#[cfg(unix)]
#[test]
fn targeted_press_key_invokes_active_context_xdotool_after_verified_focus() {
    let dir = temp_dir("targeted-key-verified-focus");
    let state = dir.join("active-window.txt");
    std::fs::write(&state, "0x1").expect("write active state");
    let log = dir.join("commands.log");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 800 600 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  echo \"$2\" > '{state_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
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

    let output = run_cli_with_path(
        &[
            "press-key",
            "--window-id",
            "0x2",
            "--key",
            "Enter",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status should succeed");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["input_sent"], true);
    assert_eq!(report["keyboard"]["command"], "xdotool");
    assert!(log_contents.contains("wmctrl -ia 0x2"));
    assert!(log_contents.contains("xdotool key --clearmodifiers Return"));
    assert_eq!(report["keyboard"]["requested_key"], "Enter");
    assert_eq!(report["keyboard"]["normalized_key"], "Return");
    assert!(
        !log_contents.contains("--window"),
        "xdotool direct-window mode must not be used: {log_contents}"
    );
}

#[cfg(unix)]
#[test]
fn targeted_press_key_normalizes_backspace_alias_after_verified_focus() {
    let dir = temp_dir("targeted-key-backspace-alias");
    let state = dir.join("active-window.txt");
    std::fs::write(&state, "0x1").expect("write active state");
    let log = dir.join("commands.log");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 800 600 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  echo \"$2\" > '{state_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
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

    let output = run_cli_with_path(
        &[
            "press-key",
            "--window-id",
            "0x2",
            "--key",
            "Backspace",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status should succeed");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["keyboard"]["requested_key"], "Backspace");
    assert_eq!(report["keyboard"]["normalized_key"], "BackSpace");
    assert!(log_contents.contains("xdotool key --clearmodifiers BackSpace"));
    assert!(!log_contents.contains("--window"));
}

#[cfg(unix)]
#[test]
fn targeted_input_treats_xdotool_semantic_stderr_as_failure() {
    let dir = temp_dir("targeted-key-semantic-stderr");
    let state = dir.join("active-window.txt");
    std::fs::write(&state, "0x1").expect("write active state");
    let log = dir.join("commands.log");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000002 0 112 30 40 800 600 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  echo \"$2\" > '{state_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
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
        &format!("#!/bin/sh\necho \"xdotool $*\" >> '{log_path}'\necho 'No such key name: NotAKey. Ignoring it.' >&2\nexit 0\n"),
    );

    let output = run_cli_with_path(
        &[
            "press-key",
            "--window-id",
            "0x2",
            "--key",
            "NotAKey",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "semantic stderr should fail even with exit 0"
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "InputBackendFailed");
    assert_eq!(report["keyboard"]["semantic_stderr_error"], true);
    assert!(report["keyboard"]["detail"]
        .as_str()
        .unwrap_or_default()
        .contains("No such key name"));
}

#[cfg(unix)]
#[test]
fn targeted_type_text_uses_unicode_keysyms_after_verified_focus() {
    let dir = temp_dir("targeted-type-unicode-keysyms");
    let state = dir.join("active-window.txt");
    std::fs::write(&state, "0x1").expect("write active state");
    let log = dir.join("commands.log");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 800 600 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  echo \"$2\" > '{state_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
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

    let sample = "Привет";
    let output = run_cli_with_path(
        &[
            "type-text",
            "--window-id",
            "0x2",
            "--text",
            sample,
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status should succeed");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["keyboard"]["route"], "xdotool-unicode-keysyms");
    assert!(
        log_contents.contains("xdotool key --clearmodifiers U041F U0440 U0438 U0432 U0435 U0442"),
        "log={log_contents}"
    );
    assert!(!log_contents.contains("--window"));
}

#[cfg(unix)]
#[test]
fn targeted_type_text_uses_clipboard_fallback_when_unicode_keysyms_fail() {
    let dir = temp_dir("targeted-type-clipboard-fallback");
    let state = dir.join("active-window.txt");
    std::fs::write(&state, "0x1").expect("write active state");
    let clipboard = dir.join("clipboard.txt");
    std::fs::write(&clipboard, "previous clipboard").expect("write clipboard");
    let log = dir.join("commands.log");
    let state_path = state.display();
    let log_path = log.display();
    let clipboard_path = clipboard.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            r#"#!/bin/sh
if [ "$1" = "-lpGx" ]; then
cat <<'OUT'
0x00000001 0 111 10 20 800 600 app.App testhost First Window
0x00000002 0 112 30 40 800 600 app.App testhost Target Window
OUT
elif [ "$1" = "-ia" ]; then
  echo "wmctrl $*" >> '{log_path}'
  echo "$2" > '{state_path}'
  exit 0
else
  echo "unexpected wmctrl args: $*" >&2
  exit 2
fi
"#
        ),
    );
    write_executable(
        &dir.join("xprop"),
        &format!(
            r#"#!/bin/sh
if [ "$1" = "-root" ]; then
  id=$(cat '{state_path}')
  echo "_NET_ACTIVE_WINDOW(WINDOW): window id # $id"
else
  echo "unexpected xprop args: $*" >&2
  exit 2
fi
"#
        ),
    );
    write_executable(
        &dir.join("xdotool"),
        &format!(
            r#"#!/bin/sh
echo "xdotool $*" >> '{log_path}'
case "$*" in
  *U041F*) echo 'No such key name: U041F. Ignoring it.' >&2; exit 0;;
  *ctrl+v*) exit 0;;
  *) exit 0;;
esac
"#
        ),
    );
    write_executable(
        &dir.join("xclip"),
        &format!(
            r#"#!/bin/sh
echo "xclip $*" >> '{log_path}'
if [ "$*" = "-selection clipboard -o" ]; then cat '{clipboard_path}'; exit 0; fi
cat > '{clipboard_path}'
exit 0
"#
        ),
    );

    let output = run_cli_with_path(
        &[
            "type-text",
            "--window-id",
            "0x2",
            "--text",
            "Привет",
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let clipboard_after = std::fs::read_to_string(&clipboard).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "fallback should succeed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["keyboard"]["route"], "clipboard-paste");
    assert_eq!(report["keyboard"]["semantic_stderr_error"], true);
    assert!(report["keyboard"]["detail"]
        .as_str()
        .unwrap_or_default()
        .contains("clipboard restored"));
    assert!(log_contents.contains("xdotool key --clearmodifiers U041F"));
    assert!(log_contents.contains("xdotool key --clearmodifiers ctrl+v"));
    assert_eq!(clipboard_after, "previous clipboard");
}

#[cfg(unix)]
#[test]
fn targeted_type_text_reports_missing_keyboard_backend_after_verified_focus() {
    let dir = temp_dir("targeted-type-no-xdotool");
    let state = dir.join("active-window.txt");
    std::fs::write(&state, "0x1").expect("write active state");
    let log = dir.join("commands.log");
    let state_path = state.display();
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\n/bin/cat <<'OUT'\n0x00000001 0 111 10 20 800 600 app.App testhost First Window\n0x00000002 0 112 30 40 800 600 app.App testhost Target Window\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  echo \"wmctrl $*\" >> '{log_path}'\n  echo \"$2\" > '{state_path}'\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  id=$(/bin/cat '{state_path}')\n  echo \"_NET_ACTIVE_WINDOW(WINDOW): window id # $id\"\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    std::os::unix::fs::symlink("/bin/sh", dir.join("sh")).expect("link sh");

    let output = run_cli_with_exact_path(
        &[
            "type-text",
            "--window-id",
            "0x2",
            "--text",
            "hello",
            "--json",
        ],
        dir.display().to_string(),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "missing backend should fail");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "InputBackendUnavailable");
    assert_eq!(report["focus"]["exact_window_focused"], true);
    assert!(log_contents.contains("wmctrl -ia 0x2"));
    assert!(!log_contents.contains("xdotool"));
}
