use std::io::Write;

use crate::{
    accessibility, app_state, coordinates, doctor, focus, input, list_windows, pointer,
    target_window,
};

pub fn handle_cli<W, E>(args: &[String], mut stdout: W, mut stderr: E) -> i32
where
    W: Write,
    E: Write,
{
    if args == ["--help"] || args == ["-h"] {
        let _ = stdout.write_all(crate::USAGE.as_bytes());
        return 0;
    }

    if args == ["doctor", "--json"] {
        return doctor::handle_cli(args, stdout, stderr, || Ok(doctor::report_from_system()));
    }

    if args == ["mcp"] {
        crate::desktop_env::hydrate_mcp_desktop_env();
        return crate::mcp::serve_stdio(std::io::stdin().lock(), stdout);
    }

    if args == ["list-windows", "--json"] {
        let report = list_windows::report_from_system();
        return match serde_json::to_vec(&report) {
            Ok(mut json) => {
                json.push(b'\n');
                match stdout.write_all(&json) {
                    Ok(()) => 0,
                    Err(err) => {
                        let _ = writeln!(stderr, "failed to write list-windows report: {err}");
                        1
                    }
                }
            }
            Err(err) => {
                let _ = writeln!(stderr, "failed to serialize list-windows report: {err}");
                1
            }
        };
    }

    if args == ["focused-window", "--json"] {
        let report = focus::focused_report_from_system();
        return match serde_json::to_vec(&report) {
            Ok(mut json) => {
                json.push(b'\n');
                match stdout.write_all(&json) {
                    Ok(()) => 0,
                    Err(err) => {
                        let _ = writeln!(stderr, "failed to write focused-window report: {err}");
                        1
                    }
                }
            }
            Err(err) => {
                let _ = writeln!(stderr, "failed to serialize focused-window report: {err}");
                1
            }
        };
    }

    if args.len() == 4
        && args[0] == "focus-window"
        && args[1] == "--window-id"
        && args[3] == "--json"
    {
        let requested_window_id = match crate::x11_id::parse_x11_window_id(&args[2]) {
            Ok(id) => id,
            Err(err) => {
                let _ = writeln!(stderr, "invalid X11 window id: {err:?}");
                return 1;
            }
        };
        let report = focus::focus_window_report_from_system(requested_window_id);
        let success = report.success;
        return match serde_json::to_vec(&report) {
            Ok(mut json) => {
                json.push(b'\n');
                match stdout.write_all(&json) {
                    Ok(()) if success => 0,
                    Ok(()) => 1,
                    Err(err) => {
                        let _ = writeln!(stderr, "failed to write focus-window report: {err}");
                        1
                    }
                }
            }
            Err(err) => {
                let _ = writeln!(stderr, "failed to serialize focus-window report: {err}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "type-text") {
        return match parse_targeted_input_args(&args[1..], "--text") {
            Ok((target, value)) => {
                let report = input::type_text_report_from_system(target, value);
                write_json_report(&report, report.success, stdout, stderr, "type-text")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "press-key") {
        return match parse_targeted_input_args(&args[1..], "--key") {
            Ok((target, value)) => {
                let report = input::press_key_report_from_system(target, value);
                write_json_report(&report, report.success, stdout, stderr, "press-key")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "click") {
        return match parse_click_args(&args[1..]) {
            Ok((target, global, point, button, count)) => {
                let report =
                    pointer::click_report_from_system(target, global, point, button, count);
                write_json_report(&report, report.success, stdout, stderr, "click")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "scroll") {
        return match parse_scroll_args(&args[1..]) {
            Ok((target, global, point, button, amount)) => {
                let report =
                    pointer::scroll_report_from_system(target, global, point, button, amount);
                write_json_report(&report, report.success, stdout, stderr, "scroll")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "drag") {
        return match parse_drag_args(&args[1..]) {
            Ok((target, global, start, end, button)) => {
                let report = pointer::drag_report_from_system(target, global, start, end, button);
                write_json_report(&report, report.success, stdout, stderr, "drag")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "accessibility-tree") {
        return match parse_accessibility_tree_args(&args[1..]) {
            Ok(window_id) => {
                let report = accessibility::accessibility_tree_report_from_system(window_id);
                write_json_report(
                    &report,
                    report.success,
                    stdout,
                    stderr,
                    "accessibility-tree",
                )
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "window-bounds") {
        return match parse_window_id_json_args(&args[1..], "window-bounds") {
            Ok(window_id) => {
                let report = coordinates::window_bounds_report_from_system(window_id);
                write_json_report(&report, report.success, stdout, stderr, "window-bounds")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "screenshot-crop") {
        return match parse_screenshot_crop_args(&args[1..]) {
            Ok((window_id, request, output_path)) => {
                let report = coordinates::screenshot_crop_report_from_system(
                    window_id,
                    request,
                    output_path,
                );
                write_json_report(&report, report.success, stdout, stderr, "screenshot-crop")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "get-app-state") {
        return match parse_get_app_state_args(&args[1..]) {
            Ok(params) => {
                let report = app_state::get_app_state_report_from_system(params);
                write_json_report(&report, true, stdout, stderr, "get-app-state")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args.first().is_some_and(|arg| arg == "target-window") {
        return match parse_target_window_args(&args[1..]) {
            Ok(params) => {
                let report = target_window::target_window_report_from_system(params);
                write_json_report(&report, report.success, stdout, stderr, "target-window")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    if args == ["target-context", "--json"] {
        let report = target_window::target_context_report_from_system();
        return write_json_report(&report, report.success, stdout, stderr, "target-context");
    }

    if args.first().is_some_and(|arg| arg == "release-window") {
        return match parse_release_window_args(&args[1..]) {
            Ok(params) => {
                let report = target_window::release_window_report_from_system(params);
                write_json_report(&report, report.success, stdout, stderr, "release-window")
            }
            Err(message) => {
                let _ = writeln!(stderr, "{message}");
                1
            }
        };
    }

    let _ = writeln!(stderr, "unsupported command; try --help");
    1
}

fn parse_click_args(
    args: &[String],
) -> Result<(input::WindowTarget, bool, pointer::Point, u8, u32), String> {
    let mut target = input::WindowTarget::default();
    let mut global = false;
    let mut json = false;
    let mut x = None;
    let mut y = None;
    let mut button = 1;
    let mut count = 1;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--global" => {
                global = true;
                index += 1;
            }
            "--x" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--x requires a value".to_string())?;
                x = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid x coordinate: {value}"))?,
                );
                index += 2;
            }
            "--y" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--y requires a value".to_string())?;
                y = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid y coordinate: {value}"))?,
                );
                index += 2;
            }
            "--button" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--button requires a value".to_string())?;
                button = parse_pointer_button(value)?;
                index += 2;
            }
            "--count" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--count requires a value".to_string())?;
                count = value
                    .parse::<u32>()
                    .map_err(|_| format!("invalid click count: {value}"))?;
                index += 2;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                target.window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            "--title" => {
                target.title = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--title requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--wm-class" => {
                target.wm_class = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--wm-class requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--pid" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--pid requires a value".to_string())?;
                target.pid = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("invalid pid: {value}"))?,
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for click: {other}")),
        }
    }
    if !json {
        return Err("pointer commands require --json".to_string());
    }
    Ok((
        target,
        global,
        pointer::Point {
            x: x.ok_or_else(|| "--x is required".to_string())?,
            y: y.ok_or_else(|| "--y is required".to_string())?,
        },
        button,
        count,
    ))
}

fn parse_pointer_button(value: &str) -> Result<u8, String> {
    match value.to_ascii_lowercase().as_str() {
        "left" | "primary" | "1" => Ok(1),
        "middle" | "2" => Ok(2),
        "right" | "secondary" | "3" => Ok(3),
        _ => Err(format!("unsupported pointer button: {value}")),
    }
}

fn parse_scroll_args(
    args: &[String],
) -> Result<(input::WindowTarget, bool, pointer::Point, u8, u32), String> {
    let mut target = input::WindowTarget::default();
    let mut global = false;
    let mut json = false;
    let mut x = None;
    let mut y = None;
    let mut button = None;
    let mut amount = 1;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--global" => {
                global = true;
                index += 1;
            }
            "--x" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--x requires a value".to_string())?;
                x = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid x coordinate: {value}"))?,
                );
                index += 2;
            }
            "--y" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--y requires a value".to_string())?;
                y = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid y coordinate: {value}"))?,
                );
                index += 2;
            }
            "--direction" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--direction requires a value".to_string())?;
                button = Some(parse_scroll_button(value)?);
                index += 2;
            }
            "--amount" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--amount requires a value".to_string())?;
                amount = value
                    .parse::<u32>()
                    .map_err(|_| format!("invalid scroll amount: {value}"))?;
                index += 2;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                target.window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            "--title" => {
                target.title = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--title requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--wm-class" => {
                target.wm_class = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--wm-class requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--pid" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--pid requires a value".to_string())?;
                target.pid = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("invalid pid: {value}"))?,
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for scroll: {other}")),
        }
    }
    if !json {
        return Err("pointer commands require --json".to_string());
    }
    Ok((
        target,
        global,
        pointer::Point {
            x: x.ok_or_else(|| "--x is required".to_string())?,
            y: y.ok_or_else(|| "--y is required".to_string())?,
        },
        button.ok_or_else(|| "--direction is required".to_string())?,
        amount,
    ))
}

fn parse_scroll_button(value: &str) -> Result<u8, String> {
    match value.to_ascii_lowercase().as_str() {
        "up" => Ok(4),
        "down" => Ok(5),
        "left" => Ok(6),
        "right" => Ok(7),
        _ => Err(format!("unsupported scroll direction: {value}")),
    }
}

fn parse_drag_args(
    args: &[String],
) -> Result<
    (
        input::WindowTarget,
        bool,
        pointer::Point,
        pointer::Point,
        u8,
    ),
    String,
> {
    let mut target = input::WindowTarget::default();
    let mut global = false;
    let mut json = false;
    let mut start_x = None;
    let mut start_y = None;
    let mut end_x = None;
    let mut end_y = None;
    let mut button = 1;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--global" => {
                global = true;
                index += 1;
            }
            "--start-x" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--start-x requires a value".to_string())?;
                start_x = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid start-x coordinate: {value}"))?,
                );
                index += 2;
            }
            "--start-y" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--start-y requires a value".to_string())?;
                start_y = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid start-y coordinate: {value}"))?,
                );
                index += 2;
            }
            "--end-x" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--end-x requires a value".to_string())?;
                end_x = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid end-x coordinate: {value}"))?,
                );
                index += 2;
            }
            "--end-y" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--end-y requires a value".to_string())?;
                end_y = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid end-y coordinate: {value}"))?,
                );
                index += 2;
            }
            "--button" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--button requires a value".to_string())?;
                button = parse_pointer_button(value)?;
                index += 2;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                target.window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            "--title" => {
                target.title = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--title requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--wm-class" => {
                target.wm_class = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--wm-class requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--pid" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--pid requires a value".to_string())?;
                target.pid = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("invalid pid: {value}"))?,
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for drag: {other}")),
        }
    }
    if !json {
        return Err("pointer commands require --json".to_string());
    }
    Ok((
        target,
        global,
        pointer::Point {
            x: start_x.ok_or_else(|| "--start-x is required".to_string())?,
            y: start_y.ok_or_else(|| "--start-y is required".to_string())?,
        },
        pointer::Point {
            x: end_x.ok_or_else(|| "--end-x is required".to_string())?,
            y: end_y.ok_or_else(|| "--end-y is required".to_string())?,
        },
        button,
    ))
}

