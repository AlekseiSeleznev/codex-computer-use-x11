use std::process::Command;

fn run_cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(args)
        .output()
        .expect("run codex-computer-use-x11")
}

fn run_doctor_with_ydotool_env(socket: &str, runtime: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(["doctor", "--json"])
        .env("YDOTOOL_SOCKET", socket)
        .env("XDG_RUNTIME_DIR", runtime)
        .output()
        .expect("run codex-computer-use-x11")
}

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
        .env("XDG_SESSION_TYPE", "x11")
        .env("XDG_CURRENT_DESKTOP", "X-Cinnamon")
        .env("DESKTOP_SESSION", "cinnamon")
        .env("PATH", path)
        .output()
        .expect("run codex-computer-use-x11")
}

#[cfg(unix)]
fn run_doctor_with_fake_desktop(dir: &std::path::Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(["doctor", "--json"])
        .env("DISPLAY", ":99")
        .env("XDG_SESSION_TYPE", "x11")
        .env("XDG_CURRENT_DESKTOP", "X-Cinnamon")
        .env("DESKTOP_SESSION", "cinnamon")
        .env("PATH", path_with_fake_commands(dir))
        .env_remove("NO_AT_BRIDGE")
        .output()
        .expect("run codex-computer-use-x11")
}

#[cfg(unix)]
fn write_doctor_probe_desktop_commands(dir: &std::path::Path, python_script: &str) {
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\ncat <<'OUT'\n_NET_SUPPORTING_WM_CHECK(WINDOW): window id # 0x2600006\n_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2\nOUT\nelif [ \"$1\" = \"-id\" ]; then\ncat <<'OUT'\n_NET_WM_PID(CARDINAL) = 4242\nWM_CLIENT_MACHINE(STRING) = \"testhost\"\nWM_NAME(STRING) = \"GTK Fixture\"\n_NET_WM_NAME(UTF8_STRING) = \"GTK Fixture\"\nWM_CLASS(STRING) = \"gtk-fixture\", \"GtkFixture\"\n_NET_WM_WINDOW_TYPE(ATOM) = _NET_WM_WINDOW_TYPE_NORMAL\nOUT\nelse\n  exit 2\nfi\n",
    );
    for tool in ["wmctrl", "xdotool", "ydotool"] {
        write_executable(&dir.join(tool), "#!/bin/sh\nexit 0\n");
    }
    write_executable(
        &dir.join("busctl"),
        "#!/bin/sh\nif echo \"$*\" | grep -q 'org.freedesktop.portal.RemoteDesktop'; then\n  echo 'NAME TYPE SIGNATURE RESULT/VALUE FLAGS'\n  exit 0\nfi\nexit 0\n",
    );
    write_executable(
        &dir.join("gdbus"),
        r#"#!/bin/sh
args="$*"
if echo "$args" | grep -q 'org.a11y.Bus.GetAddress'; then
  echo "('unix:path=/run/user/1000/at-spi/bus_0,guid=fake',)"
  exit 0
fi
if echo "$args" | grep -q 'org.gnome.Shell.Screenshot'; then
  echo 'method ScreenshotArea'
  exit 0
fi
if echo "$args" | grep -q 'org.freedesktop.portal.Desktop'; then
  echo 'method Screenshot'
  echo 'readonly u version = 2;'
  exit 0
fi
exit 0
"#,
    );
    write_executable(&dir.join("python3"), python_script);
}

#[cfg(unix)]
fn run_cli_with_path_and_probe_timeout(
    args: &[&str],
    path: String,
    timeout_ms: &str,
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(args)
        .env("DISPLAY", ":99")
        .env("XDG_SESSION_TYPE", "x11")
        .env("XDG_CURRENT_DESKTOP", "X-Cinnamon")
        .env("DESKTOP_SESSION", "cinnamon")
        .env("PATH", path)
        .env("CODEX_X11_COMMAND_TIMEOUT_MS", timeout_ms)
        .env_remove("NO_AT_BRIDGE")
        .output()
        .expect("run codex-computer-use-x11")
}

