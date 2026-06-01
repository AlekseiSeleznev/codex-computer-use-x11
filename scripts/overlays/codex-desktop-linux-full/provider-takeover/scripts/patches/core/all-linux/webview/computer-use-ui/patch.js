"use strict";

const {
  applyLinuxComputerUseRendererAvailabilityPatch,
  applyLinuxComputerUseInstallFlowPatch,
  applyX11ComputerUseSettingsRowPatch,
} = require("../../../../computer-use.js");

module.exports = [
  {
    id: "linux-computer-use-ui-availability",
    phase: "webview-asset",
    order: 1100,
    ciPolicy: "opt-in",
    enabled: (context) => context.enableComputerUseUi,
    pattern: /^(use-model-settings|apps|use-in-app-browser-use-availability|use-is-plugins-enabled)-.*\.js$/,
    missingDescription: "Computer Use availability bundle",
    skipDescription: "Linux Computer Use UI availability patch",
    apply: applyLinuxComputerUseRendererAvailabilityPatch,
  },
  {
    id: "linux-computer-use-install-flow",
    phase: "webview-asset",
    order: 1110,
    ciPolicy: "opt-in",
    enabled: (context) => context.enableComputerUseUi,
    pattern: /^(use-plugin-install-flow|plugins-availability)-.*\.js$/,
    missingDescription: "plugin install flow bundle",
    skipDescription: "Linux Computer Use install flow patch",
    apply: applyLinuxComputerUseInstallFlowPatch,
  },
  {
    id: "linux-x11-computer-use-provider-takeover",
    phase: "webview-asset",
    order: 1120,
    ciPolicy: "opt-in",
    enabled: (context) => context.enableComputerUseUi,
    pattern: /^computer-use-settings-.*\.js$/,
    missingDescription: "Computer Use settings bundle",
    skipDescription: "X11 Computer Use provider takeover patch",
    apply: applyX11ComputerUseSettingsRowPatch,
  },
];
