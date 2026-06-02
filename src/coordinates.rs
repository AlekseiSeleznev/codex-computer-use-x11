use std::path::{Path, PathBuf};
use std::process::Command;

use crate::doctor::{BACKEND_ID, PROJECT_NAME};
use crate::list_windows::{self, WindowBounds, WindowInfo, WindowListReport};

#[derive(Debug, Clone, Copy)]
pub struct CropRequest {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScreenshotCropReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub success: bool,
    pub screenshot_invoked: bool,
    pub window: Option<WindowInfo>,
    pub crop: Option<CropRectReport>,
    pub output_path: String,
    pub provider: Option<String>,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: ScreenshotCropDiagnostics,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CropRectReport {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub source: String,
    pub clamped: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScreenshotCropDiagnostics {
    pub ok: bool,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub provider_detail: String,
    pub listing: list_windows::WindowListingDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowBoundsReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub success: bool,
    pub window: Option<WindowInfo>,
    pub bounds: Option<WindowBounds>,
    pub coordinate_model: CoordinateModel,
    pub error_code: Option<String>,
    pub note: String,
    pub diagnostics: BoundsDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoordinateModel {
    pub space: String,
    pub origin: String,
    pub x_y_type: String,
    pub width_height_type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BoundsDiagnostics {
    pub ok: bool,
    pub primary_source: String,
    pub bounds_semantics: String,
    pub alternate_sources: Vec<AlternateBoundsSource>,
    pub bounds_agree: Option<bool>,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub listing: list_windows::WindowListingDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlternateBoundsSource {
    pub source: String,
    pub bounds: Option<WindowBounds>,
    pub ok: bool,
    pub detail: String,
}

pub fn window_bounds_report_from_system(window_id: u64) -> WindowBoundsReport {
    let listing = list_windows::report_from_system();
    window_bounds_report_from_listing(window_id, listing)
}

pub fn window_bounds_report_from_listing(
    window_id: u64,
    listing: WindowListReport,
) -> WindowBoundsReport {
    let Some(window) = listing
        .windows
        .iter()
        .find(|window| window.window_id == window_id)
        .cloned()
    else {
        return failure(
            listing,
            None,
            None,
            "WindowNotFound",
            format!("No window matched window_id {window_id}."),
            vec!["requested window was not present in current X11 listing".to_string()],
        );
    };

    let Some(bounds) = window.bounds.clone() else {
        return failure(
            listing,
            Some(window),
            None,
            "MissingBounds",
            "The requested window did not include known bounds in the current listing.",
            vec!["primary listing source did not provide usable bounds".to_string()],
        );
    };

    success(listing, window, bounds)
}

fn success(
    listing: WindowListReport,
    window: WindowInfo,
    bounds: WindowBounds,
) -> WindowBoundsReport {
    let (alternate_sources, bounds_agree, degraded_reasons) =
        alternate_bounds_diagnostics(&window, &bounds);
    WindowBoundsReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        success: true,
        window: Some(window),
        bounds: Some(bounds),
        coordinate_model: coordinate_model(),
        error_code: None,
        note: "Bounds are reported in X11 root/global pixels; primary geometry comes from wmctrl and may represent frame rather than client-content bounds.".to_string(),
        diagnostics: BoundsDiagnostics {
            ok: true,
            primary_source: "wmctrl -lpGx".to_string(),
            bounds_semantics: "wmctrl geometry is treated as the primary X11 root coordinate source; frame/client-area geometry can differ and must be checked through bounds provenance diagnostics when precision matters.".to_string(),
            alternate_sources,
            bounds_agree,
            blockers: Vec::new(),
            degraded_reasons,
            listing: listing.diagnostics,
        },
    }
}

fn failure(
    listing: WindowListReport,
    window: Option<WindowInfo>,
    bounds: Option<WindowBounds>,
    error_code: impl Into<String>,
    note: impl Into<String>,
    blockers: Vec<String>,
) -> WindowBoundsReport {
    WindowBoundsReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        success: false,
        window,
        bounds,
        coordinate_model: coordinate_model(),
        error_code: Some(error_code.into()),
        note: note.into(),
        diagnostics: BoundsDiagnostics {
            ok: false,
            primary_source: "wmctrl -lpGx".to_string(),
            bounds_semantics: "wmctrl geometry is treated as the primary X11 root coordinate source; frame/client-area geometry can differ and must be checked through bounds provenance diagnostics when precision matters.".to_string(),
            alternate_sources: Vec::new(),
            bounds_agree: None,
            blockers,
            degraded_reasons: Vec::new(),
            listing: listing.diagnostics,
        },
    }
}

pub fn screenshot_crop_report_from_system(
    window_id: u64,
    request: CropRequest,
    output_path: String,
) -> ScreenshotCropReport {
    let listing = list_windows::report_from_system();
    screenshot_crop_report_from_listing(window_id, request, output_path, listing)
}

pub fn screenshot_crop_report_from_listing(
    window_id: u64,
    request: CropRequest,
    output_path: String,
    listing: WindowListReport,
) -> ScreenshotCropReport {
    let output_path = match resolve_screenshot_output_path(&output_path) {
        Ok(path) => path,
        Err(err) => {
            return screenshot_failure(
                listing,
                None,
                None,
                err.output_path,
                err.error_code,
                err.note,
                err.blockers,
            );
        }
    };
    let Some(window) = listing
        .windows
        .iter()
        .find(|window| window.window_id == window_id)
        .cloned()
    else {
        return screenshot_failure(
            listing,
            None,
            None,
            output_path,
            "WindowNotFound",
            format!("No window matched window_id {window_id}."),
            vec!["requested window was not present in current X11 listing".to_string()],
        );
    };
    let Some(bounds) = window.bounds.clone() else {
        return screenshot_failure(
            listing,
            Some(window),
            None,
            output_path,
            "MissingBounds",
            "The requested window did not include known bounds in the current listing.",
            vec!["primary listing source did not provide usable bounds".to_string()],
        );
    };
    let Some(crop) = crop_from_request(&request, &bounds) else {
        return screenshot_failure(
            listing,
            Some(window),
            None,
            output_path,
            "InvalidCropRect",
            "Crop rectangle requires either no explicit rectangle or all of --x, --y, --width, and --height with positive dimensions.",
            vec!["invalid crop rectangle arguments".to_string()],
        );
    };
    if !rect_inside_bounds(&crop, &bounds) {
        return screenshot_failure(
            listing,
            Some(window),
            Some(crop),
            output_path,
            "CropOutsideTargetBounds",
            "Did not invoke screenshot provider because the requested root-coordinate crop is outside the target window bounds.",
            vec!["crop rectangle must be inside target bounds before screenshot capture".to_string()],
        );
    }

    let (crop, screen_degraded) = match clamp_crop_with_system_geometry(crop.clone()) {
        Ok(result) => result,
        Err(reason) => {
            return screenshot_failure(
                listing,
                Some(window),
                Some(crop),
                output_path,
                "CropOutsideScreen",
                "Did not invoke screenshot provider because the requested crop does not intersect the reported root screen geometry.",
                vec![reason],
            );
        }
    };

    invoke_screenshot_area_provider(listing, window, crop, output_path, screen_degraded)
}

struct OutputPathError {
    output_path: String,
    error_code: &'static str,
    note: &'static str,
    blockers: Vec<String>,
}

fn resolve_screenshot_output_path(output_path: &str) -> Result<String, OutputPathError> {
    let raw = Path::new(output_path);
    if output_path.trim().is_empty() {
        return Err(OutputPathError {
            output_path: output_path.to_string(),
            error_code: "InvalidOutputPath",
            note: "Did not invoke screenshot provider because the output path was invalid.",
            blockers: vec!["output path must not be empty".to_string()],
        });
    }

    let resolved: PathBuf = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(raw),
            Err(err) => {
                return Err(OutputPathError {
                    output_path: output_path.to_string(),
                    error_code: "InvalidOutputPath",
                    note: "Did not invoke screenshot provider because the current working directory could not be resolved.",
                    blockers: vec![format!("failed to resolve current working directory: {err}")],
                });
            }
        }
    };

    let Some(parent) = resolved.parent() else {
        return Err(OutputPathError {
            output_path: resolved.display().to_string(),
            error_code: "InvalidOutputPath",
            note: "Did not invoke screenshot provider because the output path was invalid.",
            blockers: vec!["output path did not include a parent directory".to_string()],
        });
    };
    if !parent.exists() {
        return Err(OutputPathError {
            output_path: resolved.display().to_string(),
            error_code: "OutputPathUnavailable",
            note: "Did not invoke screenshot provider because the output parent directory does not exist.",
            blockers: vec![format!(
                "output parent directory does not exist: {}",
                parent.display()
            )],
        });
    }
    if !parent.is_dir() {
        return Err(OutputPathError {
            output_path: resolved.display().to_string(),
            error_code: "OutputPathUnavailable",
            note:
                "Did not invoke screenshot provider because the output parent is not a directory.",
            blockers: vec![format!(
                "output parent is not a directory: {}",
                parent.display()
            )],
        });
    }

    Ok(resolved.display().to_string())
}

fn crop_from_request(request: &CropRequest, bounds: &WindowBounds) -> Option<CropRectReport> {
    match (request.x, request.y, request.width, request.height) {
        (None, None, None, None) => Some(CropRectReport {
            x: bounds.x?,
            y: bounds.y?,
            width: positive(bounds.width)?,
            height: positive(bounds.height)?,
            source: "window_bounds".to_string(),
            clamped: false,
        }),
        (Some(x), Some(y), Some(width), Some(height)) => Some(CropRectReport {
            x,
            y,
            width: positive(width)?,
            height: positive(height)?,
            source: "explicit_root_rect".to_string(),
            clamped: false,
        }),
        _ => None,
    }
}

fn positive(value: u32) -> Option<u32> {
    (value > 0).then_some(value)
}

fn rect_inside_bounds(rect: &CropRectReport, bounds: &WindowBounds) -> bool {
    let Some(bounds_x) = bounds.x else {
        return false;
    };
    let Some(bounds_y) = bounds.y else {
        return false;
    };
    let rect_right = i64::from(rect.x) + i64::from(rect.width);
    let rect_bottom = i64::from(rect.y) + i64::from(rect.height);
    let bounds_right = i64::from(bounds_x) + i64::from(bounds.width);
    let bounds_bottom = i64::from(bounds_y) + i64::from(bounds.height);
    i64::from(rect.x) >= i64::from(bounds_x)
        && i64::from(rect.y) >= i64::from(bounds_y)
        && rect_right <= bounds_right
        && rect_bottom <= bounds_bottom
}

fn clamp_crop_with_system_geometry(
    crop: CropRectReport,
) -> Result<(CropRectReport, Vec<String>), String> {
    match screen_geometry_from_system() {
        Some(screen) => clamp_crop_to_screen(crop, &screen)
            .map(|clamped| {
                let degraded = if clamped.clamped {
                    vec!["crop rectangle was clamped to reported X11 root screen geometry".to_string()]
                } else {
                    Vec::new()
                };
                (clamped, degraded)
            })
            .ok_or_else(|| "crop rectangle did not intersect reported X11 root screen geometry".to_string()),
        None => Ok((
            crop,
            vec!["screen geometry was unavailable; crop was validated only against target window bounds".to_string()],
        )),
    }
}

fn screen_geometry_from_system() -> Option<ScreenGeometry> {
    if list_windows::command_exists("xrandr") {
        if let Ok(output) = command_output("xrandr", &["--listmonitors".to_string()]) {
            if let Some(geometry) = parse_xrandr_listmonitors_geometry(&output) {
                return Some(geometry);
            }
        }
    }
    if list_windows::command_exists("xdpyinfo") {
        if let Ok(output) = command_output("xdpyinfo", &[]) {
            return parse_xdpyinfo_dimensions_geometry(&output);
        }
    }
    None
}

pub fn parse_xdpyinfo_dimensions_geometry(output: &str) -> Option<ScreenGeometry> {
    for line in output.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("dimensions:") else {
            continue;
        };
        let dims = rest.split_whitespace().next()?;
        let (width, height) = dims.split_once('x')?;
        let width = width.parse::<u32>().ok()?;
        let height = height.parse::<u32>().ok()?;
        return (width > 0 && height > 0).then_some(ScreenGeometry {
            x: 0,
            y: 0,
            width,
            height,
        });
    }
    None
}

