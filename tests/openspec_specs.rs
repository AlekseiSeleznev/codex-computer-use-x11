use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_text(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    std::fs::read_to_string(path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

#[test]
fn canonical_specs_have_current_purpose_metadata() {
    let specs_dir = repo_root().join("openspec/specs");
    let mut checked = 0usize;
    for entry in std::fs::read_dir(&specs_dir).expect("read openspec/specs") {
        let entry = entry.expect("spec dir entry");
        let spec_path = entry.path().join("spec.md");
        if !spec_path.is_file() {
            continue;
        }
        checked += 1;
        let text = read_text(&spec_path);
        let purpose = text
            .split("## Purpose")
            .nth(1)
            .and_then(|tail| tail.split("## Requirements").next())
            .unwrap_or_else(|| {
                panic!(
                    "{} should contain Purpose before Requirements",
                    spec_path.display()
                )
            })
            .trim();
        assert!(
            !purpose.is_empty(),
            "{} purpose must be non-empty",
            spec_path.display()
        );
        assert!(
            !purpose.contains("TBD") && !purpose.contains("Update Purpose after archive"),
            "{} purpose is stale placeholder text: {purpose}",
            spec_path.display()
        );
        assert!(
            purpose.len() >= 40,
            "{} purpose should describe the current canonical scope: {purpose}",
            spec_path.display()
        );
    }
    assert!(
        checked >= 18,
        "expected current canonical specs, checked {checked}"
    );
}

#[test]
fn architecture_references_archived_provider_takeover_evidence() {
    let architecture = read_text(repo_root().join("ARCHITECTURE.md"));
    let archived = "openspec/changes/archive/2026-06-01-replace-bundled-computer-use-with-x11-provider/reports/fresh-target-install-takeover-20260601T102256Z.json";
    let stale = "openspec/changes/replace-bundled-computer-use-with-x11-provider/reports/fresh-target-install-takeover-20260601T102256Z.json";
    assert!(
        architecture.contains(archived),
        "ARCHITECTURE.md should point at archived provider-takeover evidence"
    );
    assert!(
        !architecture.contains(stale),
        "ARCHITECTURE.md must not point at the pre-archive active change path"
    );
    assert!(
        repo_root().join(archived).is_file(),
        "archived provider-takeover evidence should exist"
    );

    let final_dod = read_text(repo_root().join("docs/final-architecture-dod.md"));
    assert!(
        final_dod.contains(archived),
        "final DoD should point at archived provider-takeover evidence"
    );
    assert!(
        !final_dod.contains(stale),
        "final DoD must not point at the pre-archive active change path"
    );
}

#[test]
fn architecture_contains_runtime_overview_without_naming_the_notation() {
    let architecture = read_text(repo_root().join("ARCHITECTURE.md"));
    assert!(
        architecture.contains("## Runtime architecture overview"),
        "ARCHITECTURE.md should include the runtime architecture overview"
    );
    for required in [
        "Codex plugin host",
        "codex-computer-use-x11",
        "x11_* MCP tools",
        "Linux Mint Cinnamon on X11",
        "AT-SPI collector",
        "install/uninstall scripts",
        "Reversible source overlay",
    ] {
        assert!(
            architecture.contains(required),
            "runtime architecture overview should include {required:?}"
        );
    }

    let overview = architecture
        .split("## Runtime architecture overview")
        .nth(1)
        .and_then(|tail| tail.split("## Boundaries").next())
        .expect("runtime overview section");
    assert!(
        !overview.contains("C4") && !overview.contains("c4"),
        "runtime overview should not name the diagram notation"
    );
}

#[test]
fn readme_has_tracked_github_friendly_hero_image() {
    let readme = read_text(repo_root().join("README.md"));
    let hero = "assets/readme-hero.png";
    assert!(
        readme.contains(&format!("![codex-computer-use-x11 hero]({hero})")),
        "README should embed the tracked hero image"
    );
    let hero_path = repo_root().join(hero);
    assert!(hero_path.is_file(), "hero image should exist at {hero}");
    let metadata = std::fs::metadata(&hero_path).expect("hero image metadata");
    assert!(metadata.len() > 10_000, "hero image should not be empty");
    assert!(
        metadata.len() < 2_000_000,
        "hero image should stay reasonably small for GitHub README rendering"
    );
}

#[test]
fn readme_top_block_is_centered_and_release_links_are_trimmed() {
    let readme = read_text(repo_root().join("README.md"));
    assert!(
        readme.starts_with("<div align=\"center\">\n\n# codex-computer-use-x11"),
        "README should start with a centered GitHub header block"
    );
    let top_block = readme
        .split("</div>")
        .next()
        .expect("README top centered block");
    assert!(
        top_block.find("![Version]")
            < top_block.find("![codex-computer-use-x11 hero](assets/readme-hero.png)"),
        "badges should be above the hero image"
    );
    assert!(
        top_block.find("Cinnamon/X11 Computer Use for Codex")
            < top_block.find("![codex-computer-use-x11 hero](assets/readme-hero.png)"),
        "short project description should be above the hero image"
    );
    assert!(
        !top_block.contains("Install release")
            && !top_block.contains("Changelog")
            && !top_block.contains(("README".to_owned() + ".ru.md").as_str())
            && !top_block.contains(("Рус".to_owned() + "ский").as_str()),
        "top block should not contain removed localized README links"
    );
    assert!(
        !top_block.contains("OpenSpec-validated"),
        "top block should not show the OpenSpec validated badge"
    );
    assert!(
        readme.contains("## License")
            && readme.contains("This project is licensed under the [MIT License](LICENSE)."),
        "README should include a license section"
    );
}

#[test]
fn localized_readme_is_absent_and_unlinked() {
    let root = repo_root();
    let removed_readme = "README".to_owned() + ".ru.md";
    let removed_language = "Рус".to_owned() + "ский";
    assert!(
        !root.join(&removed_readme).exists(),
        "localized README should be removed"
    );
    for path in ["README.md", "CHANGELOG.md", "scripts/check-overlay"] {
        let content = read_text(root.join(path));
        assert!(
            !content.contains(&removed_readme) && !content.contains(&removed_language),
            "{path} should not link to the removed localized README"
        );
    }
}