fn parse_targeted_input_args(
    args: &[String],
    payload_flag: &str,
) -> Result<(input::WindowTarget, String), String> {
    let mut target = input::WindowTarget::default();
    let mut payload = None;
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                target.window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            "--title" => {
                target.title = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--title requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--wm-class" => {
                target.wm_class = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--wm-class requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--pid" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--pid requires a value".to_string())?;
                target.pid = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("invalid pid: {value}"))?,
                );
                index += 2;
            }
            flag if flag == payload_flag => {
                payload = Some(
                    args.get(index + 1)
                        .ok_or_else(|| format!("{payload_flag} requires a value"))?
                        .clone(),
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for targeted input: {other}")),
        }
    }
    if !json {
        return Err("targeted input commands require --json".to_string());
    }
    let payload = payload.ok_or_else(|| format!("{payload_flag} is required"))?;
    Ok((target, payload))
}

fn parse_get_app_state_args(args: &[String]) -> Result<app_state::GetAppStateParams, String> {
    let mut target = input::WindowTarget::default();
    let mut include_screenshot = true;
    let mut screenshot_output = None;
    let mut inline_screenshot = false;
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--no-screenshot" => {
                include_screenshot = false;
                index += 1;
            }
            "--screenshot-output" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--screenshot-output requires a value".to_string())?;
                screenshot_output = Some(std::path::PathBuf::from(value));
                index += 2;
            }
            "--inline-screenshot" => {
                inline_screenshot = true;
                index += 1;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                target.window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            "--title" => {
                target.title = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--title requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--wm-class" => {
                target.wm_class = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--wm-class requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--pid" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--pid requires a value".to_string())?;
                target.pid = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("invalid pid: {value}"))?,
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for get-app-state: {other}")),
        }
    }
    if !json {
        return Err("get-app-state requires --json".to_string());
    }
    Ok(app_state::GetAppStateParams {
        target,
        include_screenshot,
        screenshot_output,
        inline_screenshot,
    })
}

