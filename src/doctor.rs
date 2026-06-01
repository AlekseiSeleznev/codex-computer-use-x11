use std::collections::{BTreeMap, HashMap};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

pub const PROJECT_NAME: &str = "codex-computer-use-x11";
pub const BACKEND_ID: &str = "x11-ewmh";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DoctorReport {
    pub project: String,
    pub version: String,
    pub backend: String,
    pub readiness: Readiness,
    pub capabilities: Capabilities,
    pub checks: Vec<Check>,
    pub environment: EnvironmentReport,
    pub tools: ToolsReport,
    pub x11: X11Report,
    pub ydotool: YdotoolReport,
    pub portals: PortalReport,
    pub accessibility: AccessibilityReport,
    pub input: InputReport,
    pub source_overlay: SourceOverlayReport,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Readiness {
    pub ok: bool,
    pub blockers: Vec<String>,
    pub degraded_reasons: Vec<String>,
    pub required_baseline: String,
    pub blockers_detailed: Vec<ReadinessIssue>,
    pub degraded_acceptable: Vec<ReadinessIssue>,
    pub optional_enrichments: Vec<ReadinessIssue>,
    pub unsupported_out_of_scope: Vec<ReadinessIssue>,
    pub can_query_windows: bool,
    pub can_focus_apps: bool,
    pub can_focus_windows: bool,
    pub can_send_development_input: bool,
    pub recommended_next_step: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReadinessIssue {
    pub code: String,
    pub reason_category: String,
    pub detail: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Capabilities {
    pub implemented: Vec<String>,
    pub planned: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Check {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentReport {
    pub display_present: bool,
    pub wayland_display_present: bool,
    pub session_type: Option<String>,
    pub desktop: Option<String>,
    pub is_x11: bool,
    pub is_cinnamon: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolsReport {
    pub commands: Vec<CommandAvailability>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandAvailability {
    pub name: String,
    pub available: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct X11Report {
    pub ewmh: EwmhReport,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EwmhReport {
    pub supporting_wm_check_window: Option<u64>,
    pub active_window_seen: bool,
    pub can_query_windows: bool,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct YdotoolReport {
    pub candidates: Vec<YdotoolCandidate>,
    pub connectable_socket: Option<String>,
    pub ok: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct YdotoolCandidate {
    pub path: String,
    pub checked: bool,
    pub connectable: bool,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PortalReport {
    pub remote_desktop: PortalInterfaceReport,
    pub screenshot: ScreenshotReport,
    pub screencast: ReportOnlyFact,
    pub input_capture: ReportOnlyFact,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PortalInterfaceReport {
    pub available: bool,
    pub methods: Vec<String>,
    pub properties: Vec<String>,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScreenshotReport {
    pub xdg_portal_available: bool,
    pub xdg_portal_methods: Vec<String>,
    pub gnome_shell_dbus_available: bool,
    pub gnome_shell_methods: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportOnlyFact {
    pub present: bool,
    pub report_only: bool,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessibilityReport {
    pub atspi_bus_available: bool,
    pub tree_available: bool,
    pub blocker: Option<String>,
    pub diagnostic_state: String,
    pub reason_category: Option<String>,
    pub recommendation: String,
    pub bridge_env: AccessibilityBridgeEnv,
    pub match_outcome: Option<String>,
    pub candidate_count: Option<usize>,
    pub controlled_fixture_pass: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessibilityBridgeEnv {
    pub no_at_bridge_present: bool,
    pub no_at_bridge_value: Option<String>,
    pub gtk_modules: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InputReport {
    pub abs_pointer: InputBackendFact,
    pub ydotool: InputBackendFact,
    pub remote_desktop: InputBackendFact,
    pub xdotool_candidate: InputBackendFact,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InputBackendFact {
    pub available: bool,
    pub detail: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceOverlayReport {
    pub strict_portal_required: bool,
    pub target_vocabulary: Vec<String>,
    pub screenshot_provider_mapping: Vec<String>,
    pub target_checkout_modified: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ProbeFacts {
    pub env: HashMap<String, String>,
    pub tools: BTreeMap<String, bool>,
    pub xprop_root_output: Option<Result<String, String>>,
    pub ydotool_candidates: Vec<(String, bool)>,
    pub remote_desktop_introspection: Option<String>,
    pub screenshot_portal_introspection: Option<String>,
    pub gnome_shell_screenshot_introspection: Option<String>,
    pub atspi_bus_available: bool,
    pub atspi_tree_available: bool,
    pub atspi_match_outcome: Option<String>,
    pub atspi_candidate_count: Option<usize>,
    pub atspi_controlled_fixture_pass: bool,
    pub uinput_read_write: bool,
}

#[derive(Debug)]
pub struct DoctorError(pub String);

impl std::fmt::Display for DoctorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for DoctorError {}

pub fn handle_cli<F, W, E>(args: &[String], mut stdout: W, mut stderr: E, build_report: F) -> i32
where
    F: FnOnce() -> Result<DoctorReport, DoctorError>,
    W: Write,
    E: Write,
{
    if args == ["--help"] || args == ["-h"] {
        let _ = stdout.write_all(crate::USAGE.as_bytes());
        return 0;
    }

    if args != ["doctor", "--json"] {
        let _ = writeln!(stderr, "unsupported command; try --help");
        return 1;
    }

    let report = match build_report() {
        Ok(report) => report,
        Err(err) => {
            let _ = writeln!(stderr, "failed to build doctor report: {err}");
            return 1;
        }
    };

    match serde_json::to_vec(&report) {
        Ok(mut json) => {
            json.push(b'\n');
            match stdout.write_all(&json) {
                Ok(()) => 0,
                Err(err) => {
                    let _ = writeln!(stderr, "failed to write doctor report: {err}");
                    1
                }
            }
        }
        Err(err) => {
            let _ = writeln!(stderr, "failed to serialize doctor report: {err}");
            1
        }
    }
}

pub fn report_from_system() -> DoctorReport {
    report_from_probe(gather_system_facts())
}

pub fn bootstrap_report() -> DoctorReport {
    report_from_probe(ProbeFacts::default())
}

pub fn report_from_probe(facts: ProbeFacts) -> DoctorReport {
    let env = environment_report(&facts.env);
    let tools = tools_report(&facts.tools);
    let ewmh = ewmh_report(&env, &tools, facts.xprop_root_output.as_ref());
    let ydotool = ydotool_report(facts.ydotool_candidates);
    let portals = portal_report(
        facts.remote_desktop_introspection.as_deref(),
        facts.screenshot_portal_introspection.as_deref(),
        facts.gnome_shell_screenshot_introspection.as_deref(),
    );
    let accessibility = accessibility_report(
        &facts.env,
        facts.atspi_bus_available,
        facts.atspi_tree_available,
        facts.atspi_match_outcome,
        facts.atspi_candidate_count,
        facts.atspi_controlled_fixture_pass,
    );
    let input = input_report(
        facts.uinput_read_write,
        ydotool.ok,
        portals.remote_desktop.available,
        command_available(&tools, "xdotool"),
    );
    let readiness = readiness_report(
        &env,
        &tools,
        &ewmh,
        &ydotool,
        &portals,
        &accessibility,
        &input,
    );
    let checks = checks_report(&readiness, &tools, &ewmh);

    DoctorReport {
        project: PROJECT_NAME.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        backend: BACKEND_ID.to_string(),
        readiness,
        capabilities: Capabilities {
            implemented: vec![
                "doctor-json".to_string(),
                "doctor-capability-detection".to_string(),
                "x11-ewmh-windowing".to_string(),
                "x11-ewmh-window-listing".to_string(),
                "x11-ewmh-focus-with-verification".to_string(),
                "x11-targeted-input-safety".to_string(),
                "x11-pointer-actions".to_string(),
                "x11-atspi-window-correlation".to_string(),
                "x11-screenshot-coordinate-model".to_string(),
                "x11-get-app-state-integration".to_string(),
                "x11-target-window-groups-overlays".to_string(),
                "codex-x11-e2e-test-harness".to_string(),
                "x11-packaging-docs-upstreaming".to_string(),
                "x11-computer-use-architecture-dod".to_string(),
                "standalone-codex-mcp-plugin".to_string(),
                "codex-source-overlay-extension".to_string(),
            ],
            planned: Vec::new(),
        },
        checks,
        environment: env,
        tools,
        x11: X11Report { ewmh },
        ydotool,
        portals,
        accessibility,
        input,
        source_overlay: SourceOverlayReport {
            strict_portal_required: true,
            target_vocabulary: vec![
                "PortalReport".to_string(),
                "InputReport".to_string(),
                "WindowingReport".to_string(),
                "ReadinessReport".to_string(),
            ],
            screenshot_provider_mapping: vec![
                "xdg-portal-screenshot".to_string(),
                "gnome-shell-compatible-dbus".to_string(),
            ],
            target_checkout_modified: false,
        },
    }
}

fn gather_system_facts() -> ProbeFacts {
    let env: HashMap<String, String> = std::env::vars()
        .filter(|(key, _)| {
            matches!(
                key.as_str(),
                "DISPLAY"
                    | "WAYLAND_DISPLAY"
                    | "XDG_SESSION_TYPE"
                    | "XDG_CURRENT_DESKTOP"
                    | "DESKTOP_SESSION"
                    | "YDOTOOL_SOCKET"
                    | "XDG_RUNTIME_DIR"
                    | "NO_AT_BRIDGE"
                    | "GTK_MODULES"
            )
        })
        .collect();

    let tool_names = ["busctl", "gdbus", "wmctrl", "xprop", "xdotool", "ydotool"];
    let tools = tool_names
        .iter()
        .map(|name| ((*name).to_string(), command_exists(name)))
        .collect::<BTreeMap<_, _>>();

    let xprop_root_output =
        if env.get("DISPLAY").is_some_and(|v| !v.is_empty()) && tools.get("xprop") == Some(&true) {
            match run_command(
                "xprop",
                &["-root", "_NET_SUPPORTING_WM_CHECK", "_NET_ACTIVE_WINDOW"],
            ) {
                Ok(Some(output)) if output.status.success() => {
                    Some(Ok(String::from_utf8_lossy(&output.stdout).to_string()))
                }
                Ok(Some(output)) => Some(Err(String::from_utf8_lossy(&output.stderr)
                    .trim()
                    .to_string())),
                Ok(None) => Some(Err("command timed out".to_string())),
                Err(err) => Some(Err(err.to_string())),
            }
        } else {
            None
        };

    let ydotool_candidates = ydotool_candidate_paths(&env)
        .into_iter()
        .map(|(path, label)| {
            let connectable = unix_stream_connectable(path);
            (label, connectable)
        })
        .collect();

    let uinput_read_write = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/uinput")
        .is_ok();
    let remote_desktop_introspection = tools.get("busctl").copied().unwrap_or(false).then(|| {
        run_command_stdout(
            "busctl",
            &[
                "--user",
                "introspect",
                "org.freedesktop.portal.Desktop",
                "/org/freedesktop/portal/desktop",
                "org.freedesktop.portal.RemoteDesktop",
            ],
        )
        .unwrap_or_default()
    });
    let screenshot_portal_introspection = tools.get("gdbus").copied().unwrap_or(false).then(|| {
        run_command_stdout(
            "gdbus",
            &[
                "introspect",
                "--session",
                "--dest",
                "org.freedesktop.portal.Desktop",
                "--object-path",
                "/org/freedesktop/portal/desktop",
            ],
        )
        .unwrap_or_default()
    });
    let gnome_shell_screenshot_introspection =
        tools.get("gdbus").copied().unwrap_or(false).then(|| {
            run_command_stdout(
                "gdbus",
                &[
                    "introspect",
                    "--session",
                    "--dest",
                    "org.gnome.Shell.Screenshot",
                    "--object-path",
                    "/org/gnome/Shell/Screenshot",
                ],
            )
            .unwrap_or_default()
        });
    let atspi_bus_available = tools.get("gdbus").copied().unwrap_or(false)
        && run_command_stdout(
            "gdbus",
            &[
                "call",
                "--session",
                "--dest",
                "org.a11y.Bus",
                "--object-path",
                "/org/a11y/bus",
                "--method",
                "org.a11y.Bus.GetAddress",
            ],
        )
        .is_some_and(|output| output.contains("unix:") || output.contains("tcp:"));

    ProbeFacts {
        env,
        tools,
        xprop_root_output,
        ydotool_candidates,
        remote_desktop_introspection,
        screenshot_portal_introspection,
        gnome_shell_screenshot_introspection,
        atspi_bus_available,
        atspi_tree_available: false,
        atspi_match_outcome: None,
        atspi_candidate_count: None,
        atspi_controlled_fixture_pass: false,
        uinput_read_write,
    }
}

fn run_command_stdout(command: &str, args: &[&str]) -> Option<String> {
    match run_command(command, args) {
        Ok(Some(output)) if output.status.success() => {
            Some(String::from_utf8_lossy(&output.stdout).to_string())
        }
        _ => None,
    }
}

fn run_command(command: &str, args: &[&str]) -> std::io::Result<Option<std::process::Output>> {
    let timeout = command_timeout();
    let mut child = Command::new(command)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    let started = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output().map(Some);
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn command_timeout() -> Duration {
    std::env::var("CODEX_X11_COMMAND_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .map(Duration::from_millis)
        .unwrap_or_else(|| Duration::from_secs(2))
}

fn unix_stream_connectable(path: PathBuf) -> bool {
    let timeout = command_timeout();
    let (sender, receiver) = std::sync::mpsc::channel();
    let spawn_result = std::thread::Builder::new()
        .name("ydotool-socket-probe".to_string())
        .spawn(move || {
            let _ = sender.send(UnixStream::connect(path).is_ok());
        });
    if spawn_result.is_err() {
        return false;
    }

    receiver.recv_timeout(timeout).unwrap_or(false)
}

fn command_exists(name: &str) -> bool {
    Command::new("sh")
        .args(["-c", "command -v -- \"$1\" >/dev/null 2>&1", "sh", name])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn ydotool_candidate_paths(env: &HashMap<String, String>) -> Vec<(PathBuf, String)> {
    let mut paths = Vec::new();
    if let Some(path) = env.get("YDOTOOL_SOCKET").filter(|value| !value.is_empty()) {
        paths.push((PathBuf::from(path), "env:YDOTOOL_SOCKET".to_string()));
    }
    if let Some(runtime) = env.get("XDG_RUNTIME_DIR").filter(|value| !value.is_empty()) {
        paths.push((
            PathBuf::from(runtime).join(".ydotool_socket"),
            "env:XDG_RUNTIME_DIR/.ydotool_socket".to_string(),
        ));
    }
    paths.push((
        PathBuf::from("/tmp/.ydotool_socket"),
        "/tmp/.ydotool_socket".to_string(),
    ));
    paths
}

fn environment_report(env: &HashMap<String, String>) -> EnvironmentReport {
    let display_present = env.get("DISPLAY").is_some_and(|value| !value.is_empty());
    let wayland_display_present = env
        .get("WAYLAND_DISPLAY")
        .is_some_and(|value| !value.is_empty());
    let session_type = env.get("XDG_SESSION_TYPE").cloned();
    let desktop = env
        .get("XDG_CURRENT_DESKTOP")
        .or_else(|| env.get("DESKTOP_SESSION"))
        .cloned();
    let is_x11 = session_type
        .as_deref()
        .map(|value| value.eq_ignore_ascii_case("x11"))
        .unwrap_or(display_present);
    let is_cinnamon = desktop
        .as_deref()
        .map(|value| value.to_ascii_lowercase().contains("cinnamon"))
        .unwrap_or(false);

    EnvironmentReport {
        display_present,
        wayland_display_present,
        session_type,
        desktop,
        is_x11,
        is_cinnamon,
    }
}

fn tools_report(facts: &BTreeMap<String, bool>) -> ToolsReport {
    let commands = ["busctl", "gdbus", "wmctrl", "xprop", "xdotool", "ydotool"]
        .into_iter()
        .map(|name| CommandAvailability {
            name: name.to_string(),
            available: facts.get(name).copied().unwrap_or(false),
        })
        .collect();
    ToolsReport { commands }
}

fn command_available(tools: &ToolsReport, name: &str) -> bool {
    tools
        .commands
        .iter()
        .any(|tool| tool.name == name && tool.available)
}

fn ewmh_report(
    env: &EnvironmentReport,
    tools: &ToolsReport,
    xprop_output: Option<&Result<String, String>>,
) -> EwmhReport {
    if !env.display_present {
        return EwmhReport {
            supporting_wm_check_window: None,
            active_window_seen: false,
            can_query_windows: false,
            detail: "DISPLAY is unset".to_string(),
        };
    }
    if !command_available(tools, "wmctrl") || !command_available(tools, "xprop") {
        return EwmhReport {
            supporting_wm_check_window: None,
            active_window_seen: false,
            can_query_windows: false,
            detail: "wmctrl or xprop is unavailable".to_string(),
        };
    }

    match xprop_output {
        Some(Ok(output)) => parse_ewmh_xprop(output),
        Some(Err(err)) => EwmhReport {
            supporting_wm_check_window: None,
            active_window_seen: false,
            can_query_windows: false,
            detail: format!("xprop failed: {err}"),
        },
        None => EwmhReport {
            supporting_wm_check_window: None,
            active_window_seen: false,
            can_query_windows: false,
            detail: "xprop probe not run".to_string(),
        },
    }
}

pub fn parse_ewmh_xprop(output: &str) -> EwmhReport {
    let supporting = output
        .lines()
        .find(|line| line.starts_with("_NET_SUPPORTING_WM_CHECK"))
        .and_then(extract_xprop_window_id);
    let active_window_seen = output
        .lines()
        .any(|line| line.starts_with("_NET_ACTIVE_WINDOW"));
    let can_query_windows = supporting.is_some_and(|id| id != 0);
    let detail = if can_query_windows {
        "EWMH supporting window is present".to_string()
    } else if supporting == Some(0) {
        "EWMH supporting window id is zero".to_string()
    } else {
        "_NET_SUPPORTING_WM_CHECK is absent".to_string()
    };

    EwmhReport {
        supporting_wm_check_window: supporting,
        active_window_seen,
        can_query_windows,
        detail,
    }
}

fn extract_xprop_window_id(line: &str) -> Option<u64> {
    let (_, value) = line.split_once("#")?;
    let value = value.trim();
    let value = value.strip_prefix("0x").unwrap_or(value);
    u64::from_str_radix(value, 16).ok()
}

fn ydotool_report(candidates: Vec<(String, bool)>) -> YdotoolReport {
    let mut first_connectable = None;
    let candidates: Vec<YdotoolCandidate> = candidates
        .into_iter()
        .filter(|(path, _)| !path.is_empty())
        .map(|(path, connectable)| {
            if connectable && first_connectable.is_none() {
                first_connectable = Some(path.clone());
            }
            YdotoolCandidate {
                path,
                checked: true,
                connectable,
                detail: if connectable {
                    "connectable"
                } else {
                    "not connectable"
                }
                .to_string(),
            }
        })
        .collect();
    let ok = first_connectable.is_some();
    YdotoolReport {
        candidates,
        connectable_socket: first_connectable,
        ok,
    }
}

fn portal_report(
    remote: Option<&str>,
    screenshot: Option<&str>,
    gnome_shell: Option<&str>,
) -> PortalReport {
    let remote_methods = parse_introspection_members(remote.unwrap_or_default(), "method");
    let remote_properties = parse_introspection_members(remote.unwrap_or_default(), "property");
    let has_session = ["CreateSession", "SelectDevices", "Start"]
        .iter()
        .all(|name| remote_methods.iter().any(|method| method == name));
    let has_input = remote_methods
        .iter()
        .any(|method| method.starts_with("NotifyPointer") || method.starts_with("NotifyKeyboard"));
    let has_device_types = remote_properties
        .iter()
        .any(|property| property == "AvailableDeviceTypes");
    let remote_available = has_session && (has_input || has_device_types);

    let screenshot_methods = parse_introspection_members(screenshot.unwrap_or_default(), "method");
    let gnome_methods = parse_introspection_members(gnome_shell.unwrap_or_default(), "method");

    PortalReport {
        remote_desktop: PortalInterfaceReport {
            available: remote_available,
            methods: remote_methods,
            properties: remote_properties,
            detail: if remote_available {
                "RemoteDesktop exposes session and input methods"
            } else {
                "RemoteDesktop methods/properties are insufficient"
            }
            .to_string(),
        },
        screenshot: ScreenshotReport {
            xdg_portal_available: screenshot_methods
                .iter()
                .any(|method| method == "Screenshot"),
            xdg_portal_methods: screenshot_methods,
            gnome_shell_dbus_available: gnome_methods.iter().any(|method| method == "Screenshot"),
            gnome_shell_methods: gnome_methods,
        },
        screencast: ReportOnlyFact {
            present: false,
            report_only: true,
            detail: "ScreenCast is reported but does not gate readiness".to_string(),
        },
        input_capture: ReportOnlyFact {
            present: false,
            report_only: true,
            detail: "InputCapture is reported but does not gate readiness".to_string(),
        },
    }
}

pub fn parse_introspection_members(output: &str, kind: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let mut parts = line.split_whitespace();
            let first = parts.next()?;
            let second = parts.next();
            let raw_name = if first == kind {
                second?
            } else if second == Some(kind) {
                first
            } else if kind == "method" {
                gdbus_method_name(line)?
            } else if kind == "property" {
                gdbus_property_name(first, second, line)?
            } else {
                return None;
            };
            Some(
                raw_name
                    .trim_end_matches(';')
                    .trim_end_matches('(')
                    .to_string(),
            )
        })
        .collect()
}

fn gdbus_method_name(line: &str) -> Option<&str> {
    if line.starts_with("interface ")
        || line.starts_with("node ")
        || line.starts_with('{')
        || line.starts_with('}')
        || line.starts_with("//")
        || line.starts_with("readonly ")
    {
        return None;
    }
    let name = line.split_once('(')?.0.trim();
    (!name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.'))
    .then_some(name.rsplit('.').next().unwrap_or(name))
}

fn gdbus_property_name<'a>(
    first: &'a str,
    second: Option<&'a str>,
    line: &'a str,
) -> Option<&'a str> {
    if first == "readonly" {
        return second.and_then(|_| line.split_whitespace().nth(2));
    }
    None
}

fn accessibility_report(
    env: &HashMap<String, String>,
    bus: bool,
    tree: bool,
    match_outcome: Option<String>,
    candidate_count: Option<usize>,
    controlled_fixture_pass: bool,
) -> AccessibilityReport {
    let bridge_env = accessibility_bridge_env(env);
    let diagnostic_state = if controlled_fixture_pass {
        "controlled_fixture_pass".to_string()
    } else if !bus {
        "atspi_bus_unavailable".to_string()
    } else if !tree && bridge_env.no_at_bridge_present {
        "atspi_gtk_bridge_disabled_by_environment".to_string()
    } else if !tree {
        "atspi_tree_extraction_unavailable".to_string()
    } else if let Some(outcome) = match_outcome.as_deref() {
        match outcome {
            "no_matching_app_subtree" | "ambiguous_match" | "matched_subtree" => {
                outcome.to_string()
            }
            other => format!("atspi_{other}"),
        }
    } else {
        "tree_extraction_available".to_string()
    };
    let blocker = match diagnostic_state.as_str() {
        "atspi_bus_unavailable" => Some("AT-SPI bus unreachable".to_string()),
        "atspi_tree_extraction_unavailable" => {
            Some("AT-SPI bus reachable but tree extraction unavailable".to_string())
        }
        "atspi_gtk_bridge_disabled_by_environment" => {
            Some("AT-SPI bus reachable but GTK/ATK bridge disabled by NO_AT_BRIDGE".to_string())
        }
        "no_matching_app_subtree" => Some(
            "AT-SPI tree extraction worked but no app subtree matched the selected target"
                .to_string(),
        ),
        "ambiguous_match" => {
            Some("AT-SPI tree extraction worked but the target match was ambiguous".to_string())
        }
        _ => None,
    };
    let reason_category = blocker
        .as_ref()
        .map(|_| "environment_limitation".to_string());
    let recommendation = match diagnostic_state.as_str() {
        "atspi_bus_unavailable" => "Enable or expose the AT-SPI bus for richer Cinnamon/X11 accessibility diagnostics".to_string(),
        "atspi_gtk_bridge_disabled_by_environment" => "Remove or avoid inheriting NO_AT_BRIDGE=1 for GTK fixture/application processes, restart the affected Cinnamon/Codex session or fixture process, then verify semantic accessibility with the controlled GTK fixture".to_string(),
        "atspi_tree_extraction_unavailable" => "Enable AT-SPI tree extraction or install the GTK/ATK bridge for richer Cinnamon/X11 element diagnostics".to_string(),
        "no_matching_app_subtree" => "Use a controlled GTK fixture or inspect correlation signals; do not return an arbitrary subtree".to_string(),
        "ambiguous_match" => "Add stronger fixture identity signals or resolve competing AT-SPI candidates before returning a subtree".to_string(),
        "controlled_fixture_pass" => "Controlled GTK fixture AT-SPI evidence passed".to_string(),
        _ => "AT-SPI tree extraction is available for optional semantic diagnostics".to_string(),
    };
    AccessibilityReport {
        atspi_bus_available: bus,
        tree_available: tree,
        blocker,
        diagnostic_state,
        reason_category,
        recommendation,
        bridge_env,
        match_outcome,
        candidate_count,
        controlled_fixture_pass,
    }
}

fn accessibility_bridge_env(env: &HashMap<String, String>) -> AccessibilityBridgeEnv {
    AccessibilityBridgeEnv {
        no_at_bridge_present: env.contains_key("NO_AT_BRIDGE"),
        no_at_bridge_value: env.get("NO_AT_BRIDGE").map(|value| match value.as_str() {
            "0" | "1" => value.clone(),
            "" => "<empty>".to_string(),
            _ => "<set>".to_string(),
        }),
        gtk_modules: env.get("GTK_MODULES").cloned(),
    }
}

fn input_report(
    abs_pointer: bool,
    ydotool: bool,
    remote_desktop: bool,
    xdotool_candidate: bool,
) -> InputReport {
    InputReport {
        abs_pointer: InputBackendFact {
            available: abs_pointer,
            detail: if abs_pointer {
                "/dev/uinput read/write available"
            } else {
                "/dev/uinput read/write unavailable"
            }
            .to_string(),
        },
        ydotool: InputBackendFact {
            available: ydotool,
            detail: if ydotool {
                "ydotool socket connectable"
            } else {
                "no connectable ydotool socket"
            }
            .to_string(),
        },
        remote_desktop: InputBackendFact {
            available: remote_desktop,
            detail: if remote_desktop {
                "strict RemoteDesktop portal available"
            } else {
                "strict RemoteDesktop portal unavailable"
            }
            .to_string(),
        },
        xdotool_candidate: InputBackendFact {
            available: xdotool_candidate,
            detail: if xdotool_candidate {
                "xdotool installed as candidate only"
            } else {
                "xdotool unavailable"
            }
            .to_string(),
        },
    }
}

fn readiness_report(
    env: &EnvironmentReport,
    tools: &ToolsReport,
    ewmh: &EwmhReport,
    ydotool: &YdotoolReport,
    portals: &PortalReport,
    accessibility: &AccessibilityReport,
    input: &InputReport,
) -> Readiness {
    let mut blockers = Vec::new();
    let mut degraded_reasons = Vec::new();
    let mut blockers_detailed = Vec::new();
    let mut degraded_acceptable = Vec::new();
    let mut optional_enrichments = Vec::new();
    let mut unsupported_out_of_scope = Vec::new();

    if !env.display_present {
        let detail = "DISPLAY is unset; X11/EWMH window queries unavailable".to_string();
        blockers.push(detail.clone());
        blockers_detailed.push(readiness_issue(
            "x11_display_unavailable",
            "environment_limitation",
            detail,
            "Start an X11 session with DISPLAY set before claiming Cinnamon/X11 readiness",
        ));
    }
    if env.display_present
        && (!command_available(tools, "wmctrl") || !command_available(tools, "xprop"))
    {
        let detail = "wmctrl or xprop is missing; X11/EWMH window queries unavailable".to_string();
        blockers.push(detail.clone());
        blockers_detailed.push(readiness_issue(
            "x11_ewmh_tools_unavailable",
            "environment_limitation",
            detail,
            "Install wmctrl and xprop for the X11/EWMH baseline",
        ));
    }
    if env.display_present
        && command_available(tools, "wmctrl")
        && command_available(tools, "xprop")
        && !ewmh.can_query_windows
    {
        blockers.push(ewmh.detail.clone());
        blockers_detailed.push(readiness_issue(
            "x11_ewmh_probe_failed",
            "environment_limitation",
            ewmh.detail.clone(),
            "Inspect the X11 window manager EWMH support and xprop output",
        ));
    }
    if let Some(blocker) = &accessibility.blocker {
        degraded_reasons.push(blocker.clone());
        degraded_acceptable.push(readiness_issue(
            accessibility.diagnostic_state.clone(),
            accessibility
                .reason_category
                .as_deref()
                .unwrap_or("environment_limitation"),
            blocker.clone(),
            accessibility.recommendation.clone(),
        ));
    }
    if !portals.remote_desktop.available {
        let detail = "RemoteDesktop portal unavailable or incomplete".to_string();
        degraded_reasons.push(detail.clone());
        optional_enrichments.push(readiness_issue(
            "remote_desktop_portal_unavailable",
            "unsupported_out_of_scope",
            detail,
            "RemoteDesktop portal is diagnostic-only for this X11 baseline; do not require it for Cinnamon/X11 readiness",
        ));
    }
    if !ydotool.ok && !input.abs_pointer.available && !input.remote_desktop.available {
        let detail = "no development input backend is available".to_string();
        blockers.push(detail.clone());
        blockers_detailed.push(readiness_issue(
            "development_input_backend_unavailable",
            "environment_limitation",
            detail,
            "Enable /dev/uinput, ydotoold, or strict RemoteDesktop input before claiming development input readiness",
        ));
    }

    if env.wayland_display_present
        || env
            .session_type
            .as_deref()
            .is_some_and(|session| session.eq_ignore_ascii_case("wayland"))
    {
        unsupported_out_of_scope.push(readiness_issue(
            "wayland_runtime_out_of_scope",
            "unsupported_out_of_scope",
            "Wayland or WAYLAND_DISPLAY was observed, but this change supports only the Cinnamon/X11 baseline",
            "Use an X11 session for this baseline; design Wayland support only in a future scoped change",
        ));
    }

    let can_send_development_input =
        input.abs_pointer.available || input.ydotool.available || input.remote_desktop.available;
    let recommended_next_step = if !ewmh.can_query_windows {
        "Install/enable X11 EWMH tools and window-manager support".to_string()
    } else if !can_send_development_input {
        "Enable a development input backend: /dev/uinput, ydotoold, or RemoteDesktop portal input"
            .to_string()
    } else if accessibility.diagnostic_state == "atspi_gtk_bridge_disabled_by_environment" {
        "Remove or avoid inheriting NO_AT_BRIDGE=1, restart the affected Cinnamon/Codex session or fixture process, then verify with the controlled GTK fixture".to_string()
    } else if !accessibility.tree_available {
        "Enable AT-SPI tree extraction for richer element diagnostics".to_string()
    } else {
        "Ready for fixture-backed X11 doctor follow-up checks".to_string()
    };

    let can_focus_windows = ewmh.can_query_windows && ewmh.active_window_seen;
    let can_focus_apps = can_focus_windows;

    Readiness {
        ok: blockers.is_empty(),
        blockers,
        degraded_reasons,
        required_baseline: "cinnamon-x11".to_string(),
        blockers_detailed,
        degraded_acceptable,
        optional_enrichments,
        unsupported_out_of_scope,
        can_query_windows: ewmh.can_query_windows,
        can_focus_apps,
        can_focus_windows,
        can_send_development_input,
        recommended_next_step,
    }
}

fn readiness_issue(
    code: impl Into<String>,
    reason_category: impl Into<String>,
    detail: impl Into<String>,
    recommendation: impl Into<String>,
) -> ReadinessIssue {
    ReadinessIssue {
        code: code.into(),
        reason_category: reason_category.into(),
        detail: detail.into(),
        recommendation: recommendation.into(),
    }
}

fn checks_report(readiness: &Readiness, tools: &ToolsReport, ewmh: &EwmhReport) -> Vec<Check> {
    vec![
        Check {
            name: "doctor-report-schema".to_string(),
            ok: true,
            detail: "expanded additive doctor JSON report is serializable".to_string(),
        },
        Check {
            name: "read-only-probes".to_string(),
            ok: true,
            detail: "doctor uses read-only environment, command, socket, and fixture-safe probes"
                .to_string(),
        },
        Check {
            name: "x11-ewmh-query".to_string(),
            ok: readiness.can_query_windows,
            detail: ewmh.detail.clone(),
        },
        Check {
            name: "tool-availability".to_string(),
            ok: tools.commands.iter().all(|tool| tool.available),
            detail: tools
                .commands
                .iter()
                .map(|tool| format!("{}={}", tool.name, tool.available))
                .collect::<Vec<_>>()
                .join(","),
        },
        Check {
            name: "doctor-internal-self-check".to_string(),
            ok: true,
            detail: "readiness aggregation completed".to_string(),
        },
    ]
}

pub fn report_with_env(env: &[(&str, &str)]) -> DoctorReport {
    let facts = ProbeFacts {
        env: env
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect(),
        ..ProbeFacts::default()
    };
    report_from_probe(facts)
}

pub fn fake_tools(all_available: bool) -> BTreeMap<String, bool> {
    ["busctl", "gdbus", "wmctrl", "xprop", "xdotool", "ydotool"]
        .into_iter()
        .map(|name| (name.to_string(), all_available))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_facts() -> ProbeFacts {
        ProbeFacts {
            env: HashMap::from([
                ("DISPLAY".to_string(), ":0".to_string()),
                ("XDG_SESSION_TYPE".to_string(), "x11".to_string()),
                (
                    "XDG_CURRENT_DESKTOP".to_string(),
                    "X-Cinnamon".to_string(),
                ),
            ]),
            tools: fake_tools(true),
            xprop_root_output: Some(Ok("_NET_SUPPORTING_WM_CHECK(WINDOW): window id # 0x1234abcd\n_NET_ACTIVE_WINDOW(WINDOW): window id # 0x1234abcd\n".to_string())),
            ydotool_candidates: vec![("/tmp/.ydotool_socket".to_string(), true)],
            remote_desktop_introspection: Some("method CreateSession\nmethod SelectDevices\nmethod Start\nmethod NotifyPointerMotion\nproperty AvailableDeviceTypes\n".to_string()),
            screenshot_portal_introspection: Some(
                "method Screenshot\nmethod PickColor\nproperty version\n".to_string(),
            ),
            gnome_shell_screenshot_introspection: Some(
                "method Screenshot\nmethod ScreenshotArea\nmethod ScreenshotWindow\n".to_string(),
            ),
            atspi_bus_available: true,
            atspi_tree_available: true,
            atspi_match_outcome: None,
            atspi_candidate_count: None,
            atspi_controlled_fixture_pass: false,
            uinput_read_write: true,
        }
    }

    #[test]
    fn doctor_env_classifies_cinnamon_x11() {
        let report = report_from_probe(base_facts());
        assert!(report.environment.display_present);
        assert!(report.environment.is_x11);
        assert!(report.environment.is_cinnamon);
        assert_eq!(report.backend, BACKEND_ID);
    }

    #[test]
    fn doctor_no_display_returns_structured_report() {
        let report = report_from_probe(ProbeFacts::default());
        assert_eq!(report.project, PROJECT_NAME);
        assert!(!report.readiness.can_query_windows);
        assert!(report
            .readiness
            .blockers
            .iter()
            .any(|blocker| blocker.contains("DISPLAY")));
    }

    #[test]
    fn doctor_env_does_not_serialize_dbus_addresses_or_socket_paths() {
        let report = report_with_env(&[
            ("DISPLAY", ":0"),
            ("DBUS_SESSION_BUS_ADDRESS", "unix:path=/run/user/1000/bus"),
            ("YDOTOOL_SOCKET", "/private/runtime/socket"),
        ]);
        let json = serde_json::to_string(&report).unwrap();
        assert!(!json.contains("DBUS_SESSION_BUS_ADDRESS"));
        assert!(!json.contains("unix:path="));
        assert!(!json.contains("/private/runtime/socket"));
    }

    #[test]
    fn doctor_tools_report_available_and_missing_commands() {
        let mut facts = base_facts();
        facts.tools.insert("xprop".to_string(), false);
        let report = report_from_probe(facts);
        let xprop = report
            .tools
            .commands
            .iter()
            .find(|tool| tool.name == "xprop")
            .unwrap();
        assert!(!xprop.available);
        assert!(!report.readiness.can_query_windows);
    }

    #[test]
    fn doctor_can_query_windows_requires_x11_tools_and_xprop_probe() {
        let report = report_from_probe(base_facts());
        assert!(report.readiness.can_query_windows);
        assert_eq!(report.x11.ewmh.supporting_wm_check_window, Some(0x1234abcd));
    }

    #[test]
    fn doctor_can_query_windows_false_for_missing_display_tool_or_absent_ewmh() {
        let zero = parse_ewmh_xprop("_NET_SUPPORTING_WM_CHECK(WINDOW): window id # 0x0\n_NET_ACTIVE_WINDOW(WINDOW): window id # 0x1234abcd\n");
        assert!(!zero.can_query_windows);
        let absent = parse_ewmh_xprop("_NET_ACTIVE_WINDOW(WINDOW): window id # 0x1234abcd\n");
        assert!(!absent.can_query_windows);
    }

    #[test]
    fn doctor_ydotool_socket_continues_after_stale_candidates() {
        let mut facts = base_facts();
        facts.ydotool_candidates = vec![
            ("/stale.sock".to_string(), false),
            ("/tmp/.ydotool_socket".to_string(), true),
        ];
        let report = report_from_probe(facts);
        assert!(report.ydotool.ok);
        assert_eq!(
            report.ydotool.connectable_socket.as_deref(),
            Some("/tmp/.ydotool_socket")
        );
        assert_eq!(report.ydotool.candidates.len(), 2);
    }

    #[test]
    fn doctor_ydotool_socket_falls_back_when_env_candidate_absent() {
        let mut facts = base_facts();
        facts.ydotool_candidates = vec![("/tmp/.ydotool_socket".to_string(), true)];
        let report = report_from_probe(facts);
        assert_eq!(
            report.ydotool.connectable_socket.as_deref(),
            Some("/tmp/.ydotool_socket")
        );
    }

    #[test]
    fn doctor_ydotool_socket_reports_no_connectable_socket() {
        let mut facts = base_facts();
        facts.ydotool_candidates = vec![("/stale.sock".to_string(), false)];
        facts.uinput_read_write = false;
        facts.remote_desktop_introspection = Some(String::new());
        let report = report_from_probe(facts);
        assert!(!report.ydotool.ok);
        assert!(report.ydotool.connectable_socket.is_none());
        assert!(!report.readiness.can_send_development_input);
    }

    #[test]
    fn portal_parser_rejects_empty_remote_desktop_table() {
        let mut facts = base_facts();
        facts.remote_desktop_introspection = Some(String::new());
        let report = report_from_probe(facts);
        assert!(!report.portals.remote_desktop.available);
    }

    #[test]
    fn portal_parser_accepts_remote_desktop_with_required_methods() {
        let report = report_from_probe(base_facts());
        assert!(report.portals.remote_desktop.available);
        assert!(report
            .portals
            .remote_desktop
            .methods
            .iter()
            .any(|m| m == "CreateSession"));
        assert!(report
            .portals
            .remote_desktop
            .methods
            .iter()
            .any(|m| m == "NotifyPointerMotion"));
    }

    #[test]
    fn portal_parser_accepts_screenshot_v2_method() {
        let report = report_from_probe(base_facts());
        assert!(report.portals.screenshot.xdg_portal_available);
        assert!(report
            .portals
            .screenshot
            .xdg_portal_methods
            .iter()
            .any(|m| m == "Screenshot"));
    }

    #[test]
    fn screenshot_parser_accepts_cinnamon_gnome_shell_methods() {
        let report = report_from_probe(base_facts());
        assert!(report.portals.screenshot.gnome_shell_dbus_available);
        assert!(report
            .portals
            .screenshot
            .gnome_shell_methods
            .iter()
            .any(|m| m == "ScreenshotWindow"));
    }

    #[test]
    fn portal_report_marks_screencast_input_capture_report_only() {
        let mut facts = base_facts();
        facts.remote_desktop_introspection = Some(String::new());
        facts.uinput_read_write = true;
        let report = report_from_probe(facts);
        assert!(report.portals.screencast.report_only);
        assert!(report.portals.input_capture.report_only);
        assert!(
            report.readiness.ok,
            "report-only portal facts must not add blockers"
        );
    }

    #[test]
    fn doctor_accessibility_distinguishes_bus_from_enabled_state() {
        let mut missing_bus = base_facts();
        missing_bus.atspi_bus_available = false;
        missing_bus.atspi_tree_available = false;
        assert_eq!(
            report_from_probe(missing_bus)
                .accessibility
                .blocker
                .as_deref(),
            Some("AT-SPI bus unreachable")
        );
        let mut missing_tree = base_facts();
        missing_tree.atspi_bus_available = true;
        missing_tree.atspi_tree_available = false;
        assert_eq!(
            report_from_probe(missing_tree)
                .accessibility
                .blocker
                .as_deref(),
            Some("AT-SPI bus reachable but tree extraction unavailable")
        );
    }

    #[test]
    fn doctor_input_backends_derive_abs_pointer_locally() {
        let mut facts = base_facts();
        facts.uinput_read_write = true;
        facts.ydotool_candidates.clear();
        facts.remote_desktop_introspection = Some(String::new());
        let report = report_from_probe(facts);
        assert!(report.input.abs_pointer.available);
        assert!(report.readiness.can_send_development_input);
    }

    #[test]
    fn doctor_readiness_ok_is_false_when_blockers_exist() {
        let report = report_from_probe(ProbeFacts::default());
        assert!(!report.readiness.ok);
        assert!(!report.readiness.blockers.is_empty());
    }

    #[test]
    fn doctor_degraded_reasons_are_separate_from_blockers() {
        let mut facts = base_facts();
        facts.atspi_tree_available = false;
        let report = report_from_probe(facts);
        assert!(report.readiness.ok);
        assert!(report.readiness.blockers.is_empty());
        assert!(report
            .readiness
            .degraded_reasons
            .iter()
            .any(|reason| reason.contains("AT-SPI")));
    }

    #[test]
    fn doctor_development_input_is_or_of_supported_backends() {
        let mut facts = base_facts();
        facts.uinput_read_write = false;
        facts.ydotool_candidates.clear();
        facts.remote_desktop_introspection = Some(String::new());
        assert!(
            !report_from_probe(facts.clone())
                .readiness
                .can_send_development_input
        );
        facts.ydotool_candidates = vec![("/tmp/.ydotool_socket".to_string(), true)];
        assert!(
            report_from_probe(facts)
                .readiness
                .can_send_development_input
        );
    }

    #[test]
    fn doctor_focus_booleans_track_verified_x11_window_focus() {
        let report = report_from_probe(base_facts());
        assert!(report.readiness.can_focus_apps);
        assert!(report.readiness.can_focus_windows);

        let no_display = report_from_probe(ProbeFacts::default());
        assert!(!no_display.readiness.can_focus_apps);
        assert!(!no_display.readiness.can_focus_windows);
    }

    #[test]
    fn doctor_capabilities_reflect_finalized_v1_windowing() {
        let report = report_from_probe(base_facts());
        for capability in [
            "doctor-json",
            "doctor-capability-detection",
            "x11-ewmh-windowing",
            "x11-ewmh-window-listing",
            "x11-ewmh-focus-with-verification",
        ] {
            assert!(
                report
                    .capabilities
                    .implemented
                    .iter()
                    .any(|item| item == capability),
                "missing implemented capability {capability:?}: {:?}",
                report.capabilities.implemented
            );
        }
        assert!(
            !report
                .capabilities
                .planned
                .iter()
                .any(|item| item == "x11-ewmh-windowing"),
            "stale finalized v1 placeholder must not remain planned"
        );
    }

    #[test]
    fn doctor_readiness_ok_when_all_conditions_met() {
        let report = report_from_probe(base_facts());
        assert!(report.readiness.ok, "{:#?}", report.readiness);
    }

    #[test]
    fn doctor_readiness_exposes_additive_x11_taxonomy() {
        let mut facts = base_facts();
        facts.remote_desktop_introspection = Some(String::new());
        facts.atspi_tree_available = false;
        facts
            .env
            .insert("WAYLAND_DISPLAY".to_string(), "wayland-0".to_string());

        let report = report_from_probe(facts);
        assert!(
            report.readiness.ok,
            "optional degradations must not fail X11 baseline: {:#?}",
            report.readiness
        );
        assert_eq!(report.readiness.required_baseline, "cinnamon-x11");
        assert!(report.readiness.blockers_detailed.is_empty());
        assert!(report
            .readiness
            .degraded_acceptable
            .iter()
            .any(|item| item.code == "atspi_tree_extraction_unavailable"
                && item.reason_category == "environment_limitation"));
        assert!(report
            .readiness
            .optional_enrichments
            .iter()
            .any(|item| item.code == "remote_desktop_portal_unavailable"));
        assert!(report
            .readiness
            .unsupported_out_of_scope
            .iter()
            .any(|item| item.code == "wayland_runtime_out_of_scope"));
    }

    #[test]
    fn doctor_readiness_details_x11_window_blocker() {
        let report = report_from_probe(ProbeFacts::default());
        assert!(!report.readiness.ok);
        assert!(report
            .readiness
            .blockers_detailed
            .iter()
            .any(|item| item.code == "x11_display_unavailable"
                && item.reason_category == "environment_limitation"));
    }

    #[test]
    fn doctor_accessibility_reports_canonical_diagnostic_states() {
        let mut missing_bus = base_facts();
        missing_bus.atspi_bus_available = false;
        missing_bus.atspi_tree_available = false;
        let report = report_from_probe(missing_bus);
        assert_eq!(
            report.accessibility.diagnostic_state,
            "atspi_bus_unavailable"
        );
        assert_eq!(
            report.accessibility.reason_category.as_deref(),
            Some("environment_limitation")
        );

        let mut missing_tree = base_facts();
        missing_tree.atspi_bus_available = true;
        missing_tree.atspi_tree_available = false;
        let report = report_from_probe(missing_tree);
        assert_eq!(
            report.accessibility.diagnostic_state,
            "atspi_tree_extraction_unavailable"
        );
        assert!(report.accessibility.recommendation.contains("AT-SPI"));

        let mut no_match = base_facts();
        no_match.atspi_match_outcome = Some("no_matching_app_subtree".to_string());
        no_match.atspi_candidate_count = Some(3);
        let report = report_from_probe(no_match);
        assert_eq!(
            report.accessibility.diagnostic_state,
            "no_matching_app_subtree"
        );
        assert_eq!(report.accessibility.candidate_count, Some(3));

        let mut ambiguous = base_facts();
        ambiguous.atspi_match_outcome = Some("ambiguous_match".to_string());
        ambiguous.atspi_candidate_count = Some(2);
        let report = report_from_probe(ambiguous);
        assert_eq!(report.accessibility.diagnostic_state, "ambiguous_match");

        let mut pass = base_facts();
        pass.atspi_controlled_fixture_pass = true;
        let report = report_from_probe(pass);
        assert_eq!(
            report.accessibility.diagnostic_state,
            "controlled_fixture_pass"
        );
        assert!(report.accessibility.controlled_fixture_pass);
    }

    #[test]
    fn doctor_accessibility_reports_bridge_disabled_environment() {
        let mut facts = base_facts();
        facts
            .env
            .insert("NO_AT_BRIDGE".to_string(), "1".to_string());
        facts
            .env
            .insert("GTK_MODULES".to_string(), "gail:atk-bridge".to_string());
        facts.atspi_bus_available = true;
        facts.atspi_tree_available = false;

        let report = report_from_probe(facts);

        assert_eq!(
            report.accessibility.diagnostic_state,
            "atspi_gtk_bridge_disabled_by_environment"
        );
        assert!(report.accessibility.atspi_bus_available);
        assert!(!report.accessibility.tree_available);
        assert_eq!(
            report.accessibility.reason_category.as_deref(),
            Some("environment_limitation")
        );
        assert!(report
            .accessibility
            .recommendation
            .contains("NO_AT_BRIDGE=1"));
        assert!(report
            .accessibility
            .recommendation
            .contains("controlled GTK fixture"));
        assert!(report
            .readiness
            .recommended_next_step
            .contains("NO_AT_BRIDGE=1"));
        assert!(report.readiness.ok);

        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("\"no_at_bridge_present\":true"));
        assert!(json.contains("\"no_at_bridge_value\":\"1\""));
        assert!(json.contains("\"gtk_modules\":\"gail:atk-bridge\""));
        assert!(!json.contains("DBUS_SESSION_BUS_ADDRESS"));
    }

    #[test]
    fn doctor_accessibility_keeps_non_bridge_tree_unavailable_when_env_absent() {
        let mut facts = base_facts();
        facts.atspi_bus_available = true;
        facts.atspi_tree_available = false;

        let report = report_from_probe(facts);

        assert_eq!(
            report.accessibility.diagnostic_state,
            "atspi_tree_extraction_unavailable"
        );
        assert!(!report.accessibility.recommendation.contains("NO_AT_BRIDGE"));
    }

    #[test]
    fn doctor_recommended_next_step_priority_is_deterministic() {
        let no_display = report_from_probe(ProbeFacts::default());
        assert!(no_display
            .readiness
            .recommended_next_step
            .contains("X11 EWMH"));
        let mut no_input = base_facts();
        no_input.uinput_read_write = false;
        no_input.ydotool_candidates.clear();
        no_input.remote_desktop_introspection = Some(String::new());
        let report = report_from_probe(no_input);
        assert!(report
            .readiness
            .recommended_next_step
            .contains("development input"));
    }

    #[test]
    fn doctor_remote_desktop_gap_is_report_only_when_x11_input_path_works() {
        let mut facts = base_facts();
        facts.remote_desktop_introspection = Some(String::new());
        let report = report_from_probe(facts);
        assert!(report.readiness.ok, "{:#?}", report.readiness);
        assert!(report
            .readiness
            .degraded_reasons
            .iter()
            .any(|reason| reason.contains("RemoteDesktop portal unavailable")));
        assert!(
            !report
                .readiness
                .recommended_next_step
                .contains("RemoteDesktop"),
            "RemoteDesktop portal should not be recommended as the main next step when X11/EWMH and local input are usable: {}",
            report.readiness.recommended_next_step
        );
    }

    #[test]
    fn doctor_source_overlay_is_static_acceptance_metadata() {
        let report = report_from_probe(base_facts());
        assert!(report.source_overlay.strict_portal_required);
        assert!(report
            .source_overlay
            .target_vocabulary
            .iter()
            .any(|name| name == "PortalReport"));
        assert!(report
            .source_overlay
            .screenshot_provider_mapping
            .iter()
            .any(|name| name == "gnome-shell-compatible-dbus"));
    }

    #[test]
    fn doctor_does_not_modify_configured_target_checkout() {
        let report = report_from_probe(base_facts());
        assert!(!report.source_overlay.target_checkout_modified);
    }

    #[test]
    fn doctor_cli_success_emits_single_json_object() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let status = handle_cli(
            &["doctor".to_string(), "--json".to_string()],
            &mut out,
            &mut err,
            || Ok(report_from_probe(base_facts())),
        );
        assert_eq!(status, 0);
        assert!(err.is_empty());
        assert_eq!(out.iter().filter(|&&byte| byte == b'\n').count(), 1);
        serde_json::from_slice::<serde_json::Value>(&out).unwrap();
    }

    #[test]
    fn doctor_cli_no_display_emits_single_json_object() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let status = handle_cli(
            &["doctor".to_string(), "--json".to_string()],
            &mut out,
            &mut err,
            || Ok(report_from_probe(ProbeFacts::default())),
        );
        assert_eq!(status, 0);
        assert!(err.is_empty());
        let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(value["readiness"]["can_query_windows"], false);
    }

    #[test]
    fn doctor_cli_rejects_unsupported_usage() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let status = handle_cli(&["doctor".to_string()], &mut out, &mut err, || {
            Ok(report_from_probe(base_facts()))
        });
        assert_ne!(status, 0);
        assert!(out.is_empty());
        assert!(String::from_utf8(err).unwrap().contains("unsupported"));
    }

    #[test]
    fn doctor_cli_internal_failure_reports_stderr_without_partial_json() {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let status = handle_cli(
            &["doctor".to_string(), "--json".to_string()],
            &mut out,
            &mut err,
            || Err(DoctorError("boom".to_string())),
        );
        assert_ne!(status, 0);
        assert!(out.is_empty());
        assert!(String::from_utf8(err)
            .unwrap()
            .contains("failed to build doctor report"));
    }
}
