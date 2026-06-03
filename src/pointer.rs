use crate::doctor::{BACKEND_ID, PROJECT_NAME};
use crate::focus::FocusWindowReport;
use crate::input::{self, TargetCandidate, WindowTarget};
use crate::list_windows::{self, WindowInfo, WindowListingDiagnostics};

const MAX_DRAG_AXIS_DELTA: i64 = 4096;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub enum PointerAction {
    Click {
        point: Point,
        button: u8,
        count: u32,
    },
    Scroll {
        point: Point,
        button: u8,
        amount: u32,
    },
    Drag {
        start: Point,
        end: Point,
        button: u8,
    },
}

impl PointerAction {
    fn name(&self) -> &'static str {
        match self {
            PointerAction::Click { .. } => "click",
            PointerAction::Scroll { .. } => "scroll",
            PointerAction::Drag { .. } => "drag",
        }
    }

    fn points(&self) -> Vec<Point> {
        match self {
            PointerAction::Click { point, .. } | PointerAction::Scroll { point, .. } => {
                vec![*point]
            }
            PointerAction::Drag { start, end, .. } => vec![*start, *end],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PointerActionReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub action: String,
    pub success: bool,
    pub input_sent: bool,
    pub targeted: bool,
    pub verification_mode: String,
    pub target: Option<WindowInfo>,
    pub focus: Option<FocusWindowReport>,
    pub pointer: Option<PointerAttempt>,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: PointerDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PointerDiagnostics {
    pub ok: bool,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub candidates: Vec<TargetCandidate>,
    pub listing: Option<WindowListingDiagnostics>,
    pub validation: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PointerAttempt {
    pub command: String,
    pub args: Vec<String>,
    pub ok: bool,
    pub detail: String,
    pub active_context: bool,
    pub used_direct_window: bool,
    pub global_injector: bool,
}

pub fn click_report_from_system(
    target: WindowTarget,
    global: bool,
    point: Point,
    button: u8,
    count: u32,
) -> PointerActionReport {
    report_from_listing(
        PointerAction::Click {
            point,
            button,
            count: clamp_click_count(count),
        },
        target,
        global,
        list_windows::report_from_system(),
    )
}

pub fn scroll_report_from_system(
    target: WindowTarget,
    global: bool,
    point: Point,
    button: u8,
    amount: u32,
) -> PointerActionReport {
    report_from_listing(
        PointerAction::Scroll {
            point,
            button,
            amount: clamp_scroll_amount(amount),
        },
        target,
        global,
        list_windows::report_from_system(),
    )
}

pub fn drag_report_from_system(
    target: WindowTarget,
    global: bool,
    start: Point,
    end: Point,
    button: u8,
) -> PointerActionReport {
    report_from_listing(
        PointerAction::Drag { start, end, button },
        target,
        global,
        list_windows::report_from_system(),
    )
}

pub fn report_from_listing(
    action: PointerAction,
    target: WindowTarget,
    global: bool,
    listing: list_windows::WindowListReport,
) -> PointerActionReport {
    if !target.has_target() {
        if global {
            if let Err((code, note)) = validate_global_action(&action) {
                return failure(
                    &action,
                    None,
                    None,
                    None,
                    code,
                    note,
                    Vec::new(),
                    Some(listing.diagnostics),
                    false,
                    "global_unverified",
                    vec!["global pointer action is not window-isolated".to_string()],
                );
            }
            if !list_windows::command_exists("xdotool") {
                return failure(
                    &action,
                    None,
                    None,
                    None,
                    "InputBackendUnavailable",
                    "Did not send global pointer input because xdotool is unavailable for standalone pointer injection.",
                    Vec::new(),
                    Some(listing.diagnostics),
                    false,
                    "global_unverified",
                    vec!["global pointer action is not window-isolated".to_string()],
                );
            }
            let pointer = run_pointer_backend(&action);
            if pointer.ok {
                return success_with_optional_target(
                    &action,
                    listing,
                    None,
                    None,
                    pointer,
                    false,
                    "global_unverified",
                    vec!["global pointer action is not window-isolated".to_string()],
                );
            }
            return failure(
                &action,
                None,
                None,
                Some(pointer),
                "InputBackendFailed",
                "Global pointer backend command failed.",
                Vec::new(),
                Some(listing.diagnostics),
                false,
                "global_unverified",
                vec!["global pointer action is not window-isolated".to_string()],
            );
        }
        return failure(
            &action,
            None,
            None,
            None,
            "MissingTarget",
            "No target selector was provided; pointer injection is not window-isolated unless --global is explicit.",
            Vec::new(),
            Some(listing.diagnostics),
            true,
            "targeted_focus_required",
            Vec::new(),
        );
    }

    let requested_window = match input::resolve_target(&listing.windows, &target) {
        Ok(window) => window,
        Err(error) => {
            return failure(
                &action,
                None,
                None,
                None,
                error.code,
                error.note,
                error.candidates,
                Some(listing.diagnostics),
                true,
                "targeted_focus_required",
                Vec::new(),
            );
        }
    };

    let validation = validate_points(&action, &requested_window);
    if let Err((code, note)) = validation {
        return failure(
            &action,
            Some(requested_window),
            None,
            None,
            code,
            note,
            Vec::new(),
            Some(listing.diagnostics),
            true,
            "targeted_focus_required",
            Vec::new(),
        );
    }

    let focus =
        crate::focus::focus_window_report_from_listing(requested_window.window_id, listing.clone());
    if !focus.success || !focus.exact_window_focused {
        return failure(
            &action,
            Some(requested_window),
            Some(focus),
            None,
            "FocusNotVerified",
            "Did not send pointer input because exact target-window focus verification failed.",
            Vec::new(),
            Some(listing.diagnostics),
            true,
            "targeted_focus_required",
            Vec::new(),
        );
    }

    if !list_windows::command_exists("xdotool") {
        return failure(
            &action,
            Some(requested_window),
            Some(focus),
            None,
            "InputBackendUnavailable",
            "Did not send pointer input because xdotool is unavailable for standalone pointer injection.",
            Vec::new(),
            Some(listing.diagnostics),
            true,
            "targeted_focus_verified",
            Vec::new(),
        );
    }

    let pointer = run_pointer_backend(&action);
    if pointer.ok {
        success_with_optional_target(
            &action,
            listing,
            Some(requested_window),
            Some(focus),
            pointer,
            true,
            "targeted_focus_verified",
            Vec::new(),
        )
    } else {
        failure(
            &action,
            Some(requested_window),
            Some(focus),
            Some(pointer),
            "InputBackendFailed",
            "Pointer backend command failed after focus verification.",
            Vec::new(),
            Some(listing.diagnostics),
            true,
            "targeted_focus_verified",
            Vec::new(),
        )
    }
}

fn validate_points(
    action: &PointerAction,
    window: &WindowInfo,
) -> Result<(), (&'static str, String)> {
    let Some(bounds) = &window.bounds else {
        return Err((
            "MissingBounds",
            "Target window has no known bounds for pointer validation.".to_string(),
        ));
    };
    let (Some(left), Some(top)) = (bounds.x, bounds.y) else {
        return Err((
            "MissingBounds",
            "Target window bounds have unknown origin for pointer validation.".to_string(),
        ));
    };
    let right = i64::from(left) + i64::from(bounds.width);
    let bottom = i64::from(top) + i64::from(bounds.height);
    for point in action.points() {
        let x = i64::from(point.x);
        let y = i64::from(point.y);
        if x < i64::from(left) || x >= right || y < i64::from(top) || y >= bottom {
            return Err((
                "PointOutsideTargetBounds",
                format!(
                    "Point ({}, {}) is outside target window bounds ({}, {}, {}, {}).",
                    point.x, point.y, left, top, bounds.width, bounds.height
                ),
            ));
        }
    }
    if let PointerAction::Drag { start, end, .. } = action {
        let dx = (i64::from(end.x) - i64::from(start.x)).abs();
        let dy = (i64::from(end.y) - i64::from(start.y)).abs();
        if dx > MAX_DRAG_AXIS_DELTA || dy > MAX_DRAG_AXIS_DELTA {
            return Err((
                "DragDistanceTooLarge",
                format!(
                    "Drag delta ({dx}, {dy}) exceeds the safety limit of {MAX_DRAG_AXIS_DELTA} pixels per axis."
                ),
            ));
        }
    }
    Ok(())
}

fn validate_global_action(action: &PointerAction) -> Result<(), (&'static str, String)> {
    if let PointerAction::Drag { start, end, .. } = action {
        let dx = (i64::from(end.x) - i64::from(start.x)).abs();
        let dy = (i64::from(end.y) - i64::from(start.y)).abs();
        if dx > MAX_DRAG_AXIS_DELTA || dy > MAX_DRAG_AXIS_DELTA {
            return Err((
                "DragDistanceTooLarge",
                format!(
                    "Drag delta ({dx}, {dy}) exceeds the safety limit of {MAX_DRAG_AXIS_DELTA} pixels per axis."
                ),
            ));
        }
    }
    Ok(())
}

fn clamp_click_count(count: u32) -> u32 {
    count.clamp(1, 10)
}

fn clamp_scroll_amount(amount: u32) -> u32 {
    amount.clamp(1, 20)
}

fn run_pointer_backend(action: &PointerAction) -> PointerAttempt {
    let args = match action {
        PointerAction::Click {
            point,
            button,
            count,
        } => vec![
            "mousemove".to_string(),
            "--sync".to_string(),
            point.x.to_string(),
            point.y.to_string(),
            "click".to_string(),
            "--repeat".to_string(),
            count.to_string(),
            button.to_string(),
        ],
        PointerAction::Scroll {
            point,
            button,
            amount,
        } => vec![
            "mousemove".to_string(),
            "--sync".to_string(),
            point.x.to_string(),
            point.y.to_string(),
            "click".to_string(),
            "--repeat".to_string(),
            amount.to_string(),
            button.to_string(),
        ],
        PointerAction::Drag { start, end, button } => vec![
            "mousemove".to_string(),
            "--sync".to_string(),
            start.x.to_string(),
            start.y.to_string(),
            "mousedown".to_string(),
            button.to_string(),
            "mousemove".to_string(),
            "--sync".to_string(),
            end.x.to_string(),
            end.y.to_string(),
            "mouseup".to_string(),
            button.to_string(),
        ],
    };
    let output = std::process::Command::new("xdotool").args(&args).output();
    match output {
        Ok(output) => PointerAttempt {
            command: "xdotool".to_string(),
            args,
            ok: output.status.success(),
            detail: command_detail(&output),
            active_context: true,
            used_direct_window: false,
            global_injector: true,
        },
        Err(err) => PointerAttempt {
            command: "xdotool".to_string(),
            args,
            ok: false,
            detail: err.to_string(),
            active_context: true,
            used_direct_window: false,
            global_injector: true,
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn success_with_optional_target(
    action: &PointerAction,
    listing: list_windows::WindowListReport,
    target: Option<WindowInfo>,
    focus: Option<FocusWindowReport>,
    pointer: PointerAttempt,
    targeted: bool,
    verification_mode: impl Into<String>,
    mut degraded_reasons: Vec<String>,
) -> PointerActionReport {
    degraded_reasons.extend(listing.diagnostics.degraded_reasons.clone());
    PointerActionReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: action.name().to_string(),
        success: true,
        input_sent: true,
        targeted,
        verification_mode: verification_mode.into(),
        target,
        focus,
        pointer: Some(pointer),
        error_code: None,
        note: "Pointer input was sent through active-context xdotool only after bounds and exact X11 active-window focus verification; xdotool --window direct events were not used as a safety boundary."
            .to_string(),
        diagnostics: PointerDiagnostics {
            ok: listing.diagnostics.blockers.is_empty(),
            blockers: listing.diagnostics.blockers.clone(),
            degraded_reasons,
            candidates: Vec::new(),
            listing: Some(listing.diagnostics),
            validation: Vec::new(),
        },
    }
}

fn command_detail(output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stderr.is_empty() {
        return stderr;
    }
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !stdout.is_empty() {
        return stdout;
    }
    if output.status.success() {
        "command exited successfully".to_string()
    } else {
        format!("command exited with status {}", output.status)
    }
}

#[allow(clippy::too_many_arguments)]
fn failure(
    action: &PointerAction,
    target: Option<WindowInfo>,
    focus: Option<FocusWindowReport>,
    pointer: Option<PointerAttempt>,
    error_code: impl Into<String>,
    note: impl Into<String>,
    candidates: Vec<TargetCandidate>,
    listing: Option<WindowListingDiagnostics>,
    targeted: bool,
    verification_mode: impl Into<String>,
    mut degraded_reasons: Vec<String>,
) -> PointerActionReport {
    let error_code = error_code.into();
    let mut blockers = listing
        .as_ref()
        .map(|listing| listing.blockers.clone())
        .unwrap_or_default();
    blockers.push(error_code.clone());
    if let Some(listing) = &listing {
        degraded_reasons.extend(listing.degraded_reasons.clone());
    }
    PointerActionReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: action.name().to_string(),
        success: false,
        input_sent: false,
        targeted,
        verification_mode: verification_mode.into(),
        target,
        focus,
        pointer,
        error_code: Some(error_code),
        note: note.into(),
        diagnostics: PointerDiagnostics {
            ok: false,
            blockers,
            degraded_reasons,
            candidates,
            listing,
            validation: Vec::new(),
        },
    }
}
