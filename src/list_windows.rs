use std::process::Command;

use crate::doctor::{BACKEND_ID, PROJECT_NAME};

#[derive(Debug, Clone, Default)]
pub struct WindowListProbeFacts {
    pub env_display: Option<String>,
    pub wmctrl_available: bool,
    pub xprop_available: bool,
    pub wmctrl_output: Option<Result<String, String>>,
    pub xprop_root_output: Option<Result<String, String>>,
    pub local_hostname: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowListReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub windows: Vec<WindowInfo>,
    pub diagnostics: WindowListingDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowInfo {
    pub window_id: u64,
    pub title: Option<String>,
    pub app_id: Option<String>,
    pub wm_class: Option<String>,
    pub pid: Option<u32>,
    pub bounds: Option<WindowBounds>,
    pub workspace: Option<i32>,
    pub focused: bool,
    pub hidden: bool,
    pub client_type: Option<String>,
    pub backend: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowBounds {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowListingDiagnostics {
    pub ok: bool,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub commands: Vec<CommandStatus>,
    pub parse_errors: Vec<String>,
    pub window_metadata: Vec<WindowMetadata>,
    pub focused_window: Option<u64>,
    pub enrichment: EnrichmentDiagnostics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnrichmentDiagnostics {
    pub per_window_xprop_enabled: bool,
    pub xprop_id_calls: usize,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandStatus {
    pub name: String,
    pub available: bool,
    pub ok: bool,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowMetadata {
    pub window_id: Option<u64>,
    pub raw_id: String,
    pub raw_wm_class: String,
    pub source: String,
    pub pid_reliability: PidReliability,
    pub non_application: bool,
    pub owned_by_project: bool,
    pub internal: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PidReliability {
    Reliable,
    Unreliable,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ParsedWmctrl {
    pub windows: Vec<WindowInfo>,
    pub metadata: Vec<WindowMetadata>,
    pub parse_errors: Vec<String>,
}

pub fn parse_wmctrl_lpgx(output: &str, local_hostname: Option<&str>) -> ParsedWmctrl {
    let mut windows = Vec::new();
    let mut metadata = Vec::new();
    let mut parse_errors = Vec::new();

    for (line_index, raw_line) in output.lines().enumerate() {
        let line = raw_line.trim_start();
        if line.trim().is_empty() {
            continue;
        }
        let Some((fields, title_remainder)) = split_wmctrl_fixed_fields(line) else {
            parse_errors.push(format!(
                "line {}: expected at least 9 wmctrl fields",
                line_index + 1
            ));
            continue;
        };

        let raw_id = fields[0].clone();
        let window_id = match crate::x11_id::parse_x11_window_id(&raw_id) {
            Ok(id) => id,
            Err(err) => {
                parse_errors.push(format!(
                    "line {}: invalid window id {raw_id}: {err:?}",
                    line_index + 1
                ));
                continue;
            }
        };
        let workspace = fields[1].parse::<i32>().ok();
        let raw_pid = fields[2].parse::<u32>().ok();
        let x = match fields[3].parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                parse_errors.push(format!("line {}: invalid geometry x", line_index + 1));
                continue;
            }
        };
        let y = match fields[4].parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                parse_errors.push(format!("line {}: invalid geometry y", line_index + 1));
                continue;
            }
        };
        let width = match parse_positive_dimension(&fields[5]) {
            Ok(value) => value,
            Err(_) => {
                parse_errors.push(format!("line {}: invalid geometry width", line_index + 1));
                continue;
            }
        };
        let height = match parse_positive_dimension(&fields[6]) {
            Ok(value) => value,
            Err(_) => {
                parse_errors.push(format!("line {}: invalid geometry height", line_index + 1));
                continue;
            }
        };
        let raw_wm_class = fields[7].clone();
        let host = fields[8].as_str();
        let title = non_empty_string(title_remainder);
        let (app_id, wm_class, class_warning) = map_wm_class(&raw_wm_class);
        let pid_reliability = pid_reliability(raw_pid, host, local_hostname);
        let pid = match (raw_pid, &pid_reliability) {
            (Some(pid), PidReliability::Reliable | PidReliability::Unknown) if pid > 2 => Some(pid),
            _ => None,
        };
        let mut warnings = Vec::new();
        if let Some(warning) = class_warning {
            warnings.push(warning);
        }
        if pid_reliability != PidReliability::Reliable {
            warnings.push("PID reliability is not verified".to_string());
        }
        let owned_by_project = is_project_owned_internal_window(&raw_wm_class, title.as_deref());
        let non_application = raw_wm_class.to_ascii_lowercase().contains("desktop")
            || raw_wm_class.to_ascii_lowercase().contains("dock")
            || raw_wm_class.to_ascii_lowercase().contains("panel")
            || owned_by_project;
        if non_application {
            warnings.push("window may be a non-application target".to_string());
        }
        if owned_by_project {
            warnings.push("window is project-owned internal overlay/helper UI".to_string());
        }

        if !owned_by_project {
            windows.push(WindowInfo {
                window_id,
                title,
                app_id,
                wm_class,
                pid,
                bounds: Some(WindowBounds {
                    x: Some(x),
                    y: Some(y),
                    width,
                    height,
                }),
                workspace,
                focused: false,
                hidden: false,
                client_type: None,
                backend: BACKEND_ID.to_string(),
            });
        }
        metadata.push(WindowMetadata {
            window_id: Some(window_id),
            raw_id,
            raw_wm_class,
            source: "wmctrl -lpGx".to_string(),
            pid_reliability,
            non_application,
            owned_by_project,
            internal: owned_by_project,
            warnings,
        });
    }

    ParsedWmctrl {
        windows,
        metadata,
        parse_errors,
    }
}

pub fn parse_active_window_xprop(output: &str) -> Option<u64> {
    crate::focus::parse_active_window_xprop_state(output).active_window()
}

pub fn mark_focused_window(windows: &mut [WindowInfo], active_window: Option<u64>) {
    for window in windows {
        window.focused = active_window.is_some_and(|active| window.window_id == active);
    }
}

fn split_wmctrl_fixed_fields(line: &str) -> Option<(Vec<String>, &str)> {
    let mut fields = Vec::with_capacity(9);
    let mut in_field = false;
    let mut field_start = 0;
    let mut remainder_start = line.len();

    for (idx, ch) in line.char_indices() {
        if ch.is_whitespace() {
            if in_field {
                fields.push(line[field_start..idx].to_string());
                in_field = false;
                if fields.len() == 9 {
                    remainder_start = line[idx..]
                        .char_indices()
                        .find_map(|(offset, c)| (!c.is_whitespace()).then_some(idx + offset))
                        .unwrap_or(line.len());
                    break;
                }
            }
        } else if !in_field {
            in_field = true;
            field_start = idx;
        }
    }

    if in_field && fields.len() < 9 {
        fields.push(line[field_start..].to_string());
    }

    (fields.len() >= 9).then_some((fields, &line[remainder_start..]))
}

fn parse_positive_dimension(value: &str) -> Result<u32, ()> {
    let parsed = value.parse::<i64>().map_err(|_| ())?;
    if parsed <= 0 || parsed > u32::MAX as i64 {
        return Err(());
    }
    Ok(parsed as u32)
}

fn non_empty_string(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn map_wm_class(raw: &str) -> (Option<String>, Option<String>, Option<String>) {
    let raw = raw.trim();
    if raw.is_empty() || raw == "N/A" {
        return (None, None, None);
    }
    if let Some((instance, class)) = raw.rsplit_once('.') {
        return (non_empty_string(instance), non_empty_string(class), None);
    }
    (
        Some(raw.to_string()),
        Some(raw.to_string()),
        Some("app_id fell back to no-dot WM_CLASS value".to_string()),
    )
}

fn is_project_owned_internal_window(raw_wm_class: &str, title: Option<&str>) -> bool {
    let class = raw_wm_class.to_ascii_lowercase();
    let title = title.unwrap_or_default().to_ascii_lowercase();
    class.contains("codex-computer-use-x11-overlay")
        || class.contains("codex-computer-use-x11-helper")
        || title.contains("codex-computer-use-x11-overlay")
        || title.contains("codex-computer-use-x11-helper")
}

fn pid_reliability(pid: Option<u32>, host: &str, local_hostname: Option<&str>) -> PidReliability {
    let Some(pid) = pid else {
        return PidReliability::Unknown;
    };
    if pid <= 2 {
        return PidReliability::Unreliable;
    }
    match local_hostname.filter(|value| !value.trim().is_empty()) {
        Some(local) if !host.trim().is_empty() && host != local => PidReliability::Unreliable,
        Some(_) => PidReliability::Reliable,
        None => PidReliability::Unknown,
    }
}

pub fn report_from_system() -> WindowListReport {
    let display = std::env::var("DISPLAY")
        .ok()
        .filter(|value| !value.is_empty());
    let wmctrl_available = command_exists("wmctrl");
    let xprop_available = command_exists("xprop");
    let wmctrl_output = if display.is_some() && wmctrl_available {
        Some(run_command("wmctrl", &["-lpGx"]))
    } else {
        None
    };
    let xprop_root_output = if display.is_some() && xprop_available {
        Some(run_command("xprop", &["-root", "_NET_ACTIVE_WINDOW"]))
    } else {
        None
    };
    report_from_probe(WindowListProbeFacts {
        env_display: display,
        wmctrl_available,
        xprop_available,
        wmctrl_output,
        xprop_root_output,
        local_hostname: local_hostname(),
    })
}

pub fn report_from_probe(facts: WindowListProbeFacts) -> WindowListReport {
    let mut blockers = Vec::new();
    let mut degraded_reasons = Vec::new();
    let mut commands = Vec::new();
    let mut windows = Vec::new();
    let mut parse_errors = Vec::new();
    let mut window_metadata = Vec::new();
    let mut focused_window = None;

    let display_present = facts
        .env_display
        .as_deref()
        .is_some_and(|value| !value.is_empty());
    if !display_present {
        blockers.push("DISPLAY is unset; X11 window listing is unavailable".to_string());
    }

    commands.push(CommandStatus {
        name: "wmctrl".to_string(),
        available: facts.wmctrl_available,
        ok: facts.wmctrl_available,
        detail: if facts.wmctrl_available {
            "wmctrl is available".to_string()
        } else {
            "wmctrl is unavailable".to_string()
        },
    });
    commands.push(CommandStatus {
        name: "xprop".to_string(),
        available: facts.xprop_available,
        ok: facts.xprop_available,
        detail: if facts.xprop_available {
            "xprop is available".to_string()
        } else {
            "xprop is unavailable".to_string()
        },
    });

    if display_present && !facts.wmctrl_available {
        blockers.push("wmctrl is required for the MVP X11 window listing backend".to_string());
    }

    if display_present && facts.wmctrl_available {
        match facts.wmctrl_output {
            Some(Ok(output)) => {
                let parsed = parse_wmctrl_lpgx(&output, facts.local_hostname.as_deref());
                windows = parsed.windows;
                parse_errors = parsed.parse_errors;
                window_metadata = parsed.metadata;
            }
            Some(Err(err)) => blockers.push(format!("wmctrl failed: {err}")),
            None => blockers.push("wmctrl probe was not run".to_string()),
        }
    }

    if !parse_errors.is_empty() {
        degraded_reasons.push(format!(
            "{} wmctrl row(s) could not be parsed",
            parse_errors.len()
        ));
    }

    if display_present && !windows.is_empty() {
        if !facts.xprop_available {
            degraded_reasons
                .push("active window lookup unavailable because xprop is missing".to_string());
        } else {
            match facts.xprop_root_output {
                Some(Ok(output)) => {
                    let active_state = crate::focus::parse_active_window_xprop_state(&output);
                    focused_window = active_state.active_window();
                    match active_state {
                        crate::focus::ActiveWindowState::Active(_) => {}
                        crate::focus::ActiveWindowState::NoActive => {
                            degraded_reasons.push("no active X11 window was reported".to_string())
                        }
                        crate::focus::ActiveWindowState::Missing
                        | crate::focus::ActiveWindowState::Invalid => degraded_reasons
                            .push("active window id was not found in xprop output".to_string()),
                    }
                    mark_focused_window(&mut windows, focused_window);
                }
                Some(Err(err)) => {
                    degraded_reasons.push(format!("active window lookup failed: {err}"))
                }
                None => degraded_reasons.push("active window lookup was not run".to_string()),
            }
        }
    }

    WindowListReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        windows,
        diagnostics: WindowListingDiagnostics {
            ok: blockers.is_empty(),
            blockers,
            degraded_reasons,
            commands,
            parse_errors,
            window_metadata,
            focused_window,
            enrichment: EnrichmentDiagnostics {
                per_window_xprop_enabled: false,
                xprop_id_calls: 0,
                detail: "per-window xprop enrichment is disabled by default; client_type and hidden state may be unknown".to_string(),
            },
        },
    }
}

pub(crate) fn command_exists(name: &str) -> bool {
    Command::new("sh")
        .args(["-c", "command -v -- \"$1\" >/dev/null 2>&1", "sh", name])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn run_command(command: &str, args: &[&str]) -> Result<String, String> {
    match Command::new(command).args(args).output() {
        Ok(output) if output.status.success() => {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        Ok(output) => Err(String::from_utf8_lossy(&output.stderr).trim().to_string()),
        Err(err) => Err(err.to_string()),
    }
}

fn local_hostname() -> Option<String> {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .ok()
        .and_then(|value| non_empty_string(&value))
        .or_else(|| {
            std::env::var("HOSTNAME")
                .ok()
                .and_then(|value| non_empty_string(&value))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wmctrl_parser_maps_normal_window() {
        let parsed = parse_wmctrl_lpgx(
            "0x05624b36  0 26840 1920 64 1920 1008 gnome-terminal-server.Gnome-terminal ashp Terminal title\n",
            Some("ashp"),
        );
        assert!(parsed.parse_errors.is_empty(), "{:?}", parsed.parse_errors);
        assert_eq!(parsed.windows.len(), 1);
        let window = &parsed.windows[0];
        assert_eq!(window.window_id, 0x05624b36);
        assert_eq!(window.workspace, Some(0));
        assert_eq!(window.pid, Some(26840));
        assert_eq!(window.bounds.as_ref().unwrap().x, Some(1920));
        assert_eq!(window.bounds.as_ref().unwrap().y, Some(64));
        assert_eq!(window.bounds.as_ref().unwrap().width, 1920);
        assert_eq!(window.bounds.as_ref().unwrap().height, 1008);
        assert_eq!(window.app_id.as_deref(), Some("gnome-terminal-server"));
        assert_eq!(window.wm_class.as_deref(), Some("Gnome-terminal"));
        assert_eq!(window.title.as_deref(), Some("Terminal title"));
        assert_eq!(window.backend, BACKEND_ID);
        assert!(!window.focused);
        assert!(!window.hidden);
    }

    #[test]
    fn wmctrl_parser_preserves_unicode_title_remainder() {
        let parsed = parse_wmctrl_lpgx(
            "0x00000001  1 123 0 0 800 600 app.App host Заголовок с пробелами 🚀 флаг 🇷🇺\n",
            Some("host"),
        );
        assert!(parsed.parse_errors.is_empty(), "{:?}", parsed.parse_errors);
        assert_eq!(
            parsed.windows[0].title.as_deref(),
            Some("Заголовок с пробелами 🚀 флаг 🇷🇺")
        );
    }

    #[test]
    fn wmctrl_parser_validates_ids_and_geometry() {
        let parsed = parse_wmctrl_lpgx(
            "0x5624b36  0 123 -100 -20 640 480 app.App host Negative coords\n\
             0x05624b36 0 123 0 0 640 480 app.App host Padded same id\n\
             0x2 0 123 0 0 0 480 app.App host Bad zero width\n\
             0x3 0 123 0 0 -1 480 app.App host Bad negative width\n\
             0x4 0 123 0 0 nope 480 app.App host Bad nonnumeric width\n",
            Some("host"),
        );
        assert_eq!(parsed.windows[0].window_id, parsed.windows[1].window_id);
        assert_eq!(parsed.windows[0].bounds.as_ref().unwrap().x, Some(-100));
        assert_eq!(parsed.windows[0].bounds.as_ref().unwrap().y, Some(-20));
        assert_eq!(parsed.windows.len(), 2);
        assert_eq!(parsed.parse_errors.len(), 3);
        assert!(parsed
            .parse_errors
            .iter()
            .any(|error| error.contains("invalid geometry")));
    }

    #[test]
    fn list_windows_reports_class_and_pid_reliability() {
        let parsed = parse_wmctrl_lpgx(
            "0x1 0 0 0 0 100 100 instance.Class local Normal
             0x2 0 2 0 0 100 100 nodot local Service
             0x3 0 300 0 0 100 100 remote.App remotehost Remote
",
            Some("local"),
        );
        assert_eq!(parsed.windows[0].app_id.as_deref(), Some("instance"));
        assert_eq!(parsed.windows[0].wm_class.as_deref(), Some("Class"));
        assert_eq!(
            parsed.metadata[0].pid_reliability,
            PidReliability::Unreliable
        );
        assert_eq!(parsed.windows[0].pid, None);
        assert_eq!(parsed.windows[1].app_id.as_deref(), Some("nodot"));
        assert_eq!(parsed.windows[1].wm_class.as_deref(), Some("nodot"));
        assert!(parsed.metadata[1]
            .warnings
            .iter()
            .any(|warning| warning.contains("app_id fell back")));
        assert_eq!(
            parsed.metadata[2].pid_reliability,
            PidReliability::Unreliable
        );
        assert_eq!(parsed.windows[2].pid, None);
    }

    #[test]
    fn list_windows_marks_focused_window_from_xprop() {
        let mut parsed = parse_wmctrl_lpgx(
            "0x1 0 101 0 0 100 100 app.App host First
0x00000002 0 102 0 0 100 100 app.App host Second
",
            Some("host"),
        );
        let active = parse_active_window_xprop(
            "_NET_ACTIVE_WINDOW(WINDOW): window id # 0x2
",
        )
        .expect("active id parses");
        mark_focused_window(&mut parsed.windows, Some(active));
        assert!(!parsed.windows[0].focused);
        assert!(parsed.windows[1].focused);
    }
    #[test]
    fn list_windows_degrades_when_active_lookup_fails() {
        let facts = WindowListProbeFacts {
            env_display: Some(":99".to_string()),
            wmctrl_available: true,
            xprop_available: true,
            wmctrl_output: Some(Ok("0x1 0 101 0 0 100 100 app.App host First
"
            .to_string())),
            xprop_root_output: Some(Err("xprop failed".to_string())),
            local_hostname: Some("host".to_string()),
        };
        let report = report_from_probe(facts);
        assert_eq!(report.windows.len(), 1);
        assert!(!report.windows[0].focused);
        assert!(report
            .diagnostics
            .degraded_reasons
            .iter()
            .any(|reason| reason.contains("active window")));
    }

    #[test]
    fn list_windows_excludes_project_overlay_windows() {
        let parsed = parse_wmctrl_lpgx(
            "0x1 0 101 0 0 100 100 codex-computer-use-x11-overlay.codex-computer-use-x11-overlay host codex-computer-use-x11-overlay
0x2 0 102 0 0 100 100 app.App host Real App
",
            Some("host"),
        );
        assert_eq!(parsed.windows.len(), 1);
        assert_eq!(parsed.windows[0].window_id, 2);
        assert!(parsed.metadata.iter().any(|item| item.owned_by_project));
    }

    #[test]
    fn list_windows_does_not_spawn_unbounded_xprop_per_window() {
        let facts = WindowListProbeFacts {
            env_display: Some(":99".to_string()),
            wmctrl_available: true,
            xprop_available: true,
            wmctrl_output: Some(Ok("0x1 0 101 0 0 100 100 app.App host First
0x2 0 102 0 0 100 100 app.App host Second
"
            .to_string())),
            xprop_root_output: Some(Ok("_NET_ACTIVE_WINDOW(WINDOW): window id # 0x1
"
            .to_string())),
            local_hostname: Some("host".to_string()),
        };
        let report = report_from_probe(facts);
        assert!(!report.diagnostics.enrichment.per_window_xprop_enabled);
        assert_eq!(report.diagnostics.enrichment.xprop_id_calls, 0);
        assert!(report
            .windows
            .iter()
            .all(|window| window.client_type.is_none()));
    }

    #[test]
    fn list_windows_degrades_when_wmctrl_missing_or_fails() {
        let missing = WindowListProbeFacts {
            env_display: Some(":99".to_string()),
            wmctrl_available: false,
            ..WindowListProbeFacts::default()
        };
        let missing_report = report_from_probe(missing);
        assert!(missing_report.windows.is_empty());
        assert!(missing_report
            .diagnostics
            .blockers
            .iter()
            .any(|blocker| blocker.contains("wmctrl")));

        let failed = WindowListProbeFacts {
            env_display: Some(":99".to_string()),
            wmctrl_available: true,
            wmctrl_output: Some(Err("wmctrl exited 1".to_string())),
            ..WindowListProbeFacts::default()
        };
        let failed_report = report_from_probe(failed);
        assert!(failed_report.windows.is_empty());
        assert!(failed_report
            .diagnostics
            .blockers
            .iter()
            .any(|blocker| blocker.contains("wmctrl failed")));
    }
}
