use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn assert_contains(haystack: &str, needle: &str, context: &str) {
    assert!(
        haystack.contains(needle),
        "{context} should contain {needle:?}\n--- text ---\n{haystack}"
    );
}

#[test]
fn readme_v1_quick_start_links_required_docs() {
    let readme = read_text("README.md");

    for required in [
        "## Quick start",
        "## Documentation",
        "standalone user-local Codex MCP plugin",
        "reversible source overlay",
        "generic X11/EWMH",
        "`x11-ewmh`",
        "Cinnamon Wayland",
        "out of scope for v1",
        "native `.deb`/`.rpm`/AppImage packaging is out of scope for this repository stage",
        "docs/install-uninstall.md",
        "docs/troubleshooting.md",
        "docs/upstreaming.md",
        "docs/license-attribution.md",
        "docs/release-checklist.md",
        "docs/final-architecture-dod.md",
        "scripts/validate-final-dod.py",
    ] {
        assert_contains(&readme, required, "README v1 quick start");
    }
    assert!(
        !readme.contains("source overlay target remains read-only"),
        "README must not describe implemented source overlay as always read-only"
    );
    assert!(
        !readme.contains("future work may adapt"),
        "README must not describe implemented source overlay as future-only"
    );
}

fn temp_dir(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-packaging-docs-{name}-{}-{nanos}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn assert_command_success(command: &mut std::process::Command, label: &str) {
    let output = command
        .output()
        .unwrap_or_else(|err| panic!("run {label}: {err}"));
    assert!(
        output.status.success(),
        "{label} should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn install_uninstall_docs_reference_real_scripts_and_safe_commands() {
    let docs = read_text("docs/install-uninstall.md");

    for required in [
        "# Install/uninstall guide",
        "standalone user-local Codex MCP plugin",
        "scripts/install-codex-plugin.sh --dry-run",
        "scripts/install-codex-plugin.sh --activate-accessibility --dry-run --report-json",
        "scripts/install-codex-plugin.sh",
        "scripts/uninstall-codex-plugin.sh --dry-run",
        "scripts/uninstall-codex-plugin.sh --dry-run --report-json",
        "scripts/uninstall-codex-plugin.sh",
        "reversible source overlay",
        "scripts/status-codex-source-overlay.sh --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\"",
        "scripts/install-codex-source-overlay.sh --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\"",
        "cargo test -p codex-computer-use-linux x11_ewmh --manifest-path \"$CODEX_DESKTOP_LINUX_FULL_PATH/Cargo.toml\"",
        "scripts/uninstall-codex-source-overlay.sh --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\"",
        "scripts/install-x11-provider-takeover.sh --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\" --dry-run",
        "scripts/install-x11-provider-takeover.sh --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\" --dry-run --report-json /tmp/x11-provider-install-report.json",
        "scripts/uninstall-x11-provider-takeover.sh --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\" --dry-run",
        "scripts/uninstall-x11-provider-takeover.sh --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\" --dry-run --report-json /tmp/x11-provider-uninstall-report.json",
        "x11_doctor",
        "x11_get_app_state include_screenshot=true",
        "x11_accessibility_tree",
        "provider takeover marker",
        "full uninstall restore",
        "manifest-backed backups",
        "safe blocker",
        "fully restart Codex Desktop",
        "git -C \"$CODEX_DESKTOP_LINUX_FULL_PATH\" status --short",
        "scripts/e2e/codex-plugin-smoke.sh --fake",
        "scripts/e2e/codex-source-overlay-smoke.sh --fake",
        "state=drifted",
        "does not write `/opt`, `openai-bundled`, or the bundled `computer-use` cache",
    ] {
        assert_contains(&docs, required, "install/uninstall docs");
    }

    for script in [
        "scripts/install-codex-plugin.sh",
        "scripts/uninstall-codex-plugin.sh",
        "scripts/status-codex-source-overlay.sh",
        "scripts/install-codex-source-overlay.sh",
        "scripts/uninstall-codex-source-overlay.sh",
        "scripts/install-x11-provider-takeover.sh",
        "scripts/uninstall-x11-provider-takeover.sh",
        "scripts/e2e/codex-plugin-smoke.sh",
        "scripts/e2e/codex-source-overlay-smoke.sh",
    ] {
        assert!(repo_root().join(script).is_file(), "{script} should exist");
    }

    let temp = temp_dir("plugin-dry-run");
    let codex_home = temp.join("codex-home");
    assert_command_success(
        std::process::Command::new(repo_root().join("scripts/install-codex-plugin.sh"))
            .arg("--dry-run")
            .current_dir(repo_root())
            .env("CODEX_HOME", &codex_home)
            .env("CODEX_CONFIG_FILE", codex_home.join("config.toml")),
        "install-codex-plugin --dry-run",
    );
    assert_command_success(
        std::process::Command::new(repo_root().join("scripts/uninstall-codex-plugin.sh"))
            .arg("--dry-run")
            .current_dir(repo_root())
            .env("CODEX_HOME", &codex_home)
            .env("CODEX_CONFIG_FILE", codex_home.join("config.toml")),
        "uninstall-codex-plugin --dry-run",
    );
    let _ = std::fs::remove_dir_all(temp);

    for script in [
        "scripts/status-codex-source-overlay.sh",
        "scripts/install-codex-source-overlay.sh",
        "scripts/uninstall-codex-source-overlay.sh",
        "scripts/install-x11-provider-takeover.sh",
        "scripts/uninstall-x11-provider-takeover.sh",
        "scripts/e2e/codex-plugin-smoke.sh",
        "scripts/e2e/codex-source-overlay-smoke.sh",
    ] {
        assert_command_success(
            std::process::Command::new(repo_root().join(script))
                .arg("--help")
                .current_dir(repo_root()),
            &format!("{script} --help"),
        );
    }
}

#[test]
fn license_attribution_docs_classify_references_and_commands() {
    let docs = read_text("docs/license-attribution.md");

    for required in [
        "# License and attribution notes",
        "Observed during 2026-05-31 research refresh",
        "agent-sh/computer-use-linux",
        "MIT",
        "MONTBRAIN/vadgr-computer-use",
        "Apache-2.0",
        "joe223/sootie",
        "NOASSERTION",
        "hightemp/go_computer_use_mcp_server",
        "NO LICENSE ENDPOINT",
        "linuxmint/cinnamon",
        "GPL-2.0",
        "Conservatory/wmctrl",
        "jordansissel/xdotool",
        "BSD-3-Clause",
        "ReimuNotMoe/ydotool",
        "AGPL-3.0",
        "psychon/x11rb",
        "runtime command dependency",
        "Invoking an installed command at runtime is distinct from copying, vendoring, or adapting its source code.",
        "copy-safe only with attribution",
        "copy-unsafe for MIT upstream code",
        "No external source code is copied or vendored by this repository stage.",
        "Re-check license metadata before copying code or making upstream release claims.",
    ] {
        assert_contains(&docs, required, "license/attribution docs");
    }
}

#[test]
fn upstreaming_docs_separate_backend_and_wrapper_targets() {
    let docs = read_text("docs/upstreaming.md");

    for required in [
        "# Upstreaming guide",
        "upstream target matrix",
        "backend-upstream",
        "wrapper-integration",
        "agent-sh/computer-use-linux",
        "CODEX_DESKTOP_LINUX_FULL_PATH",
        "codex-desktop-linux",
        "computer-use-linux/",
        "packaging, launcher, update-manager, linux-features, and bundled plugin staging",
        "Do not mix backend and wrapper changes in one pull request",
        "source overlay is reversible staging evidence, not a long-lived fork",
        "activate_window",
        "get_app_state",
        "type_text",
        "press_key",
        "click",
        "scroll",
        "drag",
        "fresh target research before PR work",
    ] {
        assert_contains(&docs, required, "upstreaming docs");
    }
}

#[test]
fn troubleshooting_docs_cover_degraded_layers_and_drift() {
    let docs = read_text("docs/troubleshooting.md");

    for required in [
        "# Troubleshooting",
        "cargo run -- doctor --json",
        "XDG_SESSION_TYPE",
        "XDG_CURRENT_DESKTOP",
        "wmctrl",
        "xprop",
        "xdotool",
        "ydotool",
        "X11-only compatibility facts",
        "debug-only context",
        "Screenshot",
        "AT-SPI",
        "degraded",
        "standalone plugin",
        "CODEX_HOME",
        "source-overlay drift",
        "state=drifted",
        "do not overwrite unowned target code",
        "target/e2e-logs",
        "fake mode",
        "live mode",
        "No secrets",
    ] {
        assert_contains(&docs, required, "troubleshooting docs");
    }
}

#[test]
fn release_checklist_requires_validation_evidence_and_secret_safety() {
    let docs = read_text("docs/release-checklist.md");

    for required in [
        "# Release checklist",
        "openspec validate --all --strict",
        "make fmt",
        "make check",
        "make test",
        "cargo test --test packaging_docs",
        "scripts/validate-final-dod.py",
        "docs/final-architecture-dod.md",
        "scripts/e2e/codex-plugin-smoke.sh --fake",
        "scripts/e2e/codex-source-overlay-smoke.sh --fake",
        "scripts/e2e/codex-x11-e2e.py validate-matrix",
        "optional live",
        "scripts/e2e/codex-source-overlay-smoke.sh --live --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\"",
        "scripts/uninstall-codex-source-overlay.sh --target \"$CODEX_DESKTOP_LINUX_FULL_PATH\"",
        "git status --short",
        "git -C \"$CODEX_DESKTOP_LINUX_FULL_PATH\" status --short",
        "license refresh",
        "Do not read, print, commit, archive, or copy `.secrets.local.env`.",
        "CODEX_DESKTOP_LINUX_FULL_PATH",
        "tokens, credentials, private local configuration",
        "archive",
        "push",
    ] {
        assert_contains(&docs, required, "release checklist");
    }
    assert!(
        !docs.contains("openspec validate prepare-x11-backend-packaging-docs-upstreaming --type change --strict"),
        "release checklist must not require validating an archived active change"
    );
}

#[test]
fn production_readiness_docs_define_x11_pass_degraded_fail_and_safe_retest() {
    let readme = read_text("README.md");
    for required in [
        "## Production readiness evidence",
        "PASS means",
        "DEGRADED means",
        "FAIL means",
        "reason_category",
        "controlled fixtures only",
        "Wayland and RemoteDesktop/portal-required runtime paths are outside the current standalone plugin scope",
        "scripts/e2e/codex-plugin-smoke.sh --live --industrial --fake-live-fixtures",
        "scripts/e2e/codex-x11-e2e.py validate-matrix --industrial",
    ] {
        assert_contains(&readme, required, "README production readiness docs");
    }

    let troubleshooting = read_text("docs/troubleshooting.md");
    for required in [
        "PASS / DEGRADED / FAIL",
        "AT-SPI bus reachable but tree extraction unavailable",
        "atspi_gtk_bridge_disabled_by_environment",
        "NO_AT_BRIDGE=1",
        "remove or avoid inheriting `NO_AT_BRIDGE`",
        "at-spi2-core",
        "libatk-adaptor",
        "libatk-bridge2.0-0t64",
        "libatspi2.0-0t64",
        "at-spi2-registryd",
        "controlled GTK fixture",
        "expected_fake_fixture_limitation",
        "unsupported_out_of_scope",
        "not safe to test input against real user applications",
        "get-app-state --json` no longer emits inline screenshot blobs by default",
        "--screenshot-output <path>",
        "--no-screenshot",
        "--inline-screenshot",
        "unsafe opt-in",
        "RemoteDesktop portal and Wayland support are out of current standalone-plugin",
        "do not fix",
    ] {
        assert_contains(
            &troubleshooting,
            required,
            "troubleshooting production readiness docs",
        );
    }
    assert!(
        !troubleshooting.contains("NO_AT_BRIDGE=0"),
        "troubleshooting must not recommend NO_AT_BRIDGE=0 as bridge enablement"
    );

    let release = read_text("docs/release-checklist.md");
    for required in [
        "Production claim evidence",
        "doctor --json",
        "fake smoke",
        "metadata-only live smoke",
        "controlled live fixture smoke",
        "no inline screenshot data URLs",
        "--screenshot-output <path>",
        "--inline-screenshot",
        "unique neutral controlled",
    ] {
        assert_contains(&release, required, "release production claim evidence docs");
    }
}

#[test]
fn troubleshooting_docs_explain_atspi_bridge_disabled_remediation() {
    let troubleshooting = read_text("docs/troubleshooting.md");
    for required in [
        "AT-SPI bus reachable but tree extraction unavailable",
        "atspi_gtk_bridge_disabled_by_environment",
        "atspi_bus_available=true",
        "tree_available=false",
        "at-spi2-core",
        "libatk-adaptor",
        "libatk-bridge2.0-0t64",
        "libatspi2.0-0t64",
        "toolkit-accessibility",
        "at-spi-bus-launcher",
        "at-spi2-registryd",
        "NO_AT_BRIDGE=1",
        "unset `NO_AT_BRIDGE`",
        "restart the affected Cinnamon/Codex session or fixture process",
        "controlled GTK fixture",
        "not safe to test input against real user applications",
    ] {
        assert_contains(
            &troubleshooting,
            required,
            "AT-SPI bridge-disabled troubleshooting docs",
        );
    }
    assert!(
        !troubleshooting.contains("NO_AT_BRIDGE=0"),
        "docs must not recommend NO_AT_BRIDGE=0 as the bridge-enable path"
    );
}

#[test]
fn context_format_examples_do_not_link_to_missing_placeholder_files() {
    let docs = read_text(".codex/skills/grill-with-docs/CONTEXT-FORMAT.md");
    for placeholder_link in [
        "[Ordering](./src/ordering/CONTEXT.md)",
        "[Billing](./src/billing/CONTEXT.md)",
        "[Fulfillment](./src/fulfillment/CONTEXT.md)",
    ] {
        assert!(
            !docs.contains(placeholder_link),
            "placeholder example should not be encoded as a local Markdown link: {placeholder_link}"
        );
    }
    for example in [
        "Ordering (`src/ordering/CONTEXT.md`)",
        "Billing (`src/billing/CONTEXT.md`)",
        "Fulfillment (`src/fulfillment/CONTEXT.md`)",
    ] {
        assert_contains(&docs, example, "context format examples");
    }
}

#[test]
fn adapter_contract_docs_are_linked_and_status_safe() {
    let readme = read_text("README.md");
    let install = read_text("INSTALL_CODEX.md");
    let changelog = read_text("CHANGELOG.md");
    let adapter = read_text("docs/codex-desktop-linux-x11-ewmh-adapter.md");

    for text in [&readme, &install] {
        assert_contains(
            text,
            "docs/codex-desktop-linux-x11-ewmh-adapter.md",
            "adapter docs link",
        );
        assert_contains(
            text,
            "Prepared adapter contract for optional linux-features/x11-ewmh-computer-use integration in codex-desktop-linux.",
            "adapter prepared wording",
        );
        assert!(
            !text.contains("Upstream integration is merged")
                && !text.contains("Enabled by default in codex-desktop-linux"),
            "docs must not overstate upstream status"
        );
    }

    for required in [
        "# codex-desktop-linux X11/EWMH adapter contract",
        "This repository remains the source of truth",
        "linux-features/x11-ewmh-computer-use/",
        "disabled by default",
        "must not modify core Computer Use",
        "must not replace the bundled `computer-use` plugin",
        "must not change global doctor behavior",
        "No submodules",
        "CODEX_X11_COMPUTER_USE_SOURCE=/path/to/codex-computer-use-x11",
        "verify sha256 before staging",
        "RemoteDesktop/Wayland",
        "debug-only",
    ] {
        assert_contains(&adapter, required, "adapter contract doc");
    }

    for required in [
        "Unreleased",
        "release tarball + sha256",
        "upstream adapter contract",
        "downstream adapter scaffold",
    ] {
        assert_contains(&changelog, required, "changelog adapter prep");
    }
    assert!(
        !changelog.contains("Published v0.1.3"),
        "changelog must not claim a release was published"
    );
}

#[test]
fn adapter_contract_records_backend_flavor_guidance() {
    let adapter = read_text("docs/codex-desktop-linux-x11-ewmh-adapter.md");

    for required in [
        "agent-sh/computer-use-linux",
        "selectable backend/flavor",
        "future evaluation path",
        "separate change",
        "default upstream-ready path",
        "must not change default `codex-desktop-linux` Computer Use behavior",
    ] {
        assert_contains(&adapter, required, "adapter backend flavor guidance");
    }
}

#[test]
fn downstream_adapter_scaffold_matches_linux_feature_contract() {
    let root =
        repo_root().join("adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use");
    for required in [
        "feature.json",
        "README.md",
        "stage.sh",
        "patches.js",
        "test.js",
    ] {
        assert!(
            root.join(required).is_file(),
            "scaffold {required} should exist"
        );
    }

    let feature: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("feature.json")).unwrap()).unwrap();
    assert_eq!(feature["id"], "x11-ewmh-computer-use");
    assert_eq!(feature["defaultEnabled"], false);
    assert_eq!(feature["entrypoints"]["stageHook"], "./stage.sh");
    assert_eq!(feature["entrypoints"]["patchDescriptors"], "./patches.js");

    let scaffold_readme = std::fs::read_to_string(root.join("README.md")).unwrap();
    for required in [
        "linux-features/features.json",
        "\"enabled\": [\"x11-ewmh-computer-use\"]",
        "Linux Mint Cinnamon on X11",
        "x11-ewmh",
        "CODEX_X11_COMPUTER_USE_SOURCE=/path/to/codex-computer-use-x11",
        "CODEX_X11_COMPUTER_USE_BINARY",
        "CODEX_X11_COMPUTER_USE_RELEASE_TARBALL",
        "CODEX_X11_COMPUTER_USE_RELEASE_SHA256",
        "no core Computer Use replacement",
        "no Wayland/RemoteDesktop baseline",
        "no default enablement",
        "no submodule",
        "no global doctor changes",
    ] {
        assert_contains(&scaffold_readme, required, "scaffold README");
    }
    for tool in [
        "x11_doctor",
        "x11_list_windows",
        "x11_focused_window",
        "x11_focus_window",
        "x11_accessibility_tree",
        "x11_type_text",
        "x11_press_key",
        "x11_click",
        "x11_scroll",
        "x11_drag",
        "x11_get_app_state",
        "x11_target_window",
        "x11_target_context",
        "x11_release_window",
    ] {
        assert_contains(&scaffold_readme, tool, "scaffold README tool list");
    }
}

#[test]
fn scaffold_readme_records_backend_flavor_guidance() {
    let scaffold_readme =
        read_text("adapters/codex-desktop-linux/linux-features/x11-ewmh-computer-use/README.md");

    for required in [
        "Upstream alignment",
        "separate `codex-computer-use-x11` plugin",
        "agent-sh/computer-use-linux",
        "selectable backend/flavor",
        "separate future investigation",
        "no backend/flavor experiment may require enabling this feature by default",
        "modifying core Computer Use behavior",
    ] {
        assert_contains(
            &scaffold_readme,
            required,
            "scaffold backend flavor guidance",
        );
    }
}
