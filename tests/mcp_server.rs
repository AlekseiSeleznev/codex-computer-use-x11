use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

struct McpHarness {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl McpHarness {
    fn start() -> Self {
        let mut command = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"));
        command
            .arg("mcp")
            .env_remove("DISPLAY")
            .env("CODEX_X11_DISABLE_DESKTOP_ENV_HYDRATION", "1");
        Self::start_with_command(command)
    }

    #[cfg(unix)]
    fn start_with_fake_x11(dir: &std::path::Path) -> Self {
        let mut command = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"));
        command
            .arg("mcp")
            .env("DISPLAY", ":99")
            .env("HOSTNAME", "testhost")
            .env("CODEX_X11_DISABLE_DESKTOP_ENV_HYDRATION", "1")
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    dir.display(),
                    std::env::var("PATH").unwrap_or_default()
                ),
            );
        Self::start_with_command(command)
    }

    #[cfg(unix)]
    fn start_with_desktop_env_fixture(dir: &std::path::Path, fixture: &std::path::Path) -> Self {
        let mut command = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"));
        command
            .arg("mcp")
            .env_remove("DISPLAY")
            .env_remove("XDG_SESSION_TYPE")
            .env_remove("XDG_CURRENT_DESKTOP")
            .env_remove("DESKTOP_SESSION")
            .env_remove("DBUS_SESSION_BUS_ADDRESS")
            .env_remove("XAUTHORITY")
            .env("CODEX_X11_DESKTOP_ENV_FIXTURE", fixture)
            .env("HOSTNAME", "testhost")
            .env_remove("CODEX_X11_DISABLE_DESKTOP_ENV_HYDRATION")
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    dir.display(),
                    std::env::var("PATH").unwrap_or_default()
                ),
            );
        Self::start_with_command(command)
    }

    #[cfg(unix)]
    fn start_with_explicit_display_and_desktop_env_fixture(
        dir: &std::path::Path,
        fixture: &std::path::Path,
    ) -> Self {
        let mut command = Command::new(env!("CARGO_BIN_EXE_codex-computer-use-x11"));
        command
            .arg("mcp")
            .env("DISPLAY", ":99")
            .env_remove("XDG_SESSION_TYPE")
            .env_remove("XDG_CURRENT_DESKTOP")
            .env_remove("DESKTOP_SESSION")
            .env_remove("DBUS_SESSION_BUS_ADDRESS")
            .env_remove("XAUTHORITY")
            .env("CODEX_X11_DESKTOP_ENV_FIXTURE", fixture)
            .env("HOSTNAME", "testhost")
            .env_remove("CODEX_X11_DISABLE_DESKTOP_ENV_HYDRATION")
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    dir.display(),
                    std::env::var("PATH").unwrap_or_default()
                ),
            );
        Self::start_with_command(command)
    }

    fn start_with_command(mut command: Command) -> Self {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start mcp server");

        let stdin = child.stdin.take().expect("mcp stdin");
        let stdout = BufReader::new(child.stdout.take().expect("mcp stdout"));

        Self {
            child,
            stdin,
            stdout,
        }
    }

    fn send(&mut self, message: serde_json::Value) {
        writeln!(self.stdin, "{message}").expect("write mcp message");
        self.stdin.flush().expect("flush mcp message");
    }

    fn read_response(&mut self) -> serde_json::Value {
        let mut line = String::new();
        self.stdout.read_line(&mut line).expect("read mcp response");
        assert!(!line.is_empty(), "mcp server closed stdout unexpectedly");
        serde_json::from_str(&line).expect("valid mcp response json")
    }

    fn initialize(&mut self) {
        self.send(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "codex-computer-use-x11-test", "version": "0.0.0"}
            }
        }));
        let response = self.read_response();
        assert_eq!(response["id"], 1);
        assert!(response["result"]["capabilities"]["tools"].is_object());

        self.send(serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }));
    }

    fn call_tool(
        &mut self,
        id: u64,
        name: &str,
        arguments: serde_json::Value,
    ) -> serde_json::Value {
        self.send(serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": {
                "name": name,
                "arguments": arguments
            }
        }));
        self.read_response()
    }
}

