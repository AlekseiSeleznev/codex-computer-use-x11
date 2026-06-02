use std::{io::Write, process::Stdio};

use crate::doctor::{BACKEND_ID, PROJECT_NAME};
use crate::focus::FocusWindowReport;
use crate::list_windows::{self, WindowInfo, WindowListingDiagnostics};

#[derive(Debug, Clone, Default)]
pub struct WindowTarget {
    pub window_id: Option<u64>,
    pub title: Option<String>,
    pub wm_class: Option<String>,
    pub pid: Option<u32>,
}

impl WindowTarget {
    pub fn has_target(&self) -> bool {
        self.window_id.is_some()
            || self.pid.is_some()
            || self
                .title
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
            || self
                .wm_class
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
    }
}

#[derive(Debug, Clone)]
pub enum TargetedInputAction {
    TypeText { text: String },
    PressKey { key: String },
}

impl TargetedInputAction {
    fn name(&self) -> &'static str {
        match self {
            TargetedInputAction::TypeText { .. } => "type_text",
            TargetedInputAction::PressKey { .. } => "press_key",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetedInputReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub action: String,
    pub success: bool,
    pub input_sent: bool,
    pub target: Option<WindowInfo>,
    pub focus: Option<FocusWindowReport>,
    pub keyboard: Option<KeyboardAttempt>,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: TargetedInputDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetedInputDiagnostics {
    pub ok: bool,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub candidates: Vec<TargetCandidate>,
    pub listing: WindowListingDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetCandidate {
    pub window_id: u64,
    pub title: Option<String>,
    pub wm_class: Option<String>,
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyboardAttempt {
    pub command: String,
    pub args: Vec<String>,
    pub ok: bool,
    pub detail: String,
    pub active_context: bool,
    pub used_direct_window: bool,
    pub route: String,
    pub requested_key: Option<String>,
    pub normalized_key: Option<String>,
    pub semantic_stderr_error: bool,
}

pub fn type_text_report_from_system(target: WindowTarget, text: String) -> TargetedInputReport {
    report_from_listing(
        TargetedInputAction::TypeText { text },
        target,
        list_windows::report_from_system(),
    )
}

pub fn press_key_report_from_system(target: WindowTarget, key: String) -> TargetedInputReport {
    report_from_listing(
        TargetedInputAction::PressKey { key },
        target,
        list_windows::report_from_system(),
    )
}

pub fn report_from_listing(
    action: TargetedInputAction,
    target: WindowTarget,
    listing: list_windows::WindowListReport,
) -> TargetedInputReport {
    if !target.has_target() {
        return failure(
            &action,
            listing,
            None,
            None,
            "MissingTarget",
            "No target selector was provided; global keyboard injection is not window-isolated and is not allowed by this safe targeted command.",
            Vec::new(),
        );
    }

    let requested_window = match resolve_target(&listing.windows, &target) {
        Ok(window) => window,
        Err(error) => {
            return failure(
                &action,
                listing,
                None,
                None,
                error.code,
                error.note,
                error.candidates,
            );
        }
    };

    let focus =
        crate::focus::focus_window_report_from_listing(requested_window.window_id, listing.clone());
    if !focus.success || !focus.exact_window_focused {
        return failure(
            &action,
            listing,
            Some(requested_window),
            Some(focus),
            "FocusNotVerified",
            "Did not send input because exact target-window focus verification failed.",
            Vec::new(),
        );
    }

    if !list_windows::command_exists("xdotool") {
        return failure(
            &action,
            listing,
            Some(requested_window),
            Some(focus),
            "InputBackendUnavailable",
            "Did not send input because xdotool is unavailable for standalone keyboard injection.",
            Vec::new(),
        );
    }

    let keyboard = run_keyboard_backend(&action);
    if keyboard.ok {
        success(&action, listing, requested_window, focus, keyboard)
    } else {
        failure_with_keyboard(
            &action,
            listing,
            Some(requested_window),
            Some(focus),
            Some(keyboard),
            "InputBackendFailed",
            "Keyboard backend command failed after focus verification.",
            Vec::new(),
        )
    }
}

pub struct ResolveError {
    pub code: &'static str,
    pub note: String,
    pub candidates: Vec<TargetCandidate>,
}

pub fn resolve_target(
    windows: &[WindowInfo],
    target: &WindowTarget,
) -> Result<WindowInfo, ResolveError> {
    if let Some(window_id) = target.window_id {
        return windows
            .iter()
            .find(|window| window.window_id == window_id)
            .cloned()
            .ok_or_else(|| ResolveError {
                code: "WindowNotFound",
                note: format!("No window matched window_id {window_id}."),
                candidates: Vec::new(),
            });
    }

    let matches = windows
        .iter()
        .filter(|window| target.pid.is_none_or(|pid| window.pid == Some(pid)))
        .filter(|window| {
            normalized(target.wm_class.as_deref()).is_none_or(|requested| {
                window
                    .wm_class
                    .as_deref()
                    .is_some_and(|actual| actual.eq_ignore_ascii_case(&requested))
            })
        })
        .filter(|window| {
            normalized(target.title.as_deref()).is_none_or(|requested| {
                let requested = requested.to_ascii_lowercase();
                window
                    .title
                    .as_deref()
                    .is_some_and(|actual| actual.to_ascii_lowercase().contains(&requested))
            })
        })
        .cloned()
        .collect::<Vec<_>>();

    match matches.as_slice() {
        [window] => Ok(window.clone()),
        [] => Err(ResolveError {
            code: "WindowNotFound",
            note: "No window matched the provided target selectors.".to_string(),
            candidates: Vec::new(),
        }),
        _ => Err(ResolveError {
            code: "AmbiguousTarget",
            note: "Target selectors matched multiple windows; add window_id to disambiguate."
                .to_string(),
            candidates: matches.iter().map(candidate_from_window).collect(),
        }),
    }
}

fn candidate_from_window(window: &WindowInfo) -> TargetCandidate {
    TargetCandidate {
        window_id: window.window_id,
        title: window.title.clone(),
        wm_class: window.wm_class.clone(),
        pid: window.pid,
    }
}

fn normalized(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn failure(
    action: &TargetedInputAction,
    listing: list_windows::WindowListReport,
    target: Option<WindowInfo>,
    focus: Option<FocusWindowReport>,
    error_code: impl Into<String>,
    note: impl Into<String>,
    candidates: Vec<TargetCandidate>,
) -> TargetedInputReport {
    failure_with_keyboard(
        action, listing, target, focus, None, error_code, note, candidates,
    )
}

#[allow(clippy::too_many_arguments)]
fn failure_with_keyboard(
    action: &TargetedInputAction,
    listing: list_windows::WindowListReport,
    target: Option<WindowInfo>,
    focus: Option<FocusWindowReport>,
    keyboard: Option<KeyboardAttempt>,
    error_code: impl Into<String>,
    note: impl Into<String>,
    candidates: Vec<TargetCandidate>,
) -> TargetedInputReport {
    let mut blockers = listing.diagnostics.blockers.clone();
    let error_code = error_code.into();
    blockers.push(error_code.clone());
    TargetedInputReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: action.name().to_string(),
        success: false,
        input_sent: false,
        target,
        focus,
        keyboard,
        error_code: Some(error_code),
        note: note.into(),
        diagnostics: TargetedInputDiagnostics {
            ok: false,
            blockers,
            degraded_reasons: listing.diagnostics.degraded_reasons.clone(),
            candidates,
            listing: listing.diagnostics,
        },
    }
}

fn success(
    action: &TargetedInputAction,
    listing: list_windows::WindowListReport,
    target: WindowInfo,
    focus: FocusWindowReport,
    keyboard: KeyboardAttempt,
) -> TargetedInputReport {
    TargetedInputReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        action: action.name().to_string(),
        success: true,
        input_sent: true,
        target: Some(target),
        focus: Some(focus),
        keyboard: Some(keyboard),
        error_code: None,
        note: "Input was sent through active-context xdotool only after exact X11 active-window focus verification; xdotool --window direct events were not used as a safety boundary."
            .to_string(),
        diagnostics: TargetedInputDiagnostics {
            ok: listing.diagnostics.blockers.is_empty(),
            blockers: listing.diagnostics.blockers.clone(),
            degraded_reasons: listing.diagnostics.degraded_reasons.clone(),
            candidates: Vec::new(),
            listing: listing.diagnostics,
        },
    }
}

fn run_keyboard_backend(action: &TargetedInputAction) -> KeyboardAttempt {
    let (args, route, requested_key, normalized_key) = match action {
        TargetedInputAction::TypeText { text } if !text.is_ascii() => {
            return run_unicode_text_backend(text);
        }
        TargetedInputAction::TypeText { text } => (
            vec![
                "type".to_string(),
                "--clearmodifiers".to_string(),
                text.clone(),
            ],
            "xdotool-type".to_string(),
            None,
            None,
        ),
        TargetedInputAction::PressKey { key } => {
            let normalized = normalize_key_alias(key);
            (
                vec![
                    "key".to_string(),
                    "--clearmodifiers".to_string(),
                    normalized.clone(),
                ],
                "xdotool-key".to_string(),
                Some(key.clone()),
                Some(normalized),
            )
        }
    };
    let output = std::process::Command::new("xdotool").args(&args).output();
    match output {
        Ok(output) => {
            let detail = command_detail(&output);
            let semantic_stderr_error = has_xdotool_semantic_stderr_error(&output);
            KeyboardAttempt {
                command: "xdotool".to_string(),
                args,
                ok: output.status.success() && !semantic_stderr_error,
                detail,
                active_context: true,
                used_direct_window: false,
                route,
                requested_key,
                normalized_key,
                semantic_stderr_error,
            }
        }
        Err(err) => KeyboardAttempt {
            command: "xdotool".to_string(),
            args,
            ok: false,
            detail: err.to_string(),
            active_context: true,
            used_direct_window: false,
            route,
            requested_key,
            normalized_key,
            semantic_stderr_error: false,
        },
    }
}

fn run_unicode_text_backend(text: &str) -> KeyboardAttempt {
    let keysyms = unicode_keysyms(text);
    let mut args = vec!["key".to_string(), "--clearmodifiers".to_string()];
    args.extend(keysyms);
    let output = std::process::Command::new("xdotool").args(&args).output();
    match output {
        Ok(output) => {
            let detail = command_detail(&output);
            let semantic_stderr_error = has_xdotool_semantic_stderr_error(&output);
            let primary_ok = output.status.success() && !semantic_stderr_error;
            if primary_ok {
                KeyboardAttempt {
                    command: "xdotool".to_string(),
                    args,
                    ok: true,
                    detail,
                    active_context: true,
                    used_direct_window: false,
                    route: "xdotool-unicode-keysyms".to_string(),
                    requested_key: None,
                    normalized_key: None,
                    semantic_stderr_error: false,
                }
            } else if let Some(fallback) =
                run_clipboard_paste_fallback(text, &detail, semantic_stderr_error)
            {
                fallback
            } else {
                KeyboardAttempt {
                    command: "xdotool".to_string(),
                    args,
                    ok: false,
                    detail,
                    active_context: true,
                    used_direct_window: false,
                    route: "xdotool-unicode-keysyms".to_string(),
                    requested_key: None,
                    normalized_key: None,
                    semantic_stderr_error,
                }
            }
        }
        Err(err) => {
            let detail = err.to_string();
            if let Some(fallback) = run_clipboard_paste_fallback(text, &detail, false) {
                fallback
            } else {
                KeyboardAttempt {
                    command: "xdotool".to_string(),
                    args,
                    ok: false,
                    detail,
                    active_context: true,
                    used_direct_window: false,
                    route: "xdotool-unicode-keysyms".to_string(),
                    requested_key: None,
                    normalized_key: None,
                    semantic_stderr_error: false,
                }
            }
        }
    }
}

fn unicode_keysyms(text: &str) -> Vec<String> {
    text.chars()
        .map(|ch| format!("U{:04X}", ch as u32))
        .collect()
}

fn run_clipboard_paste_fallback(
    text: &str,
    primary_detail: &str,
    primary_semantic_error: bool,
) -> Option<KeyboardAttempt> {
    let clipboard = ClipboardCommand::available()?;
    let previous = clipboard.read();
    let mut details = vec![format!(
        "unicode keysym route failed before clipboard fallback: {primary_detail}"
    )];

    if let Err(error) = clipboard.write(text) {
        details.push(format!("clipboard write failed: {error}"));
        return Some(KeyboardAttempt {
            command: clipboard.command_name().to_string(),
            args: clipboard
                .write_args()
                .into_iter()
                .map(str::to_string)
                .collect(),
            ok: false,
            detail: details.join("; "),
            active_context: true,
            used_direct_window: false,
            route: "clipboard-paste".to_string(),
            requested_key: None,
            normalized_key: None,
            semantic_stderr_error: primary_semantic_error,
        });
    }

    let paste_args = vec![
        "key".to_string(),
        "--clearmodifiers".to_string(),
        "ctrl+v".to_string(),
    ];
    let paste = std::process::Command::new("xdotool")
        .args(&paste_args)
        .output();
    let (paste_ok, paste_detail, paste_semantic) = match paste {
        Ok(output) => (
            output.status.success() && !has_xdotool_semantic_stderr_error(&output),
            command_detail(&output),
            has_xdotool_semantic_stderr_error(&output),
        ),
        Err(error) => (false, error.to_string(), false),
    };
    details.push(format!("paste command: {paste_detail}"));

    let mut restore_failed = false;
    if let Ok(previous) = previous {
        match clipboard.write_bytes(&previous) {
            Ok(()) => {
                details.push("clipboard restored".to_string());
            }
            Err(error) => {
                restore_failed = true;
                details.push(format!("clipboard_restore_failed: {error}"));
            }
        }
    } else {
        details.push("previous clipboard unavailable; restore skipped".to_string());
    }

    let mut args = vec![clipboard.command_name().to_string()];
    args.extend(clipboard.write_args().into_iter().map(str::to_string));
    args.extend(["&&".to_string(), "xdotool".to_string()]);
    args.extend(paste_args);

    Some(KeyboardAttempt {
        command: "clipboard-paste".to_string(),
        args,
        ok: paste_ok && !restore_failed,
        detail: details.join("; "),
        active_context: true,
        used_direct_window: false,
        route: "clipboard-paste".to_string(),
        requested_key: None,
        normalized_key: None,
        semantic_stderr_error: primary_semantic_error || paste_semantic,
    })
}

enum ClipboardCommand {
    Xclip,
    Xsel,
}

impl ClipboardCommand {
    fn available() -> Option<Self> {
        if list_windows::command_exists("xclip") {
            Some(Self::Xclip)
        } else if list_windows::command_exists("xsel") {
            Some(Self::Xsel)
        } else {
            None
        }
    }

    fn command_name(&self) -> &'static str {
        match self {
            Self::Xclip => "xclip",
            Self::Xsel => "xsel",
        }
    }

    fn read_args(&self) -> Vec<&'static str> {
        match self {
            Self::Xclip => vec!["-selection", "clipboard", "-o"],
            Self::Xsel => vec!["--clipboard", "--output"],
        }
    }

    fn write_args(&self) -> Vec<&'static str> {
        match self {
            Self::Xclip => vec!["-selection", "clipboard"],
            Self::Xsel => vec!["--clipboard", "--input"],
        }
    }

    fn read(&self) -> Result<Vec<u8>, String> {
        std::process::Command::new(self.command_name())
            .args(self.read_args())
            .output()
            .map_err(|error| error.to_string())
            .and_then(|output| {
                if output.status.success() {
                    Ok(output.stdout)
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
                }
            })
    }

    fn write(&self, text: &str) -> Result<(), String> {
        self.write_bytes(text.as_bytes())
    }

    fn write_bytes(&self, bytes: &[u8]) -> Result<(), String> {
        let mut child = std::process::Command::new(self.command_name())
            .args(self.write_args())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| error.to_string())?;
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(bytes).map_err(|error| error.to_string())?;
        }
        let output = child
            .wait_with_output()
            .map_err(|error| error.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }
}

fn normalize_key_alias(key: &str) -> String {
    match key.trim() {
        "Enter" | "enter" => "Return".to_string(),
        "Backspace" | "backspace" | "BackSpace" => "BackSpace".to_string(),
        other => other.to_string(),
    }
}

fn has_xdotool_semantic_stderr_error(output: &std::process::Output) -> bool {
    let stderr = String::from_utf8_lossy(&output.stderr).to_ascii_lowercase();
    stderr.contains("no such key name") || stderr.contains("ignoring it")
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