fn parse_target_window_args(args: &[String]) -> Result<target_window::TargetWindowParams, String> {
    let mut target = input::WindowTarget::default();
    let mut group = None;
    let mut color = target_window::TargetColor::Blue;
    let mut overlay = false;
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--overlay" => {
                overlay = true;
                index += 1;
            }
            "--group" => {
                group = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--group requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--color" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--color requires a value".to_string())?;
                color = target_window::TargetColor::parse(value)?;
                index += 2;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                target.window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            "--title" => {
                target.title = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--title requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--wm-class" => {
                target.wm_class = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--wm-class requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            "--pid" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--pid requires a value".to_string())?;
                target.pid = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("invalid pid: {value}"))?,
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for target-window: {other}")),
        }
    }
    if !json {
        return Err("target-window requires --json".to_string());
    }
    Ok(target_window::TargetWindowParams {
        target,
        group,
        color,
        overlay,
    })
}

fn parse_release_window_args(
    args: &[String],
) -> Result<target_window::ReleaseWindowParams, String> {
    let mut window_id = None;
    let mut all = false;
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--all" => {
                all = true;
                index += 1;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for release-window: {other}")),
        }
    }
    if !json {
        return Err("release-window requires --json".to_string());
    }
    if all && window_id.is_some() {
        return Err("release-window accepts either --all or --window-id, not both".to_string());
    }
    Ok(target_window::ReleaseWindowParams { window_id, all })
}