impl Drop for McpHarness {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn mcp_server_lists_x11_tools() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    mcp.send(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    }));

    let response = mcp.read_response();
    assert_eq!(response["id"], 2);
    let tools = response["result"]["tools"].as_array().expect("tools array");
    let names: Vec<_> = tools
        .iter()
        .map(|tool| tool["name"].as_str().expect("tool name"))
        .collect();
    assert_eq!(
        names,
        vec![
            "x11_doctor",
            "x11_list_windows",
            "x11_focused_window",
            "x11_focus_window",
            "x11_type_text",
            "x11_press_key",
            "x11_click",
            "x11_scroll",
            "x11_drag",
            "x11_accessibility_tree",
            "x11_get_app_state",
            "x11_target_window",
            "x11_release_window",
            "x11_target_context"
        ]
    );
    for tool in tools {
        assert!(tool["description"].as_str().unwrap_or_default().len() > 10);
        assert_eq!(tool["inputSchema"]["type"], "object");
    }
    assert!(!names.contains(&"computer-use"));
    assert!(!names.contains(&"activate_window"));
    assert!(!names.contains(&"type_text"));
    assert!(!names.contains(&"press_key"));
    assert!(!names.contains(&"click"));
    assert!(!names.contains(&"scroll"));
    assert!(!names.contains(&"drag"));
    assert!(!names.contains(&"accessibility_tree"));
    assert!(!names.contains(&"get_app_state"));
}

#[test]
fn mcp_server_calls_x11_doctor() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    mcp.send(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "x11_doctor",
            "arguments": {}
        }
    }));

    let response = mcp.read_response();
    assert_eq!(response["id"], 3);
    assert!(
        response.get("error").is_none(),
        "tools/call should return a tool result, not protocol error: {response}"
    );
    assert_eq!(response["result"]["isError"], false);
    let content = response["result"]["content"]
        .as_array()
        .expect("content array");
    let text = content[0]["text"].as_str().expect("text content");
    let report: serde_json::Value = serde_json::from_str(text).expect("doctor json text");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["backend"], "x11-ewmh");
    assert!(report["readiness"].is_object());
}

#[test]
fn mcp_server_calls_window_tools() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    let list_response = mcp.call_tool(4, "x11_list_windows", serde_json::json!({}));
    assert_eq!(list_response["id"], 4);
    assert_eq!(list_response["result"]["isError"], false);
    let list_text = list_response["result"]["content"][0]["text"]
        .as_str()
        .expect("list text");
    let list_report: serde_json::Value =
        serde_json::from_str(list_text).expect("list windows json text");
    assert_eq!(list_report["project"], "codex-computer-use-x11");
    assert_eq!(list_report["backend"], "x11-ewmh");
    assert!(list_report["windows"].is_array());

    let focused_response = mcp.call_tool(5, "x11_focused_window", serde_json::json!({}));
    assert_eq!(focused_response["id"], 5);
    assert_eq!(focused_response["result"]["isError"], false);
    let focused_text = focused_response["result"]["content"][0]["text"]
        .as_str()
        .expect("focused text");
    let focused_report: serde_json::Value =
        serde_json::from_str(focused_text).expect("focused window json text");
    assert_eq!(focused_report["project"], "codex-computer-use-x11");
    assert!(focused_report.get("focused_window").is_some());

    let focus_response = mcp.call_tool(
        6,
        "x11_focus_window",
        serde_json::json!({
            "window_id": "0x1"
        }),
    );
    assert_eq!(focus_response["id"], 6);
    assert_eq!(focus_response["result"]["isError"], true);
    let focus_text = focus_response["result"]["content"][0]["text"]
        .as_str()
        .expect("focus text");
    let focus_report: serde_json::Value =
        serde_json::from_str(focus_text).expect("focus window json text");
    assert_eq!(focus_report["project"], "codex-computer-use-x11");
    assert_eq!(focus_report["success"], false);
    assert_eq!(focus_report["error_code"], "WindowNotFound");
}

#[test]
fn mcp_server_calls_x11_get_app_state_without_screenshot() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    let response = mcp.call_tool(
        31,
        "x11_get_app_state",
        serde_json::json!({
            "include_screenshot": false
        }),
    );
    assert_eq!(response["id"], 31);
    assert_eq!(response["result"]["isError"], false);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("app-state text");
    let report: serde_json::Value = serde_json::from_str(text).expect("app-state json");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["backend"], "x11-ewmh");
    assert_eq!(report["screenshot"], serde_json::Value::Null);
    assert_eq!(report["screenshot_error"], serde_json::Value::Null);
    assert!(report["diagnostics"].is_object());
}

