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
fn write_window_commands(dir: &std::path::Path, rows: &str) {
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n{rows}OUT\nelif [ \"$1\" = \"-ia\" ]; then\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );
}

#[cfg(unix)]
fn run_cli(dir: &std::path::Path, state: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(args)
        .env("DISPLAY", ":99")
        .env("HOSTNAME", "testhost")
        .env("PATH", path_with_fake_commands(dir))
        .env("CODEX_X11_TARGET_STATE", state)
        .output()
        .expect("run codex-computer-use-x11")
}

#[cfg(unix)]
#[test]
fn saves_and_releases_target_window() {
    let dir = temp_dir("target-window-save-release");
    let state = dir.join("target-state.json");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Editor\n",
    );

    let target = run_cli(
        &dir,
        &state,
        &["target-window", "--window-id", "0x2", "--json"],
    );
    assert!(
        target.status.success(),
        "target status: {:?}\nstderr={}\nstdout={}",
        target.status,
        String::from_utf8_lossy(&target.stderr),
        String::from_utf8_lossy(&target.stdout)
    );
    assert_eq!(String::from_utf8_lossy(&target.stderr), "");
    let target_report: serde_json::Value =
        serde_json::from_slice(&target.stdout).expect("target-window json");
    assert_eq!(target_report["success"], true);
    assert_eq!(target_report["target"]["window"]["window_id"], 2);
    assert_eq!(target_report["state"]["active_group_id"], "default");
    assert_eq!(
        target_report["state"]["groups"][0]["windows"][0]["window"]["title"],
        "Editor"
    );
    assert_eq!(target_report["overlay"]["requested"], false);

    let context = run_cli(&dir, &state, &["target-context", "--json"]);
    assert!(
        context.status.success(),
        "context status: {:?}",
        context.status
    );
    let context_report: serde_json::Value =
        serde_json::from_slice(&context.stdout).expect("target-context json");
    assert_eq!(context_report["success"], true);
    assert_eq!(
        context_report["state"]["groups"][0]["windows"][0]["stale"],
        false
    );

    let release = run_cli(
        &dir,
        &state,
        &["release-window", "--window-id", "0x2", "--json"],
    );
    assert!(
        release.status.success(),
        "release status: {:?}",
        release.status
    );
    let release_report: serde_json::Value =
        serde_json::from_slice(&release.stdout).expect("release-window json");
    assert_eq!(release_report["success"], true);
    assert_eq!(release_report["released_count"], 1);
    assert_eq!(
        release_report["state"]["groups"][0]["windows"]
            .as_array()
            .unwrap()
            .len(),
        0
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn refuses_ambiguous_title_without_saving_state() {
    let dir = temp_dir("target-window-ambiguous-title");
    let state = dir.join("target-state.json");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Editor Alpha\n0x00000003 0 223 30 40 800 600 app.App testhost Editor Beta\n",
    );

    let output = run_cli(
        &dir,
        &state,
        &["target-window", "--title", "Editor", "--json"],
    );
    assert!(!output.status.success(), "ambiguous target should fail");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["error_code"], "AmbiguousTarget");
    assert_eq!(
        report["diagnostics"]["candidates"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(
        report["state"]["groups"].as_array().unwrap().is_empty(),
        "ambiguous target should not create groups"
    );

    let context = run_cli(&dir, &state, &["target-context", "--json"]);
    let context_report: serde_json::Value =
        serde_json::from_slice(&context.stdout).expect("context json");
    assert!(context_report["state"]["groups"]
        .as_array()
        .unwrap()
        .is_empty());

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn marks_vanished_target_stale() {
    let dir = temp_dir("target-window-stale");
    let state = dir.join("target-state.json");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Editor\n",
    );
    let target = run_cli(
        &dir,
        &state,
        &["target-window", "--window-id", "0x2", "--json"],
    );
    assert!(target.status.success(), "initial target should save");

    write_window_commands(
        &dir,
        "0x00000003 0 223 30 40 800 600 app.App testhost Other\n",
    );
    let context = run_cli(&dir, &state, &["target-context", "--json"]);
    assert!(context.status.success(), "context should still emit json");
    let report: serde_json::Value = serde_json::from_slice(&context.stdout).expect("context json");
    assert_eq!(report["success"], true);
    assert_eq!(report["diagnostics"]["stale_removed"][0]["window_id"], 2);
    assert!(report["state"]["groups"][0]["windows"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        report["state"]["groups"][0]["active_window_id"],
        serde_json::Value::Null
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn groups_are_idempotent_and_move_existing_window() {
    let dir = temp_dir("target-window-groups");
    let state = dir.join("target-state.json");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Editor\n0x00000003 0 223 30 40 900 700 mail.Mail testhost Email\n",
    );

    let first = run_cli(
        &dir,
        &state,
        &[
            "target-window",
            "--window-id",
            "0x2",
            "--group",
            "data-entry",
            "--color",
            "green",
            "--json",
        ],
    );
    assert!(first.status.success(), "first target should save");
    let second = run_cli(
        &dir,
        &state,
        &[
            "target-window",
            "--window-id",
            "0x2",
            "--group",
            "data-entry",
            "--color",
            "green",
            "--json",
        ],
    );
    assert!(second.status.success(), "idempotent retarget should pass");
    let report: serde_json::Value = serde_json::from_slice(&second.stdout).expect("json");
    assert_eq!(
        report["state"]["groups"][0]["windows"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let third = run_cli(
        &dir,
        &state,
        &[
            "target-window",
            "--window-id",
            "0x3",
            "--group",
            "data-entry",
            "--color",
            "blue",
            "--json",
        ],
    );
    assert!(third.status.success(), "second window should save");
    let report: serde_json::Value = serde_json::from_slice(&third.stdout).expect("json");
    assert_eq!(
        report["state"]["groups"][0]["windows"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(report["state"]["groups"][0]["active_window_id"], 3);

    let moved = run_cli(
        &dir,
        &state,
        &[
            "target-window",
            "--window-id",
            "0x2",
            "--group",
            "review",
            "--color",
            "purple",
            "--json",
        ],
    );
    assert!(moved.status.success(), "move to another group should pass");
    let report: serde_json::Value = serde_json::from_slice(&moved.stdout).expect("json");
    let groups = report["state"]["groups"].as_array().unwrap();
    let data_entry = groups
        .iter()
        .find(|g| g["group_id"] == "data-entry")
        .unwrap();
    let review = groups.iter().find(|g| g["group_id"] == "review").unwrap();
    assert_eq!(data_entry["windows"].as_array().unwrap().len(), 1);
    assert_eq!(data_entry["windows"][0]["window"]["window_id"], 3);
    assert_eq!(review["windows"].as_array().unwrap().len(), 1);
    assert_eq!(review["windows"][0]["window"]["window_id"], 2);

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn overlay_failure_is_warning_and_target_is_saved() {
    let dir = temp_dir("target-window-overlay-warning");
    let state = dir.join("target-state.json");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Editor\n",
    );

    let output = run_cli(
        &dir,
        &state,
        &["target-window", "--window-id", "0x2", "--overlay", "--json"],
    );
    assert!(
        output.status.success(),
        "overlay unsupported should not fail target save"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert_eq!(report["success"], true);
    assert_eq!(report["target"]["window"]["window_id"], 2);
    assert_eq!(report["overlay"]["requested"], true);
    assert_eq!(report["overlay"]["shown"], false);
    assert!(report["overlay"]["warning"]
        .as_str()
        .unwrap_or_default()
        .contains("overlay provider"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
fn run_cli_with_overlay_log(
    dir: &std::path::Path,
    state: &std::path::Path,
    overlay_log: &std::path::Path,
    args: &[&str],
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(args)
        .env("DISPLAY", ":99")
        .env("HOSTNAME", "testhost")
        .env("PATH", path_with_fake_commands(dir))
        .env("CODEX_X11_TARGET_STATE", state)
        .env("CODEX_X11_OVERLAY_LOG", overlay_log)
        .output()
        .expect("run codex-computer-use-x11")
}

#[cfg(unix)]
#[test]
fn overlay_provider_shows_excludes_and_hides() {
    let dir = temp_dir("target-window-overlay-provider");
    let state = dir.join("target-state.json");
    let overlay_log = dir.join("overlay.log");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Editor\n0x00000009 0 333 10 20 800 4 codex-computer-use-x11-overlay.codex-computer-use-x11-overlay testhost codex-computer-use-x11-overlay\n",
    );

    let output = run_cli_with_overlay_log(
        &dir,
        &state,
        &overlay_log,
        &["target-window", "--window-id", "0x2", "--overlay", "--json"],
    );
    assert!(
        output.status.success(),
        "overlay target should pass: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json");
    assert_eq!(report["overlay"]["requested"], true);
    assert_eq!(report["overlay"]["shown"], true);
    assert_eq!(report["overlay"]["provider"], "test-overlay-log");
    assert!(report["overlay"]["warning"].is_null());
    assert_eq!(report["target"]["window"]["window_id"], 2);

    let context =
        run_cli_with_overlay_log(&dir, &state, &overlay_log, &["target-context", "--json"]);
    let context_report: serde_json::Value = serde_json::from_slice(&context.stdout).expect("json");
    let windows = context_report["diagnostics"]["listing"]["window_metadata"]
        .as_array()
        .unwrap();
    assert!(windows.iter().any(|item| item["owned_by_project"] == true));
    assert!(context_report["diagnostics"]["listing"]["focused_window"].is_number());

    let release = run_cli_with_overlay_log(
        &dir,
        &state,
        &overlay_log,
        &["release-window", "--window-id", "0x2", "--json"],
    );
    assert!(release.status.success(), "release should pass");
    let release_report: serde_json::Value = serde_json::from_slice(&release.stdout).expect("json");
    assert_eq!(release_report["overlay"]["requested"], true);
    assert_eq!(release_report["overlay"]["provider"], "test-overlay-log");
    let log = std::fs::read_to_string(&overlay_log).expect("overlay log");
    assert!(log.contains("show window=2"), "log={log}");
    assert!(log.contains("hide window=2"), "log={log}");

    let _ = std::fs::remove_dir_all(&dir);
}
