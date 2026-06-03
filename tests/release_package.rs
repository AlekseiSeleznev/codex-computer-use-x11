use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_text(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(path.as_ref())
        .unwrap_or_else(|err| panic!("read {}: {err}", path.as_ref().display()))
}

fn temp_dir(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-release-package-{name}-{}-{nanos}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn version() -> String {
    read_text(repo_root().join("VERSION")).trim().to_string()
}

fn package_release(output_dir: &Path) -> PathBuf {
    let script = repo_root().join("scripts/package-release.sh");
    let output = Command::new(&script)
        .current_dir(repo_root())
        .arg("--output-dir")
        .arg(output_dir)
        .arg("--skip-build")
        .env(
            "CODEX_X11_PACKAGE_BINARY",
            env!("CARGO_BIN_EXE_codex-computer-use-x11"),
        )
        .output()
        .unwrap_or_else(|err| panic!("run {}: {err}", script.display()));
    assert!(
        output.status.success(),
        "package-release should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let artifact = output_dir.join(format!(
        "codex-computer-use-x11-v{}-x86_64-unknown-linux-gnu.tar.gz",
        version()
    ));
    assert!(
        artifact.is_file(),
        "artifact should exist: {}",
        artifact.display()
    );
    artifact
}

fn sha256_file_for(artifact: &Path) -> PathBuf {
    PathBuf::from(format!("{}.sha256", artifact.display()))
}

fn tar_listing(artifact: &Path) -> Vec<String> {
    let output = Command::new("tar")
        .args(["-tzf", artifact.to_str().unwrap()])
        .output()
        .expect("list tarball");
    assert!(
        output.status.success(),
        "tar listing should succeed\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}

fn extract_tarball(artifact: &Path, dest: &Path) {
    let output = Command::new("tar")
        .args([
            "-xzf",
            artifact.to_str().unwrap(),
            "-C",
            dest.to_str().unwrap(),
        ])
        .output()
        .expect("extract tarball");
    assert!(
        output.status.success(),
        "tar extract should succeed\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn package_release_script_creates_versioned_tarball_and_checksum() {
    let temp = temp_dir("artifact-checksum");
    let artifact = package_release(&temp);
    let checksum = sha256_file_for(&artifact);

    assert!(
        checksum.is_file(),
        "checksum should exist: {}",
        checksum.display()
    );
    assert!(
        artifact
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains(&format!("v{}", version())),
        "artifact filename should include VERSION"
    );

    let output = Command::new("sha256sum")
        .arg("--check")
        .arg(checksum.file_name().unwrap())
        .current_dir(&temp)
        .output()
        .expect("run sha256sum --check");
    assert!(
        output.status.success(),
        "sha256 verification should pass\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn release_tarball_contains_ready_plugin_bundle() {
    let temp = temp_dir("bundle");
    let artifact = package_release(&temp);
    let extract_dir = temp.join("extract");
    std::fs::create_dir_all(&extract_dir).unwrap();
    extract_tarball(&artifact, &extract_dir);

    let root = extract_dir.join("codex-computer-use-x11");
    let binary = root.join("bin/codex-computer-use-x11");
    assert!(binary.is_file(), "binary should exist");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&binary).unwrap().permissions().mode();
        assert_ne!(mode & 0o111, 0, "binary should be executable");
    }

    let mcp: Value = serde_json::from_str(&read_text(root.join(".mcp.json"))).unwrap();
    let server = &mcp["mcpServers"]["codex-computer-use-x11"];
    assert_eq!(server["command"], "./bin/codex-computer-use-x11");
    assert_eq!(server["args"], serde_json::json!(["mcp"]));
    assert_eq!(server["cwd"], ".");

    let plugin: Value =
        serde_json::from_str(&read_text(root.join(".codex-plugin/plugin.json"))).unwrap();
    assert_eq!(plugin["name"], "codex-computer-use-x11");
    assert_eq!(plugin["version"], version());
    assert_eq!(plugin["interface"]["displayName"], "X11 Computer Use");
    assert_eq!(
        plugin["interface"]["shortDescription"],
        "Standalone x11_* tools for Linux X11/EWMH"
    );
    assert!(root.join("assets/app-icon.png").is_file());

    let metadata: Value =
        serde_json::from_str(&read_text(root.join("RELEASE-METADATA.json"))).unwrap();
    assert_eq!(metadata["plugin_name"], "codex-computer-use-x11");
    assert_eq!(metadata["version"], version());
    assert_eq!(metadata["command"], "./bin/codex-computer-use-x11");
    assert_eq!(metadata["args"], serde_json::json!(["mcp"]));
    assert_eq!(metadata["display_name"], "X11 Computer Use");
    assert_eq!(metadata["baseline"], "x11-ewmh / Cinnamon X11");
    assert!(metadata["sha256"].as_str().unwrap().len() >= 64);

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn release_tarball_excludes_forbidden_files() {
    let temp = temp_dir("forbidden");
    let artifact = package_release(&temp);
    let listing = tar_listing(&artifact);
    assert!(
        listing
            .iter()
            .any(|path| path == "codex-computer-use-x11/.mcp.json"),
        "sanity check: listing should include plugin files: {listing:?}"
    );

    let forbidden_substrings = [".git/", "target/", ".codex/session/", ".secrets"];
    for entry in &listing {
        for forbidden in forbidden_substrings {
            assert!(
                !entry.contains(forbidden),
                "tarball entry {entry:?} should not contain forbidden substring {forbidden:?}"
            );
        }
        assert!(
            !entry.ends_with(".env") && !entry.ends_with(".local.env"),
            "tarball entry {entry:?} should not include local env files"
        );
        assert!(
            !entry.ends_with(".bak") && !entry.contains(".bak."),
            "tarball entry {entry:?} should not include backup files"
        );
    }

    let _ = std::fs::remove_dir_all(temp);
}
