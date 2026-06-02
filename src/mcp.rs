use std::io::{BufRead, Write};

use serde_json::{json, Value};

const PROTOCOL_VERSION: &str = "2025-06-18";

pub fn serve_stdio<R, W>(mut reader: R, mut writer: W) -> i32
where
    R: BufRead,
    W: Write,
{
    let mut line = String::new();
    let mut target_state = crate::target_window::TargetState::default();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => return 0,
            Ok(_) => {}
            Err(err) => {
                let _ = write_response(
                    &mut writer,
                    json_rpc_error(
                        Value::Null,
                        -32603,
                        format!("failed to read request: {err}"),
                    ),
                );
                return 1;
            }
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let message: Value = match serde_json::from_str(trimmed) {
            Ok(message) => message,
            Err(err) => {
                let _ = write_response(
                    &mut writer,
                    json_rpc_error(Value::Null, -32700, format!("parse error: {err}")),
                );
                continue;
            }
        };

        if let Some(response) = handle_message(message, &mut target_state) {
            if write_response(&mut writer, response).is_err() {
                return 1;
            }
        }
    }
}

fn handle_message(
    message: Value,
    target_state: &mut crate::target_window::TargetState,
) -> Option<Value> {
    let id = message.get("id").cloned();
    let method = message.get("method").and_then(Value::as_str);

    if id.is_none() && method == Some("notifications/initialized") {
        return None;
    }

    let id = id.unwrap_or(Value::Null);
    let Some(method) = method else {
        return Some(json_rpc_error(id, -32600, "missing JSON-RPC method"));
    };

    match method {
        "initialize" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {
                    "tools": {
                        "listChanged": false
                    }
                },
                "serverInfo": {
                    "name": crate::doctor::PROJECT_NAME,
                    "version": env!("CARGO_PKG_VERSION")
                }
            }
        })),
        "tools/list" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": tool_definitions()
            }
        })),
        "tools/call" => Some(match call_tool(message.get("params"), target_state) {
            Ok(result) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": result
            }),
            Err(message) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": tool_error(message)
            }),
        }),
        _ => Some(json_rpc_error(
            id,
            -32601,
            format!("unsupported method: {method}"),
        )),
    }
}

fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "x11_doctor",
            "description": "Return codex-computer-use-x11 readiness and capability diagnostics as JSON.",
            "inputSchema": no_args_schema(),
        }),
        json!({
            "name": "x11_list_windows",
            "description": "Return the standalone X11/EWMH window listing report as JSON.",
            "inputSchema": no_args_schema(),
        }),
        json!({
            "name": "x11_focused_window",
            "description": "Return the current X11/EWMH focused window report as JSON.",
            "inputSchema": no_args_schema(),
        }),
        json!({
            "name": "x11_focus_window",
            "description": "Attempt to focus a listed X11/EWMH window and verify the active window id.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "window_id": {
                        "type": "string",
                        "description": "X11 window id in decimal or hexadecimal form, such as 12345 or 0x3039."
                    }
                },
                "required": ["window_id"],
                "additionalProperties": false
            },
        }),
        json!({
            "name": "x11_type_text",
            "description": "Type text into a targeted X11/EWMH window only after exact active-window focus verification.",
            "inputSchema": targeted_keyboard_schema("text", "Literal text to type after verified focus."),
        }),
        json!({
            "name": "x11_press_key",
            "description": "Press a key in a targeted X11/EWMH window only after exact active-window focus verification.",
            "inputSchema": targeted_keyboard_schema("key", "Key or key combination to press after verified focus."),
        }),
        json!({
            "name": "x11_click",
            "description": "Click global/root X11 coordinates after target bounds and focus verification, or with explicit global unverified mode.",
            "inputSchema": pointer_point_schema(),
        }),
        json!({
            "name": "x11_scroll",
            "description": "Scroll at global/root X11 coordinates after target bounds and focus verification, or with explicit global unverified mode.",
            "inputSchema": pointer_scroll_schema(),
        }),
        json!({
            "name": "x11_drag",
            "description": "Drag between global/root X11 coordinates after target bounds and focus verification, or with explicit global unverified mode.",
            "inputSchema": pointer_drag_schema(),
        }),
        json!({
            "name": "x11_accessibility_tree",
            "description": "Return an AT-SPI accessibility tree correlated to a concrete X11/EWMH window id, or a structured degraded/ambiguous report.",
            "inputSchema": accessibility_tree_schema(),
        }),
        json!({
            "name": "x11_get_app_state",
            "description": "Return composed X11/EWMH app state: optional window context, screenshot, AT-SPI tree, diagnostics, and per-layer errors.",
            "inputSchema": app_state_schema(),
        }),
        json!({
            "name": "x11_target_window",
            "description": "Save a resolved X11/EWMH window as explicit MCP process target context, optionally requesting a degraded visual overlay.",
            "inputSchema": target_window_schema(),
        }),
        json!({
            "name": "x11_release_window",
            "description": "Release a saved X11/EWMH target window or all target windows from this MCP process context.",
            "inputSchema": release_window_schema(),
        }),
        json!({
            "name": "x11_target_context",
            "description": "Return current MCP process target-window group context after stale validation.",
            "inputSchema": no_args_schema(),
        }),
    ]
}

