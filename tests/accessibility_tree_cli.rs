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
fn accessibility_tree_returns_high_confidence_subtree_for_reliable_pid() {
    let dir = temp_dir("accessibility-tree-high-confidence");
    let log = dir.join("collector.log");
    let host = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "testhost".to_string());
    let host = host.trim();
    write_window_commands(
        &dir,
        &format!("0x00000002 0 4242 10 20 800 600 code.Code {host} Editor Alpha"),
    );
    write_executable(
        &dir.join("python3"),
        &format!(
            "#!/bin/sh\necho \"python3 $*\" >> '{}'\ncat <<'JSON'\n{{\"ok\":true,\"candidates\":[{{\"object_ref\":\":1.42/org/a11y/atspi/accessible/root\",\"name\":\"Editor Alpha\",\"role\":\"application\",\"pid\":4242,\"bounds\":{{\"x\":12,\"y\":22,\"width\":780,\"height\":560}},\"focused\":true,\"states\":[\"active\",\"showing\"],\"nodes\":[{{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.42/org/a11y/atspi/accessible/root\",\"role\":\"application\",\"name\":\"Editor Alpha\",\"child_count\":1,\"bounds\":{{\"x\":12,\"y\":22,\"width\":780,\"height\":560}},\"states\":[\"active\",\"showing\"],\"actions\":[],\"supports_editable_text\":false}}]}}],\"diagnostics\":{{\"truncated\":false}}}}\nJSON\n",
            log.display()
        ),
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status should succeed");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["backend"], "x11-ewmh");
    assert_eq!(report["success"], true);
    assert_eq!(report["window"]["window_id"], 2);
    assert_eq!(report["correlation"]["status"], "matched");
    assert_eq!(report["correlation"]["confidence"], "high");
    assert!(report["correlation"]["score"].as_i64().unwrap_or_default() >= 70);
    assert_eq!(
        report["correlation"]["matched_object_ref"],
        ":1.42/org/a11y/atspi/accessible/root"
    );
    assert_eq!(report["tree"].as_array().unwrap().len(), 1);
    assert!(report["error_code"].is_null());
    assert!(
        log_contents.contains("python3"),
        "collector should run after window resolution"
    );
}

#[cfg(unix)]
#[test]
fn accessibility_tree_refuses_missing_window_before_atspi_collection() {
    let dir = temp_dir("accessibility-tree-missing-window");
    let log = dir.join("collector.log");
    let host = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "testhost".to_string());
    let host = host.trim();
    write_window_commands(
        &dir,
        &format!("0x00000002 0 4242 10 20 800 600 code.Code {host} Editor Alpha"),
    );
    write_executable(
        &dir.join("python3"),
        &format!(
            "#!/bin/sh\necho \"python3 $*\" >> '{}'\ncat <<'JSON'\n{{\"ok\":true,\"candidates\":[]}}\nJSON\n",
            log.display()
        ),
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x99", "--json"],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "missing window should fail");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["window"], serde_json::Value::Null);
    assert_eq!(report["error_code"], "WindowNotFound");
    assert_eq!(report["tree"].as_array().unwrap().len(), 0);
    assert_eq!(
        log_contents, "",
        "collector should not run for missing window"
    );
}

#[cfg(unix)]
#[test]
fn accessibility_tree_uses_non_pid_evidence_when_pid_unreliable() {
    let dir = temp_dir("accessibility-tree-unreliable-pid");
    write_window_commands(
        &dir,
        "0x00000002 0 4242 10 20 800 600 code.Code remotehost Editor Alpha",
    );
    write_executable(
        &dir.join("python3"),
        "#!/bin/sh\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.1/bad-pid\",\"name\":\"Background Service\",\"role\":\"application\",\"pid\":4242,\"bounds\":{\"x\":2000,\"y\":2000,\"width\":100,\"height\":100},\"nodes\":[]},{\"object_ref\":\":1.2/editor\",\"name\":\"Editor Alpha\",\"role\":\"application\",\"pid\":9999,\"bounds\":{\"x\":11,\"y\":21,\"width\":790,\"height\":590},\"focused\":true,\"states\":[\"active\"],\"nodes\":[{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.2/editor\",\"role\":\"application\",\"name\":\"Editor Alpha\",\"child_count\":0,\"bounds\":{\"x\":11,\"y\":21,\"width\":790,\"height\":590},\"states\":[\"active\"],\"actions\":[],\"supports_editable_text\":false}]}]}\nJSON\n",
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "non-PID evidence should match");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["correlation"]["status"], "matched");
    assert_eq!(report["correlation"]["confidence"], "medium");
    assert_eq!(report["correlation"]["matched_object_ref"], ":1.2/editor");
    let reasons = report["correlation"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        reasons.contains("PID was not treated as reliable"),
        "{reasons}"
    );
    assert!(!reasons.contains("reliable PID matched"), "{reasons}");
}

