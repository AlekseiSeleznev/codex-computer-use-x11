use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn temp_dir(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "codex-computer-use-x11-final-dod-{name}-{}-{nanos}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn run_validator(document: &Path) -> Output {
    Command::new(repo_root().join("scripts/validate-final-dod.py"))
        .current_dir(repo_root())
        .args(["--document", document.to_str().unwrap()])
        .output()
        .expect("run final DoD validator")
}

fn collect_top_level_adr_references(text: &str) -> BTreeSet<String> {
    let mut refs = BTreeSet::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some((_, after_prefix)) = rest.split_once("adr/") {
            let end = after_prefix
                .find(|ch: char| {
                    ch.is_whitespace() || ch == ')' || ch == '`' || ch == ',' || ch == ';'
                })
                .unwrap_or(after_prefix.len());
            let candidate = &after_prefix[..end];
            if candidate.starts_with(char::is_numeric) && candidate.ends_with(".md") {
                refs.insert(format!("adr/{candidate}"));
            }
            rest = &after_prefix[end..];
        }
    }
    refs
}

const ALL_DECISIONS_JSON: &str = r#"[
  "backend_identity",
  "window_model",
  "command_execution_seam",
  "shell_out_vs_native_x11",
  "diagnostics_readiness",
  "input_safety_invariant",
  "pointer_keyboard_backend_priority",
  "atspi_correlation",
  "screenshot_coordinate_model",
  "get_app_state_integration",
  "plugin_source_overlay_strategy",
  "licensing_upstream_policy",
  "cinnamon_extension_wayland_scope"
]"#;

fn valid_row(id: &str) -> String {
    format!(
        r#"{{"id":"{id}","capability":"{id}","required_for_v1":"yes","status":"pass","evidence":["test evidence"],"degraded_behavior":"none"}}"#
    )
}

fn required_rows_json(skip: Option<&str>) -> String {
    let rows = [
        "doctor_capabilities",
        "list_windows",
        "focused_window",
        "focus_window_verification",
        "safe_target_resolution",
        "get_app_state_x11_context",
        "keyboard_type_text",
        "keyboard_press_key",
        "pointer_click",
        "pointer_scroll",
        "pointer_drag",
        "stock_activate_window",
        "stock_mousemove_absence",
        "cinnamon_x11_input_backend",
        "screenshot_global_provider",
        "screenshot_window_crop_bounds",
        "atspi_tree",
        "atspi_action_value_set",
        "terminal_context_selectors",
        "standalone_codex_mcp_plugin",
        "source_overlay",
        "e2e_from_codex",
        "uninstall_rollback",
    ];
    let values = rows
        .iter()
        .filter(|row| Some(**row) != skip)
        .map(|row| valid_row(row))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("[\n{values}\n]")
}

fn fixture_doc(decisions_json: &str, rows_json: &str) -> String {
    format!(
        r#"# Final Architecture and DoD

Research refresh 2026-05-31 target checkout current external references.
License refresh records runtime command invocation distinct from source copying.
Final answer: yes for Cinnamon/X11 v1 baseline; degraded/unsupported for Cinnamon Wayland and unsafe unverified input.

```json final-dod-decisions
{decisions_json}
```

```json final-dod-capability-matrix
{rows_json}
```
"#
    )
}

#[test]
fn final_dod_validator_rejects_missing_capability_rows() {
    let temp = temp_dir("missing-row");
    let doc = temp.join("dod.md");
    std::fs::write(
        &doc,
        fixture_doc(
            ALL_DECISIONS_JSON,
            &required_rows_json(Some("pointer_drag")),
        ),
    )
    .unwrap();

    let output = run_validator(&doc);
    assert!(!output.status.success(), "missing row should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("pointer_drag") && stderr.contains("missing capability row"),
        "stderr should name missing row, got:\n{stderr}"
    );

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn final_dod_validator_rejects_missing_decision_and_evidence() {
    let temp = temp_dir("missing-decision-evidence");
    let doc = temp.join("dod.md");
    let rows = required_rows_json(None).replace(
        "\"id\":\"atspi_action_value_set\",\"capability\":\"atspi_action_value_set\",\"required_for_v1\":\"yes\",\"status\":\"pass\",\"evidence\":[\"test evidence\"],\"degraded_behavior\":\"none\"",
        "\"id\":\"atspi_action_value_set\",\"capability\":\"atspi_action_value_set\",\"required_for_v1\":\"should\",\"status\":\"degraded\",\"evidence\":[],\"degraded_behavior\":\"\"",
    );
    let decisions = ALL_DECISIONS_JSON.replace("  \"window_model\",\n", "");
    std::fs::write(&doc, fixture_doc(&decisions, &rows)).unwrap();

    let output = run_validator(&doc);
    assert!(
        !output.status.success(),
        "missing decision/evidence should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("window_model") && stderr.contains("missing decision topic"),
        "stderr should name missing decision, got:\n{stderr}"
    );
    assert!(
        stderr.contains("atspi_action_value_set") && stderr.contains("evidence"),
        "stderr should name empty evidence, got:\n{stderr}"
    );
    assert!(
        stderr.contains("degraded_behavior"),
        "stderr should name degraded behavior, got:\n{stderr}"
    );

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn final_dod_validator_accepts_tracked_final_report() {
    let output = run_validator(&repo_root().join("docs/final-architecture-dod.md"));
    assert!(
        output.status.success(),
        "tracked final DoD should validate\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Final X11 Computer Use DoD complete"),
        "stdout={stdout}"
    );
}

#[test]
fn final_dod_docs_record_adr_and_architecture_snapshot() {
    let adr = std::fs::read_to_string(
        repo_root().join("adr/0009-adopt-final-cinnamon-x11-v1-dod-baseline.md"),
    )
    .expect("ADR 0009 should exist");
    assert!(adr.contains("Accepted"));
    assert!(adr.contains("x11-ewmh"));
    assert!(adr.contains("Cinnamon/X11 v1"));
    assert!(adr.contains("ADR 0008 remains in force"));

    let architecture = std::fs::read_to_string(repo_root().join("ARCHITECTURE.md"))
        .expect("ARCHITECTURE.md should exist");
    assert!(architecture.contains("0009-adopt-final-cinnamon-x11-v1-dod-baseline"));
    assert!(architecture.contains("yes for the documented Cinnamon/X11 `x11-ewmh` baseline"));

    let adr_readme = std::fs::read_to_string(repo_root().join("adr/README.md"))
        .expect("adr/README.md should exist");
    assert!(adr_readme.contains("0009-adopt-final-cinnamon-x11-v1-dod-baseline"));
}

#[test]
fn architecture_and_adr_index_reference_only_tracked_adr_files() {
    let mut refs = BTreeSet::new();
    for doc in ["ARCHITECTURE.md", "adr/README.md"] {
        let text = std::fs::read_to_string(repo_root().join(doc))
            .unwrap_or_else(|err| panic!("read {doc}: {err}"));
        refs.extend(collect_top_level_adr_references(&text));
    }

    assert!(
        !refs.is_empty(),
        "expected ADR references in architecture docs"
    );
    let missing = refs
        .iter()
        .filter(|path| !repo_root().join(path).is_file())
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "architecture docs reference missing ADR files: {missing:?}"
    );
}