#[cfg(unix)]
#[test]
fn mcp_get_app_state_writes_screenshot_path_without_inline_blob() {
    let dir = temp_dir("mcp-app-state-screenshot-path");
    write_mcp_window_commands(&dir);
    write_mcp_fake_gdbus_screenshot(&dir);
    let screenshot = dir.join("mcp-app-state.png");

    let mut mcp = McpHarness::start_with_fake_x11(&dir);
    mcp.initialize();
    let response = mcp.call_tool(
        33,
        "x11_get_app_state",
        serde_json::json!({
            "window_id": "0x2",
            "screenshot_output": screenshot.to_str().unwrap()
        }),
    );
    assert_eq!(response["id"], 33);
    assert_eq!(response["result"]["isError"], false);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("app-state text");
    assert!(!text.contains("data:image"));
    assert!(!text.contains(";base64,"));
    let report: serde_json::Value = serde_json::from_str(text).expect("app-state json");
    assert_eq!(report["screenshot"]["path"], screenshot.to_str().unwrap());
    assert_eq!(report["screenshot"]["data_url"], serde_json::Value::Null);
    assert!(screenshot.is_file());
    assert!(std::fs::metadata(&screenshot).unwrap().len() > 0);

    let inline = dir.join("mcp-app-state-inline.png");
    let inline_response = mcp.call_tool(
        34,
        "x11_get_app_state",
        serde_json::json!({
            "window_id": "0x2",
            "screenshot_output": inline.to_str().unwrap(),
            "inline_screenshot": true
        }),
    );
    assert_eq!(inline_response["result"]["isError"], false);
    let inline_text = inline_response["result"]["content"][0]["text"]
        .as_str()
        .expect("inline app-state text");
    let inline_report: serde_json::Value = serde_json::from_str(inline_text).expect("inline json");
    assert!(inline_report["screenshot"]["data_url"]
        .as_str()
        .unwrap_or_default()
        .starts_with("data:image/png;base64,"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn mcp_get_app_state_rejects_malformed_window_id() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    let response = mcp.call_tool(
        32,
        "x11_get_app_state",
        serde_json::json!({
            "window_id": {"bad": true},
            "include_screenshot": false
        }),
    );
    assert_eq!(response["id"], 32);
    assert_eq!(response["result"]["isError"], true);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("error text");
    assert!(text.contains("window_id"));
}

#[test]
fn mcp_focus_window_requires_window_id() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    let response = mcp.call_tool(7, "x11_focus_window", serde_json::json!({}));
    assert_eq!(response["id"], 7);
    assert_eq!(response["result"]["isError"], true);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("error text");
    assert!(
        text.contains("window_id"),
        "error should mention window_id: {text}"
    );
}

#[test]
fn mcp_targeted_input_tools_refuse_missing_target() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    let response = mcp.call_tool(
        8,
        "x11_type_text",
        serde_json::json!({
            "text": "hello"
        }),
    );
    assert_eq!(response["id"], 8);
    assert_eq!(response["result"]["isError"], true);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("type_text report text");
    let report: serde_json::Value = serde_json::from_str(text).expect("valid targeted input json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "MissingTarget");

    let response = mcp.call_tool(
        9,
        "x11_press_key",
        serde_json::json!({
            "key": "Enter"
        }),
    );
    assert_eq!(response["id"], 9);
    assert_eq!(response["result"]["isError"], true);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("press_key report text");
    let report: serde_json::Value = serde_json::from_str(text).expect("valid targeted input json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "MissingTarget");

    let response = mcp.call_tool(
        10,
        "x11_click",
        serde_json::json!({
            "x": 50,
            "y": 60
        }),
    );
    assert_eq!(response["id"], 10);
    assert_eq!(response["result"]["isError"], true);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("click report text");
    let report: serde_json::Value = serde_json::from_str(text).expect("valid pointer report json");
    assert_eq!(report["success"], false);
    assert_eq!(report["input_sent"], false);
    assert_eq!(report["error_code"], "MissingTarget");
}

#[test]
fn mcp_accessibility_tree_requires_window_id() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    let response = mcp.call_tool(30, "x11_accessibility_tree", serde_json::json!({}));
    assert_eq!(response["id"], 30);
    assert_eq!(response["result"]["isError"], true);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("error text");
    assert!(
        text.contains("window_id"),
        "error should mention window_id: {text}"
    );
}

#[test]
fn mcp_accessibility_tree_returns_report_failures_as_json_tool_errors() {
    let mut mcp = McpHarness::start();
    mcp.initialize();

    let response = mcp.call_tool(
        31,
        "x11_accessibility_tree",
        serde_json::json!({"window_id": "0x1"}),
    );
    assert_eq!(response["id"], 31);
    assert_eq!(response["result"]["isError"], true);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("report text");
    let report: serde_json::Value = serde_json::from_str(text).expect("valid accessibility json");
    assert_eq!(report["project"], "codex-computer-use-x11");
    assert_eq!(report["success"], false);
    assert_eq!(report["error_code"], "WindowNotFound");
    assert_eq!(report["tree"].as_array().unwrap().len(), 0);
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
fn write_mcp_window_commands(dir: &std::path::Path) {
    write_executable(
        &dir.join("wmctrl"),
        "#!/bin/sh\nif [ \"$1\" = \"-lpGx\" ]; then\ncat <<'OUT'\n0x00000002 0 222 10 20 800 600 app.App testhost Editor\nOUT\nelif [ \"$1\" = \"-ia\" ]; then\n  exit 0\nelse\n  echo \"unexpected wmctrl args: $*\" >&2\n  exit 2\nfi\n",
    );
    write_executable(
        &dir.join("xprop"),
        "#!/bin/sh\nif [ \"$1\" = \"-root\" ]; then\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\nelse\n  echo \"unexpected xprop args: $*\" >&2\n  exit 2\nfi\n",
    );
}

#[cfg(unix)]
fn write_mcp_tiny_png(path: &std::path::Path) {
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
fn write_mcp_fake_gdbus_screenshot(dir: &std::path::Path) {
    let fixture = dir.join("fixture.png");
    write_mcp_tiny_png(&fixture);
    write_executable(
        &dir.join("gdbus"),
        &format!(
            "#!/bin/sh\nargs=\"$*\"\nif echo \"$args\" | grep -q 'org.gnome.Shell.Screenshot.Screenshot'; then\n  for last do :; done\n  cp '{}' \"$last\"\n  printf \"(true, '%s')\\n\" \"$last\"\n  exit 0\nfi\nif echo \"$args\" | grep -q 'org.gnome.Shell.Screenshot'; then\n  echo 'method Screenshot'\n  echo 'method ScreenshotArea'\n  exit 0\nfi\nif echo \"$args\" | grep -q 'org.freedesktop.portal.Desktop'; then\n  echo 'method Screenshot'\n  exit 0\nfi\nexit 0\n",
            fixture.display()
        ),
    );
}

#[cfg(unix)]
fn write_mcp_doctor_commands(dir: &std::path::Path, expected_display: Option<&str>) {
    write_executable(&dir.join("wmctrl"), "#!/bin/sh\nexit 0\n");
    let guard = expected_display
        .map(|display| format!("if [ \"${{DISPLAY:-}}\" != \"{display}\" ]; then echo \"unexpected DISPLAY=${{DISPLAY:-}}\" >&2; exit 3; fi\n"))
        .unwrap_or_default();
    write_executable(
        &dir.join("xprop"),
        &format!(
            "#!/bin/sh\n{guard}if [ \"$1\" = \"-root\" ]; then\n  echo '_NET_SUPPORTING_WM_CHECK(WINDOW): window id # 0x1234'\n  echo '_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2'\n  exit 0\nfi\necho \"unexpected xprop args: $*\" >&2\nexit 2\n"
        ),
    );
}

#[cfg(unix)]
fn doctor_report_from_text_response(response: &serde_json::Value) -> (String, serde_json::Value) {
    assert_eq!(response["result"]["isError"], false);
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("doctor report text")
        .to_string();
    let report: serde_json::Value = serde_json::from_str(&text).expect("doctor report json");
    (text, report)
}

#[cfg(unix)]
#[test]
fn mcp_server_hydrates_desktop_env_for_doctor() {
    let dir = temp_dir("mcp-env-hydrates");
    write_mcp_doctor_commands(&dir, Some(":99"));
    let fixture = dir.join("desktop.env");
    std::fs::write(
        &fixture,
        "DISPLAY=:99\nXDG_SESSION_TYPE=x11\nXDG_CURRENT_DESKTOP=Cinnamon\nXAUTHORITY=/tmp/secret-xauth\nDBUS_SESSION_BUS_ADDRESS=unix:path=/tmp/secret-bus\n",
    )
    .expect("write desktop env fixture");

    let mut mcp = McpHarness::start_with_desktop_env_fixture(&dir, &fixture);
    mcp.initialize();

    let response = mcp.call_tool(60, "x11_doctor", serde_json::json!({}));
    let (text, report) = doctor_report_from_text_response(&response);
    assert_eq!(report["environment"]["display_present"], true);
    assert_eq!(report["environment"]["session_type"], "x11");
    assert_eq!(report["environment"]["desktop"], "Cinnamon");
    assert_eq!(report["x11"]["ewmh"]["can_query_windows"], true);
    assert!(!text.contains("secret-xauth"));
    assert!(!text.contains("secret-bus"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn mcp_server_preserves_explicit_display_during_hydration() {
    let dir = temp_dir("mcp-env-preserve-display");
    write_mcp_doctor_commands(&dir, Some(":99"));
    let fixture = dir.join("desktop.env");
    std::fs::write(
        &fixture,
        "DISPLAY=:0\nXDG_SESSION_TYPE=x11\nXDG_CURRENT_DESKTOP=Cinnamon\n",
    )
    .expect("write desktop env fixture");

    let mut mcp = McpHarness::start_with_explicit_display_and_desktop_env_fixture(&dir, &fixture);
    mcp.initialize();

    let response = mcp.call_tool(61, "x11_doctor", serde_json::json!({}));
    let (_text, report) = doctor_report_from_text_response(&response);
    assert_eq!(report["environment"]["display_present"], true);
    assert_eq!(report["environment"]["session_type"], "x11");
    assert_eq!(report["environment"]["desktop"], "Cinnamon");
    assert_eq!(report["x11"]["ewmh"]["can_query_windows"], true);

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn mcp_server_tracks_target_window_context() {
    let dir = temp_dir("mcp-target-context");
    write_mcp_window_commands(&dir);
    let mut mcp = McpHarness::start_with_fake_x11(&dir);
    mcp.initialize();

    mcp.send(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 40,
        "method": "tools/list",
        "params": {}
    }));
    let list = mcp.read_response();
    let names: Vec<_> = list["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"x11_target_window"));
    assert!(names.contains(&"x11_release_window"));
    assert!(names.contains(&"x11_target_context"));

    let target = mcp.call_tool(
        41,
        "x11_target_window",
        serde_json::json!({"window_id": "0x2", "group": "data-entry", "color": "green", "overlay": false}),
    );
    assert_eq!(target["result"]["isError"], false);
    let target_json: serde_json::Value =
        serde_json::from_str(target["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(target_json["target"]["window"]["window_id"], 2);

    let context = mcp.call_tool(42, "x11_target_context", serde_json::json!({}));
    assert_eq!(context["result"]["isError"], false);
    let context_json: serde_json::Value =
        serde_json::from_str(context["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(context_json["state"]["groups"][0]["group_id"], "data-entry");
    assert_eq!(
        context_json["state"]["groups"][0]["windows"][0]["window"]["window_id"],
        2
    );

    let release = mcp.call_tool(
        43,
        "x11_release_window",
        serde_json::json!({"window_id": "0x2"}),
    );
    assert_eq!(release["result"]["isError"], false);
    let release_json: serde_json::Value =
        serde_json::from_str(release["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(release_json["released_count"], 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn mcp_target_window_rejects_malformed_arguments() {
    let dir = temp_dir("mcp-target-malformed");
    write_mcp_window_commands(&dir);
    let mut mcp = McpHarness::start_with_fake_x11(&dir);
    mcp.initialize();

    let response = mcp.call_tool(
        50,
        "x11_target_window",
        serde_json::json!({"window_id": {"bad": true}, "color": "green"}),
    );
    assert_eq!(response["result"]["isError"], true);
    assert!(response["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or_default()
        .contains("window_id must be a string or number"));

    let context = mcp.call_tool(51, "x11_target_context", serde_json::json!({}));
    let context_json: serde_json::Value =
        serde_json::from_str(context["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert!(context_json["state"]["groups"]
        .as_array()
        .unwrap()
        .is_empty());

    let response = mcp.call_tool(
        52,
        "x11_target_window",
        serde_json::json!({"window_id": "0x2", "color": "chartreuse"}),
    );
    assert_eq!(response["result"]["isError"], true);
    assert!(response["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or_default()
        .contains("invalid color"));

    let _ = std::fs::remove_dir_all(&dir);
}
