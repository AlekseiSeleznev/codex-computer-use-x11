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
fn run_cli_with_path_in_cwd(
    args: &[&str],
    path: String,
    cwd: &std::path::Path,
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(args)
        .current_dir(cwd)
        .env("DISPLAY", ":99")
        .env("HOSTNAME", "testhost")
        .env("PATH", path)
        .output()
        .expect("run codex-computer-use-x11")
}

#[cfg(unix)]
fn write_minimal_png(path: &std::path::Path) {
    std::fs::write(
        path,
        [
            0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, b'I', b'H',
            b'D', b'R',
        ],
    )
    .expect("write png fixture");
}

#[cfg(unix)]
fn write_window_commands(dir: &std::path::Path, wmctrl_output: &str) {
    write_executable(
        &dir.join("wmctrl"),
        &format!(
            "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n{wmctrl_output}\nOUT\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n"
        ),
    );
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );
}

#[cfg(unix)]
#[test]
fn window_bounds_reports_signed_root_coordinates() {
    let dir = temp_dir("window-bounds-signed-root");
    write_window_commands(
        &dir,
        "0x00000002 0 222 -1280 24 1000 700 app.App testhost Negative Monitor Window",
    );

    let output = run_cli_with_path(
        &["window-bounds", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "status: {:?}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["backend"], "x11-ewmh");
    assert_eq!(report["success"], true);
    assert_eq!(report["window"]["window_id"], 2);
    assert_eq!(report["bounds"]["x"], -1280);
    assert_eq!(report["bounds"]["y"], 24);
    assert_eq!(report["bounds"]["width"], 1000);
    assert_eq!(report["bounds"]["height"], 700);
    assert_eq!(
        report["coordinate_model"]["space"],
        "x11_root_global_pixels"
    );
    assert_eq!(report["coordinate_model"]["x_y_type"], "Option<i32>");
    assert_eq!(report["diagnostics"]["primary_source"], "wmctrl -lpGx");
    assert!(report["diagnostics"]["bounds_semantics"]
        .as_str()
        .unwrap_or_default()
        .contains("frame/client"));
}

#[cfg(unix)]
#[test]
fn window_bounds_reports_xwininfo_disagreement() {
    let dir = temp_dir("window-bounds-xwininfo-disagreement");
    write_window_commands(
        &dir,
        "0x00000002 0 222 3840 0 1920 1040 app.App testhost Browser Window",
    );
    write_executable(
        &dir.join("xwininfo"),
        "#!/bin/sh\nif [ \"$1\" = \"-id\" ]; then\ncat <<'OUT'\n\nxwininfo: Window id: 0x2 \"Browser Window\"\n\n  Absolute upper-left X:  1920\n  Absolute upper-left Y:  0\n  Width: 1920\n  Height: 1040\nOUT\nelse\n  echo \"unexpected xwininfo args: $*\" >&2\n  exit 2\nfi\n",
    );

    let output = run_cli_with_path(
        &["window-bounds", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "status: {:?}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["bounds"]["x"], 3840);
    assert_eq!(report["diagnostics"]["bounds_agree"], false);
    let alternate = report["diagnostics"]["alternate_sources"]
        .as_array()
        .expect("alternate sources")
        .iter()
        .find(|source| source["source"] == "xwininfo -id")
        .expect("xwininfo alternate source");
    assert_eq!(alternate["bounds"]["x"], 1920);
    assert!(report["diagnostics"]["degraded_reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item
            .as_str()
            .unwrap_or_default()
            .contains("geometry sources disagreed")));
}

#[cfg(unix)]
#[test]
fn screenshot_crop_refuses_outside_target_before_provider() {
    let dir = temp_dir("screenshot-crop-outside-target");
    let log = dir.join("gdbus.log");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 100 100 app.App testhost Target Window",
    );
    write_executable(
        &dir.join("gdbus"),
        &format!(
            "#!/bin/sh\necho \"gdbus $*\" >> '{}'\nexit 0\n",
            log.display()
        ),
    );

    let output = run_cli_with_path(
        &[
            "screenshot-crop",
            "--window-id",
            "0x2",
            "--x",
            "0",
            "--y",
            "20",
            "--width",
            "50",
            "--height",
            "50",
            "--output",
            dir.join("crop.png").to_str().unwrap(),
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "outside target crop should fail");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["error_code"], "CropOutsideTargetBounds");
    assert_eq!(report["screenshot_invoked"], false);
    assert_eq!(
        log_contents, "",
        "provider should not be invoked before validation succeeds"
    );
}

#[cfg(unix)]
#[test]
fn screenshot_crop_resolves_relative_output_path_before_provider_call() {
    let dir = temp_dir("screenshot-crop-relative-output");
    let log = dir.join("gdbus.log");
    let fixture_png = dir.join("fixture.png");
    write_minimal_png(&fixture_png);
    std::fs::create_dir_all(dir.join("relative")).expect("create relative output parent");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Target Window",
    );
    write_executable(
        &dir.join("gdbus"),
        &format!(
            "#!/bin/sh\nfor last do :; done\necho \"gdbus $*\" >> '{}'\nprintf \"(true, '%s')\\n\" \"$last\"\ncp '{}' \"$last\"\nexit 0\n",
            log.display(),
            fixture_png.display()
        ),
    );

    let output = run_cli_with_path_in_cwd(
        &[
            "screenshot-crop",
            "--window-id",
            "0x2",
            "--output",
            "relative/crop.png",
            "--json",
        ],
        path_with_fake_commands(&dir),
        &dir,
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let expected_output = dir.join("relative/crop.png");
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "status: {:?}\nstderr: {}\nstdout: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["output_path"], expected_output.to_str().unwrap());
    assert!(
        log_contents.contains(expected_output.to_str().unwrap()),
        "{log_contents}"
    );
    assert!(
        !log_contents.contains(" relative/crop.png"),
        "provider must receive resolved absolute output path: {log_contents}"
    );
}

#[cfg(unix)]
#[test]
fn screenshot_crop_rejects_missing_output_parent_before_provider_call() {
    let dir = temp_dir("screenshot-crop-missing-output-parent");
    let log = dir.join("gdbus.log");
    let output_path = dir.join("missing/crop.png");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Target Window",
    );
    write_executable(
        &dir.join("gdbus"),
        &format!(
            "#!/bin/sh\necho \"gdbus $*\" >> '{}'\nexit 0\n",
            log.display()
        ),
    );

    let output = run_cli_with_path(
        &[
            "screenshot-crop",
            "--window-id",
            "0x2",
            "--output",
            output_path.to_str().unwrap(),
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "missing output parent should fail before provider\nstdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["screenshot_invoked"], false);
    assert_eq!(report["error_code"], "OutputPathUnavailable");
    assert_eq!(log_contents, "", "provider should not be invoked");
}

#[cfg(unix)]
#[test]
fn screenshot_crop_provider_false_without_output_is_failure() {
    let dir = temp_dir("screenshot-crop-provider-false");
    let output_path = dir.join("crop.png");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Target Window",
    );
    write_executable(
        &dir.join("gdbus"),
        &format!(
            "#!/bin/sh\nprintf \"(false, '{}')\\n\"\nexit 0\n",
            output_path.display()
        ),
    );

    let output = run_cli_with_path(
        &[
            "screenshot-crop",
            "--window-id",
            "0x2",
            "--output",
            output_path.to_str().unwrap(),
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let output_exists = output_path.exists();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "provider false/no output should fail\nstdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert!(
        !output_exists,
        "fake provider should not create output file"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["screenshot_invoked"], true);
    assert_eq!(report["error_code"], "ScreenshotOutputMissing");
    assert!(report["diagnostics"]["provider_detail"]
        .as_str()
        .unwrap_or_default()
        .contains("(false"));
}

#[cfg(unix)]
#[test]
fn screenshot_crop_rejects_empty_output_file() {
    let dir = temp_dir("screenshot-crop-empty-output");
    let output_path = dir.join("crop.png");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Target Window",
    );
    write_executable(
        &dir.join("gdbus"),
        &format!(
            "#!/bin/sh\nfor last do :; done\n: > \"$last\"\nprintf \"(true, '%s')\\n\" \"$last\"\nexit 0\n"
        ),
    );

    let output = run_cli_with_path(
        &[
            "screenshot-crop",
            "--window-id",
            "0x2",
            "--output",
            output_path.to_str().unwrap(),
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "empty output should fail\nstdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["screenshot_invoked"], true);
    assert_eq!(report["error_code"], "ScreenshotOutputEmpty");
}

#[cfg(unix)]
#[test]
fn screenshot_crop_rejects_non_png_output_file() {
    let dir = temp_dir("screenshot-crop-non-png-output");
    let output_path = dir.join("crop.png");
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Target Window",
    );
    write_executable(
        &dir.join("gdbus"),
        "#!/bin/sh\nfor last do :; done\nprintf 'not a png' > \"$last\"\nprintf \"(true, '%s')\\n\" \"$last\"\nexit 0\n",
    );

    let output = run_cli_with_path(
        &[
            "screenshot-crop",
            "--window-id",
            "0x2",
            "--output",
            output_path.to_str().unwrap(),
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "non-png output should fail\nstdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["screenshot_invoked"], true);
    assert_eq!(report["error_code"], "ScreenshotOutputInvalidFormat");
}

#[cfg(unix)]
#[test]
fn screenshot_crop_invokes_gdbus_with_validated_rect() {
    let dir = temp_dir("screenshot-crop-gdbus");
    let log = dir.join("gdbus.log");
    let output_path = dir.join("crop.png");
    let fixture_png = dir.join("fixture.png");
    write_minimal_png(&fixture_png);
    write_window_commands(
        &dir,
        "0x00000002 0 222 10 20 800 600 app.App testhost Target Window",
    );
    write_executable(
        &dir.join("gdbus"),
        &format!(
            "#!/bin/sh\necho \"gdbus $*\" >> '{}'\n# gdbus returns a tuple-like result on success\nprintf \"(true, '%s')\\n\" \"{}\"\ncp '{}' \"{}\"\nexit 0\n",
            log.display(),
            output_path.display(),
            fixture_png.display(),
            output_path.display()
        ),
    );

    let output = run_cli_with_path(
        &[
            "screenshot-crop",
            "--window-id",
            "0x2",
            "--output",
            output_path.to_str().unwrap(),
            "--json",
        ],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let output_exists = output_path.exists();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        output.status.success(),
        "status: {:?}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert!(output_exists, "fake provider should touch output file");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["screenshot_invoked"], true);
    assert_eq!(report["provider"], "gnome_shell_screenshot_area");
    assert_eq!(report["crop"]["x"], 10);
    assert_eq!(report["crop"]["y"], 20);
    assert_eq!(report["crop"]["width"], 800);
    assert_eq!(report["crop"]["height"], 600);
    assert_eq!(report["crop"]["source"], "window_bounds");
    assert!(
        report.get("data_url").is_none(),
        "report must not serialize screenshot pixels"
    );
    assert!(
        log_contents.contains("org.gnome.Shell.Screenshot.ScreenshotArea 10 20 800 600 false"),
        "{log_contents}"
    );
    assert!(
        log_contents.contains(output_path.to_str().unwrap()),
        "{log_contents}"
    );
}
