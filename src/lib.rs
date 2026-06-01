pub const USAGE: &str = "Usage:
  codex-computer-use-x11 doctor --json
  codex-computer-use-x11 list-windows --json
  codex-computer-use-x11 focused-window --json
  codex-computer-use-x11 focus-window --window-id <id> --json
  codex-computer-use-x11 type-text (--window-id <id>|--title <text>|--wm-class <class>|--pid <pid>) --text <text> --json
  codex-computer-use-x11 press-key (--window-id <id>|--title <text>|--wm-class <class>|--pid <pid>) --key <key> --json
  codex-computer-use-x11 click (--window-id <id>|--title <text>|--wm-class <class>|--pid <pid>|--global) --x <x> --y <y> --json
  codex-computer-use-x11 scroll (--window-id <id>|--title <text>|--wm-class <class>|--pid <pid>|--global) --x <x> --y <y> --direction <up|down|left|right> --json
  codex-computer-use-x11 drag (--window-id <id>|--title <text>|--wm-class <class>|--pid <pid>|--global) --start-x <x> --start-y <y> --end-x <x> --end-y <y> --json
  codex-computer-use-x11 accessibility-tree --window-id <id> --json
  codex-computer-use-x11 window-bounds --window-id <id> --json
  codex-computer-use-x11 screenshot-crop --window-id <id> [--x <x> --y <y> --width <w> --height <h>] --output <path> --json
  codex-computer-use-x11 get-app-state [--window-id <id>|--title <text>|--wm-class <class>|--pid <pid>] [--no-screenshot] [--screenshot-output <path>] [--inline-screenshot] --json
  codex-computer-use-x11 target-window (--window-id <id>|--title <text>|--wm-class <class>|--pid <pid>) [--group <id>] [--color <color>] [--overlay] --json
  codex-computer-use-x11 target-context --json
  codex-computer-use-x11 release-window (--window-id <id>|--all) --json
  codex-computer-use-x11 mcp
  codex-computer-use-x11 --help
";

pub mod accessibility;
pub mod app_state;
pub mod cli;
pub mod coordinates;
pub mod desktop_env;
pub mod doctor;
pub mod focus;
pub mod input;
pub mod list_windows;
pub mod mcp;
pub mod pointer;
pub mod target_window;
pub mod x11_id;
