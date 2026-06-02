use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::accessibility::{self, AccessibilityDiagnostics, AccessibilityNode, CorrelationResult};
use crate::doctor::{self, BACKEND_ID, PROJECT_NAME};
use crate::input::{self, TargetCandidate, WindowTarget};
use crate::list_windows::{self, WindowInfo, WindowListingDiagnostics};

#[derive(Debug, Clone)]
pub struct GetAppStateParams {
    pub target: WindowTarget,
    pub include_screenshot: bool,
    pub screenshot_output: Option<PathBuf>,
    pub inline_screenshot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAppStateReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub window_context: Option<WindowInfo>,
    pub window_error: Option<String>,
    pub screenshot: Option<ScreenshotCapture>,
    pub screenshot_error: Option<String>,
    pub accessibility_tree: Vec<AccessibilityNode>,
    pub accessibility_error: Option<String>,
    pub diagnostics: AppStateDiagnostics,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotCapture {
    pub mime_type: String,
    pub path: String,
    pub source: String,
    pub width: u32,
    pub height: u32,
    pub size_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStateDiagnostics {
    pub doctor: doctor::DoctorReport,
    pub window_listing: Option<WindowListingDiagnostics>,
    pub target_candidates: Vec<TargetCandidate>,
    pub accessibility_correlation: Option<CorrelationResult>,
    pub accessibility_diagnostics: Option<AccessibilityDiagnostics>,
    pub layers: Vec<LayerStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerStatus {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

pub fn get_app_state_report_from_system(params: GetAppStateParams) -> GetAppStateReport {
    let doctor = doctor::report_from_system();
    let listing = list_windows::report_from_system();
    let (window_context, window_error, target_candidates) =
        resolve_window_context(&listing, &params.target);

    let (
        accessibility_tree,
        accessibility_error,
        accessibility_correlation,
        accessibility_diagnostics,
    ) = match window_context.as_ref() {
        Some(window) => {
            let report = accessibility::accessibility_tree_report_from_system(window.window_id);
            let error = (!report.success).then(|| {
                format!(
                    "{}: {}",
                    report
                        .error_code
                        .clone()
                        .unwrap_or_else(|| "AccessibilityUnavailable".to_string()),
                    report.note
                )
            });
            (
                report.tree,
                error,
                Some(report.correlation),
                Some(report.diagnostics),
            )
        }
        None if params.target.has_target() => (
            Vec::new(),
            Some("No resolved window context; AT-SPI correlation was not attempted.".to_string()),
            None,
            None,
        ),
        None => (Vec::new(), None, None, None),
    };

    let (screenshot, screenshot_error) = if params.include_screenshot {
        match capture_screenshot_from_system(
            params.screenshot_output.as_deref(),
            params.inline_screenshot,
        ) {
            Ok(capture) => (Some(capture), None),
            Err(error) => (None, Some(error)),
        }
    } else {
        (None, None)
    };

    let layers = layer_statuses(
        &window_context,
        &window_error,
        &screenshot,
        &screenshot_error,
        &accessibility_tree,
        &accessibility_error,
        params.include_screenshot,
    );
    let message = message_from_layers(&layers, &window_context, &window_error);

    GetAppStateReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        window_context,
        window_error,
        screenshot,
        screenshot_error,
        accessibility_tree,
        accessibility_error,
        diagnostics: AppStateDiagnostics {
            doctor,
            window_listing: Some(listing.diagnostics),
            target_candidates,
            accessibility_correlation,
            accessibility_diagnostics,
            layers,
        },
        message,
    }
}

fn capture_screenshot_from_system(
    requested_output: Option<&Path>,
    inline_screenshot: bool,
) -> Result<ScreenshotCapture, String> {
    if !list_windows::command_exists("gdbus") {
        return Err("gdbus is unavailable for standalone app-state screenshot capture".to_string());
    }

    let path = screenshot_output_path(requested_output)?;
    let path_string = path
        .to_str()
        .ok_or_else(|| "screenshot output path is not valid UTF-8".to_string())?
        .to_string();
    let args = [
        "call",
        "--session",
        "--dest",
        "org.gnome.Shell.Screenshot",
        "--object-path",
        "/org/gnome/Shell/Screenshot",
        "--method",
        "org.gnome.Shell.Screenshot.Screenshot",
        "false",
        "false",
        path_string.as_str(),
    ];

    match Command::new("gdbus").args(args).output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("(false") {
                return Err(format!(
                    "gdbus Screenshot reported false for output {}: {}",
                    path.display(),
                    stdout.trim()
                ));
            }
            read_png_as_capture(&path, inline_screenshot)
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(if stderr.is_empty() {
                format!("gdbus Screenshot failed with status {}", output.status)
            } else {
                stderr
            })
        }
        Err(error) => Err(error.to_string()),
    }
}

fn screenshot_output_path(requested_output: Option<&Path>) -> Result<PathBuf, String> {
    let path = match requested_output {
        Some(path) => {
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                std::env::current_dir()
                    .map_err(|error| format!("failed to resolve current directory: {error}"))?
                    .join(path)
            }
        }
        None => generated_png_path()?,
    };

