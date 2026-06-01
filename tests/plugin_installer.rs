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

fn assert_success(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "stdout:\n{stdout}\nstderr:\n{stderr}"
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