#[test]
fn doctor_cli_success_json() {
    let output = run_cli(&["doctor", "--json"]);
    assert!(output.status.success(), "status: {:?}", output.status);
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(report["backend"], "x11-ewmh");
    assert!(report["readiness"]["ok"].is_boolean());
    assert!(report["readiness"]["blockers"].is_array());
    assert!(report["readiness"]["degraded_reasons"].is_array());
    for field in [
        "can_query_windows",
        "can_focus_apps",
        "can_focus_windows",
        "can_send_development_input",
    ] {
        assert!(
            report["readiness"][field].is_boolean(),
            "{field} should be boolean"
        );
    }

    let implemented = report["capabilities"]["implemented"].as_array().unwrap();
    assert!(implemented.iter().any(|item| item == "doctor-json"));
    assert!(implemented
        .iter()
        .any(|item| item == "doctor-capability-detection"));
    assert!(implemented.iter().any(|item| item == "x11-ewmh-windowing"));
    assert!(implemented
        .iter()
        .any(|item| item == "x11-ewmh-focus-with-verification"));
    assert!(report["capabilities"]["planned"].is_array());
    assert!(
        !report["capabilities"]["planned"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item == "x11-ewmh-windowing"),
        "finalized v1 windowing must not remain planned"
    );

    let checks = report["checks"].as_array().expect("checks array");
    for name in [
        "doctor-report-schema",
        "read-only-probes",
        "doctor-internal-self-check",
    ] {
        let check = checks
            .iter()
            .find(|check| check["name"] == name)
            .unwrap_or_else(|| panic!("missing check {name}"));
        assert_eq!(check["ok"], true, "check {name} should be ok");
        assert!(!check["detail"].as_str().unwrap_or_default().is_empty());
    }

    assert!(checks
        .iter()
        .all(|check| check["name"] != "no-live-x11-probes"));
}

#[test]
fn doctor_cli_arguments() {
    for flag in ["--help", "-h"] {
        let output = run_cli(&[flag]);
        assert!(output.status.success(), "{flag} should exit 0");
        assert!(String::from_utf8_lossy(&output.stderr).is_empty());
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("doctor --json"),
            "{flag} usage should mention doctor --json"
        );
    }

    for args in [&["unknown"][..], &["doctor"][..]] {
        let output = run_cli(args);
        assert!(!output.status.success(), "{args:?} should fail");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unsupported"),
            "{args:?} should explain unsupported invocation"
        );
    }
}

#[test]
fn doctor_cli_redacts_env_derived_ydotool_socket_paths() {
    let private_socket = "/home/alice/private/ydotool.sock";
    let private_runtime = "/run/user/12345-private";
    let output = run_doctor_with_ydotool_env(private_socket, private_runtime);
    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    let json = serde_json::to_string(&report).unwrap();

    assert!(!json.contains(private_socket));
    assert!(!json.contains(&format!("{private_runtime}/.ydotool_socket")));
    assert!(json.contains("env:YDOTOOL_SOCKET"));
    assert!(json.contains("env:XDG_RUNTIME_DIR/.ydotool_socket"));
    assert!(json.contains("/tmp/.ydotool_socket"));
}

#[cfg(unix)]
#[test]
fn doctor_live_probe_gathers_portal_screenshot_and_atspi_facts() {
    let dir = temp_dir("doctor-live-probes");
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\ncat <<'OUT'\n_NET_SUPPORTING_WM_CHECK(WINDOW): window id # 0x2600006\n_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2\nOUT\nelse\n  exit 2\nfi\n",
    );
    for tool in ["wmctrl", "xdotool", "ydotool"] {
        write_executable(&dir.join(tool), "#!/bin/sh\nexit 0\n");
    }
    write_executable(
        &dir.join("busctl"),
        "#!/bin/sh\nif echo \"$*\" | grep -q 'org.freedesktop.portal.RemoteDesktop'; then\n  echo 'NAME TYPE SIGNATURE RESULT/VALUE FLAGS'\n  exit 0\nfi\nexit 0\n",
    );
    write_executable(
        &dir.join("gdbus"),
        r#"#!/bin/sh
args="$*"
if echo "$args" | grep -q 'org.a11y.Bus.GetAddress'; then
  echo "('unix:path=/run/user/1000/at-spi/bus_0,guid=fake',)"
  exit 0
fi
if echo "$args" | grep -q 'org.gnome.Shell.Screenshot'; then
  cat <<'OUT'
interface org.gnome.Shell.Screenshot {
  Screenshot(in  b include_frame, in  b include_cursor, in  s filename, out b success, out s filename_used);
  ScreenshotArea(in i x, in i y, in i width, in i height, in b flash, in s filename, out b success, out s filename_used);
}
OUT
  exit 0
fi
if echo "$args" | grep -q 'org.freedesktop.portal.Desktop'; then
  cat <<'OUT'
interface org.freedesktop.portal.Screenshot {
  Screenshot(in s parent_window, in a{sv} options, out o handle);
  readonly u version = 2;
}
OUT
  exit 0
fi
exit 0
"#,
    );

    let output = run_cli_with_path(&["doctor", "--json"], path_with_fake_commands(&dir));
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["x11"]["ewmh"]["can_query_windows"], true);
    assert_eq!(report["portals"]["remote_desktop"]["available"], false);
    assert_eq!(
        report["portals"]["screenshot"]["xdg_portal_available"],
        true
    );
    assert_eq!(
        report["portals"]["screenshot"]["gnome_shell_dbus_available"],
        true
    );
    assert_eq!(report["accessibility"]["atspi_bus_available"], true);
    let json = serde_json::to_string(&report).unwrap();
    assert!(!json.contains("unix:path=/run/user/1000/at-spi"));
}