fn invoke_screenshot_area_provider(
    listing: WindowListReport,
    window: WindowInfo,
    crop: CropRectReport,
    output_path: String,
    screen_degraded: Vec<String>,
) -> ScreenshotCropReport {
    if !list_windows::command_exists("gdbus") {
        return screenshot_failure(
            listing,
            Some(window),
            Some(crop),
            output_path,
            "ScreenshotProviderUnavailable",
            "Did not invoke screenshot provider because gdbus is unavailable.",
            vec!["gdbus is required for the standalone GNOME Shell-compatible ScreenshotArea smoke provider".to_string()],
        );
    }

    let args = vec![
        "call".to_string(),
        "--session".to_string(),
        "--dest".to_string(),
        "org.gnome.Shell.Screenshot".to_string(),
        "--object-path".to_string(),
        "/org/gnome/Shell/Screenshot".to_string(),
        "--method".to_string(),
        "org.gnome.Shell.Screenshot.ScreenshotArea".to_string(),
        crop.x.to_string(),
        crop.y.to_string(),
        crop.width.to_string(),
        crop.height.to_string(),
        "false".to_string(),
        output_path.clone(),
    ];

    match command_output("gdbus", &args) {
        Ok(output) if screenshot_provider_reported_success(&output) => {
            match verify_png_output(&output_path) {
                Ok(verification) => ScreenshotCropReport {
                    project: PROJECT_NAME.to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    backend: BACKEND_ID.to_string(),
                    success: true,
                    screenshot_invoked: true,
                    window: Some(window),
                    crop: Some(crop),
                    output_path,
                    provider: Some("gnome_shell_screenshot_area".to_string()),
                    error_code: None,
                    note: "Screenshot crop was captured through a standalone GNOME Shell-compatible ScreenshotArea boundary after root-coordinate validation. Screenshot pixels are written only to the caller-provided output path and are not serialized in this report.".to_string(),
                    diagnostics: ScreenshotCropDiagnostics {
                        ok: true,
                        blockers: Vec::new(),
                        degraded_reasons: screen_degraded,
                        provider_detail: format!(
                            "gdbus ScreenshotArea returned success: {}; output verified: {}",
                            output.trim(),
                            verification
                        ),
                        listing: listing.diagnostics,
                    },
                },
                Err(verification_error) => screenshot_provider_output_failure(
                    listing,
                    window,
                    crop,
                    output_path,
                    verification_error.error_code,
                    verification_error.note,
                    verification_error.blockers,
                    format!(
                        "gdbus ScreenshotArea returned success but output verification failed: {}; {}",
                        output.trim(),
                        verification_error.detail
                    ),
                    screen_degraded,
                ),
            }
        }
        Ok(output) => screenshot_provider_output_failure(
            listing,
            window,
            crop,
            output_path,
            "ScreenshotOutputMissing",
            "Standalone screenshot provider reported failure and did not produce a verified output file.",
            vec!["screenshot provider returned false for ScreenshotArea".to_string()],
            format!("gdbus ScreenshotArea returned failure: {}", output.trim()),
            screen_degraded,
        ),
        Err(err) => screenshot_failure(
            listing,
            Some(window),
            Some(crop),
            output_path,
            "ScreenshotProviderFailed",
            "Standalone screenshot provider failed after crop validation.",
            vec![format!("gdbus ScreenshotArea failed: {err}")],
        ),
    }
}

