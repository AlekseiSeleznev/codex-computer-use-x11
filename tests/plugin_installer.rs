use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn temp_dir(name: &str) -> PathBuf {
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

fn run_script(script: &str, args: &[&str], codex_home: &Path) -> Output {
    Command::new(format!("scripts/{script}"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CODEX_HOME", codex_home)
        .env("CODEX_CONFIG_FILE", codex_home.join("config.toml"))
        .env(
            "CODEX_X11_PLUGIN_BINARY",
            env!("CARGO_BIN_EXE_codex-computer-use-x11"),
        )
        .output()
        .unwrap_or_else(|err| panic!("run scripts/{script}: {err}"))
}

fn run_script_with_path(script: &str, args: &[&str], codex_home: &Path, path: &str) -> Output {
    Command::new(format!("scripts/{script}"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CODEX_HOME", codex_home)
        .env("CODEX_CONFIG_FILE", codex_home.join("config.toml"))
        .env(
            "CODEX_X11_PLUGIN_BINARY",
            env!("CARGO_BIN_EXE_codex-computer-use-x11"),
        )
        .env("PATH", path)
        .output()
        .unwrap_or_else(|err| panic!("run scripts/{script}: {err}"))
}

#[cfg(unix)]
fn write_executable(path: &Path, content: &str) {
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
fn path_with_fake_commands(dir: &Path) -> String {
    format!(
        "{}:{}",
        dir.display(),
        std::env::var("PATH").unwrap_or_default()
    )
}

fn assert_success(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

#[cfg(unix)]
#[test]
fn plugin_installer_records_accessibility_manifest_before_state() {
    let temp = temp_dir("plugin-installer-accessibility-manifest");
    let codex_home = temp.join("codex");
    let fake_bin = temp.join("bin");
    std::fs::create_dir_all(&fake_bin).expect("create fake bin");
    write_executable(
        &fake_bin.join("gsettings"),
        "#!/bin/sh\nif [ \"$1\" = \"get\" ] && [ \"$2\" = \"org.gnome.desktop.interface\" ] && [ \"$3\" = \"toolkit-accessibility\" ]; then\n  echo false\n  exit 0\nfi\necho unexpected gsettings \"$@\" >&2\nexit 2\n",
    );
    write_executable(
        &fake_bin.join("systemctl"),
        "#!/bin/sh\nif [ \"$1\" = \"--user\" ] && [ \"$2\" = \"show-environment\" ]; then\n  echo NO_AT_BRIDGE=1\n  exit 0\nfi\necho unexpected systemctl \"$@\" >&2\nexit 2\n",
    );
    write_executable(
        &fake_bin.join("dbus-update-activation-environment"),
        "#!/bin/sh\necho dbus-update-activation-environment should not run during dry-run >&2\nexit 2\n",
    );

    let output = run_script_with_path(
        "install-codex-plugin.sh",
        &["--activate-accessibility", "--dry-run", "--report-json"],
        &codex_home,
        &path_with_fake_commands(&fake_bin),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let _ = std::fs::remove_dir_all(&temp);

    assert_success(&output);
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|err| panic!("parse report JSON: {err}\n{stdout}"));
    assert_eq!(report["operation"], "install-codex-plugin");
    assert_eq!(report["dry_run"], true);

    let entries = report["entries"].as_array().expect("entries array");
    for required in [
        "plugin_cache",
        "plugin_marketplace",
        "plugin_config",
        "org.gnome.desktop.interface toolkit-accessibility",
        "NO_AT_BRIDGE",
        "GTK_MODULES",
        "QT_ACCESSIBILITY",
    ] {
        assert!(
            entries
                .iter()
                .any(|entry| entry["path_or_key"].as_str() == Some(required)),
            "missing manifest entry {required}: {report}"
        );
    }

    let toolkit = entries
        .iter()
        .find(|entry| {
            entry["path_or_key"].as_str()
                == Some("org.gnome.desktop.interface toolkit-accessibility")
        })
        .expect("toolkit-accessibility entry");
    assert_eq!(toolkit["surface"], "gsettings");
    assert_eq!(toolkit["before"]["value"], false);
    assert_eq!(toolkit["after"]["value"], true);
    assert_eq!(toolkit["installer_changed"], true);
    assert_eq!(toolkit["completed"], false);

    let no_at_bridge = entries
        .iter()
        .find(|entry| entry["path_or_key"].as_str() == Some("NO_AT_BRIDGE"))
        .expect("NO_AT_BRIDGE entry");
    assert_eq!(no_at_bridge["surface"], "activation_env");
    assert_eq!(no_at_bridge["before"]["value"], "1");
    assert_eq!(no_at_bridge["after"]["present"], false);
    assert_eq!(no_at_bridge["installer_changed"], true);
    assert_eq!(no_at_bridge["completed"], false);

    assert!(
        !codex_home.exists(),
        "accessibility dry-run report must not create CODEX_HOME"
    );
}

#[cfg(unix)]
#[test]
fn plugin_installer_writes_accessibility_manifest_and_applies_setup() {
    let temp = temp_dir("plugin-installer-accessibility-write");
    let codex_home = temp.join("codex");
    let fake_bin = temp.join("bin");
    let log = temp.join("commands.log");
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    std::fs::create_dir_all(&fake_bin).expect("create fake bin");
    write_executable(
        &fake_bin.join("gsettings"),
        &format!(
            "#!/bin/sh\necho gsettings \"$@\" >> '{}'\nif [ \"$1\" = \"get\" ]; then echo false; exit 0; fi\nexit 0\n",
            log.display()
        ),
    );
    write_executable(
        &fake_bin.join("systemctl"),
        &format!(
            "#!/bin/sh\necho systemctl \"$@\" >> '{}'\nif [ \"$1\" = \"--user\" ] && [ \"$2\" = \"show-environment\" ]; then\n  echo NO_AT_BRIDGE=1\n  exit 0\nfi\nexit 0\n",
            log.display()
        ),
    );
    write_executable(
        &fake_bin.join("dbus-update-activation-environment"),
        &format!(
            "#!/bin/sh\necho dbus-update-activation-environment \"$@\" >> '{}'\nexit 0\n",
            log.display()
        ),
    );

    let output = run_script_with_path(
        "install-codex-plugin.sh",
        &["--activate-accessibility"],
        &codex_home,
        &path_with_fake_commands(&fake_bin),
    );
    assert_success(&output);

    let manifest_path = codex_home.join("state/codex-computer-use-x11/install-manifest.json");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&manifest_path).expect("read manifest"))
            .expect("parse manifest");
    let command_log = std::fs::read_to_string(&log).expect("read command log");
    let _ = std::fs::remove_dir_all(&temp);

    assert_eq!(manifest["operation"], "install-codex-plugin");
    assert_eq!(manifest["dry_run"], false);
    let entries = manifest["entries"].as_array().expect("entries array");
    assert!(
        entries.iter().any(|entry| entry["completed"] == true),
        "write-mode manifest should record completed entries: {manifest}"
    );
    assert!(
        command_log
            .contains("gsettings set org.gnome.desktop.interface toolkit-accessibility true"),
        "installer should enable toolkit accessibility: {command_log}"
    );
    assert!(
        command_log.contains("systemctl --user unset-environment NO_AT_BRIDGE"),
        "installer should neutralize disabling NO_AT_BRIDGE=1: {command_log}"
    );
    assert!(
        command_log.contains("systemctl --user set-environment GTK_MODULES=gail:atk-bridge"),
        "installer should set GTK bridge activation env: {command_log}"
    );
    assert!(
        command_log.contains("systemctl --user set-environment QT_ACCESSIBILITY=1"),
        "installer should set Qt accessibility activation env: {command_log}"
    );
}

#[cfg(unix)]
#[test]
fn plugin_uninstaller_restores_manifest_owned_accessibility_state() {
    let temp = temp_dir("plugin-uninstaller-accessibility-restore");
    let codex_home = temp.join("codex");
    let fake_bin = temp.join("bin");
    let log = temp.join("commands.log");
    std::fs::create_dir_all(codex_home.join("state/codex-computer-use-x11"))
        .expect("create state dir");
    std::fs::create_dir_all(&fake_bin).expect("create fake bin");
    std::fs::write(
        codex_home.join("state/codex-computer-use-x11/install-manifest.json"),
        serde_json::json!({
            "schema_version": 1,
            "operation": "install-codex-plugin",
            "entries": [
                {
                    "surface": "gsettings",
                    "path_or_key": "org.gnome.desktop.interface toolkit-accessibility",
                    "before": {"present": true, "value": false},
                    "after": {"present": true, "value": true},
                    "installer_changed": true,
                    "completed": true
                },
                {
                    "surface": "activation_env",
                    "path_or_key": "NO_AT_BRIDGE",
                    "before": {"present": true, "value": "1"},
                    "after": {"present": false},
                    "installer_changed": true,
                    "completed": true
                },
                {
                    "surface": "activation_env",
                    "path_or_key": "GTK_MODULES",
                    "before": {"present": true, "value": "gail:atk-bridge"},
                    "after": {"present": true, "value": "gail:atk-bridge"},
                    "installer_changed": false,
                    "completed": true
                },
                {
                    "surface": "activation_env",
                    "path_or_key": "QT_ACCESSIBILITY",
                    "before": {"present": false, "value": null},
                    "after": {"present": true, "value": "1"},
                    "installer_changed": true,
                    "completed": true
                }
            ]
        })
        .to_string()
            + "\n",
    )
    .expect("write manifest");

    write_executable(
        &fake_bin.join("gsettings"),
        &format!(
            "#!/bin/sh\necho gsettings \"$@\" >> '{}'\nif [ \"$1\" = \"get\" ]; then echo true; exit 0; fi\nexit 0\n",
            log.display()
        ),
    );
    write_executable(
        &fake_bin.join("systemctl"),
        &format!(
            "#!/bin/sh\necho systemctl \"$@\" >> '{}'\nif [ \"$1\" = \"--user\" ] && [ \"$2\" = \"show-environment\" ]; then\n  echo GTK_MODULES=gail:atk-bridge\n  echo QT_ACCESSIBILITY=1\n  exit 0\nfi\nexit 0\n",
            log.display()
        ),
    );
    write_executable(
        &fake_bin.join("dbus-update-activation-environment"),
        &format!(
            "#!/bin/sh\necho dbus-update-activation-environment \"$@\" >> '{}'\nexit 0\n",
            log.display()
        ),
    );

    let output = run_script_with_path(
        "uninstall-codex-plugin.sh",
        &["--report-json"],
        &codex_home,
        &path_with_fake_commands(&fake_bin),
    );
    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|err| panic!("parse report JSON: {err}\n{stdout}"));
    let command_log = std::fs::read_to_string(&log).expect("read command log");
    let _ = std::fs::remove_dir_all(&temp);

    assert_eq!(report["operation"], "uninstall-codex-plugin");
    assert_eq!(report["blockers"].as_array().unwrap().len(), 0);
    assert!(
        command_log
            .contains("gsettings set org.gnome.desktop.interface toolkit-accessibility false"),
        "toolkit accessibility should be restored to false: {command_log}"
    );
    assert!(
        command_log.contains("systemctl --user set-environment NO_AT_BRIDGE=1"),
        "NO_AT_BRIDGE=1 should be restored: {command_log}"
    );
    assert!(
        command_log.contains("systemctl --user unset-environment QT_ACCESSIBILITY"),
        "QT_ACCESSIBILITY should be restored to absence: {command_log}"
    );
    assert!(
        !command_log.contains("set-environment GTK_MODULES"),
        "installer_changed=false values must not be restored: {command_log}"
    );
}

#[cfg(unix)]
#[test]
fn plugin_uninstaller_reports_accessibility_drift_blocker() {
    let temp = temp_dir("plugin-uninstaller-accessibility-drift");
    let codex_home = temp.join("codex");
    let fake_bin = temp.join("bin");
    let log = temp.join("commands.log");
    std::fs::create_dir_all(codex_home.join("state/codex-computer-use-x11"))
        .expect("create state dir");
    std::fs::create_dir_all(&fake_bin).expect("create fake bin");
    std::fs::write(
        codex_home.join("state/codex-computer-use-x11/install-manifest.json"),
        serde_json::json!({
            "schema_version": 1,
            "operation": "install-codex-plugin",
            "entries": [
                {
                    "surface": "gsettings",
                    "path_or_key": "org.gnome.desktop.interface toolkit-accessibility",
                    "before": {"present": true, "value": false},
                    "after": {"present": true, "value": true},
                    "installer_changed": true,
                    "completed": true
                }
            ]
        })
        .to_string()
            + "\n",
    )
    .expect("write manifest");
    write_executable(
        &fake_bin.join("gsettings"),
        &format!(
            "#!/bin/sh\necho gsettings \"$@\" >> '{}'\nif [ \"$1\" = \"get\" ]; then echo false; exit 0; fi\nexit 0\n",
            log.display()
        ),
    );
    write_executable(
        &fake_bin.join("systemctl"),
        "#!/bin/sh\nif [ \"$1\" = \"--user\" ] && [ \"$2\" = \"show-environment\" ]; then exit 0; fi\nexit 0\n",
    );

    let output = run_script_with_path(
        "uninstall-codex-plugin.sh",
        &["--report-json"],
        &codex_home,
        &path_with_fake_commands(&fake_bin),
    );
    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|err| panic!("parse report JSON: {err}\n{stdout}"));
    let command_log = std::fs::read_to_string(&log).expect("read command log");
    let _ = std::fs::remove_dir_all(&temp);

    let blockers = report["blockers"].as_array().expect("blockers array");
    assert_eq!(blockers.len(), 1, "expected one drift blocker: {report}");
    assert_eq!(blockers[0]["reason"], "drift");
    assert_eq!(
        blockers[0]["path_or_key"],
        "org.gnome.desktop.interface toolkit-accessibility"
    );
    assert!(
        !command_log.contains("gsettings set"),
        "drifted state must not be blindly overwritten: {command_log}"
    );
}

#[test]
fn plugin_installer_dry_run_writes_nothing() {
    let temp = temp_dir("plugin-installer-dry-run");
    let codex_home = temp.join("codex");

    let output = run_script("install-codex-plugin.sh", &["--dry-run"], &codex_home);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let _ = std::fs::remove_dir_all(&temp);

    assert_success(&output);
    assert!(
        stdout.contains("DRY RUN"),
        "stdout should mention dry run: {stdout}"
    );
    assert!(
        stdout.contains("codex-computer-use-x11"),
        "stdout should mention owned namespace: {stdout}"
    );
    assert!(
        !codex_home.exists(),
        "dry-run must not create CODEX_HOME or plugin files"
    );
}

#[test]
fn plugin_installer_creates_owned_bundle_and_config() {
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    let temp = temp_dir("plugin-installer-install");
    let codex_home = temp.join("codex");
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    std::fs::write(
        codex_home.join("config.toml"),
        "[plugins.\"unrelated@market\"]\nenabled = true\n",
    )
    .expect("write unrelated config");

    let output = run_script("install-codex-plugin.sh", &[], &codex_home);
    assert_success(&output);

    let version = env!("CARGO_PKG_VERSION");
    let cache_root = codex_home.join("plugins/cache/codex-computer-use-x11/codex-computer-use-x11");
    let plugin_dir = cache_root.join(version);
    let plugin_json = plugin_dir.join(".codex-plugin/plugin.json");
    let mcp_json = plugin_dir.join(".mcp.json");
    let bin = plugin_dir.join("bin/codex-computer-use-x11");
    let icon = plugin_dir.join("assets/app-icon.png");
    assert!(plugin_json.is_file(), "plugin manifest should exist");
    assert!(mcp_json.is_file(), "mcp manifest should exist");
    assert!(bin.is_file(), "binary should be copied");
    assert!(icon.is_file(), "project-owned plugin icon should be copied");
    #[cfg(unix)]
    assert!(
        bin.metadata().unwrap().permissions().mode() & 0o111 != 0,
        "binary should be executable"
    );

    let plugin_manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&plugin_json).unwrap()).unwrap();
    assert_eq!(plugin_manifest["name"], "codex-computer-use-x11");
    assert_eq!(plugin_manifest["version"], version);
    assert_eq!(plugin_manifest["mcpServers"], "./.mcp.json");
    assert_eq!(
        plugin_manifest["homepage"],
        "https://github.com/AlekseiSeleznev/codex-computer-use-x11"
    );
    assert_eq!(plugin_manifest["author"]["name"], "AlekseiSeleznev");
    assert_eq!(
        plugin_manifest["author"]["url"],
        "https://github.com/AlekseiSeleznev"
    );
    assert_eq!(
        plugin_manifest["interface"]["displayName"],
        "X11 Computer Use"
    );
    assert_eq!(
        plugin_manifest["interface"]["developerName"],
        "AlekseiSeleznev"
    );
    assert_eq!(
        plugin_manifest["interface"]["websiteURL"],
        "https://github.com/AlekseiSeleznev/codex-computer-use-x11"
    );
    assert_eq!(
        plugin_manifest["interface"]["logo"],
        "./assets/app-icon.png"
    );
    assert!(
        plugin_manifest["interface"]
            .get("privacyPolicyURL")
            .is_none(),
        "project manifest should not invent a privacy policy link"
    );
    assert!(
        plugin_manifest["interface"]
            .get("termsOfServiceURL")
            .is_none(),
        "project manifest should not invent a terms link"
    );
    let manifest_text = std::fs::read_to_string(&plugin_json).unwrap();
    assert!(
        !manifest_text.contains("AlekseiSelin"),
        "manifest should not contain stale repository owner"
    );
    let long_description = plugin_manifest["interface"]["longDescription"]
        .as_str()
        .expect("longDescription should be a string");
    assert!(
        !long_description.contains("AlekseiSelin"),
        "longDescription should not mention stale repository owner"
    );
    for expected in [
        "readiness diagnostics",
        "pointer actions",
        "accessibility tree",
        "app state",
        "target-window context",
    ] {
        assert!(
            long_description.contains(expected),
            "longDescription should mention {expected:?}: {long_description}"
        );
    }
    let default_prompts = plugin_manifest["interface"]["defaultPrompt"]
        .as_array()
        .expect("defaultPrompt should be an array");
    assert!(
        default_prompts.iter().any(|prompt| prompt
            .as_str()
            .is_some_and(|prompt| prompt.contains("x11_get_app_state"))),
        "defaultPrompt should guide users to app-state inspection: {default_prompts:?}"
    );
    assert!(
        default_prompts.iter().any(|prompt| prompt
            .as_str()
            .is_some_and(|prompt| prompt.contains("x11_target_window"))),
        "defaultPrompt should guide users to target-window context: {default_prompts:?}"
    );

    let mcp_manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&mcp_json).unwrap()).unwrap();
    assert_eq!(
        mcp_manifest["mcpServers"]["codex-computer-use-x11"]["command"],
        "./bin/codex-computer-use-x11"
    );
    assert_eq!(
        mcp_manifest["mcpServers"]["codex-computer-use-x11"]["args"],
        serde_json::json!(["mcp"])
    );

    let latest_target = std::fs::read_link(cache_root.join("latest")).expect("latest symlink");
    assert_eq!(latest_target, PathBuf::from(version));

    let marketplace_root = codex_home.join("plugins/marketplaces/codex-computer-use-x11");
    let marketplace_file = marketplace_root.join(".agents/plugins/marketplace.json");
    let marketplace: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&marketplace_file).unwrap()).unwrap();
    assert_eq!(marketplace["name"], "codex-computer-use-x11");
    assert_eq!(marketplace["interface"]["displayName"], "X11 Computer Use");
    assert_eq!(marketplace["plugins"].as_array().unwrap().len(), 1);
    assert_eq!(marketplace["plugins"][0]["name"], "codex-computer-use-x11");
    let marketplace_link =
        std::fs::read_link(marketplace_root.join("plugins/codex-computer-use-x11"))
            .expect("marketplace plugin link");
    assert_eq!(marketplace_link, cache_root.join("latest"));

    let config = std::fs::read_to_string(codex_home.join("config.toml")).unwrap();
    assert!(config.contains("[plugins.\"unrelated@market\"]"));
    assert!(config.contains("[plugins.\"codex-computer-use-x11@codex-computer-use-x11\"]"));
    assert!(config.contains("[marketplaces.codex-computer-use-x11]"));
    assert!(config.contains("source_type = \"local\""));
    assert!(config.contains(&format!("source = \"{}\"", marketplace_root.display())));

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_installer_repeated_install_is_idempotent() {
    let temp = temp_dir("plugin-installer-idempotent");
    let codex_home = temp.join("codex");
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    std::fs::write(
        codex_home.join("config.toml"),
        "# keep me\n\n[plugins.\"unrelated@market\"]\nenabled = true\n",
    )
    .expect("write unrelated config");

    assert_success(&run_script("install-codex-plugin.sh", &[], &codex_home));
    let latest =
        codex_home.join("plugins/cache/codex-computer-use-x11/codex-computer-use-x11/latest");
    std::fs::remove_file(&latest).expect("remove first latest symlink");
    std::fs::create_dir_all(&latest).expect("simulate stale non-symlink latest path");
    assert_success(&run_script("install-codex-plugin.sh", &[], &codex_home));

    let config = std::fs::read_to_string(codex_home.join("config.toml")).unwrap();
    assert_eq!(
        config
            .matches("[plugins.\"codex-computer-use-x11@codex-computer-use-x11\"]")
            .count(),
        1
    );
    assert_eq!(
        config
            .matches("[marketplaces.codex-computer-use-x11]")
            .count(),
        1
    );
    assert_eq!(config.matches("[plugins.\"unrelated@market\"]").count(), 1);
    assert!(config.contains("# keep me"));

    let marketplace_file = codex_home
        .join("plugins/marketplaces/codex-computer-use-x11/.agents/plugins/marketplace.json");
    let marketplace: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&marketplace_file).unwrap()).unwrap();
    assert_eq!(marketplace["plugins"].as_array().unwrap().len(), 1);

    assert_eq!(
        std::fs::read_link(latest).expect("latest symlink after repeated install"),
        PathBuf::from(env!("CARGO_PKG_VERSION"))
    );

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_uninstaller_removes_only_owned_files() {
    let temp = temp_dir("plugin-uninstaller-owned-only");
    let codex_home = temp.join("codex");
    std::fs::create_dir_all(codex_home.join("plugins/cache/openai-bundled/computer-use"))
        .expect("create unrelated cache");
    std::fs::write(
        codex_home.join("plugins/cache/openai-bundled/computer-use/sentinel"),
        "keep",
    )
    .expect("write unrelated sentinel");
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    std::fs::write(
        codex_home.join("config.toml"),
        "[plugins.\"unrelated@market\"]\nenabled = true\n",
    )
    .expect("write unrelated config");

    assert_success(&run_script("install-codex-plugin.sh", &[], &codex_home));
    assert_success(&run_script("uninstall-codex-plugin.sh", &[], &codex_home));

    assert!(
        !codex_home
            .join("plugins/cache/codex-computer-use-x11")
            .exists(),
        "owned cache namespace should be removed"
    );
    assert!(
        !codex_home
            .join("plugins/marketplaces/codex-computer-use-x11")
            .exists(),
        "owned marketplace should be removed"
    );
    assert!(
        codex_home
            .join("plugins/cache/openai-bundled/computer-use/sentinel")
            .is_file(),
        "unrelated bundled cache sentinel must remain"
    );
    let config = std::fs::read_to_string(codex_home.join("config.toml")).unwrap();
    assert!(config.contains("[plugins.\"unrelated@market\"]"));
    assert!(!config.contains("codex-computer-use-x11@codex-computer-use-x11"));
    assert!(!config.contains("[marketplaces.codex-computer-use-x11]"));

    let _ = std::fs::remove_dir_all(&temp);
}