fn call_tool(
    params: Option<&Value>,
    target_state: &mut crate::target_window::TargetState,
) -> Result<Value, String> {
    let params = params.ok_or_else(|| "tools/call params are required".to_string())?;
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| "tools/call params.name is required".to_string())?;

    match name {
        "x11_doctor" => report_tool_result(&crate::doctor::report_from_system(), false),
        "x11_list_windows" => report_tool_result(&crate::list_windows::report_from_system(), false),
        "x11_focused_window" => {
            report_tool_result(&crate::focus::focused_report_from_system(), false)
        }
        "x11_focus_window" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let window_id = window_id_argument(arguments)?;
            let requested_window_id = crate::x11_id::parse_x11_window_id(&window_id)
                .map_err(|err| format!("invalid X11 window_id: {err:?}"))?;
            let report = crate::focus::focus_window_report_from_system(requested_window_id);
            report_tool_result(&report, !report.success)
        }
        "x11_type_text" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let text = required_string_argument(arguments, "text")?;
            let target = target_from_arguments(arguments)?;
            let report = crate::input::type_text_report_from_system(target, text);
            report_tool_result(&report, !report.success)
        }
        "x11_press_key" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let key = required_string_argument(arguments, "key")?;
            let target = target_from_arguments(arguments)?;
            let report = crate::input::press_key_report_from_system(target, key);
            report_tool_result(&report, !report.success)
        }
        "x11_click" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let target = target_from_arguments(arguments)?;
            let global = optional_bool_argument(arguments, "global")?.unwrap_or(false);
            let point = crate::pointer::Point {
                x: required_i32_argument(arguments, "x")?,
                y: required_i32_argument(arguments, "y")?,
            };
            let button = pointer_button_argument(arguments.get("button"))?;
            let count = optional_u32_argument(arguments, "count")?.unwrap_or(1);
            let report =
                crate::pointer::click_report_from_system(target, global, point, button, count);
            report_tool_result(&report, !report.success)
        }
        "x11_scroll" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let target = target_from_arguments(arguments)?;
            let global = optional_bool_argument(arguments, "global")?.unwrap_or(false);
            let point = crate::pointer::Point {
                x: required_i32_argument(arguments, "x")?,
                y: required_i32_argument(arguments, "y")?,
            };
            let direction = required_string_argument(arguments, "direction")?;
            let button = scroll_button(&direction)?;
            let amount = optional_u32_argument(arguments, "amount")?.unwrap_or(1);
            let report =
                crate::pointer::scroll_report_from_system(target, global, point, button, amount);
            report_tool_result(&report, !report.success)
        }
        "x11_drag" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let target = target_from_arguments(arguments)?;
            let global = optional_bool_argument(arguments, "global")?.unwrap_or(false);
            let start = crate::pointer::Point {
                x: required_i32_argument(arguments, "start_x")?,
                y: required_i32_argument(arguments, "start_y")?,
            };
            let end = crate::pointer::Point {
                x: required_i32_argument(arguments, "end_x")?,
                y: required_i32_argument(arguments, "end_y")?,
            };
            let button = pointer_button_argument(arguments.get("button"))?;
            let report =
                crate::pointer::drag_report_from_system(target, global, start, end, button);
            report_tool_result(&report, !report.success)
        }
        "x11_accessibility_tree" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let window_id = window_id_argument(arguments)?;
            let requested_window_id = crate::x11_id::parse_x11_window_id(&window_id)
                .map_err(|err| format!("invalid X11 window_id: {err:?}"))?;
            let report =
                crate::accessibility::accessibility_tree_report_from_system(requested_window_id);
            report_tool_result(&report, !report.success)
        }
        "x11_get_app_state" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let target = target_from_arguments(arguments)?;
            let include_screenshot =
                optional_bool_argument(arguments, "include_screenshot")?.unwrap_or(true);
            let screenshot_output = optional_string_argument(arguments, "screenshot_output")?
                .map(std::path::PathBuf::from);
            let inline_screenshot =
                optional_bool_argument(arguments, "inline_screenshot")?.unwrap_or(false);
            let report = crate::app_state::get_app_state_report_from_system(
                crate::app_state::GetAppStateParams {
                    target,
                    include_screenshot,
                    screenshot_output,
                    inline_screenshot,
                },
            );
            report_tool_result(&report, false)
        }
        "x11_target_window" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let target = target_from_arguments(arguments)?;
            let group = optional_string_argument(arguments, "group")?;
            let color = match optional_string_argument(arguments, "color")? {
                Some(value) => crate::target_window::TargetColor::parse(&value)?,
                None => crate::target_window::TargetColor::Blue,
            };
            let overlay = optional_bool_argument(arguments, "overlay")?.unwrap_or(false);
            let report = crate::target_window::target_window_report_from_state(
                crate::target_window::TargetWindowParams {
                    target,
                    group,
                    color,
                    overlay,
                },
                target_state,
            );
            report_tool_result(&report, !report.success)
        }
        "x11_release_window" => {
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            let window_id = match arguments.get("window_id") {
                Some(Value::String(value)) if !value.trim().is_empty() => Some(
                    crate::x11_id::parse_x11_window_id(value)
                        .map_err(|err| format!("invalid X11 window_id: {err:?}"))?,
                ),
                Some(Value::Number(value)) => {
                    let value = value.to_string();
                    Some(
                        crate::x11_id::parse_x11_window_id(&value)
                            .map_err(|err| format!("invalid X11 window_id: {err:?}"))?,
                    )
                }
                Some(Value::Null) | None => None,
                Some(_) => return Err("window_id must be a string or number".to_string()),
            };
            let all = optional_bool_argument(arguments, "release_all")?.unwrap_or(false);
            let report = crate::target_window::release_window_report_from_state(
                crate::target_window::ReleaseWindowParams { window_id, all },
                target_state,
            );
            report_tool_result(&report, !report.success)
        }
        "x11_target_context" => {
            let report = crate::target_window::target_context_report_from_state(target_state);
            report_tool_result(&report, false)
        }
        _ => Err(format!("unsupported tool: {name}")),
    }
}