fn screenshot_provider_reported_success(output: &str) -> bool {
    output.trim_start().starts_with("(true")
}

struct OutputVerificationError {
    error_code: &'static str,
    note: &'static str,
    detail: String,
    blockers: Vec<String>,
}

fn verify_png_output(output_path: &str) -> Result<String, OutputVerificationError> {
    let path = Path::new(output_path);
    let metadata = std::fs::metadata(path).map_err(|err| OutputVerificationError {
        error_code: "ScreenshotOutputMissing",
        note: "Standalone screenshot provider did not produce the requested output file.",
        detail: format!("output metadata unavailable: {err}"),
        blockers: vec![format!("output file was not created: {output_path}")],
    })?;
    if !metadata.is_file() {
        return Err(OutputVerificationError {
            error_code: "ScreenshotOutputUnreadable",
            note: "Standalone screenshot provider output was not a readable file.",
            detail: "output path is not a regular file".to_string(),
            blockers: vec![format!("output path is not a regular file: {output_path}")],
        });
    }
    if metadata.len() == 0 {
        return Err(OutputVerificationError {
            error_code: "ScreenshotOutputEmpty",
            note: "Standalone screenshot provider produced an empty output file.",
            detail: "output file has zero bytes".to_string(),
            blockers: vec![format!("output file is empty: {output_path}")],
        });
    }

    let bytes = std::fs::read(path).map_err(|err| OutputVerificationError {
        error_code: "ScreenshotOutputUnreadable",
        note: "Standalone screenshot provider output could not be read.",
        detail: format!("output read failed: {err}"),
        blockers: vec![format!("output file could not be read: {output_path}")],
    })?;
    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if !bytes.starts_with(PNG_SIGNATURE) {
        return Err(OutputVerificationError {
            error_code: "ScreenshotOutputInvalidFormat",
            note: "Standalone screenshot provider output was not a PNG file.",
            detail: "output file did not begin with the PNG signature".to_string(),
            blockers: vec![format!("output file was not PNG: {output_path}")],
        });
    }

    Ok(format!(
        "path={output_path}, size_bytes={}, format=png",
        metadata.len()
    ))
}