#[cfg(unix)]
#[test]
fn accessibility_tree_refuses_ambiguous_candidates() {
    let dir = temp_dir("accessibility-tree-ambiguous");
    let host = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "testhost".to_string());
    let host = host.trim();
    write_window_commands(
        &dir,
        &format!("0x00000002 0 4242 10 20 800 600 code.Code {host} Editor Alpha"),
    );
    write_executable(
        &dir.join("python3"),
        "#!/bin/sh\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.10/editor-a\",\"name\":\"Editor Alpha\",\"role\":\"application\",\"pid\":4242,\"bounds\":{\"x\":12,\"y\":22,\"width\":780,\"height\":560},\"focused\":true,\"states\":[\"active\"],\"nodes\":[{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.10/editor-a\",\"role\":\"application\",\"name\":\"Editor Alpha\",\"child_count\":0,\"bounds\":{\"x\":12,\"y\":22,\"width\":780,\"height\":560},\"states\":[\"active\"],\"actions\":[],\"supports_editable_text\":false}]},{\"object_ref\":\":1.11/editor-b\",\"name\":\"Editor Alpha\",\"role\":\"application\",\"pid\":4242,\"bounds\":{\"x\":13,\"y\":23,\"width\":780,\"height\":560},\"focused\":true,\"states\":[\"active\"],\"nodes\":[{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.11/editor-b\",\"role\":\"application\",\"name\":\"Editor Alpha\",\"child_count\":0,\"bounds\":{\"x\":13,\"y\":23,\"width\":780,\"height\":560},\"states\":[\"active\"],\"actions\":[],\"supports_editable_text\":false}]}]}\nJSON\n",
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "ambiguous candidates should fail");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["correlation"]["status"], "ambiguous");
    assert_eq!(report["error_code"], "AmbiguousAccessibilityMatch");
    assert_eq!(report["tree"].as_array().unwrap().len(), 0);
    assert_eq!(
        report["correlation"]["candidates"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[cfg(unix)]
#[test]
fn accessibility_tree_matches_browser_by_title_class_bounds_without_pid() {
    let dir = temp_dir("accessibility-tree-browser-class");
    let host = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "testhost".to_string());
    let host = host.trim();
    write_window_commands(
        &dir,
        &format!(
            "0x00000002 0 5555 100 120 1200 900 google-chrome.Google-chrome {host} Inbox - Work"
        ),
    );
    write_executable(
        &dir.join("python3"),
        "#!/bin/sh\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.70/chrome\",\"name\":\"Google Chrome\",\"role\":\"application\",\"pid\":9999,\"bounds\":{\"x\":101,\"y\":121,\"width\":1190,\"height\":890},\"focused\":true,\"states\":[\"active\"],\"nodes\":[{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.70/chrome\",\"role\":\"application\",\"name\":\"Google Chrome\",\"child_count\":0,\"bounds\":{\"x\":101,\"y\":121,\"width\":1190,\"height\":890},\"states\":[\"active\"],\"actions\":[],\"supports_editable_text\":false}]}]}\nJSON\n",
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "browser should match without PID");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], true);
    assert_eq!(report["correlation"]["confidence"], "medium");
    assert_eq!(report["correlation"]["matched_object_ref"], ":1.70/chrome");
    let reasons = report["correlation"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(reasons.contains("wm_class/app name matched"), "{reasons}");
    assert!(!reasons.contains("reliable PID matched"), "{reasons}");
}

#[cfg(unix)]
#[test]
fn accessibility_tree_does_not_match_terminal_child_pid_alone() {
    let dir = temp_dir("accessibility-tree-terminal-child");
    let host = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "testhost".to_string());
    let host = host.trim();
    write_window_commands(
        &dir,
        &format!(
            "0x00000002 0 7000 30 40 900 500 gnome-terminal-server.Gnome-terminal {host} Terminal"
        ),
    );
    write_executable(
        &dir.join("python3"),
        "#!/bin/sh\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.80/bash\",\"name\":\"bash\",\"role\":\"application\",\"pid\":7100,\"nodes\":[]}]}\nJSON\n",
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "child PID alone should not match terminal"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["error_code"], "NoAccessibilityMatch");
    assert_eq!(report["tree"].as_array().unwrap().len(), 0);
    let reasons = report["correlation"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        reasons.contains("candidate PID did not match reliable window PID"),
        "{reasons}"
    );
}