fn window_id_argument(arguments: &Value) -> Result<String, String> {
    match arguments.get("window_id") {
        Some(Value::String(value)) if !value.trim().is_empty() => Ok(value.clone()),
        Some(Value::Number(value)) => Ok(value.to_string()),
        Some(_) => {
            Err("x11_focus_window argument window_id must be a string or number".to_string())
        }
        None => Err("x11_focus_window requires argument window_id".to_string()),
    }
}

fn target_from_arguments(arguments: &Value) -> Result<crate::input::WindowTarget, String> {
    let window_id = match arguments.get("window_id") {
        Some(Value::String(value)) if !value.trim().is_empty() => Some(
            crate::x11_id::parse_x11_window_id(value)
                .map_err(|err| format!("invalid X11 window_id: {err:?}"))?,
        ),
        Some(Value::Number(value)) => {
            let value = value.to_string();
            Some(
                crate::x11_id::parse_x11_window_id(&value)
                    .map_err(|err| format!("invalid X11 window_id: {err:?}"))?,
            )
        }
        Some(Value::Null) | None => None,
        Some(_) => return Err("window_id must be a string or number".to_string()),
    };
    let pid = match arguments.get("pid") {
        Some(Value::Number(value)) => Some(
            value
                .as_u64()
                .and_then(|pid| u32::try_from(pid).ok())
                .ok_or_else(|| "pid must be a positive u32 number".to_string())?,
        ),
        Some(Value::Null) | None => None,
        Some(_) => return Err("pid must be a number".to_string()),
    };

    Ok(crate::input::WindowTarget {
        window_id,
        pid,
        title: optional_string_argument(arguments, "title")?,
        wm_class: optional_string_argument(arguments, "wm_class")?,
    })
}

fn required_string_argument(arguments: &Value, name: &str) -> Result<String, String> {
    arguments
        .get(name)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("x11 targeted input requires non-empty {name}"))
}

fn optional_string_argument(arguments: &Value, name: &str) -> Result<Option<String>, String> {
    match arguments.get(name) {
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(Value::Null) | None => Ok(None),
        Some(_) => Err(format!("{name} must be a string")),
    }
}