#[cfg(unix)]
#[test]
fn doctor_live_probe_times_out_hung_desktop_commands() {
    let dir = temp_dir("doctor-live-probe-timeouts");
    for command in ["xprop", "busctl", "gdbus"] {
        write_executable(&dir.join(command), "#!/bin/sh\nexec sleep 30\n");
    }
    for tool in ["wmctrl", "xdotool", "ydotool"] {
        write_executable(&dir.join(tool), "#!/bin/sh\nexit 0\n");
    }

    let started = std::time::Instant::now();
    let output = run_cli_with_path_and_probe_timeout(
        &["doctor", "--json"],
        path_with_fake_commands(&dir),
        "50",
    );
    let elapsed = started.elapsed();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "doctor should bound hung live probes, elapsed: {elapsed:?}"
    );

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["x11"]["ewmh"]["can_query_windows"], false);
    assert_eq!(
        report["portals"]["screenshot"]["xdg_portal_available"],
        false
    );
    assert_eq!(report["accessibility"]["atspi_bus_available"], false);
}

#[cfg(unix)]
#[test]
fn doctor_atspi_probe_reports_tree_available_without_no_at_bridge_when_collector_succeeds() {
    let dir = temp_dir("doctor-atspi-probe-tree");
    write_doctor_probe_desktop_commands(
        &dir,
        "#!/bin/sh\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.91/gtk\",\"name\":\"GTK Fixture\",\"role\":\"application\",\"pid\":4242,\"bounds\":{\"x\":10,\"y\":20,\"width\":800,\"height\":600},\"focused\":true,\"states\":[\"active\",\"showing\"],\"nodes\":[{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.91/gtk\",\"role\":\"application\",\"name\":\"GTK Fixture\",\"child_count\":0,\"bounds\":{\"x\":10,\"y\":20,\"width\":800,\"height\":600},\"states\":[\"active\",\"showing\"],\"actions\":[],\"supports_editable_text\":false}]}],\"diagnostics\":{\"detail\":\"fake AT-SPI probe candidate\",\"truncated\":false}}\nJSON\n",
    );

    let output = run_doctor_with_fake_desktop(&dir);
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["accessibility"]["atspi_bus_available"], true);
    assert_eq!(report["accessibility"]["tree_available"], true);
    assert_eq!(report["accessibility"]["candidate_count"], 1);
    assert_eq!(report["accessibility"]["match_outcome"], "tree_available");
    assert_eq!(
        report["accessibility"]["diagnostic_state"],
        "tree_extraction_available"
    );
    let degraded = report["readiness"]["degraded_reasons"]
        .as_array()
        .expect("degraded reasons array");
    assert!(
        !degraded
            .iter()
            .any(|reason| reason == "atspi_tree_extraction_unavailable"),
        "working collector probe must prevent false degraded doctor state: {report}"
    );
}