    let parent = path.parent().ok_or_else(|| {
        format!(
            "screenshot output parent is unavailable for {}",
            path.display()
        )
    })?;
    if requested_output.is_none() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "failed to create generated screenshot output parent {}: {error}",
                parent.display()
            )
        })?;
    } else if !parent.is_dir() {
        return Err(format!(
            "screenshot output parent does not exist or is not a directory: {}",
            parent.display()
        ));
    }
    Ok(path)
}

fn generated_png_path() -> Result<PathBuf, String> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let cwd = std::env::current_dir()
        .map_err(|error| format!("failed to resolve current directory: {error}"))?;
    Ok(cwd.join("target/e2e-logs/app-state").join(format!(
        "codex-computer-use-x11-app-state-{}-{nanos}.png",
        std::process::id()
    )))
}

fn read_png_as_capture(path: &Path, inline_screenshot: bool) -> Result<ScreenshotCapture, String> {
    let bytes = fs::read(path)
        .map_err(|error| format!("failed to read screenshot file {}: {error}", path.display()))?;
    if bytes.is_empty() {
        return Err(format!("screenshot file was empty: {}", path.display()));
    }
    let (width, height) = png_dimensions(&bytes)?;
    let size_bytes = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    let data_url = if inline_screenshot {
        use base64::{engine::general_purpose::STANDARD, Engine};
        Some(format!("data:image/png;base64,{}", STANDARD.encode(bytes)))
    } else {
        None
    };
    Ok(ScreenshotCapture {
        mime_type: "image/png".to_string(),
        path: path.display().to_string(),
        source: "gnome-shell-compatible-dbus".to_string(),
        width,
        height,
        size_bytes,
        data_url,
    })
}

fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), String> {
    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if bytes.len() < 24 || &bytes[..8] != PNG_SIGNATURE || &bytes[12..16] != b"IHDR" {
        return Err("screenshot file was not a valid PNG".to_string());
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    if width == 0 || height == 0 {
        return Err(format!(
            "screenshot PNG had invalid dimensions {width}x{height}"
        ));
    }
    Ok((width, height))
}

fn resolve_window_context(
    listing: &list_windows::WindowListReport,
    target: &WindowTarget,
) -> (Option<WindowInfo>, Option<String>, Vec<TargetCandidate>) {
    if !target.has_target() {
        return (None, None, Vec::new());
    }

    match input::resolve_target(&listing.windows, target) {
        Ok(window) => (Some(window), None, Vec::new()),
        Err(error) => (
            None,
            Some(format!("{}: {}", error.code, error.note)),
            error.candidates,
        ),
    }
}

fn layer_statuses(
    window_context: &Option<WindowInfo>,
    window_error: &Option<String>,
    screenshot: &Option<ScreenshotCapture>,
    screenshot_error: &Option<String>,
    accessibility_tree: &[AccessibilityNode],
    accessibility_error: &Option<String>,
    include_screenshot: bool,
) -> Vec<LayerStatus> {
    let window = match (window_context, window_error) {
        (Some(_), _) => LayerStatus::ok("window", "window target resolved"),
        (None, Some(error)) => LayerStatus::degraded("window", error.clone()),
        (None, None) => LayerStatus::ok("window", "no window target requested"),
    };

    let screenshot = if !include_screenshot {
        LayerStatus::ok("screenshot", "screenshot capture was not requested")
    } else if screenshot.is_some() {
        LayerStatus::ok("screenshot", "screenshot captured")
    } else {
        LayerStatus::degraded(
            "screenshot",
            screenshot_error
                .clone()
                .unwrap_or_else(|| "screenshot unavailable".to_string()),
        )
    };

    let accessibility = if !accessibility_tree.is_empty() {
        LayerStatus::ok("accessibility", "accessibility tree matched")
    } else if let Some(error) = accessibility_error {
        LayerStatus::degraded("accessibility", error.clone())
    } else {
        LayerStatus::ok(
            "accessibility",
            "no target-specific accessibility tree requested",
        )
    };

    vec![window, screenshot, accessibility]
}

fn message_from_layers(
    layers: &[LayerStatus],
    window_context: &Option<WindowInfo>,
    window_error: &Option<String>,
) -> String {
    let mut message = "App state report emitted for x11-ewmh.".to_string();
    if let Some(window) = window_context {
        message.push_str(&format!(
            " Window target resolved to window_id {}.",
            window.window_id
        ));
    } else if let Some(error) = window_error {
        message.push_str(&format!(" Window target resolution failed: {error}."));
    }
    let degraded = layers
        .iter()
        .filter(|layer| !layer.ok)
        .map(|layer| format!("{}: {}", layer.name, layer.detail))
        .collect::<Vec<_>>();
    if degraded.is_empty() {
        message.push_str(" All requested layers are usable.");
    } else {
        message.push_str(&format!(" Degraded layers: {}.", degraded.join("; ")));
    }
    message
}

impl LayerStatus {
    fn ok(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ok: true,
            detail: detail.into(),
        }
    }

    fn degraded(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ok: false,
            detail: detail.into(),
        }
    }
}