fn parse_screenshot_crop_args(
    args: &[String],
) -> Result<(u64, coordinates::CropRequest, String), String> {
    let mut window_id = None;
    let mut json = false;
    let mut x = None;
    let mut y = None;
    let mut width = None;
    let mut height = None;
    let mut output_path = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            "--x" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--x requires a value".to_string())?;
                x = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid x coordinate: {value}"))?,
                );
                index += 2;
            }
            "--y" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--y requires a value".to_string())?;
                y = Some(
                    value
                        .parse::<i32>()
                        .map_err(|_| format!("invalid y coordinate: {value}"))?,
                );
                index += 2;
            }
            "--width" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--width requires a value".to_string())?;
                width = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("invalid width: {value}"))?,
                );
                index += 2;
            }
            "--height" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--height requires a value".to_string())?;
                height = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("invalid height: {value}"))?,
                );
                index += 2;
            }
            "--output" => {
                output_path = Some(
                    args.get(index + 1)
                        .ok_or_else(|| "--output requires a value".to_string())?
                        .clone(),
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for screenshot-crop: {other}")),
        }
    }
    if !json {
        return Err("screenshot-crop requires --json".to_string());
    }
    Ok((
        window_id.ok_or_else(|| "--window-id is required".to_string())?,
        coordinates::CropRequest {
            x,
            y,
            width,
            height,
        },
        output_path.ok_or_else(|| "--output is required".to_string())?,
    ))
}

fn parse_window_id_json_args(args: &[String], command_name: &str) -> Result<u64, String> {
    let mut window_id = None;
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            other => return Err(format!("unsupported argument for {command_name}: {other}")),
        }
    }
    if !json {
        return Err(format!("{command_name} requires --json"));
    }
    window_id.ok_or_else(|| "--window-id is required".to_string())
}

fn parse_accessibility_tree_args(args: &[String]) -> Result<u64, String> {
    let mut window_id = None;
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                json = true;
                index += 1;
            }
            "--window-id" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--window-id requires a value".to_string())?;
                window_id = Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window id: {err:?}"))?,
                );
                index += 2;
            }
            other => {
                return Err(format!(
                    "unsupported argument for accessibility-tree: {other}"
                ))
            }
        }
    }
    if !json {
        return Err("accessibility-tree requires --json".to_string());
    }
    window_id.ok_or_else(|| "--window-id is required".to_string())
}

fn write_json_report<T, W, E>(
    report: &T,
    success: bool,
    mut stdout: W,
    mut stderr: E,
    command_name: &str,
) -> i32
where
    T: serde::Serialize,
    W: Write,
    E: Write,
{
    match serde_json::to_vec(report) {
        Ok(mut json) => {
            json.push(b'\n');
            match stdout.write_all(&json) {
                Ok(()) if success => 0,
                Ok(()) => 1,
                Err(err) => {
                    let _ = writeln!(stderr, "failed to write {command_name} report: {err}");
                    1
                }
            }
        }
        Err(err) => {
            let _ = writeln!(stderr, "failed to serialize {command_name} report: {err}");
            1
        }
    }
}