#[cfg(unix)]
#[test]
fn doctor_atspi_probe_uses_accessibility_tree_success_contract() {
    let dir = temp_dir("doctor-atspi-large-success-contract");
    let mut nodes = String::new();
    for index in 0..900 {
        if index > 0 {
            nodes.push(',');
        }
        nodes.push_str(&format!(
            r#"{{"index":{index},"parent_index":null,"depth":0,"object_ref":":1.91/gtk/node/{index}","role":"label","name":"Accessible node {index} with enough text to exceed the pipe buffer and reproduce bounded probe collection","description":null,"child_count":0,"bounds":{{"x":10,"y":20,"width":800,"height":20}},"states":["showing","visible"],"actions":[],"supports_editable_text":false}}"#
        ));
    }
    let collector_json = format!(
        r#"{{"ok":true,"candidates":[{{"object_ref":":1.91/gtk","name":"GTK Fixture","role":"application","pid":4242,"bounds":{{"x":10,"y":20,"width":800,"height":600}},"focused":true,"states":["active","showing"],"nodes":[{nodes}]}}],"diagnostics":{{"detail":"fake accessibility-tree-compatible collector returned 1 large candidate","truncated":false}}}}"#
    );
    write_doctor_probe_desktop_commands(
        &dir,
        &format!("#!/bin/sh\ncat <<'JSON'\n{collector_json}\nJSON\n"),
    );

    let output = run_cli_with_path_and_probe_timeout(
        &["doctor", "--json"],
        path_with_fake_commands(&dir),
        "250",
    );
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["accessibility"]["atspi_bus_available"], true);
    assert_eq!(
        report["accessibility"]["tree_available"], true,
        "doctor must treat a successful collector with candidates as tree available: {report}"
    );
    assert_eq!(report["accessibility"]["candidate_count"], 1);
    assert_eq!(report["accessibility"]["match_outcome"], "tree_available");
    let degraded = report["readiness"]["degraded_reasons"]
        .as_array()
        .expect("degraded reasons array");
    assert!(
        !degraded
            .iter()
            .any(|reason| reason == "AT-SPI bus reachable but tree extraction unavailable"),
        "accessibility-tree-compatible collector success must not become a doctor tree extraction degradation: {report}"
    );
}

