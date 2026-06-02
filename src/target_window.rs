use std::{
    fs,
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    doctor::{BACKEND_ID, PROJECT_NAME},
    input::{self, TargetCandidate, WindowTarget},
    list_windows::{self, WindowInfo, WindowListingDiagnostics},
};

#[derive(Debug, Clone)]
pub struct TargetWindowParams {
    pub target: WindowTarget,
    pub group: Option<String>,
    pub color: TargetColor,
    pub overlay: bool,
}

#[derive(Debug, Clone)]
pub struct ReleaseWindowParams {
    pub window_id: Option<u64>,
    pub all: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetColor {
    Blue,
    Purple,
    Green,
    Orange,
    Red,
    Cyan,
}

impl TargetColor {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_ascii_lowercase().as_str() {
            "blue" => Ok(Self::Blue),
            "purple" => Ok(Self::Purple),
            "green" => Ok(Self::Green),
            "orange" => Ok(Self::Orange),
            "red" => Ok(Self::Red),
            "cyan" => Ok(Self::Cyan),
            other => Err(format!(
                "invalid color: {other}; expected blue, purple, green, orange, red, or cyan"
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Purple => "purple",
            Self::Green => "green",
            Self::Orange => "orange",
            Self::Red => "red",
            Self::Cyan => "cyan",
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TargetState {
    pub active_group_id: Option<String>,
    #[serde(default)]
    pub groups: Vec<WindowGroup>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowGroup {
    pub group_id: String,
    pub color: String,
    pub active_window_id: Option<u64>,
    #[serde(default)]
    pub windows: Vec<TrackedTarget>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrackedTarget {
    pub target_id: String,
    pub window: WindowInfo,
    pub color: String,
    pub active: bool,
    pub stale: bool,
    pub targeted_at_ms: u128,
    #[serde(default)]
    pub overlay_pid: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OverlayReport {
    pub requested: bool,
    pub shown: bool,
    pub provider: String,
    pub warning: Option<String>,
    #[serde(default)]
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StaleTarget {
    pub window_id: u64,
    pub target_id: String,
    pub title: Option<String>,
    pub group_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetDiagnostics {
    pub ok: bool,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub candidates: Vec<TargetCandidate>,
    pub stale_removed: Vec<StaleTarget>,
    pub listing: WindowListingDiagnostics,
    pub state_path: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetWindowReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub action: String,
    pub success: bool,
    pub target: Option<TrackedTarget>,
    pub state: TargetState,
    pub overlay: OverlayReport,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: TargetDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetContextReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub action: String,
    pub success: bool,
    pub state: TargetState,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: TargetDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReleaseWindowReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub action: String,
    pub success: bool,
    pub released_count: usize,
    pub released: Vec<TrackedTarget>,
    pub state: TargetState,
    pub overlay: OverlayReport,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: TargetDiagnostics,
}

pub fn target_window_report_from_system(params: TargetWindowParams) -> TargetWindowReport {
    let path = state_path();
    let (mut state, mut load_warnings) = load_state(path.as_ref());
    let listing = list_windows::report_from_system();
    let stale_removed = validate_state(&mut state, &listing.windows);

    if !params.target.has_target() {
        load_warnings.push("target-window requires at least one target selector".to_string());
        let diagnostics = diagnostics(
            &listing,
            Vec::new(),
            stale_removed,
            load_warnings,
            path.as_ref(),
            false,
        );
        let _ = save_state(path.as_ref(), &state);
        return TargetWindowReport {
            project: PROJECT_NAME.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            backend: BACKEND_ID.to_string(),
            action: "target_window".to_string(),
            success: false,
            target: None,
            state,
            overlay: overlay_not_requested(),
            error_code: Some("MissingTarget".to_string()),
            note: "No target selector was provided.".to_string(),
            diagnostics,
        };
    }

    let window = match input::resolve_target(&listing.windows, &params.target) {
        Ok(window) => window,
        Err(error) => {
            let diagnostics = diagnostics(
                &listing,
                error.candidates,
                stale_removed,
                load_warnings,
                path.as_ref(),
                false,
            );
            let _ = save_state(path.as_ref(), &state);
            return TargetWindowReport {
                project: PROJECT_NAME.to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                backend: BACKEND_ID.to_string(),
                action: "target_window".to_string(),
                success: false,
                target: None,
                state,
                overlay: overlay_not_requested(),
                error_code: Some(error.code.to_string()),
                note: error.note,
                diagnostics,
            };
        }
    };

    let group_id = normalize_group(params.group.as_deref());
    let mut tracked = upsert_target(&mut state, window, &group_id, params.color);
    let overlay = if params.overlay {
        show_overlay(&tracked.window, params.color)
    } else {
        overlay_not_requested()
    };
    if overlay.shown {
        tracked.overlay_pid = overlay.pid;
        set_overlay_pid(&mut state, tracked.window.window_id, overlay.pid);
    }
    let mut warnings = load_warnings;
    if let Some(warning) = overlay.warning.clone() {
        warnings.push(warning);
    }
    let save_warning = save_state(path.as_ref(), &state).err();
    if let Some(warning) = save_warning {
        warnings.push(warning);
    }
    let diagnostics = diagnostics(
        &listing,
        Vec::new(),
        stale_removed,
        warnings,
        path.as_ref(),
        true,
    );

    TargetWindowReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: "target_window".to_string(),
        success: true,
        target: Some(tracked),
        state,
        overlay,
        error_code: None,
        note: "Target window saved as explicit session context; targeted input and app-state commands still require explicit selectors.".to_string(),
        diagnostics,
    }
}

pub fn target_context_report_from_system() -> TargetContextReport {
    let path = state_path();
    let (mut state, load_warnings) = load_state(path.as_ref());
    let listing = list_windows::report_from_system();
    let stale_removed = validate_state(&mut state, &listing.windows);
    let mut warnings = load_warnings;
    if let Some(warning) = save_state(path.as_ref(), &state).err() {
        warnings.push(warning);
    }
    TargetContextReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: "target_context".to_string(),
        success: true,
        state,
        error_code: None,
        note: "Target context is explicit session state and is validated against the current window listing before being reported.".to_string(),
        diagnostics: diagnostics(&listing, Vec::new(), stale_removed, warnings, path.as_ref(), true),
    }
}

pub fn release_window_report_from_system(params: ReleaseWindowParams) -> ReleaseWindowReport {
    let path = state_path();
    let (mut state, load_warnings) = load_state(path.as_ref());
    let listing = list_windows::report_from_system();
    let stale_removed = validate_state(&mut state, &listing.windows);
    let mut released = Vec::new();
    let mut success = true;
    let mut error_code = None;
    let mut note = "Released target window state.".to_string();

    if params.all {
        for group in &mut state.groups {
            released.append(&mut group.windows);
            group.active_window_id = None;
        }
    } else if let Some(window_id) = params.window_id {
        for group in &mut state.groups {
            let mut kept = Vec::new();
            for mut target in group.windows.drain(..) {
                if target.window.window_id == window_id {
                    target.active = false;
                    released.push(target);
                } else {
                    kept.push(target);
                }
            }
            group.windows = kept;
            if group.active_window_id == Some(window_id) {
                set_first_active(group);
            }
        }
        if released.is_empty() {
            success = false;
            error_code = Some("TargetNotFound".to_string());
            note = format!("No saved target matched window_id {window_id}.");
        }
    } else {
        success = false;
        error_code = Some("MissingReleaseTarget".to_string());
        note = "release-window requires --window-id or --all.".to_string();
    }

    let mut warnings = load_warnings;
    if let Some(warning) = save_state(path.as_ref(), &state).err() {
        warnings.push(warning);
    }
    let overlay = if params.all || params.window_id.is_some() {
        hide_overlays(&released)
    } else {
        overlay_not_requested()
    };

    ReleaseWindowReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: "release_window".to_string(),
        success,
        released_count: released.len(),
        released,
        state,
        overlay,
        error_code,
        note,
        diagnostics: diagnostics(
            &listing,
            Vec::new(),
            stale_removed,
            warnings,
            path.as_ref(),
            success,
        ),
    }
}

pub fn target_window_report_from_state(
    params: TargetWindowParams,
    state: &mut TargetState,
) -> TargetWindowReport {
    let listing = list_windows::report_from_system();
    let stale_removed = validate_state(state, &listing.windows);

    if !params.target.has_target() {
        return TargetWindowReport {
            project: PROJECT_NAME.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            backend: BACKEND_ID.to_string(),
            action: "target_window".to_string(),
            success: false,
            target: None,
            state: state.clone(),
            overlay: overlay_not_requested(),
            error_code: Some("MissingTarget".to_string()),
            note: "No target selector was provided.".to_string(),
            diagnostics: diagnostics(
                &listing,
                Vec::new(),
                stale_removed,
                vec!["target-window requires at least one target selector".to_string()],
                None,
                false,
            ),
        };
    }

    let window = match input::resolve_target(&listing.windows, &params.target) {
        Ok(window) => window,
        Err(error) => {
            return TargetWindowReport {
                project: PROJECT_NAME.to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                backend: BACKEND_ID.to_string(),
                action: "target_window".to_string(),
                success: false,
                target: None,
                state: state.clone(),
                overlay: overlay_not_requested(),
                error_code: Some(error.code.to_string()),
                note: error.note,
                diagnostics: diagnostics(
                    &listing,
                    error.candidates,
                    stale_removed,
                    Vec::new(),
                    None,
                    false,
                ),
            };
        }
    };

    let group_id = normalize_group(params.group.as_deref());
    let mut tracked = upsert_target(state, window, &group_id, params.color);
    let overlay = if params.overlay {
        show_overlay(&tracked.window, params.color)
    } else {
        overlay_not_requested()
    };
    if overlay.shown {
        tracked.overlay_pid = overlay.pid;
        set_overlay_pid(state, tracked.window.window_id, overlay.pid);
    }
    let mut warnings = Vec::new();
    if let Some(warning) = overlay.warning.clone() {
        warnings.push(warning);
    }

    TargetWindowReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: "target_window".to_string(),
        success: true,
        target: Some(tracked),
        state: state.clone(),
        overlay,
        error_code: None,
        note: "Target window saved as explicit session context; targeted input and app-state commands still require explicit selectors.".to_string(),
        diagnostics: diagnostics(&listing, Vec::new(), stale_removed, warnings, None, true),
    }
}

pub fn target_context_report_from_state(state: &mut TargetState) -> TargetContextReport {
    let listing = list_windows::report_from_system();
    let stale_removed = validate_state(state, &listing.windows);
    TargetContextReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: "target_context".to_string(),
        success: true,
        state: state.clone(),
        error_code: None,
        note: "Target context is explicit MCP process state and is validated against the current window listing before being reported.".to_string(),
        diagnostics: diagnostics(&listing, Vec::new(), stale_removed, Vec::new(), None, true),
    }
}

pub fn release_window_report_from_state(
    params: ReleaseWindowParams,
    state: &mut TargetState,
) -> ReleaseWindowReport {
    let listing = list_windows::report_from_system();
    let stale_removed = validate_state(state, &listing.windows);
    let mut released = Vec::new();
    let mut success = true;
    let mut error_code = None;
    let mut note = "Released target window state.".to_string();

    if params.all {
        for group in &mut state.groups {
            released.append(&mut group.windows);
            group.active_window_id = None;
        }
    } else if let Some(window_id) = params.window_id {
        for group in &mut state.groups {
            let mut kept = Vec::new();
            for mut target in group.windows.drain(..) {
                if target.window.window_id == window_id {
                    target.active = false;
                    released.push(target);
                } else {
                    kept.push(target);
                }
            }
            group.windows = kept;
            if group.active_window_id == Some(window_id) {
                set_first_active(group);
            }
        }
        if released.is_empty() {
            success = false;
            error_code = Some("TargetNotFound".to_string());
            note = format!("No saved target matched window_id {window_id}.");
        }
    } else {
        success = false;
        error_code = Some("MissingReleaseTarget".to_string());
        note = "release-window requires window_id or release_all.".to_string();
    }

    let overlay = if params.all || params.window_id.is_some() {
        hide_overlays(&released)
    } else {
        overlay_not_requested()
    };

    ReleaseWindowReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: "release_window".to_string(),
        success,
        released_count: released.len(),
        released,
        state: state.clone(),
        overlay,
        error_code,
        note,
        diagnostics: diagnostics(
            &listing,
            Vec::new(),
            stale_removed,
            Vec::new(),
            None,
            success,
        ),
    }
}

fn upsert_target(
    state: &mut TargetState,
    window: WindowInfo,
    group_id: &str,
    color: TargetColor,
) -> TrackedTarget {
    remove_window_from_other_groups(state, window.window_id, group_id);
    state.active_group_id = Some(group_id.to_string());
    let group = ensure_group(state, group_id, color);
    for existing in &mut group.windows {
        existing.active = false;
    }
    let tracked = TrackedTarget {
        target_id: target_id(window.window_id),
        window,
        color: color.as_str().to_string(),
        active: true,
        stale: false,
        targeted_at_ms: now_ms(),
        overlay_pid: None,
    };
    if let Some(existing) = group
        .windows
        .iter_mut()
        .find(|item| item.window.window_id == tracked.window.window_id)
    {
        *existing = tracked.clone();
    } else {
        group.windows.push(tracked.clone());
    }
    group.active_window_id = Some(tracked.window.window_id);
    tracked
}

fn ensure_group<'a>(
    state: &'a mut TargetState,
    group_id: &str,
    color: TargetColor,
) -> &'a mut WindowGroup {
    if let Some(index) = state
        .groups
        .iter()
        .position(|group| group.group_id == group_id)
    {
        state.groups[index].color = color.as_str().to_string();
        return &mut state.groups[index];
    }
    state.groups.push(WindowGroup {
        group_id: group_id.to_string(),
        color: color.as_str().to_string(),
        active_window_id: None,
        windows: Vec::new(),
    });
    state.groups.last_mut().unwrap()
}

fn remove_window_from_other_groups(state: &mut TargetState, window_id: u64, target_group: &str) {
    for group in &mut state.groups {
        if group.group_id == target_group {
            continue;
        }
        group
            .windows
            .retain(|target| target.window.window_id != window_id);
        if group.active_window_id == Some(window_id) {
            set_first_active(group);
        }
    }
}

fn validate_state(state: &mut TargetState, windows: &[WindowInfo]) -> Vec<StaleTarget> {
    let mut stale = Vec::new();
    for group in &mut state.groups {
        let mut kept = Vec::new();
        for mut target in group.windows.drain(..) {
            if windows
                .iter()
                .any(|window| window.window_id == target.window.window_id)
            {
                target.stale = false;
                kept.push(target);
            } else {
                target.stale = true;
                stale.push(StaleTarget {
                    window_id: target.window.window_id,
                    target_id: target.target_id.clone(),
                    title: target.window.title.clone(),
                    group_id: group.group_id.clone(),
                });
            }
        }
        group.windows = kept;
        if group.active_window_id.is_some_and(|active| {
            !group
                .windows
                .iter()
                .any(|target| target.window.window_id == active)
        }) {
            set_first_active(group);
        }
    }
    stale
}

fn set_first_active(group: &mut WindowGroup) {
    for target in &mut group.windows {
        target.active = false;
    }
    if let Some(first) = group.windows.first_mut() {
        first.active = true;
        group.active_window_id = Some(first.window.window_id);
    } else {
        group.active_window_id = None;
    }
}

fn diagnostics(
    listing: &list_windows::WindowListReport,
    candidates: Vec<TargetCandidate>,
    stale_removed: Vec<StaleTarget>,
    mut degraded_reasons: Vec<String>,
    path: Option<&PathBuf>,
    ok: bool,
) -> TargetDiagnostics {
    degraded_reasons.extend(listing.diagnostics.degraded_reasons.clone());
    TargetDiagnostics {
        ok: ok && listing.diagnostics.blockers.is_empty(),
        blockers: listing.diagnostics.blockers.clone(),
        degraded_reasons,
        candidates,
        stale_removed,
        listing: listing.diagnostics.clone(),
        state_path: path.map(|path| path.display().to_string()),
    }
}

fn normalize_group(value: Option<&str>) -> String {
    let value = value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("default");
    let normalized = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    let trimmed = normalized.trim_matches('-');
    if trimmed.is_empty() {
        "default".to_string()
    } else {
        trimmed.to_string()
    }
}

fn target_id(window_id: u64) -> String {
    format!("x11-0x{window_id:x}")
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

fn overlay_not_requested() -> OverlayReport {
    OverlayReport {
        requested: false,
        shown: false,
        provider: "none".to_string(),
        warning: None,
        pid: None,
    }
}

fn overlay_unsupported(requested: bool) -> OverlayReport {
    OverlayReport {
        requested,
        shown: false,
        provider: "no-overlay".to_string(),
        warning: Some(
            "visual overlay provider is unavailable; target state was saved without a border"
                .to_string(),
        ),
        pid: None,
    }
}

fn show_overlay(window: &WindowInfo, color: TargetColor) -> OverlayReport {
    if let Some(path) = std::env::var_os("CODEX_X11_OVERLAY_LOG") {
        let path = PathBuf::from(path);
        let bounds = window
            .bounds
            .as_ref()
            .map(|b| format!("x={:?} y={:?} w={} h={}", b.x, b.y, b.width, b.height))
            .unwrap_or_else(|| "bounds=unknown".to_string());
        let line = format!(
            "show window={} color={} {bounds}
",
            window.window_id,
            color.as_str()
        );
        let result = append_overlay_log(&path, &line);
        return OverlayReport {
            requested: true,
            shown: result.is_ok(),
            provider: "test-overlay-log".to_string(),
            warning: result.err(),
            pid: None,
        };
    }
    if std::env::var("CODEX_X11_ENABLE_TK_OVERLAY").ok().as_deref() == Some("1") {
        return show_tk_overlay_helper(window, color);
    }
    overlay_unsupported(true)
}

fn show_tk_overlay_helper(window: &WindowInfo, color: TargetColor) -> OverlayReport {
    let Some(bounds) = window.bounds.as_ref() else {
        return overlay_unsupported(true);
    };
    let (Some(x), Some(y)) = (bounds.x, bounds.y) else {
        return overlay_unsupported(true);
    };
    if !list_windows::command_exists("python3") {
        return overlay_unsupported(true);
    }
    let color = match color {
        TargetColor::Blue => "#3b82f6",
        TargetColor::Purple => "#a855f7",
        TargetColor::Green => "#22c55e",
        TargetColor::Orange => "#f97316",
        TargetColor::Red => "#ef4444",
        TargetColor::Cyan => "#06b6d4",
    };
    let child = std::process::Command::new("python3")
        .args([
            "-c",
            TK_OVERLAY_HELPER,
            &x.to_string(),
            &y.to_string(),
            &bounds.width.to_string(),
            &bounds.height.to_string(),
            color,
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    match child {
        Ok(child) => OverlayReport {
            requested: true,
            shown: true,
            provider: "python-tk-overlay-helper".to_string(),
            warning: None,
            pid: Some(child.id()),
        },
        Err(error) => OverlayReport {
            requested: true,
            shown: false,
            provider: "python-tk-overlay-helper".to_string(),
            warning: Some(error.to_string()),
            pid: None,
        },
    }
}

fn hide_overlays(targets: &[TrackedTarget]) -> OverlayReport {
    if let Some(path) = std::env::var_os("CODEX_X11_OVERLAY_LOG") {
        let path = PathBuf::from(path);
        let mut warning = None;
        for target in targets {
            if let Err(error) = append_overlay_log(
                &path,
                &format!(
                    "hide window={}
",
                    target.window.window_id
                ),
            ) {
                warning = Some(error);
            }
        }
        return OverlayReport {
            requested: true,
            shown: false,
            provider: "test-overlay-log".to_string(),
            warning,
            pid: None,
        };
    }
    if targets.iter().any(|target| target.overlay_pid.is_some()) {
        let mut warnings = Vec::new();
        for target in targets {
            if let Some(pid) = target.overlay_pid {
                let status = std::process::Command::new("kill")
                    .arg(pid.to_string())
                    .status();
                if let Err(error) = status {
                    warnings.push(format!("failed to hide overlay pid {pid}: {error}"));
                }
            }
        }
        OverlayReport {
            requested: true,
            shown: false,
            provider: "python-tk-overlay-helper".to_string(),
            warning: (!warnings.is_empty()).then(|| warnings.join("; ")),
            pid: None,
        }
    } else {
        overlay_unsupported(true)
    }
}

fn append_overlay_log(path: &PathBuf, line: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    file.write_all(line.as_bytes())
        .map_err(|error| error.to_string())
}

fn set_overlay_pid(state: &mut TargetState, window_id: u64, pid: Option<u32>) {
    for group in &mut state.groups {
        for target in &mut group.windows {
            if target.window.window_id == window_id {
                target.overlay_pid = pid;
            }
        }
    }
}

fn state_path() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("CODEX_X11_TARGET_STATE") {
        return Some(PathBuf::from(path));
    }
    let base = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    Some(
        base.join("codex-computer-use-x11")
            .join("target-state.json"),
    )
}

fn load_state(path: Option<&PathBuf>) -> (TargetState, Vec<String>) {
    let Some(path) = path else {
        return (TargetState::default(), Vec::new());
    };
    match fs::read(path) {
        Ok(bytes) if bytes.is_empty() => (TargetState::default(), Vec::new()),
        Ok(bytes) => match serde_json::from_slice::<TargetState>(&bytes) {
            Ok(state) => (state, Vec::new()),
            Err(error) => (
                TargetState::default(),
                vec![format!(
                    "target state file {} could not be parsed and was reset: {error}",
                    path.display()
                )],
            ),
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            (TargetState::default(), Vec::new())
        }
        Err(error) => (
            TargetState::default(),
            vec![format!(
                "target state file {} could not be read and was reset: {error}",
                path.display()
            )],
        ),
    }
}

fn save_state(path: Option<&PathBuf>, state: &TargetState) -> Result<(), String> {
    let Some(path) = path else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "failed to create target state directory {}: {error}",
                parent.display()
            )
        })?;
    }
    let bytes = serde_json::to_vec_pretty(state)
        .map_err(|error| format!("failed to serialize target state: {error}"))?;
    fs::write(path, bytes).map_err(|error| {
        format!(
            "failed to write target state file {}: {error}",
            path.display()
        )
    })
}

const TK_OVERLAY_HELPER: &str = r#"
import sys, tkinter as tk
x, y, w, h = map(int, sys.argv[1:5])
color = sys.argv[5]
thickness = 4
root = tk.Tk(className='codex-computer-use-x11-overlay')
root.withdraw()
windows = []
for name, geo in [
    ('top', (x, y, w, thickness)),
    ('bottom', (x, y + max(0, h - thickness), w, thickness)),
    ('left', (x, y, thickness, h)),
    ('right', (x + max(0, w - thickness), y, thickness, h)),
]:
    win = tk.Toplevel(root, class_='codex-computer-use-x11-overlay')
    win.title('codex-computer-use-x11-overlay')
    win.overrideredirect(True)
    try:
        win.attributes('-topmost', True)
    except Exception:
        pass
    gx, gy, gw, gh = geo
    win.geometry(f'{gw}x{gh}+{gx}+{gy}')
    win.configure(bg=color)
    windows.append(win)
root.mainloop()
"#;
