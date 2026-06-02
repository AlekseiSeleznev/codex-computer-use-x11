use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

const DISABLE_ENV: &str = "CODEX_X11_DISABLE_DESKTOP_ENV_HYDRATION";
const FIXTURE_ENV: &str = "CODEX_X11_DESKTOP_ENV_FIXTURE";

const DESKTOP_ENV_ALLOWLIST: &[&str] = &[
    "DISPLAY",
    "WAYLAND_DISPLAY",
    "XDG_SESSION_TYPE",
    "XDG_CURRENT_DESKTOP",
    "XDG_SESSION_DESKTOP",
    "DBUS_SESSION_BUS_ADDRESS",
    "XDG_RUNTIME_DIR",
    "DESKTOP_SESSION",
    "XAUTHORITY",
    "HYPRLAND_INSTANCE_SIGNATURE",
    "YDOTOOL_SOCKET",
];

pub fn hydrate_mcp_desktop_env() {
    if hydration_disabled() {
        return;
    }

    let sources = desktop_env_sources();
    fill_missing_from_sources(&sources);
    fill_session_bus_from_runtime_dir();
}

fn hydration_disabled() -> bool {
    std::env::var(DISABLE_ENV)
        .ok()
        .map(|value| {
            let value = value.trim().to_ascii_lowercase();
            matches!(value.as_str(), "1" | "true" | "yes" | "on")
        })
        .unwrap_or(false)
}

fn desktop_env_sources() -> Vec<HashMap<String, String>> {
    let mut sources = Vec::new();
    if let Some(fixture) = std::env::var_os(FIXTURE_ENV) {
        if let Some(source) = read_env_file(Path::new(&fixture)).filter(|source| !source.is_empty())
        {
            sources.push(source);
        }
    }
    if let Some(source) = systemd_user_environment().filter(|source| !source.is_empty()) {
        sources.push(source);
    }
    sources.extend(parent_process_environments());
    sources
}

fn fill_missing_from_sources(sources: &[HashMap<String, String>]) {
    for key in DESKTOP_ENV_ALLOWLIST {
        if has_non_empty_env(key) {
            continue;
        }
        if let Some(value) = sources
            .iter()
            .find_map(|source| source.get(*key).filter(|value| !value.is_empty()))
        {
            std::env::set_var(key, value);
        }
    }
}

fn fill_session_bus_from_runtime_dir() {
    if has_non_empty_env("DBUS_SESSION_BUS_ADDRESS") {
        return;
    }
    let Some(runtime_dir) = std::env::var_os("XDG_RUNTIME_DIR") else {
        return;
    };
    if runtime_dir.is_empty() {
        return;
    }
    let bus = PathBuf::from(runtime_dir).join("bus");
    if bus.exists() {
        std::env::set_var(
            "DBUS_SESSION_BUS_ADDRESS",
            format!("unix:path={}", bus.display()),
        );
    }
}

fn has_non_empty_env(key: &str) -> bool {
    std::env::var_os(key).is_some_and(|value| !value.is_empty())
}

fn systemd_user_environment() -> Option<HashMap<String, String>> {
    let output = Command::new("systemctl")
        .args(["--user", "show-environment"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(parse_line_env(&String::from_utf8_lossy(&output.stdout)))
}

fn parent_process_environments() -> Vec<HashMap<String, String>> {
    let mut sources = Vec::new();
    let mut pid = parent_pid_of(std::process::id()).unwrap_or(0);
    let mut seen = std::collections::BTreeSet::new();
    for _ in 0..32 {
        if pid <= 1 || !seen.insert(pid) {
            break;
        }
        if let Some(source) = read_proc_environ(pid).filter(|source| !source.is_empty()) {
            sources.push(source);
        }
        pid = parent_pid_of(pid).unwrap_or(0);
    }
    sources
}

fn read_env_file(path: &Path) -> Option<HashMap<String, String>> {
    let text = std::fs::read_to_string(path).ok()?;
    Some(parse_line_env(&text))
}

fn parse_line_env(text: &str) -> HashMap<String, String> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            allowlisted_pair(key.trim(), value.trim())
        })
        .collect()
}

fn read_proc_environ(pid: u32) -> Option<HashMap<String, String>> {
    let bytes = std::fs::read(format!("/proc/{pid}/environ")).ok()?;
    Some(
        bytes
            .split(|byte| *byte == 0)
            .filter_map(|entry| {
                if entry.is_empty() {
                    return None;
                }
                let text = String::from_utf8_lossy(entry);
                let (key, value) = text.split_once('=')?;
                allowlisted_pair(key, value)
            })
            .collect(),
    )
}

fn parent_pid_of(pid: u32) -> Option<u32> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    status.lines().find_map(|line| {
        let value = line.strip_prefix("PPid:")?.trim();
        value.parse::<u32>().ok()
    })
}

fn allowlisted_pair(key: &str, value: &str) -> Option<(String, String)> {
    if value.is_empty() || !DESKTOP_ENV_ALLOWLIST.contains(&key) {
        return None;
    }
    Some((key.to_string(), value.to_string()))
}