#[cfg(unix)]
#[test]
fn doctor_atspi_probe_reports_tree_available_even_with_no_at_bridge_when_collector_succeeds() {
    let dir = temp_dir("doctor-atspi-bridge-disabled");
    let collector_log = dir.join("collector.log");
    write_doctor_probe_desktop_commands(
        &dir,
        &format!(
            "#!/bin/sh\necho collector-ran >> '{}'\ncat <<'JSON'\n{{\"ok\":true,\"candidates\":[{{\"object_ref\":\":1.91/gtk\",\"name\":\"GTK Fixture\",\"role\":\"application\",\"nodes\":[]}}]}}\nJSON\n",
            collector_log.display()
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(["doctor", "--json"])
        .env("DISPLAY", ":99")
        .env("XDG_SESSION_TYPE", "x11")
        .env("XDG_CURRENT_DESKTOP", "X-Cinnamon")
        .env("DESKTOP_SESSION", "cinnamon")
        .env("PATH", path_with_fake_commands(&dir))
        .env("NO_AT_BRIDGE", "1")
        .output()
        .expect("run codex-computer-use-x11");
    let collector_ran = collector_log.exists();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["accessibility"]["atspi_bus_available"], true);
    assert_eq!(report["accessibility"]["tree_available"], true);
    assert_eq!(report["accessibility"]["match_outcome"], "tree_available");
    assert_eq!(
        report["accessibility"]["diagnostic_state"],
        "tree_extraction_available"
    );
    assert_eq!(
        report["accessibility"]["bridge_env"]["no_at_bridge_value"],
        "1"
    );
    assert!(
        collector_ran,
        "doctor must run the AT-SPI tree probe because valid collector output proves tree availability even with NO_AT_BRIDGE=1"
    );
    let degraded = report["readiness"]["degraded_reasons"]
        .as_array()
        .expect("degraded reasons array");
    assert!(
        !degraded
            .iter()
            .any(|reason| reason == "AT-SPI bus reachable but tree extraction unavailable"),
        "valid collector output must not be reported as tree extraction degradation: {report}"
    );
}

#[cfg(unix)]
#[test]
fn doctor_atspi_probe_degrades_when_collector_output_invalid_even_with_no_at_bridge() {
    let dir = temp_dir("doctor-atspi-invalid-with-bridge-env");
    let collector_log = dir.join("collector-invalid.log");
    write_doctor_probe_desktop_commands(
        &dir,
        &format!(
            "#!/bin/sh\necho collector-ran >> '{}'\necho 'not-json'\n",
            collector_log.display()
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(["doctor", "--json"])
        .env("DISPLAY", ":99")
        .env("XDG_SESSION_TYPE", "x11")
        .env("XDG_CURRENT_DESKTOP", "X-Cinnamon")
        .env("DESKTOP_SESSION", "cinnamon")
        .env("PATH", path_with_fake_commands(&dir))
        .env("NO_AT_BRIDGE", "1")
        .output()
        .expect("run codex-computer-use-x11");
    let collector_ran = collector_log.exists();
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["accessibility"]["atspi_bus_available"], true);
    assert_eq!(report["accessibility"]["tree_available"], false);
    assert_eq!(
        report["accessibility"]["match_outcome"],
        "collector_unavailable"
    );
    assert_eq!(
        report["accessibility"]["diagnostic_state"],
        "atspi_gtk_bridge_disabled_by_environment"
    );
    assert!(
        collector_ran,
        "doctor should run the bounded collector before classifying true degraded output"
    );
}

#[cfg(unix)]
#[test]
fn doctor_atspi_probe_default_timeout_allows_slow_valid_collector() {
    let dir = temp_dir("doctor-atspi-slow-valid-default-timeout");
    write_doctor_probe_desktop_commands(
        &dir,
        "#!/bin/sh\nsleep 3\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.91/gtk\",\"name\":\"Slow GTK Fixture\",\"role\":\"application\",\"nodes\":[{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.91/gtk\",\"role\":\"application\",\"name\":\"Slow GTK Fixture\",\"child_count\":0,\"states\":[\"showing\"],\"actions\":[],\"supports_editable_text\":false}]}],\"diagnostics\":{\"detail\":\"slow but valid fake AT-SPI probe candidate\",\"truncated\":false}}\nJSON\n",
    );

    let output = run_doctor_with_fake_desktop(&dir);
    let _ = std::fs::remove_dir_all(&dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(
        report["accessibility"]["tree_available"], true,
        "doctor default bounded probe should allow slow valid collector output that accessibility-tree can obtain: {report}"
    );
    assert_eq!(report["accessibility"]["match_outcome"], "tree_available");
    assert_eq!(
        report["accessibility"]["diagnostic_state"],
        "tree_extraction_available"
    );
}

#[cfg(unix)]
#[test]
fn doctor_cli_x11_baseline_ignores_absent_remote_desktop_and_wayland_display() {
    let dir = temp_dir("doctor-x11-only-no-portal-noise");
    let runtime_dir = std::env::temp_dir().join(format!(
        "cx11-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    ));
    std::fs::create_dir_all(&runtime_dir).expect("create runtime dir");
    let ydotool_socket = runtime_dir.join(".ydotool_socket");
    let _listener =
        std::os::unix::net::UnixListener::bind(&ydotool_socket).expect("bind fake ydotool socket");
    write_doctor_probe_desktop_commands(
        &dir,
        "#!/bin/sh\ncat <<'JSON'\n{\"ok\":true,\"candidates\":[{\"object_ref\":\":1.91/gtk\",\"name\":\"GTK Fixture\",\"role\":\"application\",\"nodes\":[{\"index\":0,\"parent_index\":null,\"depth\":0,\"object_ref\":\":1.91/gtk\",\"role\":\"application\",\"name\":\"GTK Fixture\",\"child_count\":0,\"states\":[\"showing\"],\"actions\":[],\"supports_editable_text\":false}]}],\"diagnostics\":{\"detail\":\"fake AT-SPI probe candidate\",\"truncated\":false}}\nJSON\n",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"))
        .args(["doctor", "--json"])
        .env("DISPLAY", ":99")
        .env("XDG_SESSION_TYPE", "x11")
        .env("XDG_CURRENT_DESKTOP", "X-Cinnamon")
        .env("DESKTOP_SESSION", "cinnamon")
        .env("WAYLAND_DISPLAY", "wayland-0")
        .env("XDG_RUNTIME_DIR", &runtime_dir)
        .env("PATH", path_with_fake_commands(&dir))
        .env_remove("NO_AT_BRIDGE")
        .output()
        .expect("run codex-computer-use-x11");
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(&runtime_dir);

    assert!(output.status.success(), "status: {:?}", output.status);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid json");
    assert_eq!(report["readiness"]["ok"], true);
    assert_eq!(report["readiness"]["blockers"].as_array().unwrap().len(), 0);
    assert_eq!(
        report["readiness"]["degraded_reasons"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        report["readiness"]["optional_enrichments"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        report["readiness"]["unsupported_out_of_scope"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(report["environment"]["wayland_display_present"], true);
    assert_eq!(report["portals"]["remote_desktop"]["available"], false);
    let readiness_json = serde_json::to_string(&report["readiness"]).unwrap();
    for forbidden in [
        "RemoteDesktop portal unavailable or incomplete",
        "remote_desktop_portal_unavailable",
        "wayland_runtime_out_of_scope",
        "Wayland",
        "RemoteDesktop",
    ] {
        assert!(
            !readiness_json.contains(forbidden),
            "readiness must not contain forbidden X11-only noise {forbidden:?}: {readiness_json}"
        );
    }
    let full_json = serde_json::to_string(&report).unwrap();
    assert!(!full_json.contains("RemoteDesktop portal unavailable or incomplete"));
    assert!(!full_json.contains("remote_desktop_portal_unavailable"));
    assert!(!full_json.contains("wayland_runtime_out_of_scope"));
}