#[allow(clippy::too_many_arguments)]
fn screenshot_provider_output_failure(
    listing: WindowListReport,
    window: WindowInfo,
    crop: CropRectReport,
    output_path: String,
    error_code: impl Into<String>,
    note: impl Into<String>,
    blockers: Vec<String>,
    provider_detail: String,
    mut degraded_reasons: Vec<String>,
) -> ScreenshotCropReport {
    degraded_reasons.append(&mut listing.diagnostics.degraded_reasons.clone());
    ScreenshotCropReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        success: false,
        screenshot_invoked: true,
        window: Some(window),
        crop: Some(crop),
        output_path,
        provider: Some("gnome_shell_screenshot_area".to_string()),
        error_code: Some(error_code.into()),
        note: note.into(),
        diagnostics: ScreenshotCropDiagnostics {
            ok: false,
            blockers,
            degraded_reasons,
            provider_detail,
            listing: listing.diagnostics,
        },
    }
}

fn screenshot_failure(
    listing: WindowListReport,
    window: Option<WindowInfo>,
    crop: Option<CropRectReport>,
    output_path: String,
    error_code: impl Into<String>,
    note: impl Into<String>,
    blockers: Vec<String>,
) -> ScreenshotCropReport {
    ScreenshotCropReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        success: false,
        screenshot_invoked: false,
        window,
        crop,
        output_path,
        provider: None,
        error_code: Some(error_code.into()),
        note: note.into(),
        diagnostics: ScreenshotCropDiagnostics {
            ok: false,
            blockers,
            degraded_reasons: Vec::new(),
            provider_detail: "Codex Desktop Linux source overlay should prefer the existing screenshot.rs provider; standalone crop uses a validated provider boundary only when available.".to_string(),
            listing: listing.diagnostics,
        },
    }
}