fn required_i32_argument(arguments: &Value, name: &str) -> Result<i32, String> {
    match arguments.get(name) {
        Some(Value::Number(value)) => value
            .as_i64()
            .and_then(|value| i32::try_from(value).ok())
            .ok_or_else(|| format!("{name} must be an i32 number")),
        Some(_) => Err(format!("{name} must be a number")),
        None => Err(format!("x11 pointer action requires {name}")),
    }
}

fn optional_u32_argument(arguments: &Value, name: &str) -> Result<Option<u32>, String> {
    match arguments.get(name) {
        Some(Value::Number(value)) => value
            .as_u64()
            .and_then(|value| u32::try_from(value).ok())
            .map(Some)
            .ok_or_else(|| format!("{name} must be a positive u32 number")),
        Some(Value::Null) | None => Ok(None),
        Some(_) => Err(format!("{name} must be a number")),
    }
}

fn optional_bool_argument(arguments: &Value, name: &str) -> Result<Option<bool>, String> {
    match arguments.get(name) {
        Some(Value::Bool(value)) => Ok(Some(*value)),
        Some(Value::Null) | None => Ok(None),
        Some(_) => Err(format!("{name} must be a boolean")),
    }
}

fn pointer_button_argument(value: Option<&Value>) -> Result<u8, String> {
    match value {
        Some(Value::String(value)) => match value.to_ascii_lowercase().as_str() {
            "left" | "primary" => Ok(1),
            "middle" => Ok(2),
            "right" | "secondary" => Ok(3),
            _ => Err(format!("unsupported pointer button: {value}")),
        },
        Some(Value::Number(value)) => value
            .as_u64()
            .filter(|value| (1..=3).contains(value))
            .map(|value| value as u8)
            .ok_or_else(|| "button must be 1, 2, or 3".to_string()),
        Some(Value::Null) | None => Ok(1),
        Some(_) => Err("button must be a string or number".to_string()),
    }
}

fn scroll_button(direction: &str) -> Result<u8, String> {
    match direction.to_ascii_lowercase().as_str() {
        "up" => Ok(4),
        "down" => Ok(5),
        "left" => Ok(6),
        "right" => Ok(7),
        _ => Err(format!("unsupported scroll direction: {direction}")),
    }
}

fn report_tool_result<T: serde::Serialize>(report: &T, is_error: bool) -> Result<Value, String> {
    let report_value = serde_json::to_value(report)
        .map_err(|err| format!("failed to serialize tool result: {err}"))?;
    let text = serde_json::to_string(&report_value)
        .map_err(|err| format!("failed to serialize tool result text: {err}"))?;
    Ok(json!({
        "content": [
            {
                "type": "text",
                "text": text
            }
        ],
        "structuredContent": report_value,
        "isError": is_error
    }))
}

fn tool_error(message: impl Into<String>) -> Value {
    json!({
        "content": [
            {
                "type": "text",
                "text": message.into()
            }
        ],
        "isError": true
    })
}

fn no_args_schema() -> Value {
    json!({
        "type": "object",
        "properties": {},
        "additionalProperties": false
    })
}

fn accessibility_tree_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "window_id": {
                "type": ["string", "number"],
                "description": "X11 window id in decimal or hexadecimal form, such as 12345 or 0x3039."
            }
        },
        "required": ["window_id"],
        "additionalProperties": false
    })
}

fn target_window_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "window_id": {"type": ["string", "number"], "description": "Optional X11 window id."},
            "pid": {"type": "number", "description": "Optional process id target selector."},
            "title": {"type": "string", "description": "Optional title substring target selector."},
            "wm_class": {"type": "string", "description": "Optional WM_CLASS target selector."},
            "group": {"type": "string", "description": "Optional target group id; defaults to default."},
            "color": {"type": "string", "enum": ["blue", "purple", "green", "orange", "red", "cyan"]},
            "overlay": {"type": "boolean", "description": "Request optional visual overlay; unsupported providers return warning-only reports."}
        },
        "additionalProperties": false
    })
}

fn release_window_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "window_id": {"type": ["string", "number"], "description": "X11 window id to release."},
            "release_all": {"type": "boolean", "description": "Release all saved target windows."}
        },
        "additionalProperties": false
    })
}

