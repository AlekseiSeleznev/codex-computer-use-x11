use crate::doctor::{BACKEND_ID, PROJECT_NAME};
use crate::list_windows::{self, CommandStatus, WindowInfo, WindowListingDiagnostics};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveWindowState {
    Active(u64),
    NoActive,
    Missing,
    Invalid,
}

impl ActiveWindowState {
    pub fn active_window(self) -> Option<u64> {
        match self {
            ActiveWindowState::Active(id) => Some(id),
            ActiveWindowState::NoActive
            | ActiveWindowState::Missing
            | ActiveWindowState::Invalid => None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FocusedWindowReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub focused_window: Option<WindowInfo>,
    pub diagnostics: FocusDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FocusWindowReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub success: bool,
    pub requested_window: Option<WindowInfo>,
    pub focused_window: Option<WindowInfo>,
    pub exact_window_focused: bool,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: FocusDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FocusDiagnostics {
    pub ok: bool,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub active_window: Option<u64>,
    pub commands: Vec<CommandStatus>,
    pub activation_attempts: Vec<ActivationAttempt>,
    pub listing: WindowListingDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivationAttempt {
    pub command: String,
    pub args: Vec<String>,
    pub ok: bool,
    pub detail: String,
}

pub fn parse_active_window_xprop_state(output: &str) -> ActiveWindowState {
    let Some(line) = output
        .lines()
        .find(|line| line.trim_start().starts_with("_NET_ACTIVE_WINDOW"))
    else {
        return ActiveWindowState::Missing;
    };

    let Some(value) = line.split('#').nth(1).map(str::trim) else {
        return ActiveWindowState::Invalid;
    };

    match crate::x11_id::parse_x11_window_id(value) {
        Ok(0) => ActiveWindowState::NoActive,
        Ok(id) => ActiveWindowState::Active(id),
        Err(_) => ActiveWindowState::Invalid,
    }
}

pub fn focused_report_from_system() -> FocusedWindowReport {
    focused_report_from_listing(list_windows::report_from_system())
}

pub fn focused_report_from_listing(listing: list_windows::WindowListReport) -> FocusedWindowReport {
    let active_window = listing.diagnostics.focused_window.filter(|id| *id != 0);
    let mut degraded_reasons = listing.diagnostics.degraded_reasons.clone();
    let focused_window = active_window.and_then(|active| {
        listing
            .windows
            .iter()
            .find(|window| window.window_id == active)
            .cloned()
            .map(|mut window| {
                window.focused = true;
                window
            })
    });

    if listing.diagnostics.focused_window == Some(0) {
        degraded_reasons.push("no active X11 window was reported".to_string());
    } else if active_window.is_some() && focused_window.is_none() {
        degraded_reasons
            .push("active window could not be matched to the current window listing".to_string());
    }

    FocusedWindowReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        focused_window,
        diagnostics: FocusDiagnostics {
            ok: listing.diagnostics.blockers.is_empty(),
            blockers: listing.diagnostics.blockers.clone(),
            degraded_reasons,
            active_window,
            commands: listing.diagnostics.commands.clone(),
            activation_attempts: Vec::new(),
            listing: listing.diagnostics,
        },
    }
}

pub fn focus_window_report_from_system(requested_window_id: u64) -> FocusWindowReport {
    focus_window_report_from_listing(requested_window_id, list_windows::report_from_system())
}

pub fn focus_window_report_from_listing(
    requested_window_id: u64,
    listing: list_windows::WindowListReport,
) -> FocusWindowReport {
    let requested_window = listing
        .windows
        .iter()
        .find(|window| window.window_id == requested_window_id)
        .cloned();

    let Some(requested_window) = requested_window else {
        let active_window = listing.diagnostics.focused_window.filter(|id| *id != 0);
        return FocusWindowReport {
            project: PROJECT_NAME.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            backend: BACKEND_ID.to_string(),
            success: false,
            requested_window: None,
            focused_window: focused_window_from_listing(&listing, active_window),
            exact_window_focused: false,
            error_code: Some("WindowNotFound".to_string()),
            note: format!("No window matched window_id {requested_window_id}."),
            diagnostics: build_diagnostics(
                &listing,
                active_window,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        };
    };

    let mut activation_attempts = Vec::new();
    let mut degraded_reasons = Vec::new();
    let mut blockers = Vec::new();
    let mut observed_active = listing.diagnostics.focused_window.filter(|id| *id != 0);

    if !list_windows::command_exists("wmctrl") {
        degraded_reasons.push("wmctrl is unavailable for focus activation".to_string());
    } else {
        let attempt =
            run_activation_command("wmctrl", &["-ia", &format_x11_hex(requested_window_id)]);
        let attempt_ok = attempt.ok;
        activation_attempts.push(attempt);
        if attempt_ok {
            match lookup_active_window_from_system() {
                Ok(active) => {
                    observed_active = active;
                    if active == Some(requested_window_id) {
                        return FocusWindowReport {
                            project: PROJECT_NAME.to_string(),
                            version: env!("CARGO_PKG_VERSION").to_string(),
                            backend: BACKEND_ID.to_string(),
                            success: true,
                            requested_window: Some(requested_window),
                            focused_window: focused_window_from_listing(&listing, active),
                            exact_window_focused: true,
                            error_code: None,
                            note: "focus activation verified through a fresh active-window lookup"
                                .to_string(),
                            diagnostics: build_diagnostics(
                                &listing,
                                active,
                                blockers,
                                degraded_reasons,
                                activation_attempts,
                            ),
                        };
                    }
                    degraded_reasons.push(focus_mismatch_detail(active, requested_window_id));
                }
                Err(err) => degraded_reasons.push(err),
            }
        }
    }

    if !list_windows::command_exists("xdotool") {
        degraded_reasons.push("xdotool is unavailable for fallback focus activation".to_string());
    } else {
        let decimal_id = requested_window_id.to_string();
        let attempt = run_activation_command("xdotool", &["windowactivate", "--sync", &decimal_id]);
        let attempt_ok = attempt.ok;
        activation_attempts.push(attempt);
        if attempt_ok {
            match lookup_active_window_from_system() {
                Ok(active) => {
                    observed_active = active;
                    if active == Some(requested_window_id) {
                        return FocusWindowReport {
                            project: PROJECT_NAME.to_string(),
                            version: env!("CARGO_PKG_VERSION").to_string(),
                            backend: BACKEND_ID.to_string(),
                            success: true,
                            requested_window: Some(requested_window),
                            focused_window: focused_window_from_listing(&listing, active),
                            exact_window_focused: true,
                            error_code: None,
                            note: "focus activation verified through xdotool fallback and a fresh active-window lookup".to_string(),
                            diagnostics: build_diagnostics(
                                &listing,
                                active,
                                blockers,
                                degraded_reasons,
                                activation_attempts,
                            ),
                        };
                    }
                    degraded_reasons.push(focus_mismatch_detail(active, requested_window_id));
                }
                Err(err) => degraded_reasons.push(err),
            }
        }
    }

    let diagnostics = build_diagnostics(
        &listing,
        observed_active,
        std::mem::take(&mut blockers),
        std::mem::take(&mut degraded_reasons),
        activation_attempts,
    );
    FocusWindowReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        success: false,
        requested_window: Some(requested_window),
        focused_window: focused_window_from_listing(&listing, diagnostics.active_window),
        exact_window_focused: false,
        error_code: Some("FocusNotVerified".to_string()),
        note: "focus activation was not verified for the requested window".to_string(),
        diagnostics,
    }
}

fn build_diagnostics(
    listing: &list_windows::WindowListReport,
    active_window: Option<u64>,
    mut blockers: Vec<String>,
    mut degraded_reasons: Vec<String>,
    activation_attempts: Vec<ActivationAttempt>,
) -> FocusDiagnostics {
    blockers.extend(listing.diagnostics.blockers.clone());
    degraded_reasons.extend(listing.diagnostics.degraded_reasons.clone());
    FocusDiagnostics {
        ok: blockers.is_empty(),
        blockers,
        degraded_reasons,
        active_window,
        commands: listing.diagnostics.commands.clone(),
        activation_attempts,
        listing: listing.diagnostics.clone(),
    }
}

fn focused_window_from_listing(
    listing: &list_windows::WindowListReport,
    active_window: Option<u64>,
) -> Option<WindowInfo> {
    active_window.and_then(|active| {
        listing
            .windows
            .iter()
            .find(|window| window.window_id == active)
            .cloned()
            .map(|mut window| {
                window.focused = true;
                window
            })
    })
}

fn format_x11_hex(window_id: u64) -> String {
    format!("0x{window_id:x}")
}

fn focus_mismatch_detail(active: Option<u64>, requested_window_id: u64) -> String {
    match active {
        Some(active) => {
            format!("active window {active} did not match requested window {requested_window_id}")
        }
        None => {
            format!("no active X11 window matched requested window {requested_window_id}")
        }
    }
}

fn run_activation_command(command: &str, args: &[&str]) -> ActivationAttempt {
    let output = std::process::Command::new(command).args(args).output();
    match output {
        Ok(output) => ActivationAttempt {
            command: command.to_string(),
            args: args.iter().map(|arg| (*arg).to_string()).collect(),
            ok: output.status.success(),
            detail: command_detail(&output),
        },
        Err(err) => ActivationAttempt {
            command: command.to_string(),
            args: args.iter().map(|arg| (*arg).to_string()).collect(),
            ok: false,
            detail: err.to_string(),
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

fn lookup_active_window_from_system() -> Result<Option<u64>, String> {
    let output = std::process::Command::new("xprop")
        .args(["-root", "_NET_ACTIVE_WINDOW"])
        .output()
        .map_err(|err| format!("active window lookup failed: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "active window lookup failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    match parse_active_window_xprop_state(&String::from_utf8_lossy(&output.stdout)) {
        ActiveWindowState::Active(id) => Ok(Some(id)),
        ActiveWindowState::NoActive => Ok(None),
        ActiveWindowState::Missing | ActiveWindowState::Invalid => {
            Err("active window id was not found in xprop output".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_active_window_xprop_state, ActiveWindowState};

    #[test]
    fn active_window_parser_classifies_states() {
        assert_eq!(
            parse_active_window_xprop_state("_NET_ACTIVE_WINDOW(WINDOW): window id # 0x123456\n"),
            ActiveWindowState::Active(0x123456)
        );
        assert_eq!(
            parse_active_window_xprop_state("_NET_ACTIVE_WINDOW(WINDOW): window id # 0x0\n"),
            ActiveWindowState::NoActive
        );
        assert_eq!(
            parse_active_window_xprop_state("_NET_SUPPORTED(ATOM) = _NET_ACTIVE_WINDOW\n"),
            ActiveWindowState::Missing
        );
        assert_eq!(
            parse_active_window_xprop_state("_NET_ACTIVE_WINDOW(WINDOW): window id # nope\n"),
            ActiveWindowState::Invalid
        );
    }
}