fn alternate_bounds_diagnostics(
    window: &WindowInfo,
    primary: &WindowBounds,
) -> (Vec<AlternateBoundsSource>, Option<bool>, Vec<String>) {
    if !list_windows::command_exists("xwininfo") {
        return (Vec::new(), None, Vec::new());
    }

    let window_id = format!("0x{:x}", window.window_id);
    let mut degraded_reasons = Vec::new();
    match command_output("xwininfo", &["-id".to_string(), window_id]) {
        Ok(output) => match parse_xwininfo_bounds(&output) {
            Some(bounds) => {
                let agree = bounds_equal(primary, &bounds);
                if !agree {
                    degraded_reasons.push(
                        "geometry sources disagreed between wmctrl primary bounds and xwininfo alternate bounds".to_string(),
                    );
                }
                (
                    vec![AlternateBoundsSource {
                        source: "xwininfo -id".to_string(),
                        bounds: Some(bounds),
                        ok: true,
                        detail: "xwininfo returned alternate absolute window geometry".to_string(),
                    }],
                    Some(agree),
                    degraded_reasons,
                )
            }
            None => (
                vec![AlternateBoundsSource {
                    source: "xwininfo -id".to_string(),
                    bounds: None,
                    ok: false,
                    detail: "xwininfo output did not include parseable absolute geometry"
                        .to_string(),
                }],
                None,
                vec!["xwininfo alternate geometry could not be parsed".to_string()],
            ),
        },
        Err(err) => (
            vec![AlternateBoundsSource {
                source: "xwininfo -id".to_string(),
                bounds: None,
                ok: false,
                detail: format!("xwininfo failed: {err}"),
            }],
            None,
            vec!["xwininfo alternate geometry was unavailable".to_string()],
        ),
    }
}