#[test]
fn plugin_uninstaller_dry_run_and_absent_are_safe() {
    let temp = temp_dir("plugin-uninstaller-dry-run");
    let codex_home = temp.join("codex");
    std::fs::create_dir_all(&codex_home).expect("create codex home");
    assert_success(&run_script("install-codex-plugin.sh", &[], &codex_home));

    let dry_run = run_script("uninstall-codex-plugin.sh", &["--dry-run"], &codex_home);
    assert_success(&dry_run);
    let stdout = String::from_utf8_lossy(&dry_run.stdout);
    assert!(
        stdout.contains("DRY RUN"),
        "stdout should mention dry run: {stdout}"
    );
    assert!(
        codex_home
            .join("plugins/cache/codex-computer-use-x11/codex-computer-use-x11")
            .exists(),
        "dry-run uninstall must not remove owned cache"
    );
    assert!(
        std::fs::read_to_string(codex_home.join("config.toml"))
            .unwrap()
            .contains("codex-computer-use-x11@codex-computer-use-x11"),
        "dry-run uninstall must not edit config"
    );

    assert_success(&run_script("uninstall-codex-plugin.sh", &[], &codex_home));
    assert_success(&run_script("uninstall-codex-plugin.sh", &[], &codex_home));

    let _ = std::fs::remove_dir_all(&temp);
}
