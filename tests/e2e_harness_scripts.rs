use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn temp_dir(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-e2e-{name}-{}-{nanos}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn run_plugin_smoke(args: &[&str], codex_home: Option<&Path>, log_dir: &Path) -> Output {
    let mut command = Command::new(repo_root().join("scripts/e2e/codex-plugin-smoke.sh"));
    command
        .current_dir(repo_root())
        .args(args)
        .args(["--log-dir", log_dir.to_str().unwrap()])
        .env(
            "CODEX_X11_PLUGIN_BINARY",
            env!("CARGO_BIN_EXE_codex-computer-use-x11"),
        );
    if let Some(codex_home) = codex_home {
        command.env("CODEX_HOME", codex_home);
    }
    command.output().expect("run plugin smoke")
}

#[cfg(unix)]
fn make_executable(path: &Path, content: &str) {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    let mut file = std::fs::File::create(path).expect("create executable");
    file.write_all(content.as_bytes())
        .expect("write executable");
    let mut permissions = file.metadata().unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).expect("chmod executable");
}

#[cfg(unix)]
fn install_stale_six_tool_plugin(codex_home: &Path) {
    let plugin_dir =
        codex_home.join("plugins/cache/codex-computer-use-x11/codex-computer-use-x11/0.0.0");
    std::fs::create_dir_all(plugin_dir.join(".codex-plugin")).unwrap();
    std::fs::create_dir_all(plugin_dir.join("bin")).unwrap();
    std::fs::create_dir_all(plugin_dir.join("assets")).unwrap();
    std::fs::copy(
        repo_root().join("assets/app-icon.png"),
        plugin_dir.join("assets/app-icon.png"),
    )
    .expect("copy fixture icon");
    std::fs::write(
        plugin_dir.join(".codex-plugin/plugin.json"),
        serde_json::json!({
            "name": "codex-computer-use-x11",
            "version": "0.0.0",
            "homepage": "https://github.com/AlekseiSeleznev/codex-computer-use-x11",
            "author": {"name": "AlekseiSeleznev", "url": "https://github.com/AlekseiSeleznev"},
            "mcpServers": "./.mcp.json",
            "interface": {
                "displayName": "X11 Computer Use",
                "developerName": "AlekseiSeleznev",
                "websiteURL": "https://github.com/AlekseiSeleznev/codex-computer-use-x11",
                "logo": "./assets/app-icon.png",
                "longDescription": "Provides standalone x11_* readiness diagnostics, window listing/focus, keyboard input, pointer actions, accessibility tree, app state, and target-window context tools.",
                "defaultPrompt": [
                    "Inspect app state with x11_get_app_state",
                    "Save a target context with x11_target_window"
                ]
            }
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        plugin_dir.join(".mcp.json"),
        serde_json::json!({
            "mcpServers": {
                "codex-computer-use-x11": {
                    "command": "./bin/codex-computer-use-x11",
                    "args": ["mcp"],
                    "cwd": "."
                }
            }
        })
        .to_string(),
    )
    .unwrap();
    make_executable(
        &plugin_dir.join("bin/codex-computer-use-x11"),
        r#"#!/usr/bin/env python3
import json, sys
TOOLS = ["x11_doctor", "x11_list_windows", "x11_focused_window", "x11_focus_window", "x11_type_text", "x11_press_key"]
for line in sys.stdin:
    msg = json.loads(line)
    if msg.get("method") == "initialize":
        print(json.dumps({"jsonrpc":"2.0","id":msg["id"],"result":{"protocolVersion":"2025-06-18","capabilities":{"tools":{"listChanged":False}},"serverInfo":{"name":"codex-computer-use-x11","version":"0.0.0"}}}), flush=True)
    elif msg.get("method") == "tools/list":
        print(json.dumps({"jsonrpc":"2.0","id":msg["id"],"result":{"tools":[{"name": name, "description": name, "inputSchema": {"type":"object"}} for name in TOOLS]}}), flush=True)
"#,
    );
    let latest = plugin_dir.parent().unwrap().join("latest");
    std::os::unix::fs::symlink("0.0.0", latest).unwrap();

    let marketplace_root = codex_home.join("plugins/marketplaces/codex-computer-use-x11");
    std::fs::create_dir_all(marketplace_root.join(".agents/plugins")).unwrap();
    std::fs::create_dir_all(marketplace_root.join("plugins")).unwrap();
    std::fs::write(
        marketplace_root.join(".agents/plugins/marketplace.json"),
        serde_json::json!({
            "name": "codex-computer-use-x11",
            "interface": {"displayName": "X11 Computer Use"},
            "plugins": [{
                "name": "codex-computer-use-x11",
                "source": {"source": "local", "path": "./plugins/codex-computer-use-x11"},
                "policy": {"installation": "AVAILABLE", "authentication": "ON_INSTALL"},
                "category": "Productivity"
            }]
        })
        .to_string(),
    )
    .unwrap();
    std::os::unix::fs::symlink(
        codex_home.join("plugins/cache/codex-computer-use-x11/codex-computer-use-x11/latest"),
        marketplace_root.join("plugins/codex-computer-use-x11"),
    )
    .unwrap();
}

fn evidence_files(log_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(log_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let evidence = path.join("evidence.json");
                if evidence.is_file() {
                    files.push(evidence);
                }
            } else if path.file_name().is_some_and(|name| name == "evidence.json") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

#[test]
fn plugin_smoke_fails_clearly_when_plugin_missing() {
    let temp = temp_dir("plugin-missing");
    let codex_home = temp.join("codex-home");
    let log_dir = temp.join("logs");
    std::fs::create_dir_all(&codex_home).expect("create empty codex home");

    let output = run_plugin_smoke(
        &["--fake", "--codex-home", codex_home.to_str().unwrap()],
        Some(&codex_home),
        &log_dir,
    );

    assert!(
        !output.status.success(),
        "missing plugin should fail instead of auto-installing into supplied CODEX_HOME"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing standalone plugin installation")
            || stderr.contains("codex-computer-use-x11"),
        "stderr should identify missing plugin installation, got:\n{stderr}"
    );

    let evidence_files = evidence_files(&log_dir);
    assert!(
        !evidence_files.is_empty(),
        "failure should retain evidence under {}",
        log_dir.display()
    );
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_files[0]).unwrap()).unwrap();
    assert_eq!(evidence["delivery_path"], "standalone_plugin");
    assert_eq!(evidence["mode"], "fake");
    assert!(
        evidence["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|check| check["name"] == "marketplace_metadata" && check["status"] == "fail"),
        "evidence should record failed marketplace metadata check: {evidence}"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_smoke_fake_auto_install_validates_marketplace_metadata() {
    let temp = temp_dir("plugin-auto-install");
    let log_dir = temp.join("logs");

    let output = run_plugin_smoke(&["--fake"], None, &log_dir);
    assert!(
        output.status.success(),
        "fake plugin smoke should auto-install into an isolated CODEX_HOME\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let evidence_files = evidence_files(&log_dir);
    assert_eq!(evidence_files.len(), 1, "expected one evidence file");
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_files[0]).unwrap()).unwrap();
    assert!(
        evidence["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|check| check["name"] == "marketplace_metadata" && check["status"] == "pass"),
        "metadata check should pass: {evidence}"
    );
    assert_eq!(
        evidence["capability_matrix"]["install/rollback"]["standalone_plugin"]["status"],
        "pass"
    );
    assert!(
        evidence["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|check| check["name"] == "fresh_install_doctor_uninstall" && check["status"] == "pass"),
        "fake smoke should record fresh install → doctor/MCP → uninstall restoration check: {evidence}"
    );
    let metadata_check = evidence["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|check| check["name"] == "marketplace_metadata")
        .expect("marketplace metadata check");
    let metadata = &metadata_check["metadata"];
    assert_eq!(metadata["display_name"], "X11 Computer Use");
    assert_eq!(metadata["developer_name"], "AlekseiSeleznev");
    assert_eq!(
        metadata["website_url"],
        "https://github.com/AlekseiSeleznev/codex-computer-use-x11"
    );
    assert_eq!(metadata["logo"], "./assets/app-icon.png");
    assert_eq!(metadata["has_privacy_policy"], false);
    assert_eq!(metadata["has_terms_of_service"], false);
    assert_eq!(metadata["marketplace_display_name"], "X11 Computer Use");

    let _ = std::fs::remove_dir_all(&temp);
}

#[cfg(unix)]
#[test]
fn plugin_smoke_rejects_stale_six_tool_install() {
    let temp = temp_dir("plugin-stale-six-tools");
    let codex_home = temp.join("codex-home");
    let log_dir = temp.join("logs");
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    install_stale_six_tool_plugin(&codex_home);

    let output = run_plugin_smoke(
        &[
            "--fake",
            "--codex-home",
            codex_home.to_str().unwrap(),
            "--no-auto-install",
        ],
        Some(&codex_home),
        &log_dir,
    );

    assert!(
        !output.status.success(),
        "stale six-tool install should fail validation"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing expected MCP tools") || stderr.contains("x11_get_app_state"),
        "stderr should identify missing current tools, got:\n{stderr}"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_smoke_fake_validates_mcp_tool_list() {
    let temp = temp_dir("plugin-tools");
    let log_dir = temp.join("logs");

    let output = run_plugin_smoke(&["--fake"], None, &log_dir);
    assert!(
        output.status.success(),
        "fake plugin smoke failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence_files = evidence_files(&log_dir);
    assert_eq!(evidence_files.len(), 1);
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_files[0]).unwrap()).unwrap();
    let tools_check = evidence["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|check| check["name"] == "mcp_tools_list")
        .cloned()
        .expect("mcp_tools_list check exists");
    assert_eq!(tools_check["status"], "pass");
    let tools = tools_check["tools"].as_array().expect("tools array");
    assert!(tools.iter().any(|name| name == "x11_get_app_state"));
    assert!(tools.iter().any(|name| name == "x11_click"));
    assert!(!tools.iter().any(|name| name == "get_app_state"));
    assert!(!tools.iter().any(|name| name == "click"));

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_smoke_fake_exercises_window_routes_without_real_desktop() {
    let temp = temp_dir("plugin-window-routes");
    let log_dir = temp.join("logs");

    let output = run_plugin_smoke(&["--fake"], None, &log_dir);
    assert!(
        output.status.success(),
        "fake plugin smoke failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence_files = evidence_files(&log_dir);
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_files[0]).unwrap()).unwrap();
    let window_check = evidence["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|check| check["name"] == "fake_window_routes")
        .cloned()
        .expect("fake_window_routes check exists");
    assert_eq!(window_check["status"], "pass");
    assert_eq!(window_check["window_id"], 2);
    assert_eq!(
        evidence["capability_matrix"]["doctor/capabilities"]["standalone_plugin"]["status"],
        "pass"
    );
    assert_eq!(
        evidence["capability_matrix"]["window listing/focus"]["standalone_plugin"]["status"],
        "pass"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_smoke_fake_records_app_state_and_input_matrix() {
    let temp = temp_dir("plugin-app-state-input");
    let log_dir = temp.join("logs");

    let output = run_plugin_smoke(&["--fake"], None, &log_dir);
    assert!(
        output.status.success(),
        "fake plugin smoke failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence_files = evidence_files(&log_dir);
    let evidence_path = &evidence_files[0];
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(evidence_path).unwrap()).unwrap();
    assert!(
        evidence["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|check| check["name"] == "fake_app_state_and_input" && check["status"] == "pass"),
        "input/app-state check should pass: {evidence}"
    );
    let app_state_check = evidence["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|check| check["name"] == "fake_app_state_and_input")
        .expect("fake_app_state_and_input check");
    assert!(
        app_state_check["app_state_summary"]["layers"]
            .as_array()
            .is_some_and(|layers| !layers.is_empty()),
        "app-state summary should preserve diagnostics.layers: {app_state_check}"
    );
    assert_eq!(
        app_state_check["app_state_summary"]["screenshot"]["data_url"],
        serde_json::Value::Null,
        "app-state evidence summary must omit screenshot data_url"
    );
    if app_state_check["app_state_summary"]["screenshot"]["status"] == "present" {
        assert!(
            app_state_check["app_state_summary"]["screenshot"]["path"]
                .as_str()
                .is_some_and(|path| path.ends_with("app-state.png")),
            "app-state evidence summary should retain screenshot path metadata: {app_state_check}"
        );
    }
    assert!(
        evidence["checks"].as_array().unwrap().iter().any(|check| {
            check["name"] == "gtk_atspi_fixture"
                && check["status"] == "pass"
                && check["fixture"] == "fake-gtk"
                && check["expected_accessible_control"] == "Apply"
        }),
        "fake harness should include an AT-SPI-positive GTK fixture row: {evidence}"
    );
    assert!(
        evidence["checks"].as_array().unwrap().iter().any(|check| {
            check["name"] == "keyboard_unicode_value"
                && check["status"] == "pass"
                && check["requested_text"] == "Привет"
                && check["observed_value"] == "Привет"
                && check["route"] == "xdotool-unicode-keysyms"
        }),
        "fake harness should prove exact Cyrillic value evidence with route diagnostics: {evidence}"
    );
    assert!(
        evidence["checks"].as_array().unwrap().iter().any(|check| {
            check["name"] == "overlay_lifecycle"
                && check["status"] == "pass"
                && check["overlay_shown"] == true
                && check["release_hid_overlay"] == true
        }),
        "fake harness should prove overlay shown/release lifecycle evidence: {evidence}"
    );
    for group in [
        "get_app_state",
        "keyboard input",
        "pointer input",
        "screenshot",
        "AT-SPI",
    ] {
        let status = evidence["capability_matrix"][group]["standalone_plugin"]["status"]
            .as_str()
            .unwrap();
        assert!(
            status == "pass" || status == "degraded",
            "{group} status={status}"
        );
    }
    let run_dir = evidence_path.parent().unwrap();
    let fake_xdotool = std::fs::read_to_string(run_dir.join("fake-xdotool.log"))
        .expect("fake xdotool log should be written");
    assert!(
        fake_xdotool.contains("type"),
        "fake xdotool log={fake_xdotool}"
    );
    assert!(
        fake_xdotool.contains("click"),
        "fake xdotool log={fake_xdotool}"
    );
    assert!(
        fake_xdotool.contains("mousemove"),
        "fake xdotool log={fake_xdotool}"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

fn complete_matrix() -> serde_json::Value {
    let groups = [
        "doctor/capabilities",
        "window listing/focus",
        "get_app_state",
        "keyboard input",
        "pointer input",
        "screenshot",
        "AT-SPI",
        "install/rollback",
    ];
    let mut matrix = serde_json::Map::new();
    for group in groups {
        matrix.insert(
            group.to_string(),
            serde_json::json!({
                "standalone_plugin": {"status": "pass", "evidence": ["test"]},
                "source_overlay": {"status": "degraded", "reason": "not evaluated in fixture", "reason_category": "not_evaluated"}
            }),
        );
    }
    serde_json::Value::Object(matrix)
}

#[test]
fn matrix_validator_rejects_missing_evidence() {
    let temp = temp_dir("matrix-validator");
    let incomplete = temp.join("incomplete.json");
    std::fs::write(
        &incomplete,
        serde_json::json!({
            "capability_matrix": {
                "doctor/capabilities": {
                    "standalone_plugin": {"status": "pass", "evidence": ["x11_doctor"]}
                }
            }
        })
        .to_string(),
    )
    .unwrap();

    let output = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--evidence",
            incomplete.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run matrix validator");
    assert!(!output.status.success(), "incomplete matrix should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing evidence"), "stderr={stderr}");

    let pass_without_evidence = temp.join("pass-without-evidence.json");
    let mut pass_without_evidence_matrix = complete_matrix();
    pass_without_evidence_matrix["doctor/capabilities"]["standalone_plugin"] =
        serde_json::json!({"status": "pass"});
    std::fs::write(
        &pass_without_evidence,
        serde_json::json!({"capability_matrix": pass_without_evidence_matrix}).to_string(),
    )
    .unwrap();
    let no_evidence = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--evidence",
            pass_without_evidence.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run matrix validator on pass without evidence");
    assert!(
        !no_evidence.status.success(),
        "pass rows without concrete evidence should fail"
    );
    let stderr = String::from_utf8_lossy(&no_evidence.stderr);
    assert!(
        stderr.contains("pass evidence"),
        "stderr should identify missing pass evidence, got: {stderr}"
    );

    let complete = temp.join("complete.json");
    std::fs::write(
        &complete,
        serde_json::json!({"capability_matrix": complete_matrix()}).to_string(),
    )
    .unwrap();
    let ok = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args(["validate-matrix", "--evidence", complete.to_str().unwrap()])
        .current_dir(repo_root())
        .output()
        .expect("run matrix validator on complete evidence");
    assert!(
        ok.status.success(),
        "complete matrix should pass\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&ok.stdout),
        String::from_utf8_lossy(&ok.stderr)
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn industrial_matrix_validator_rejects_missing_fixture_setup_and_code_failure() {
    let temp = temp_dir("industrial-matrix-validator");
    let mut missing_fixture = complete_matrix();
    missing_fixture["keyboard input"]["standalone_plugin"] = serde_json::json!({
        "status": "degraded",
        "reason": "safe text fixture was not started",
        "reason_category": "missing_fixture_setup",
        "evidence": ["live plugin metadata/tools only"]
    });
    let missing_fixture_path = temp.join("missing-fixture.json");
    std::fs::write(
        &missing_fixture_path,
        serde_json::json!({"capability_matrix": missing_fixture}).to_string(),
    )
    .unwrap();

    let missing_fixture_output = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--industrial",
            "--evidence",
            missing_fixture_path.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run industrial matrix validator");
    assert!(
        !missing_fixture_output.status.success(),
        "missing fixture setup should fail industrial validation"
    );
    let stderr = String::from_utf8_lossy(&missing_fixture_output.stderr);
    assert!(
        stderr.contains("missing_fixture_setup") && stderr.contains("keyboard input"),
        "stderr should identify missing fixture setup, got: {stderr}"
    );

    let mut code_failure = complete_matrix();
    code_failure["screenshot"]["standalone_plugin"] = serde_json::json!({
        "status": "fail",
        "reason": "screenshot crop output was not PNG",
        "reason_category": "code_failure",
        "evidence": ["live-mcp/screenshot-crop.log"]
    });
    let code_failure_path = temp.join("code-failure.json");
    std::fs::write(
        &code_failure_path,
        serde_json::json!({"capability_matrix": code_failure}).to_string(),
    )
    .unwrap();
    let code_failure_output = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--industrial",
            "--evidence",
            code_failure_path.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run industrial matrix validator");
    assert!(
        !code_failure_output.status.success(),
        "code failure should fail industrial validation"
    );
    let stderr = String::from_utf8_lossy(&code_failure_output.stderr);
    assert!(
        stderr.contains("code_failure") && stderr.contains("screenshot"),
        "stderr should identify code failure, got: {stderr}"
    );

    let mut environment_degraded = complete_matrix();
    environment_degraded["AT-SPI"]["standalone_plugin"] = serde_json::json!({
        "status": "degraded",
        "reason": "atspi_gtk_bridge_disabled_by_environment: GTK accessibility bridge disabled by inherited NO_AT_BRIDGE",
        "reason_category": "environment_limitation",
        "evidence": ["doctor.accessibility.diagnostic_state=atspi_gtk_bridge_disabled_by_environment", "NO_AT_BRIDGE=1"]
    });
    let environment_path = temp.join("environment-degraded.json");
    std::fs::write(
        &environment_path,
        serde_json::json!({"capability_matrix": environment_degraded}).to_string(),
    )
    .unwrap();
    let environment_output = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--industrial",
            "--evidence",
            environment_path.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run industrial matrix validator");
    assert!(
        environment_output.status.success(),
        "environment limitation with evidence should pass industrial validation\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&environment_output.stdout),
        String::from_utf8_lossy(&environment_output.stderr)
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn matrix_validator_requires_canonical_reason_categories_for_non_pass_rows() {
    let temp = temp_dir("matrix-reason-categories");
    let mut missing_category = complete_matrix();
    missing_category["screenshot"]["standalone_plugin"] = serde_json::json!({
        "status": "degraded",
        "reason": "fake gdbus fixture is not available",
        "evidence": ["x11_get_app_state screenshot layer"]
    });
    let missing_path = temp.join("missing-category.json");
    std::fs::write(
        &missing_path,
        serde_json::json!({"capability_matrix": missing_category}).to_string(),
    )
    .unwrap();
    let missing = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--evidence",
            missing_path.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run matrix validator on missing reason_category");
    assert!(!missing.status.success());
    let stderr = String::from_utf8_lossy(&missing.stderr);
    assert!(
        stderr.contains("reason_category") && stderr.contains("screenshot"),
        "stderr should name missing reason_category and row, got: {stderr}"
    );

    let mut fake_limitation = complete_matrix();
    fake_limitation["screenshot"]["standalone_plugin"] = serde_json::json!({
        "status": "degraded",
        "reason": "fake gdbus screenshot fixture is unavailable; real live crop integrity remains required",
        "reason_category": "expected_fake_fixture_limitation",
        "evidence": ["x11_get_app_state screenshot layer"]
    });
    let fake_limitation_path = temp.join("fake-limitation.json");
    std::fs::write(
        &fake_limitation_path,
        serde_json::json!({"capability_matrix": fake_limitation}).to_string(),
    )
    .unwrap();
    let accepted = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--evidence",
            fake_limitation_path.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run matrix validator on expected fake limitation");
    assert!(
        accepted.status.success(),
        "expected fake limitation should validate outside industrial mode\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&accepted.stdout),
        String::from_utf8_lossy(&accepted.stderr)
    );

    let mut unsupported = complete_matrix();
    unsupported["doctor/capabilities"]["source_overlay"] = serde_json::json!({
        "status": "degraded",
        "reason": "Wayland runtime path is outside this X11-only baseline",
        "reason_category": "unsupported_out_of_scope",
        "evidence": ["doctor readiness unsupported_out_of_scope"]
    });
    let unsupported_path = temp.join("unsupported.json");
    std::fs::write(
        &unsupported_path,
        serde_json::json!({"capability_matrix": unsupported}).to_string(),
    )
    .unwrap();
    let unsupported_ok = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--evidence",
            unsupported_path.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run matrix validator on unsupported out-of-scope row");
    assert!(unsupported_ok.status.success());

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn app_state_summary_requires_diagnostics_layers_and_sanitizes_screenshot() {
    let temp = temp_dir("app-state-summary");
    let missing_layers = temp.join("missing-layers.json");
    std::fs::write(
        &missing_layers,
        serde_json::json!({
            "backend": "x11-ewmh",
            "layers": [{"name": "window", "ok": true, "detail": "wrong location"}],
            "screenshot": {
                "mime_type": "image/png",
                "data_url": "data:image/png;base64,AAECAwQ=",
                "path": "target/e2e-logs/app-state/test.png",
                "size_bytes": 68,
                "source": "fixture",
                "width": 1,
                "height": 1
            }
        })
        .to_string(),
    )
    .unwrap();

    let rejected = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "summarize-app-state",
            "--input",
            missing_layers.to_str().unwrap(),
            "--output",
            temp.join("missing-summary.json").to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run app-state summary on missing diagnostics.layers");
    assert!(
        !rejected.status.success(),
        "top-level layers must not be accepted as app-state diagnostics"
    );
    let stderr = String::from_utf8_lossy(&rejected.stderr);
    assert!(
        stderr.contains("diagnostics.layers"),
        "stderr should name missing diagnostics.layers, got: {stderr}"
    );

    let valid = temp.join("valid-app-state.json");
    std::fs::write(
        &valid,
        serde_json::json!({
            "backend": "x11-ewmh",
            "diagnostics": {
                "layers": [
                    {"name": "window", "ok": true, "detail": "target resolved"},
                    {"name": "screenshot", "ok": true, "detail": "captured"}
                ]
            },
            "screenshot": {
                "mime_type": "image/png",
                "data_url": "data:image/png;base64,AAECAwQ=",
                "path": "target/e2e-logs/app-state/test.png",
                "size_bytes": 68,
                "source": "fixture",
                "width": 1,
                "height": 1
            }
        })
        .to_string(),
    )
    .unwrap();
    let summary = temp.join("summary.json");
    let ok = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "summarize-app-state",
            "--input",
            valid.to_str().unwrap(),
            "--output",
            summary.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run app-state summary on valid input");
    assert!(
        ok.status.success(),
        "valid summary should pass\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&ok.stdout),
        String::from_utf8_lossy(&ok.stderr)
    );
    let summary: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&summary).unwrap()).unwrap();
    assert_eq!(summary["backend"], "x11-ewmh");
    assert_eq!(summary["layers"][0]["name"], "window");
    assert_eq!(summary["screenshot"]["mime_type"], "image/png");
    assert_eq!(summary["screenshot"]["source"], "fixture");
    assert_eq!(summary["screenshot"]["width"], 1);
    assert_eq!(summary["screenshot"]["height"], 1);
    assert_eq!(
        summary["screenshot"]["path"],
        "target/e2e-logs/app-state/test.png"
    );
    assert_eq!(summary["screenshot"]["size_bytes"], 68);
    assert_eq!(summary["screenshot"]["data_url"], serde_json::Value::Null);

    let degraded = temp.join("degraded-app-state.json");
    std::fs::write(
        &degraded,
        serde_json::json!({
            "backend": "x11-ewmh",
            "diagnostics": {
                "layers": [
                    {"name": "window", "ok": true, "detail": "target resolved"},
                    {"name": "screenshot", "ok": false, "detail": "fake gdbus unavailable"},
                    {"name": "accessibility", "ok": false, "detail": "AT-SPI bus reachable but tree extraction unavailable"}
                ]
            },
            "screenshot_error": "fake gdbus unavailable"
        })
        .to_string(),
    )
    .unwrap();
    let degraded_summary = temp.join("degraded-summary.json");
    let degraded_ok = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "summarize-app-state",
            "--input",
            degraded.to_str().unwrap(),
            "--output",
            degraded_summary.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run app-state summary on degraded input");
    assert!(degraded_ok.status.success());
    let degraded_summary: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&degraded_summary).unwrap()).unwrap();
    assert_eq!(
        degraded_summary["screenshot"]["reason_category"],
        "expected_fake_fixture_limitation"
    );
    assert_eq!(
        degraded_summary["layers"][2]["reason_category"],
        "environment_limitation"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

fn run_source_overlay_smoke(args: &[&str], log_dir: &Path) -> Output {
    Command::new(repo_root().join("scripts/e2e/codex-source-overlay-smoke.sh"))
        .current_dir(repo_root())
        .args(args)
        .args(["--log-dir", log_dir.to_str().unwrap()])
        .output()
        .expect("run source overlay smoke")
}

#[test]
fn source_overlay_smoke_fake_installs_and_uninstalls_fixture() {
    let temp = temp_dir("source-overlay-fake");
    let log_dir = temp.join("logs");

    let output = run_source_overlay_smoke(&["--fake"], &log_dir);
    assert!(
        output.status.success(),
        "fake source-overlay smoke should pass\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence_files = evidence_files(&log_dir);
    assert_eq!(evidence_files.len(), 1);
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_files[0]).unwrap()).unwrap();
    assert!(
        evidence["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|check| check["name"] == "source_overlay_reversible" && check["status"] == "pass"),
        "source overlay reversible check should pass: {evidence}"
    );
    assert_eq!(
        evidence["capability_matrix"]["install/rollback"]["source_overlay"]["status"],
        "pass"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn controlled_fixture_manager_creates_metadata_and_cleanup_records() {
    let temp = temp_dir("controlled-fixture-manager");
    let log_dir = temp.join("logs");

    let output = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args(["fixture-self-test", "--log-dir", log_dir.to_str().unwrap()])
        .current_dir(repo_root())
        .env("NO_AT_BRIDGE", "1")
        .output()
        .expect("run fixture self-test");
    assert!(
        output.status.success(),
        "fixture self-test should pass\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let evidence_path = log_dir.join("fixture-self-test/evidence.json");
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_path).unwrap()).unwrap();
    assert_eq!(evidence["status"], "pass");
    for role in ["tk", "gtk"] {
        let fixture = &evidence["fixtures"][role];
        assert_eq!(fixture["role"], role);
        assert!(
            fixture["title"]
                .as_str()
                .unwrap()
                .contains("x11-safe-fixture"),
            "fixture title should be run-scoped and controlled with neutral identity: {fixture}"
        );
        assert!(
            !fixture["title"].as_str().unwrap().contains("Codex")
                && !fixture["title"].as_str().unwrap().contains("codex"),
            "fixture title should not contain Codex/codex filter text: {fixture}"
        );
        assert!(
            !fixture["wm_class"].as_str().unwrap().contains("Codex"),
            "fixture wm_class should not contain Codex filter text: {fixture}"
        );
        assert!(
            fixture["ready_file"]
                .as_str()
                .is_some_and(|path| path.contains("ready.json")),
            "fixture should expose a readiness file: {fixture}"
        );
        assert!(
            fixture["pid"].as_i64().is_some_and(|pid| pid > 0),
            "fixture should record process id: {fixture}"
        );
    }
    let gtk = &evidence["fixtures"]["gtk"];
    assert_eq!(
        gtk["env"]["GTK_MODULES"], "gail:atk-bridge",
        "GTK fixture should record the bridge module hint: {gtk}"
    );
    assert_eq!(
        gtk["env"]["NO_AT_BRIDGE"],
        serde_json::Value::Null,
        "GTK fixture child process must remove inherited NO_AT_BRIDGE: {gtk}"
    );
    assert_eq!(
        gtk["env"]["NO_AT_BRIDGE_PRESENT"], false,
        "GTK fixture metadata should make NO_AT_BRIDGE absence explicit: {gtk}"
    );
    assert!(
        evidence["cleanup"]
            .as_array()
            .unwrap()
            .iter()
            .all(|entry| entry["terminated"] == true),
        "cleanup should terminate every fixture: {evidence}"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn controlled_fixture_manager_cleans_up_after_startup_failure() {
    let temp = temp_dir("controlled-fixture-startup-failure");
    let log_dir = temp.join("logs");

    let output = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "fixture-self-test",
            "--log-dir",
            log_dir.to_str().unwrap(),
            "--fail-role",
            "gtk",
        ])
        .current_dir(repo_root())
        .output()
        .expect("run failing fixture self-test");
    assert!(
        !output.status.success(),
        "startup failure scenario should fail but preserve cleanup evidence"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("fixture startup failed"), "stderr={stderr}");

    let evidence_path = log_dir.join("fixture-self-test/evidence.json");
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_path).unwrap()).unwrap();
    assert_eq!(evidence["status"], "fail");
    assert_eq!(evidence["failed_role"], "gtk");
    assert!(
        evidence["cleanup"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["role"] == "tk" && entry["terminated"] == true),
        "already-started fixtures must be cleaned up: {evidence}"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn controlled_fixture_manager_cleans_up_after_tool_failure() {
    let temp = temp_dir("controlled-fixture-tool-failure");
    let log_dir = temp.join("logs");

    let output = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "fixture-self-test",
            "--log-dir",
            log_dir.to_str().unwrap(),
            "--fail-after-start",
        ])
        .current_dir(repo_root())
        .output()
        .expect("run tool-failure fixture self-test");
    assert!(
        !output.status.success(),
        "tool failure scenario should fail but preserve cleanup evidence"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("tool call failed"), "stderr={stderr}");

    let evidence_path = log_dir.join("fixture-self-test/evidence.json");
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_path).unwrap()).unwrap();
    assert_eq!(evidence["status"], "fail");
    assert_eq!(evidence["target_window_released"], true);
    assert_eq!(evidence["overlay_hidden"], true);
    assert_eq!(
        evidence["cleanup"].as_array().unwrap().len(),
        2,
        "both fixtures should have cleanup records: {evidence}"
    );
    assert!(
        evidence["cleanup"]
            .as_array()
            .unwrap()
            .iter()
            .all(|entry| entry["terminated"] == true),
        "tool-failure cleanup should terminate every fixture: {evidence}"
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn safe_fixture_selection_blocks_unsafe_targets_before_tool_calls() {
    let temp = temp_dir("safe-fixture-selection");
    let log_dir = temp.join("logs");

    for (scenario, category) in [
        ("missing", "missing_fixture_setup"),
        ("duplicate", "unsafe_target_selection"),
        ("stale", "unsafe_target_selection"),
        ("overlay-helper", "unsafe_target_selection"),
        ("user-app", "unsafe_target_selection"),
    ] {
        let scenario_log = log_dir.join(scenario);
        let output = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
            .args([
                "selection-self-test",
                "--scenario",
                scenario,
                "--log-dir",
                scenario_log.to_str().unwrap(),
            ])
            .current_dir(repo_root())
            .output()
            .expect("run selection self-test");
        assert!(
            output.status.success(),
            "selection self-test {scenario} should preserve evidence without unsafe tool calls\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let evidence: serde_json::Value = serde_json::from_slice(
            &std::fs::read(scenario_log.join("selection-self-test/evidence.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(evidence["scenario"], scenario);
        assert_eq!(evidence["selected"], serde_json::Value::Null);
        assert_eq!(evidence["reason_category"], category);
        assert_eq!(evidence["tool_calls_attempted"], false);
    }

    let ok_log = log_dir.join("ok");
    let ok = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "selection-self-test",
            "--scenario",
            "ok",
            "--log-dir",
            ok_log.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("run ok selection self-test");
    assert!(ok.status.success(), "ok scenario should pass");
    let evidence: serde_json::Value = serde_json::from_slice(
        &std::fs::read(ok_log.join("selection-self-test/evidence.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(evidence["reason_category"], "fixture_pass");
    assert_eq!(evidence["selected"]["window_id"], "0x2");
    assert_eq!(evidence["tool_calls_attempted"], true);

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_smoke_live_industrial_fake_fixtures_records_fixture_backed_rows() {
    let temp = temp_dir("plugin-live-industrial-fake-fixtures");
    let log_dir = temp.join("logs");

    let output = run_plugin_smoke(
        &["--live", "--industrial", "--fake-live-fixtures"],
        None,
        &log_dir,
    );
    assert!(
        output.status.success(),
        "industrial fake-live fixture smoke should pass\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence_files = evidence_files(&log_dir);
    assert_eq!(evidence_files.len(), 1, "expected one evidence file");
    let evidence_path = &evidence_files[0];
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(evidence_path).unwrap()).unwrap();
    assert_eq!(evidence["mode"], "live");
    assert_eq!(evidence["acceptance_profile"], "industrial");
    assert!(
        !std::fs::read_to_string(evidence_path)
            .unwrap()
            .contains("data:image"),
        "ordinary evidence must not embed screenshot data URLs"
    );
    for group in [
        "doctor/capabilities",
        "window listing/focus",
        "get_app_state",
        "keyboard input",
        "pointer input",
        "screenshot",
        "AT-SPI",
    ] {
        assert_eq!(
            evidence["capability_matrix"][group]["standalone_plugin"]["status"], "pass",
            "{group} should be fixture-backed pass: {evidence}"
        );
        assert_eq!(
            evidence["capability_matrix"][group]["standalone_plugin"]["reason_category"],
            "fixture_pass",
            "{group} should carry fixture_pass category: {evidence}"
        );
    }
    assert!(
        evidence["checks"].as_array().unwrap().iter().any(|check| {
            check["name"] == "live_fixture_cleanup"
                && check["status"] == "pass"
                && check["fixture_processes_stopped"] == true
                && check["cleanup"].as_array().unwrap().len() == 2
        }),
        "fixture cleanup check should be recorded: {evidence}"
    );
    assert!(
        evidence["checks"].as_array().unwrap().iter().any(|check| {
            check["name"] == "overlay_lifecycle"
                && check["status"] == "pass"
                && check["target_context_cleared"] == true
                && check["stale_target_context"] == false
        }),
        "overlay/target release cleanup should prove no stale target context: {evidence}"
    );
    assert!(
        evidence["checks"].as_array().unwrap().iter().any(|check| {
            check["name"] == "gtk_atspi_fixture"
                && check["status"] == "pass"
                && check["env"]["GTK_MODULES"] == "gail:atk-bridge"
                && check["env"]["NO_AT_BRIDGE"] == serde_json::Value::Null
                && check["env"]["NO_AT_BRIDGE_PRESENT"] == false
        }),
        "GTK fixture env metadata should be recorded: {evidence}"
    );

    let validate = Command::new(repo_root().join("scripts/e2e/codex-x11-e2e.py"))
        .args([
            "validate-matrix",
            "--industrial",
            "--evidence",
            evidence_path.to_str().unwrap(),
        ])
        .current_dir(repo_root())
        .output()
        .expect("validate industrial fake-live evidence");
    assert!(
        validate.status.success(),
        "industrial validation should pass for fixture-backed evidence\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&validate.stdout),
        String::from_utf8_lossy(&validate.stderr)
    );

    let fake_xdotool =
        std::fs::read_to_string(evidence_path.parent().unwrap().join("fake-xdotool.log"))
            .expect("fake xdotool log should be written");
    for expected in ["type", "key", "mousemove", "click"] {
        assert!(
            fake_xdotool.contains(expected),
            "fake xdotool log={fake_xdotool}"
        );
    }

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_smoke_live_metadata_only_records_missing_fixture_setup() {
    let temp = temp_dir("plugin-live-metadata-only");
    let log_dir = temp.join("logs");

    let output = run_plugin_smoke(&["--live"], None, &log_dir);
    assert!(
        output.status.success(),
        "live metadata-only smoke should pass as diagnostic evidence\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let evidence_files = evidence_files(&log_dir);
    assert_eq!(evidence_files.len(), 1, "expected one evidence file");
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence_files[0]).unwrap()).unwrap();
    for group in [
        "window listing/focus",
        "get_app_state",
        "keyboard input",
        "pointer input",
        "screenshot",
        "AT-SPI",
    ] {
        assert_eq!(
            evidence["capability_matrix"][group]["standalone_plugin"]["status"], "degraded",
            "{group} should be degraded without controlled fixtures: {evidence}"
        );
        assert_eq!(
            evidence["capability_matrix"][group]["standalone_plugin"]["reason_category"],
            "missing_fixture_setup",
            "{group} should identify missing fixture setup: {evidence}"
        );
        assert!(
            evidence["capability_matrix"][group]["standalone_plugin"]["reason"]
                .as_str()
                .unwrap_or_default()
                .contains("not safe to test input against real user applications"),
            "{group} reason should warn against real-app fallback: {evidence}"
        );
    }

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn e2e_harness_docs_cover_live_manual_steps() {
    let docs = std::fs::read_to_string(repo_root().join("docs/e2e-harness.md"))
        .expect("docs/e2e-harness.md should exist");
    for required in [
        "scripts/e2e/codex-plugin-smoke.sh --fake",
        "scripts/e2e/codex-source-overlay-smoke.sh --fake",
        "scripts/e2e/codex-source-overlay-smoke.sh --live",
        "target/e2e-logs",
        "doctor/capabilities",
        "window listing/focus",
        "get_app_state",
        "keyboard input",
        "pointer input",
        "screenshot",
        "AT-SPI",
        "install/rollback",
        "activate_window",
        "CODEX_HOME",
        "CODEX_DESKTOP_LINUX_FULL_PATH",
    ] {
        assert!(docs.contains(required), "docs should mention {required}");
    }
}

#[test]
fn docs_cover_industrial_live_verification_and_safe_evidence() {
    let e2e = std::fs::read_to_string(repo_root().join("docs/e2e-harness.md"))
        .expect("docs/e2e-harness.md should exist");
    for required in [
        "--industrial",
        "missing_fixture_setup",
        "environment_limitation",
        "code_failure",
        "controlled fixture",
        "no input",
        "data URLs",
    ] {
        assert!(e2e.contains(required), "e2e docs should mention {required}");
    }

    let troubleshooting = std::fs::read_to_string(repo_root().join("docs/troubleshooting.md"))
        .expect("docs/troubleshooting.md should exist");
    for required in [
        "ScreenshotOutputMissing",
        "ScreenshotOutputEmpty",
        "ScreenshotOutputInvalidFormat",
        "GTK_MODULES=gail:atk-bridge",
        "missing_fixture_setup",
    ] {
        assert!(
            troubleshooting.contains(required),
            "troubleshooting docs should mention {required}"
        );
    }

    let release = std::fs::read_to_string(repo_root().join("docs/release-checklist.md"))
        .expect("docs/release-checklist.md should exist");
    for required in [
        "codex-plugin-smoke.sh --live --industrial",
        "validate-matrix --industrial",
        "controlled fixtures",
        "target/e2e-logs/<run-id>",
    ] {
        assert!(
            release.contains(required),
            "release docs should mention {required}"
        );
    }
}