pub fn parse_xwininfo_bounds(output: &str) -> Option<WindowBounds> {
    let mut x = None;
    let mut y = None;
    let mut width = None;
    let mut height = None;

    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("Absolute upper-left X:") {
            x = value.trim().parse::<i32>().ok();
        } else if let Some(value) = trimmed.strip_prefix("Absolute upper-left Y:") {
            y = value.trim().parse::<i32>().ok();
        } else if let Some(value) = trimmed.strip_prefix("Width:") {
            width = value.trim().parse::<u32>().ok();
        } else if let Some(value) = trimmed.strip_prefix("Height:") {
            height = value.trim().parse::<u32>().ok();
        }
    }

    Some(WindowBounds {
        x: Some(x?),
        y: Some(y?),
        width: width?,
        height: height?,
    })
}

fn bounds_equal(left: &WindowBounds, right: &WindowBounds) -> bool {
    left.x == right.x
        && left.y == right.y
        && left.width == right.width
        && left.height == right.height
}

fn coordinate_model() -> CoordinateModel {
    CoordinateModel {
        space: "x11_root_global_pixels".to_string(),
        origin: "X11 root window origin; monitor layouts may place known windows at negative signed coordinates".to_string(),
        x_y_type: "Option<i32>".to_string(),
        width_height_type: "u32".to_string(),
    }
}

