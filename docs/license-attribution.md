# License and attribution notes

Observed during 2026-05-31 research refresh with `gh api repos/<owner>/<repo>/license --jq .license.spdx_id`, `gh repo view`, and current public package/license metadata. This document is engineering handoff guidance, not legal advice. Re-check license metadata before copying code or making upstream release claims.

## Policy

- runtime command dependency: an installed external command that project code invokes at runtime, such as `wmctrl`, `xdotool`, or `ydotool`.
- Invoking an installed command at runtime is distinct from copying, vendoring, or adapting its source code.
- MIT, BSD-3-Clause, and Apache-2.0 references are potentially copy-safe only with attribution, license text preservation, and NOTICE/header handling where the license requires it.
- NOASSERTION, no-license, GPL, AGPL, and unclear-license sources are copy-unsafe for MIT upstream code unless a later explicit license decision changes scope.
- No external source code is copied or vendored by this repository stage.

## Reference table

| Reference | Observed SPDX/status | Allowed use in this stage | Copy policy |
| --- | --- | --- | --- |
| `agent-sh/computer-use-linux` | MIT | Primary compatible backend lineage/reference | Potentially copy-safe only with attribution |
| local `codex-desktop-linux` / `CODEX_DESKTOP_LINUX_FULL_PATH` target checkout | MIT | Primary wrapper/integration target | Potentially copy-safe only with attribution |
| `tak-uukti/linux-computer-use` | MIT | Ideas/reference for simple X11/AT-SPI/XDOTOOL flows | Potentially copy-safe only with attribution |
| `wimi321/linux-computer-use-skill` | MIT | Ideas/reference | Potentially copy-safe only with attribution |
| `BeckhamLabsLLC/linux-desktop-mcp` | MIT | Ideas/reference for AT-SPI semantic targeting and desktop MCP UX | Potentially copy-safe only with attribution |
| `Touchpoint-Labs/Touchpoint` | MIT | Ideas/reference for accessibility-first desktop automation | Potentially copy-safe only with attribution |
| `MONTBRAIN/vadgr-computer-use` | Apache-2.0 | Ideas/reference for accessibility/vision fallback | Potentially copy-safe only with attribution and NOTICE/header compliance |
| `go-vgo/robotgo` | Apache-2.0 | Ideas/reference for cross-platform automation APIs | Potentially copy-safe only with attribution and NOTICE/header compliance |
| `joe223/sootie` | NOASSERTION | Ideas/reference only | copy-unsafe for MIT upstream code until manual license review succeeds |
| `hightemp/go_computer_use_mcp_server` | NO LICENSE ENDPOINT | Ideas/reference only | copy-unsafe for MIT upstream code |
| `linuxmint/cinnamon` | GPL-2.0 | Behavior/desktop reference only | copy-unsafe for MIT upstream code |
| `linuxmint/muffin` | GPL-2.0 | Behavior/window-manager reference only | copy-unsafe for MIT upstream code |
| `linuxmint/wayland` | NO LICENSE ENDPOINT | Ideas/reference only | copy-unsafe for MIT upstream code until manual license review succeeds |
| `linuxmint/cinnamon-spices-extensions` | GPL-2.0 | Extension behavior reference only | copy-unsafe for MIT upstream code |
| `Conservatory/wmctrl` | GPL-2.0 | Runtime command dependency when installed by the user/system | Source copying/vendoring is copy-unsafe for MIT upstream code |
| `jordansissel/xdotool` | BSD-3-Clause | Runtime command dependency and possible source reference | Source copying requires BSD attribution; invocation is not source copying |
| `ReimuNotMoe/ydotool` | AGPL-3.0 | Runtime command dependency through existing Codex paths | Source copying/vendoring is copy-unsafe for MIT upstream code without separate AGPL review |
| `psychon/x11rb` | Apache-2.0 | Candidate Rust dependency/reference for future native X11 work | Dependency/source reuse requires Apache-2.0 compliance |
| `github/github-mcp-server` | MIT | Documentation/security reference for MCP server configuration | Potentially copy-safe only with attribution |

## Attribution expectations

If a future change copies or adapts compatible external code, it must record the source repository, commit or release, license, and attribution/NOTICE handling in the change artifacts before implementation. If the copied source carries Apache-2.0 NOTICE or file-header obligations, preserve them in the appropriate tracked files before merge.

This stage adds documentation and tests only. It does not introduce a NOTICE file because it does not copy or vendor external source code or assets.
