use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::doctor::{BACKEND_ID, PROJECT_NAME};
use crate::list_windows::{self, PidReliability, WindowInfo, WindowListReport, WindowMetadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityTreeReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub success: bool,
    pub window: Option<WindowInfo>,
    pub correlation: CorrelationResult,
    pub tree: Vec<AccessibilityNode>,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: AccessibilityDiagnostics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationResult {
    pub status: String,
    pub confidence: String,
    pub score: i32,
    pub matched_object_ref: Option<String>,
    pub reasons: Vec<String>,
    pub candidates: Vec<AccessibilityCandidateSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityDiagnostics {
    pub ok: bool,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub listing: list_windows::WindowListingDiagnostics,
    pub collector: CollectorDiagnostics,
    pub target_xprop: Option<TargetXpropEnrichment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetXpropEnrichment {
    pub attempted: bool,
    pub ok: bool,
    pub xprop_id_calls: usize,
    pub window_id: Option<u64>,
    pub net_wm_pid: Option<u32>,
    pub wm_client_machine: Option<String>,
    pub wm_name: Option<String>,
    pub net_wm_name: Option<String>,
    pub wm_class: Vec<String>,
    pub net_wm_window_type: Vec<String>,
    pub detail: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectorDiagnostics {
    pub ok: bool,
    pub command: Option<String>,
    pub stderr: Option<String>,
    pub detail: String,
    pub truncated: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessibilityCandidateSummary {
    pub object_ref: String,
    pub name: Option<String>,
    pub role: Option<String>,
    pub pid: Option<u32>,
    pub score: i32,
    pub confidence: String,
    pub reasons: Vec<String>,
    pub missing_signals: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct CollectorOutput {
    #[serde(default)]
    ok: bool,
    #[serde(default)]
    candidates: Vec<AtspiCandidate>,
    #[serde(default)]
    diagnostics: CollectorOutputDiagnostics,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct CollectorOutputDiagnostics {
    #[serde(default)]
    truncated: bool,
    #[serde(default)]
    detail: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct AtspiCandidate {
    object_ref: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    pid: Option<u32>,
    #[serde(default)]
    bounds: Option<AccessibilityBounds>,
    #[serde(default)]
    focused: bool,
    #[serde(default)]
    states: Vec<String>,
    #[serde(default)]
    nodes: Vec<AccessibilityNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityNode {
    pub index: u32,
    pub parent_index: Option<u32>,
    pub depth: u32,
    pub object_ref: String,
    pub role: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub child_count: i32,
    #[serde(default)]
    pub bounds: Option<AccessibilityBounds>,
    #[serde(default)]
    pub states: Vec<String>,
    #[serde(default)]
    pub actions: Vec<AccessibilityAction>,
    #[serde(default)]
    pub value: Option<AccessibilityValue>,
    #[serde(default)]
    pub supports_editable_text: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityBounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityAction {
    #[serde(default)]
    pub index: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub keybinding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityValue {
    #[serde(default)]
    pub current: Option<f64>,
    #[serde(default)]
    pub minimum: Option<f64>,
    #[serde(default)]
    pub maximum: Option<f64>,
    #[serde(default)]
    pub text: Option<String>,
}

pub fn accessibility_tree_report_from_system(window_id: u64) -> AccessibilityTreeReport {
    let listing = list_windows::report_from_system();
    if !listing
        .windows
        .iter()
        .any(|window| window.window_id == window_id)
    {
        return window_not_found_report(window_id, listing);
    }
    let collection = collect_atspi_candidates();
    report_from_listing_and_collection(window_id, listing, collection)
}

fn report_from_listing_and_collection(
    window_id: u64,
    listing: WindowListReport,
    collection: Result<CollectorOutput, String>,
) -> AccessibilityTreeReport {
    let Some(window) = listing
        .windows
        .iter()
        .find(|window| window.window_id == window_id)
        .cloned()
    else {
        return window_not_found_report(window_id, listing);
    };

    let target_xprop = target_xprop_enrichment(window.window_id);

    let collector = match collection {
        Ok(output) if output.ok => output,
        Ok(output) => {
            return failure_report(
                listing,
                Some(window),
                correlation(
                    "degraded",
                    "none",
                    0,
                    None,
                    vec!["AT-SPI collector reported unavailable.".to_string()],
                    Vec::new(),
                ),
                Vec::new(),
                "AtspiUnavailable",
                "AT-SPI collection did not return usable candidates.",
                CollectorDiagnostics {
                    ok: false,
                    command: Some(collector_command()),
                    stderr: None,
                    detail: output
                        .diagnostics
                        .detail
                        .unwrap_or_else(|| "collector returned ok=false".to_string()),
                    truncated: output.diagnostics.truncated,
                },
                Some(target_xprop),
            );
        }
        Err(error) => {
            return failure_report(
                listing,
                Some(window),
                correlation(
                    "degraded",
                    "none",
                    0,
                    None,
                    vec!["AT-SPI collector failed.".to_string()],
                    Vec::new(),
                ),
                Vec::new(),
                "AtspiUnavailable",
                "AT-SPI collection failed before correlation.",
                CollectorDiagnostics {
                    ok: false,
                    command: Some(collector_command()),
                    stderr: Some(error.clone()),
                    detail: error,
                    truncated: false,
                },
                Some(target_xprop),
            );
        }
    };

    let metadata = metadata_for_window(&listing, window.window_id);
    let mut scored = collector
        .candidates
        .iter()
        .map(|candidate| {
            score_candidate(&window, metadata.as_ref(), Some(&target_xprop), candidate)
        })
        .collect::<Vec<_>>();
    scored.sort_by_key(|candidate| std::cmp::Reverse(candidate.score));

    let Some(best) = scored.first() else {
        return failure_report(
            listing,
            Some(window),
            correlation(
                "no_match",
                "none",
                0,
                None,
                vec!["AT-SPI collector returned no candidates.".to_string()],
                Vec::new(),
            ),
            Vec::new(),
            "NoAccessibilityMatch",
            "No AT-SPI candidates were available for correlation.",
            collector_diagnostics(&collector),
            Some(target_xprop.clone()),
        );
    };

    let matching_candidates = scored
        .iter()
        .filter(|candidate| matches!(candidate.confidence.as_str(), "high" | "medium"))
        .collect::<Vec<_>>();
    if matching_candidates.len() > 1
        && matching_candidates
            .get(1)
            .is_some_and(|second| best.score - second.score <= 10)
    {
        return failure_report(
            listing,
            Some(window),
            correlation(
                "ambiguous",
                best.confidence.clone(),
                best.score,
                None,
                vec!["Multiple AT-SPI candidates had equivalent correlation scores.".to_string()],
                scored.iter().map(ScoredCandidate::summary).collect(),
            ),
            Vec::new(),
            "AmbiguousAccessibilityMatch",
            "Multiple AT-SPI candidates matched the requested X11 window; no subtree was returned.",
            collector_diagnostics(&collector),
            Some(target_xprop.clone()),
        );
    }

    if matches!(best.confidence.as_str(), "high" | "medium") {
        let tree = best.candidate.nodes.clone();
        AccessibilityTreeReport {
            project: PROJECT_NAME.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            backend: BACKEND_ID.to_string(),
            success: true,
            window: Some(window),
            correlation: correlation(
                "matched",
                best.confidence.clone(),
                best.score,
                Some(best.candidate.object_ref.clone()),
                best.reasons.clone(),
                scored.iter().map(ScoredCandidate::summary).collect(),
            ),
            tree,
            error_code: None,
            note: format!(
                "Matched X11 window to AT-SPI subtree with {} confidence.",
                best.confidence
            ),
            diagnostics: AccessibilityDiagnostics {
                ok: listing.diagnostics.ok,
                blockers: listing.diagnostics.blockers.clone(),
                degraded_reasons: listing.diagnostics.degraded_reasons.clone(),
                listing: listing.diagnostics,
                collector: collector_diagnostics(&collector),
                target_xprop: Some(target_xprop),
            },
        }
    } else {
        failure_report(
            listing,
            Some(window),
            correlation(
                "no_match",
                best.confidence.clone(),
                best.score,
                None,
                best.reasons.clone(),
                scored.iter().map(ScoredCandidate::summary).collect(),
            ),
            Vec::new(),
            "NoAccessibilityMatch",
            "No AT-SPI candidate reached high-confidence correlation.",
            collector_diagnostics(&collector),
            Some(target_xprop),
        )
    }
}

fn window_not_found_report(window_id: u64, listing: WindowListReport) -> AccessibilityTreeReport {
    failure_report(
        listing,
        None,
        correlation(
            "no_match",
            "none",
            0,
            None,
            vec![format!("No window matched window_id {window_id}.")],
            Vec::new(),
        ),
        Vec::new(),
        "WindowNotFound",
        format!("No window matched window_id {window_id}."),
        CollectorDiagnostics::default(),
        None,
    )
}

struct ScoredCandidate<'a> {
    candidate: &'a AtspiCandidate,
    score: i32,
    confidence: String,
    reasons: Vec<String>,
    missing_signals: Vec<String>,
}

impl ScoredCandidate<'_> {
    fn summary(&self) -> AccessibilityCandidateSummary {
        AccessibilityCandidateSummary {
            object_ref: self.candidate.object_ref.clone(),
            name: self.candidate.name.clone(),
            role: self.candidate.role.clone(),
            pid: self.candidate.pid,
            score: self.score,
            confidence: self.confidence.clone(),
            reasons: self.reasons.clone(),
            missing_signals: self.missing_signals.clone(),
        }
    }
}

fn score_candidate<'a>(
    window: &WindowInfo,
    metadata: Option<&WindowMetadata>,
    _target_xprop: Option<&TargetXpropEnrichment>,
    candidate: &'a AtspiCandidate,
) -> ScoredCandidate<'a> {
    let mut score = 0;
    let mut reasons = Vec::new();
    let mut missing_signals = Vec::new();
    let mut non_pid_signals = 0;
    let reliable_pid = metadata
        .map(|metadata| metadata.pid_reliability == PidReliability::Reliable)
        .unwrap_or(false);

    if reliable_pid && window.pid.is_some() && window.pid == candidate.pid {
        score += 45;
        reasons.push("reliable PID matched".to_string());
    } else if reliable_pid && window.pid.is_some() && candidate.pid.is_some() {
        reasons.push("candidate PID did not match reliable window PID".to_string());
        missing_signals.push("reliable_pid".to_string());
    } else if !reliable_pid {
        reasons.push("window PID was not treated as reliable evidence".to_string());
        missing_signals.push("reliable_pid".to_string());
    } else {
        missing_signals.push("reliable_pid".to_string());
    }

    if title_matches(window.title.as_deref(), candidate.name.as_deref()) {
        score += 25;
        non_pid_signals += 1;
        reasons.push("title/name matched".to_string());
    } else {
        missing_signals.push("title_name_match".to_string());
    }

    if class_matches(window, candidate.name.as_deref()) {
        score += 20;
        non_pid_signals += 1;
        reasons.push("wm_class/app name matched".to_string());
    } else {
        missing_signals.push("class_app_match".to_string());
    }

    if bounds_overlap(window, candidate.bounds.as_ref()) {
        score += 20;
        non_pid_signals += 1;
        reasons.push("bounds overlapped".to_string());
    } else {
        missing_signals.push("bounds_overlap".to_string());
    }

    if window.focused && (candidate.focused || has_state(&candidate.states, "active")) {
        score += 10;
        non_pid_signals += 1;
        reasons.push("focused state matched".to_string());
    } else {
        missing_signals.push("focus_match".to_string());
    }

    let confidence = if score >= 70 && reliable_pid {
        "high"
    } else if score >= 45 && non_pid_signals >= 2 {
        "medium"
    } else {
        "none"
    }
    .to_string();

    ScoredCandidate {
        candidate,
        score,
        confidence,
        reasons,
        missing_signals,
    }
}

fn title_matches(window_title: Option<&str>, candidate_name: Option<&str>) -> bool {
    let Some(window_title) = normalized(window_title) else {
        return false;
    };
    let Some(candidate_name) = normalized(candidate_name) else {
        return false;
    };
    candidate_name.contains(&window_title) || window_title.contains(&candidate_name)
}

fn class_matches(window: &WindowInfo, candidate_name: Option<&str>) -> bool {
    let candidate_tokens = token_list(candidate_name);
    if candidate_tokens.is_empty() {
        return false;
    }
    [window.wm_class.as_deref(), window.app_id.as_deref()]
        .into_iter()
        .flatten()
        .any(|value| {
            token_list(Some(value))
                .iter()
                .all(|token| candidate_tokens.contains(token))
        })
}

fn target_xprop_enrichment(window_id: u64) -> TargetXpropEnrichment {
    if !list_windows::command_exists("xprop") {
        return TargetXpropEnrichment {
            attempted: false,
            ok: false,
            xprop_id_calls: 0,
            window_id: Some(window_id),
            detail: "xprop unavailable for target-scoped enrichment".to_string(),
            ..TargetXpropEnrichment::default()
        };
    }
    let window_arg = format!("0x{window_id:x}");
    match Command::new("xprop")
        .args(["-id", window_arg.as_str()])
        .output()
    {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            parse_target_xprop(window_id, &stdout)
        }
        Ok(output) => TargetXpropEnrichment {
            attempted: true,
            ok: false,
            xprop_id_calls: 1,
            window_id: Some(window_id),
            detail: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ..TargetXpropEnrichment::default()
        },
        Err(error) => TargetXpropEnrichment {
            attempted: true,
            ok: false,
            xprop_id_calls: 1,
            window_id: Some(window_id),
            detail: error.to_string(),
            ..TargetXpropEnrichment::default()
        },
    }
}

fn parse_target_xprop(window_id: u64, output: &str) -> TargetXpropEnrichment {
    let mut enrichment = TargetXpropEnrichment {
        attempted: true,
        ok: true,
        xprop_id_calls: 1,
        window_id: Some(window_id),
        detail: "target-scoped xprop enrichment parsed".to_string(),
        ..TargetXpropEnrichment::default()
    };
    for line in output.lines() {
        if line.starts_with("_NET_WM_PID") {
            enrichment.net_wm_pid = line
                .rsplit_once('=')
                .and_then(|(_, value)| value.trim().parse().ok());
        } else if line.starts_with("WM_CLIENT_MACHINE") {
            enrichment.wm_client_machine = quoted_values(line).into_iter().next();
        } else if line.starts_with("WM_NAME") {
            enrichment.wm_name = quoted_values(line).into_iter().next();
        } else if line.starts_with("_NET_WM_NAME") {
            enrichment.net_wm_name = quoted_values(line).into_iter().next();
        } else if line.starts_with("WM_CLASS") {
            enrichment.wm_class = quoted_values(line);
        } else if line.starts_with("_NET_WM_WINDOW_TYPE") {
            enrichment.net_wm_window_type = line
                .rsplit_once('=')
                .map(|(_, value)| {
                    value
                        .split(',')
                        .map(|v| v.trim().to_string())
                        .filter(|v| !v.is_empty())
                        .collect()
                })
                .unwrap_or_default();
        }
    }
    enrichment
}

fn quoted_values(line: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find('"') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('"') else {
            break;
        };
        values.push(after[..end].to_string());
        rest = &after[end + 1..];
    }
    values
}

fn bounds_overlap(window: &WindowInfo, candidate: Option<&AccessibilityBounds>) -> bool {
    let Some(window_bounds) = window.bounds.as_ref() else {
        return false;
    };
    let Some(candidate) = candidate else {
        return false;
    };
    if candidate.width <= 0 || candidate.height <= 0 {
        return false;
    }
    let Some(window_x) = window_bounds.x else {
        return false;
    };
    let Some(window_y) = window_bounds.y else {
        return false;
    };
    let left = window_x.max(candidate.x);
    let top = window_y.max(candidate.y);
    let right = (window_x + window_bounds.width as i32).min(candidate.x + candidate.width);
    let bottom = (window_y + window_bounds.height as i32).min(candidate.y + candidate.height);
    right > left && bottom > top
}

fn has_state(states: &[String], expected: &str) -> bool {
    states
        .iter()
        .any(|state| state.eq_ignore_ascii_case(expected))
}

fn normalized(value: Option<&str>) -> Option<String> {
    value
        .map(|value| value.split_whitespace().collect::<Vec<_>>().join(" "))
        .map(|value| value.to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}

fn token_list(value: Option<&str>) -> Vec<String> {
    value
        .map(|value| {
            value
                .chars()
                .map(|ch| {
                    if ch.is_ascii_alphanumeric() {
                        ch.to_ascii_lowercase()
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
                .split_whitespace()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn metadata_for_window(listing: &WindowListReport, window_id: u64) -> Option<WindowMetadata> {
    listing
        .diagnostics
        .window_metadata
        .iter()
        .find(|metadata| metadata.window_id == Some(window_id))
        .cloned()
}

fn collect_atspi_candidates() -> Result<CollectorOutput, String> {
    if !list_windows::command_exists("python3") {
        return Err("python3 is unavailable for AT-SPI collection".to_string());
    }
    let output = Command::new("python3")
        .args(["-c", ATSPI_COLLECTOR_SCRIPT])
        .output()
        .map_err(|err| err.to_string())?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            format!("python3 exited with status {}", output.status)
        } else {
            stderr
        });
    }
    serde_json::from_slice::<CollectorOutput>(&output.stdout)
        .map_err(|err| format!("failed to parse AT-SPI collector JSON: {err}"))
}

fn collector_command() -> String {
    "python3 -c".to_string()
}

fn collector_diagnostics(output: &CollectorOutput) -> CollectorDiagnostics {
    CollectorDiagnostics {
        ok: true,
        command: Some(collector_command()),
        stderr: None,
        detail: output
            .diagnostics
            .detail
            .clone()
            .unwrap_or_else(|| "collector returned candidates".to_string()),
        truncated: output.diagnostics.truncated,
    }
}

fn correlation(
    status: impl Into<String>,
    confidence: impl Into<String>,
    score: i32,
    matched_object_ref: Option<String>,
    reasons: Vec<String>,
    candidates: Vec<AccessibilityCandidateSummary>,
) -> CorrelationResult {
    CorrelationResult {
        status: status.into(),
        confidence: confidence.into(),
        score,
        matched_object_ref,
        reasons,
        candidates,
    }
}

fn failure_report(
    listing: WindowListReport,
    window: Option<WindowInfo>,
    correlation: CorrelationResult,
    tree: Vec<AccessibilityNode>,
    error_code: impl Into<String>,
    note: impl Into<String>,
    collector: CollectorDiagnostics,
    target_xprop: Option<TargetXpropEnrichment>,
) -> AccessibilityTreeReport {
    let error_code = error_code.into();
    let mut blockers = listing.diagnostics.blockers.clone();
    blockers.push(error_code.clone());
    AccessibilityTreeReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        success: false,
        window,
        correlation,
        tree,
        error_code: Some(error_code),
        note: note.into(),
        diagnostics: AccessibilityDiagnostics {
            ok: false,
            blockers,
            degraded_reasons: listing.diagnostics.degraded_reasons.clone(),
            listing: listing.diagnostics,
            collector,
            target_xprop,
        },
    }
}

const ATSPI_COLLECTOR_SCRIPT: &str = r#"
import json
import sys
from collections import deque

MAX_APPS = 200
MAX_CANDIDATES = 240
MAX_NODES = 500
MAX_DEPTH = 8

try:
    import gi
    gi.require_version('Atspi', '2.0')
    from gi.repository import Atspi
except Exception as exc:
    print(json.dumps({"ok": False, "candidates": [], "diagnostics": {"detail": "failed to import gi.repository.Atspi: %s" % exc}}))
    raise SystemExit(0)


def safe(call, default=None):
    try:
        value = call()
        return default if value is None else value
    except Exception:
        return default


def text(value):
    if value is None:
        return None
    value = str(value).strip()
    return value or None


def object_ref(node):
    pid = safe(lambda: node.get_process_id())
    path = safe(lambda: node.path)
    node_id = safe(lambda: node.get_id())
    return "pid:%s:%s:%s" % (pid if pid is not None else "unknown", path or "unknown", node_id if node_id is not None else "unknown")


def bounds(node):
    component = safe(lambda: node.get_component_iface())
    if not component:
        return None
    extents = safe(lambda: component.get_extents(Atspi.CoordType.SCREEN))
    if not extents:
        return None
    if hasattr(extents, 'x'):
        x = int(extents.x)
        y = int(extents.y)
        width = int(extents.width)
        height = int(extents.height)
    else:
        x = int(extents[0] if len(extents) > 0 else 0)
        y = int(extents[1] if len(extents) > 1 else 0)
        width = int(extents[2] if len(extents) > 2 else 0)
        height = int(extents[3] if len(extents) > 3 else 0)
    if width <= 0 or height <= 0:
        return None
    return {"x": x, "y": y, "width": width, "height": height}


def states(node):
    state_set = safe(lambda: node.get_state_set())
    if not state_set:
        return []
    labels = []
    for name in ["ACTIVE", "FOCUSED", "SHOWING", "VISIBLE", "ENABLED", "SENSITIVE", "EDITABLE", "SELECTED"]:
        state = getattr(Atspi.StateType, name, None)
        if state is not None and safe(lambda s=state: state_set.contains(s), False):
            labels.append(name.lower())
    return labels


def actions(node):
    action = safe(lambda: node.get_action_iface())
    if not action:
        return []
    count = int(safe(lambda: action.get_n_actions(), 0) or 0)
    result = []
    for index in range(min(count, 16)):
        result.append({
            "index": index,
            "name": text(safe(lambda i=index: action.get_name(i), "")) or "",
            "description": text(safe(lambda i=index: action.get_description(i), "")) or "",
            "keybinding": text(safe(lambda i=index: action.get_key_binding(i), "")) or "",
        })
    return result


def node_summary(node, index, parent_index, depth):
    node_states = states(node)
    return {
        "index": index,
        "parent_index": parent_index,
        "depth": depth,
        "object_ref": object_ref(node),
        "role": text(safe(lambda: node.get_role_name(), "unknown")) or "unknown",
        "name": text(safe(lambda: node.get_name())),
        "description": text(safe(lambda: node.get_description())),
        "child_count": int(safe(lambda: node.get_child_count(), 0) or 0),
        "bounds": bounds(node),
        "states": node_states,
        "actions": actions(node),
        "supports_editable_text": "editable" in node_states,
    }


def snapshot(root):
    nodes = []
    queue = deque([(root, 0, None)])
    truncated = False
    while queue:
        node, depth, parent = queue.popleft()
        if len(nodes) >= MAX_NODES:
            truncated = True
            break
        index = len(nodes)
        nodes.append(node_summary(node, index, parent, depth))
        if depth >= MAX_DEPTH:
            continue
        child_count = int(safe(lambda n=node: n.get_child_count(), 0) or 0)
        for child_index in range(child_count):
            child = safe(lambda n=node, i=child_index: n.get_child_at_index(i))
            if child is not None:
                queue.append((child, depth + 1, index))
    return nodes, truncated


def candidate_from(node):
    node_states = states(node)
    nodes, truncated = snapshot(node)
    return {
        "object_ref": object_ref(node),
        "name": text(safe(lambda: node.get_name())),
        "role": text(safe(lambda: node.get_role_name(), "unknown")) or "unknown",
        "pid": safe(lambda: node.get_process_id()),
        "bounds": bounds(node),
        "focused": ("focused" in node_states or "active" in node_states),
        "states": node_states,
        "nodes": nodes,
        "_truncated": truncated,
    }


try:
    Atspi.init()
    desktop = Atspi.get_desktop(0)
    candidates = []
    truncated = False
    app_count = min(int(safe(lambda: desktop.get_child_count(), 0) or 0), MAX_APPS)
    for app_index in range(app_count):
        app = safe(lambda i=app_index: desktop.get_child_at_index(i))
        if app is None:
            continue
        for node in [app]:
            candidate = candidate_from(node)
            truncated = truncated or candidate.pop("_truncated", False)
            candidates.append(candidate)
        child_count = min(int(safe(lambda a=app: a.get_child_count(), 0) or 0), 16)
        for child_index in range(child_count):
            child = safe(lambda a=app, i=child_index: a.get_child_at_index(i))
            if child is None:
                continue
            role = (text(safe(lambda c=child: child.get_role_name(), "")) or "").lower()
            if role in ("frame", "window", "dialog", "alert") or bounds(child) is not None:
                candidate = candidate_from(child)
                truncated = truncated or candidate.pop("_truncated", False)
                candidates.append(candidate)
        if len(candidates) >= MAX_CANDIDATES:
            truncated = True
            candidates = candidates[:MAX_CANDIDATES]
            break
    print(json.dumps({"ok": True, "candidates": candidates, "diagnostics": {"truncated": truncated, "detail": "gi.repository.Atspi collector returned %d candidates" % len(candidates)}}))
except Exception as exc:
    print(json.dumps({"ok": False, "candidates": [], "diagnostics": {"detail": "AT-SPI collection failed: %s" % exc}}))
"#;