#[cfg(unix)]
#[test]
fn accessibility_tree_degrades_when_atspi_collector_unavailable() {
    let dir = temp_dir("accessibility-tree-collector-unavailable");
    let host = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "testhost".to_string());
    let host = host.trim();
    write_window_commands(
        &dir,
        &format!("0x00000002 0 4242 10 20 800 600 code.Code {host} Editor Alpha"),
    );
    write_executable(
        &dir.join("python3"),
        "#!/bin/sh\necho 'gi import failed' >&2\nexit 7\n",
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(!output.status.success(), "collector failure should degrade");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["success"], false);
    assert_eq!(report["correlation"]["status"], "degraded");
    assert_eq!(report["error_code"], "AtspiUnavailable");
    assert_eq!(report["tree"].as_array().unwrap().len(), 0);
    assert_eq!(report["diagnostics"]["collector"]["command"], "python3 -c");
    assert!(report["diagnostics"]["collector"]["stderr"]
        .as_str()
        .unwrap_or_default()
        .contains("gi import failed"));
}

#[cfg(unix)]
#[test]
fn accessibility_tree_does_not_class_match_tk_to_gtk3_candidate() {
    let dir = temp_dir("accessibility-tree-tk-gtk3-token");
    write_window_commands(
        &dir,
        "0x00000002 0 0 10 20 800 600 Tk testhost X11-CUA-SAFE-TEXT",
    );
    write_executable(
        &dir.join("python3"),
        "#!/bin/sh\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.90/gtk3\",\"name\":\"ibus-ui-gtk3\",\"role\":\"application\",\"bounds\":{\"x\":10,\"y\":20,\"width\":800,\"height\":600},\"focused\":true,\"states\":[\"active\"],\"nodes\":[{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.90/gtk3\",\"role\":\"application\",\"name\":\"ibus-ui-gtk3\",\"child_count\":0,\"bounds\":{\"x\":10,\"y\":20,\"width\":800,\"height\":600},\"states\":[\"active\"],\"actions\":[],\"supports_editable_text\":false}]}]}\nJSON\n",
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "Tk must not match gtk3 by substring"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["error_code"], "NoAccessibilityMatch");
    assert_eq!(report["tree"].as_array().unwrap().len(), 0);
    let reasons = report["correlation"]["candidates"][0]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!reasons.contains("wm_class/app name matched"), "{reasons}");
    assert!(report["correlation"]["candidates"][0]["missing_signals"]
        .as_array()
        .unwrap()
        .iter()
        .any(|value| value == "class_app_match"));
}

#[cfg(unix)]
#[test]
fn accessibility_tree_records_target_scoped_xprop_enrichment_and_missing_signals() {
    let dir = temp_dir("accessibility-tree-target-xprop");
    let log = dir.join("xprop.log");
    let log_path = log.display();
    write_executable(
        &dir.join("wmctrl"),
        "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000002 0 4242 10 20 800 600 gtk-fixture.GtkFixture testhost GTK Fixture\nOUT\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n",
    );
    write_executable(
        &dir.join("xprop"),
        &format!("#!/bin/sh\necho \"xprop $*\" >> '{log_path}'\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelif [ \"$1\" = \"-id\" ]; then\ncat <<'OUT'\n_NET_WM_PID(CARDINAL) = 4242\nWM_CLIENT_MACHINE(STRING) = \"testhost\"\nWM_NAME(STRING) = \"GTK Fixture\"\n_NET_WM_NAME(UTF8_STRING) = \"GTK Fixture\"\nWM_CLASS(STRING) = \"gtk-fixture\", \"GtkFixture\"\n_NET_WM_WINDOW_TYPE(ATOM) = _NET_WM_WINDOW_TYPE_NORMAL\nOUT\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n"),
    );
    write_executable(
        &dir.join("python3"),
        "#!/bin/sh\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.91/gtk\",\"name\":\"Other App\",\"role\":\"application\",\"bounds\":{\"x\":10,\"y\":20,\"width\":800,\"height\":600},\"nodes\":[]}]}\nJSON\n",
    );

    let output = run_cli_with_path(
        &["accessibility-tree", "--window-id", "0x2", "--json"],
        path_with_fake_commands(&dir),
    );
    let log_contents = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        !output.status.success(),
        "bounds-only candidate should not match"
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert!(log_contents.contains("xprop -id 0x2"), "log={log_contents}");
    assert_eq!(report["diagnostics"]["target_xprop"]["xprop_id_calls"], 1);
    assert_eq!(
        report["diagnostics"]["target_xprop"]["wm_class"][1],
        "GtkFixture"
    );
    assert!(report["correlation"]["candidates"][0]["missing_signals"]
        .as_array()
        .unwrap()
        .iter()
        .any(|value| value == "reliable_pid"));
    assert!(report["correlation"]["candidates"][0]["missing_signals"]
        .as_array()
        .unwrap()
        .iter()
        .any(|value| value == "title_name_match"));
}