#[allow(dead_code)]
fn command_output(command: &str, args: &[String]) -> Result<String, String> {
    match Command::new(command).args(args).output() {
        Ok(output) if output.status.success() => {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        Ok(output) => Err(String::from_utf8_lossy(&output.stderr).trim().to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub fn parse_xrandr_listmonitors_geometry(output: &str) -> Option<ScreenGeometry> {
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;
    let mut max_x: Option<i64> = None;
    let mut max_y: Option<i64> = None;

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("Monitors:") {
            continue;
        }
        let Some((width, height, x, y)) = parse_xrandr_monitor_line(trimmed) else {
            continue;
        };
        min_x = Some(min_x.map_or(x, |value| value.min(x)));
        min_y = Some(min_y.map_or(y, |value| value.min(y)));
        max_x = Some(max_x.map_or(i64::from(x) + i64::from(width), |value| {
            value.max(i64::from(x) + i64::from(width))
        }));
        max_y = Some(max_y.map_or(i64::from(y) + i64::from(height), |value| {
            value.max(i64::from(y) + i64::from(height))
        }));
    }

    let x = min_x?;
    let y = min_y?;
    let width = u32::try_from(max_x? - i64::from(x)).ok()?;
    let height = u32::try_from(max_y? - i64::from(y)).ok()?;
    (width > 0 && height > 0).then_some(ScreenGeometry {
        x,
        y,
        width,
        height,
    })
}

fn parse_xrandr_monitor_line(line: &str) -> Option<(u32, u32, i32, i32)> {
    for token in line.split_whitespace() {
        if let Some(parsed) = parse_xrandr_geometry_token(token) {
            return Some(parsed);
        }
    }
    None
}

fn parse_xrandr_geometry_token(token: &str) -> Option<(u32, u32, i32, i32)> {
    let x_index = token.find('x')?;
    let width = token[..x_index].split('/').next()?.parse::<u32>().ok()?;
    let after_x = &token[x_index + 1..];
    let offset_index = after_x
        .char_indices()
        .find_map(|(index, ch)| (ch == '+' || ch == '-').then_some(index))?;
    let height = after_x[..offset_index]
        .split('/')
        .next()?
        .parse::<u32>()
        .ok()?;
    let offsets = &after_x[offset_index..];
    let second_offset = offsets
        .char_indices()
        .skip(1)
        .find_map(|(index, ch)| (ch == '+' || ch == '-').then_some(index))?;
    let x = offsets[..second_offset].parse::<i32>().ok()?;
    let y = offsets[second_offset..].parse::<i32>().ok()?;
    Some((width, height, x, y))
}

pub fn clamp_crop_to_screen(
    mut rect: CropRectReport,
    screen: &ScreenGeometry,
) -> Option<CropRectReport> {
    let rect_left = i64::from(rect.x);
    let rect_top = i64::from(rect.y);
    let rect_right = rect_left + i64::from(rect.width);
    let rect_bottom = rect_top + i64::from(rect.height);
    let screen_left = i64::from(screen.x);
    let screen_top = i64::from(screen.y);
    let screen_right = screen_left + i64::from(screen.width);
    let screen_bottom = screen_top + i64::from(screen.height);

    let left = rect_left.max(screen_left);
    let top = rect_top.max(screen_top);
    let right = rect_right.min(screen_right);
    let bottom = rect_bottom.min(screen_bottom);
    if left >= right || top >= bottom {
        return None;
    }
    let clamped =
        left != rect_left || top != rect_top || right != rect_right || bottom != rect_bottom;
    rect.x = i32::try_from(left).ok()?;
    rect.y = i32::try_from(top).ok()?;
    rect.width = u32::try_from(right - left).ok()?;
    rect.height = u32::try_from(bottom - top).ok()?;
    rect.clamped = rect.clamped || clamped;
    Some(rect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xrandr_negative_offsets_define_root_geometry() {
        let output = "Monitors: 2\n 0: +*LEFT 1280/300x1024/240-1280+0  LEFT\n 1: +RIGHT 1920/520x1080/290+0-56  RIGHT\n";
        let geometry = parse_xrandr_listmonitors_geometry(output).expect("parse geometry");
        assert_eq!(geometry.x, -1280);
        assert_eq!(geometry.y, -56);
        assert_eq!(geometry.width, 3200);
        assert_eq!(geometry.height, 1080);

        let rect = CropRectReport {
            x: -1300,
            y: -10,
            width: 100,
            height: 100,
            source: "explicit_root_rect".to_string(),
            clamped: false,
        };
        let clamped = clamp_crop_to_screen(rect, &geometry).expect("intersects screen");
        assert_eq!(clamped.x, -1280);
        assert_eq!(clamped.y, -10);
        assert_eq!(clamped.width, 80);
        assert_eq!(clamped.height, 100);
        assert!(clamped.clamped);
    }
}