fn app_state_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "window_id": {
                "type": ["string", "number"],
                "description": "Optional X11 window id in decimal or hexadecimal form."
            },
            "pid": {
                "type": "number",
                "description": "Optional process id target selector."
            },
            "title": {
                "type": "string",
                "description": "Optional title substring target selector."
            },
            "wm_class": {
                "type": "string",
                "description": "Optional WM_CLASS target selector."
            },
            "include_screenshot": {
                "type": "boolean",
                "description": "Whether to include screenshot metadata/artifact capture; defaults to true."
            },
            "screenshot_output": {
                "type": "string",
                "description": "Optional path where app-state screenshot PNG should be written. JSON references the path instead of embedding pixels."
            },
            "inline_screenshot": {
                "type": "boolean",
                "description": "Unsafe debug opt-in to include screenshot data_url; defaults to false and must not be used for durable evidence."
            }
        },
        "additionalProperties": false
    })
}

fn targeted_keyboard_schema(payload_name: &str, payload_description: &str) -> Value {
    json!({
        "type": "object",
        "properties": {
            payload_name: {
                "type": "string",
                "description": payload_description
            },
            "window_id": {
                "type": ["string", "number"],
                "description": "X11 window id in decimal or hexadecimal form, such as 12345 or 0x3039."
            },
            "title": {
                "type": "string",
                "description": "Case-insensitive substring to match against listed window titles."
            },
            "wm_class": {
                "type": "string",
                "description": "Case-insensitive exact match against listed window WM_CLASS."
            },
            "pid": {
                "type": "number",
                "description": "Process id from the current window listing."
            }
        },
        "required": [payload_name],
        "additionalProperties": false
    })
}

fn pointer_point_schema() -> Value {
    let mut schema = pointer_base_schema();
    if let Some(properties) = schema.get_mut("properties").and_then(Value::as_object_mut) {
        properties.insert(
            "button".to_string(),
            json!({
                "type": ["string", "number"],
                "description": "Pointer button: left/primary/1, middle/2, or right/secondary/3."
            }),
        );
        properties.insert(
            "count".to_string(),
            json!({
                "type": "number",
                "description": "Click count; clamped to a safe finite range."
            }),
        );
    }
    schema["required"] = json!(["x", "y"]);
    schema
}

fn pointer_scroll_schema() -> Value {
    let mut schema = pointer_base_schema();
    if let Some(properties) = schema.get_mut("properties").and_then(Value::as_object_mut) {
        properties.insert(
            "direction".to_string(),
            json!({
                "type": "string",
                "enum": ["up", "down", "left", "right"]
            }),
        );
        properties.insert(
            "amount".to_string(),
            json!({
                "type": "number",
                "description": "Scroll amount; clamped to a safe finite range."
            }),
        );
    }
    schema["required"] = json!(["x", "y", "direction"]);
    schema
}

fn pointer_drag_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "start_x": {"type": "number"},
            "start_y": {"type": "number"},
            "end_x": {"type": "number"},
            "end_y": {"type": "number"},
            "button": {
                "type": ["string", "number"],
                "description": "Pointer button: left/primary/1, middle/2, or right/secondary/3."
            },
            "global": {
                "type": "boolean",
                "description": "Explicitly allow global unverified pointer injection without target/focus verification."
            },
            "window_id": {
                "type": ["string", "number"],
                "description": "X11 window id in decimal or hexadecimal form, such as 12345 or 0x3039."
            },
            "title": {
                "type": "string",
                "description": "Case-insensitive substring to match against listed window titles."
            },
            "wm_class": {
                "type": "string",
                "description": "Case-insensitive exact match against listed window WM_CLASS."
            },
            "pid": {
                "type": "number",
                "description": "Process id from the current window listing."
            }
        },
        "required": ["start_x", "start_y", "end_x", "end_y"],
        "additionalProperties": false
    })
}

fn pointer_base_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "x": {"type": "number"},
            "y": {"type": "number"},
            "global": {
                "type": "boolean",
                "description": "Explicitly allow global unverified pointer injection without target/focus verification."
            },
            "window_id": {
                "type": ["string", "number"],
                "description": "X11 window id in decimal or hexadecimal form, such as 12345 or 0x3039."
            },
            "title": {
                "type": "string",
                "description": "Case-insensitive substring to match against listed window titles."
            },
            "wm_class": {
                "type": "string",
                "description": "Case-insensitive exact match against listed window WM_CLASS."
            },
            "pid": {
                "type": "number",
                "description": "Process id from the current window listing."
            }
        },
        "required": [],
        "additionalProperties": false
    })
}

fn json_rpc_error(id: Value, code: i64, message: impl Into<String>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message.into()
        }
    })
}

fn write_response<W: Write>(writer: &mut W, response: Value) -> std::io::Result<()> {
    serde_json::to_writer(&mut *writer, &response)?;
    writer.write_all(b"\n")?;
    writer.flush()
}
