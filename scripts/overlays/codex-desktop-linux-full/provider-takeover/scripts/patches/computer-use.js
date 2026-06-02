"use strict";

const fs = require("node:fs");
const path = require("node:path");

const COMPUTER_USE_UI_ENV_VAR = "CODEX_LINUX_ENABLE_COMPUTER_USE_UI";
const COMPUTER_USE_UI_SETTINGS_KEY = "codex-linux-computer-use-ui-enabled";

// Computer Use has two postures: the bundled plugin gate is default-on Linux
// platform glue; the visible UI gates remain opt-in because they bypass rollout
// checks in upstream webview code.
function isComputerUseUiEnabled(env = process.env) {
  if (env[COMPUTER_USE_UI_ENV_VAR] === "1") {
    return true;
  }
  return readComputerUseUiSettingsFlag(env);
}

function readComputerUseUiSettingsFlag(env) {
  const settingsPath = computerUseUiSettingsPath(env);
  if (settingsPath == null) {
    return false;
  }
  try {
    if (!fs.existsSync(settingsPath)) {
      return false;
    }
    const raw = fs.readFileSync(settingsPath, "utf8");
    const parsed = JSON.parse(raw);
    if (parsed == null || typeof parsed !== "object" || Array.isArray(parsed)) {
      return false;
    }
    return parsed[COMPUTER_USE_UI_SETTINGS_KEY] === true;
  } catch {
    return false;
  }
}

function computerUseUiSettingsPath(env) {
  const xdgConfig = env.XDG_CONFIG_HOME;
  const home = env.HOME;
  const configHome = (xdgConfig && xdgConfig.length > 0)
    ? xdgConfig
    : home
      ? path.join(home, ".config")
      : null;
  if (configHome == null) {
    return null;
  }
  const appId = computerUseUiSettingsAppId(env);
  return path.join(configHome, appId, "settings.json");
}

function computerUseUiSettingsAppId(env) {
  const appId = env.CODEX_APP_ID || env.CODEX_LINUX_APP_ID || "codex-desktop";
  return /^[A-Za-z0-9._-]+$/.test(appId) ? appId : "codex-desktop";
}

// Lookback/lookahead windows used when searching for the nearest minified
// identifier or surrounding context around a regex anchor in the bundle.
// Sized empirically to the typical distance between a feature's anchor and
// the helper aliases it depends on.
const TRAY_GUARD_LOOKAHEAD = 1200;
const CLOSE_GATE_PREFIX_LOOKBACK = 8000;
const HANDLER_PREFIX_LOOKBACK = 12000;
const DIRECT_HANDLER_PROXIMITY = 1200;

const linuxSettingsKeys = {
  promptWindow: "codex-linux-prompt-window-enabled",
  systemTray: "codex-linux-system-tray-enabled",
  warmStart: "codex-linux-warm-start-enabled",
};

const COMPUTER_USE_PROVIDER_TAKEOVER_MARKER =
  "codex-computer-use-x11-provider-takeover:v1";
const BUNDLED_COMPUTER_USE_PLUGIN_ID = "computer-use";
const X11_COMPUTER_USE_PLUGIN_ID = "codex-computer-use-x11";

function pluginId(plugin) {
  return plugin?.id ?? plugin?.name ?? plugin?.pluginId ?? null;
}

function pluginMarketplaceName(plugin) {
  return plugin?.marketplaceName ?? plugin?.marketplace ?? plugin?.marketplaceId ?? null;
}

function pluginDisplayName(plugin) {
  return plugin?.displayName ?? plugin?.nameForDisplay ?? plugin?.name ?? plugin?.title ?? null;
}

function findPluginById(plugins, id) {
  return Array.isArray(plugins) ? plugins.find((plugin) => pluginId(plugin) === id) ?? null : null;
}

function sanitizedPlugin(plugin) {
  return {
    id: pluginId(plugin),
    marketplaceName: pluginMarketplaceName(plugin),
    displayName: pluginDisplayName(plugin),
  };
}

function sanitizedPluginList(plugins) {
  return Array.isArray(plugins) ? plugins.map(sanitizedPlugin) : [];
}

function sanitizedAvailability(availability) {
  if (availability == null || typeof availability !== "object") {
    return {
      available: false,
      isFetching: false,
      isLoading: false,
    };
  }
  return {
    available: availability.available === true,
    isFetching: availability.isFetching === true,
    isLoading: availability.isLoading === true,
  };
}

function resolveComputerUseProviderRows({
  availablePlugins = [],
  installedPlugins = [],
  computerUseAvailability = {},
  mode = "bundled",
  provider = "bundled",
} = {}) {
  const availability = sanitizedAvailability(computerUseAvailability);
  const bundledPlugin = findPluginById(availablePlugins, BUNDLED_COMPUTER_USE_PLUGIN_ID);
  const installedX11Plugin = findPluginById(installedPlugins, X11_COMPUTER_USE_PLUGIN_ID);
  const availableX11Plugin = findPluginById(availablePlugins, X11_COMPUTER_USE_PLUGIN_ID);
  const x11Plugin = installedX11Plugin ?? availableX11Plugin;
  const x11LookupSource = installedX11Plugin != null
    ? "installedPlugins"
    : availableX11Plugin != null
      ? "availablePlugins"
      : "none";
  const takeover = mode === "takeover" && provider === "x11";

  const rowDecisions = [];
  if (bundledPlugin == null) {
    rowDecisions.push({
      provider: "bundled",
      pluginId: BUNDLED_COMPUTER_USE_PLUGIN_ID,
      lookupSource: "none",
      state: takeover ? "hidden" : "absent",
      reason: takeover ? "x11-takeover-enabled" : "bundled-plugin-missing",
    });
  } else if (takeover) {
    rowDecisions.push({
      provider: "bundled",
      pluginId: BUNDLED_COMPUTER_USE_PLUGIN_ID,
      lookupSource: "availablePlugins",
      state: "hidden",
      reason: "x11-takeover-enabled",
    });
  } else if (availability.available) {
    rowDecisions.push({
      provider: "bundled",
      pluginId: BUNDLED_COMPUTER_USE_PLUGIN_ID,
      lookupSource: "availablePlugins",
      state: "shown",
      reason: "bundled-plugin-available",
    });
  } else if (availability.isLoading || availability.isFetching) {
    rowDecisions.push({
      provider: "bundled",
      pluginId: BUNDLED_COMPUTER_USE_PLUGIN_ID,
      lookupSource: "availablePlugins",
      state: "disabled",
      reason: "computer-use-availability-loading",
    });
  } else {
    rowDecisions.push({
      provider: "bundled",
      pluginId: BUNDLED_COMPUTER_USE_PLUGIN_ID,
      lookupSource: "availablePlugins",
      state: "hidden",
      reason: "computer-use-availability-false",
    });
  }

  if (takeover) {
    rowDecisions.push({
      provider: "x11",
      pluginId: X11_COMPUTER_USE_PLUGIN_ID,
      lookupSource: x11LookupSource,
      state: x11Plugin == null ? "unavailable" : "shown",
      reason: x11Plugin == null ? "x11-provider-missing" : "x11-takeover-selected",
    });
  } else if (x11Plugin != null) {
    rowDecisions.push({
      provider: "x11",
      pluginId: X11_COMPUTER_USE_PLUGIN_ID,
      lookupSource: x11LookupSource,
      state: "available",
      reason: "x11-provider-detected",
    });
  } else {
    rowDecisions.push({
      provider: "x11",
      pluginId: X11_COMPUTER_USE_PLUGIN_ID,
      lookupSource: "none",
      state: "absent",
      reason: "x11-provider-not-configured",
    });
  }

  return {
    markerVersion: COMPUTER_USE_PROVIDER_TAKEOVER_MARKER,
    mode,
    provider,
    selectedProvider: takeover ? "x11" : "bundled",
    bundledPlugin: bundledPlugin == null ? null : sanitizedPlugin(bundledPlugin),
    x11Plugin: x11Plugin == null ? null : sanitizedPlugin(x11Plugin),
    rowDecisions,
  };
}

function buildComputerUseProviderDiagnostics({
  markerVersion = COMPUTER_USE_PROVIDER_TAKEOVER_MARKER,
  asset = null,
  availablePlugins = [],
  installedPlugins = [],
  computerUseAvailability = {},
  gateFacts = {},
  mode = "bundled",
  provider = "bundled",
} = {}) {
  const resolved = resolveComputerUseProviderRows({
    availablePlugins,
    installedPlugins,
    computerUseAvailability,
    mode,
    provider,
  });
  return {
    markerVersion,
    asset,
    provider,
    mode,
    availablePlugins: sanitizedPluginList(availablePlugins),
    installedPlugins: sanitizedPluginList(installedPlugins),
    computerUseAvailability: sanitizedAvailability(computerUseAvailability),
    gateFacts: { ...gateFacts },
    rowDecisions: resolved.rowDecisions,
  };
}

function parseDestructuredParamAliases(paramsText) {
  const aliases = Object.create(null);
  for (const rawPart of paramsText.split(",")) {
    const part = rawPart.trim();
    const match = part.match(/^([A-Za-z_$][\w$]*)(?::([A-Za-z_$][\w$]*))?$/);
    if (match != null) {
      aliases[match[1]] = match[2] ?? match[1];
    }
  }
  return aliases;
}

function buildComputerUseGate({ nameExpr, availabilityProp, featuresVar, platformVar, migrateVar }) {
  return `{installWhenMissing:!0,name:${nameExpr},${availabilityProp}:({features:${featuresVar},platform:${platformVar}})=>(${platformVar}===\`darwin\`||${platformVar}===\`linux\`)&&${featuresVar}.computerUse,migrate:${migrateVar}}`;
}

function hasComputerUseLiteral(source) {
  return /(?:`computer-use`|"computer-use"|'computer-use')/.test(source);
}

function isComputerUseNameExpr(nameExpr, computerUseNameVar) {
  return /^(?:`computer-use`|"computer-use"|'computer-use')$/.test(nameExpr) ||
    nameExpr === computerUseNameVar ||
    /^[A-Za-z_$][\w$]*\.[A-Za-z_$][\w$]*$/.test(nameExpr);
}

function applyLinuxComputerUsePluginGatePatch(currentSource) {
  if (!hasComputerUseLiteral(currentSource)) {
    console.warn(
      "WARN: Could not find Computer Use plugin gate literal — skipping Linux Computer Use plugin gate patch",
    );
    return currentSource;
  }

  const computerUseNameVar = currentSource.match(/([A-Za-z_$][\w$]*)=(?:`computer-use`|"computer-use"|'computer-use')/)?.[1] ?? null;
  const nameExpressionPattern = String.raw`(?:[A-Za-z_$][\w$]*(?:\.[A-Za-z_$][\w$]*)?|` +
    String.raw`\`computer-use\`|"computer-use"|'computer-use')`;
  const gateRegex =
    new RegExp(String.raw`\{(installWhenMissing:!0,)?name:(${nameExpressionPattern}),(isEnabled|isAvailable):\(\{([^}]*)\}\)=>([^{}]*?\.computerUse),migrate:([A-Za-z_$][\w$]*)\}`, "g");
  let sawEnabledGate = false;
  let sawUnpatchableGate = false;
  let patchedGateCount = 0;
  const patchedSource = currentSource.replace(
    gateRegex,
    (gateSource, installWhenMissing, nameExpr, availabilityProp, paramsText, expression, migrateVar) => {
      if (!isComputerUseNameExpr(nameExpr, computerUseNameVar)) {
        return gateSource;
      }

      const aliases = parseDestructuredParamAliases(paramsText);
      const featuresVar = aliases.features;
      const platformVar = aliases.platform;
      if (featuresVar == null || platformVar == null) {
        sawUnpatchableGate = true;
        return gateSource;
      }

      const darwinOnlyExpression = `${platformVar}===\`darwin\`&&${featuresVar}.computerUse`;
      const linuxExpression = `(${platformVar}===\`darwin\`||${platformVar}===\`linux\`)&&${featuresVar}.computerUse`;
      if (installWhenMissing != null && expression === linuxExpression) {
        sawEnabledGate = true;
        return gateSource;
      }
      if (expression === darwinOnlyExpression || expression === linuxExpression) {
        patchedGateCount += 1;
        return buildComputerUseGate({ nameExpr, availabilityProp, featuresVar, platformVar, migrateVar });
      }
      sawUnpatchableGate = true;
      return gateSource;
    },
  );

  if (patchedGateCount > 0) {
    return patchedSource;
  }

  if (sawEnabledGate && !sawUnpatchableGate) {
    return currentSource;
  }

  if (hasComputerUseLiteral(currentSource) && currentSource.includes("computerUse")) {
    throw new Error("Required Linux Computer Use plugin gate patch failed: could not enable bundled Computer Use on Linux");
  }

  return currentSource;
}

function applyLinuxComputerUseFeaturePatch(currentSource) {
  const patchedFeaturePattern =
    /function [A-Za-z_$][\w$]*\([A-Za-z_$][\w$]*,\{env:[A-Za-z_$][\w$]*=process\.env,platform:[A-Za-z_$][\w$]*=process\.platform\}=\{\}\)\{return [A-Za-z_$][\w$]*===`linux`\?\{\.\.\.[A-Za-z_$][\w$]*,computerUse:!0,computerUseNodeRepl:!0\}:/;
  const currentPatchedFeaturePattern =
    /let [A-Za-z_$][\w$]*=[A-Za-z_$][\w$]*===`linux`\?\{\.\.\.[A-Za-z_$][\w$]*,computerUse:!0,computerUseNodeRepl:!0\}:[A-Za-z_$][\w$]*===`win32`&&[A-Za-z_$][\w$]*\.CODEX_ELECTRON_ENABLE_WINDOWS_COMPUTER_USE===`1`\?\{\.\.\.[A-Za-z_$][\w$]*,computerUse:!0,computerUseNodeRepl:!0\}:[A-Za-z_$][\w$]*,/;
  const windowsOnlyFeaturePattern =
    /function ([A-Za-z_$][\w$]*)\(([A-Za-z_$][\w$]*),\{env:([A-Za-z_$][\w$]*)=process\.env,platform:([A-Za-z_$][\w$]*)=process\.platform\}=\{\}\)\{return \4!==`win32`\|\|\3\.CODEX_ELECTRON_ENABLE_WINDOWS_COMPUTER_USE!==`1`\?\2:\{\.\.\.\2,computerUse:!0,computerUseNodeRepl:!0\}\}/g;
  const currentWindowsOnlyFeaturePattern =
    /let ([A-Za-z_$][\w$]*)=([A-Za-z_$][\w$]*)===`win32`&&([A-Za-z_$][\w$]*)\.CODEX_ELECTRON_ENABLE_WINDOWS_COMPUTER_USE===`1`\?\{\.\.\.([A-Za-z_$][\w$]*),computerUse:!0,computerUseNodeRepl:!0\}:\4,/g;

  let changed = false;
  let patchedSource = currentSource.replace(
    windowsOnlyFeaturePattern,
    (_, fnName, featuresVar, envVar, platformVar) => {
      changed = true;
      return `function ${fnName}(${featuresVar},{env:${envVar}=process.env,platform:${platformVar}=process.platform}={}){return ${platformVar}===\`linux\`?{...${featuresVar},computerUse:!0,computerUseNodeRepl:!0}:${platformVar}!==\`win32\`||${envVar}.CODEX_ELECTRON_ENABLE_WINDOWS_COMPUTER_USE!==\`1\`?${featuresVar}:{...${featuresVar},computerUse:!0,computerUseNodeRepl:!0}}`;
    },
  );
  patchedSource = patchedSource.replace(
    currentWindowsOnlyFeaturePattern,
    (_, gateVar, platformVar, envVar, featuresVar) => {
      changed = true;
      return `let ${gateVar}=${platformVar}===\`linux\`?{...${featuresVar},computerUse:!0,computerUseNodeRepl:!0}:${platformVar}===\`win32\`&&${envVar}.CODEX_ELECTRON_ENABLE_WINDOWS_COMPUTER_USE===\`1\`?{...${featuresVar},computerUse:!0,computerUseNodeRepl:!0}:${featuresVar},`;
    },
  );

  if (changed) {
    return patchedSource;
  }

  if (patchedFeaturePattern.test(currentSource) || currentPatchedFeaturePattern.test(currentSource)) {
    return currentSource;
  }

  if (currentSource.includes("CODEX_ELECTRON_ENABLE_WINDOWS_COMPUTER_USE")) {
    console.warn(
      "WARN: Could not find Computer Use desktop feature gate — skipping Linux Computer Use feature patch",
    );
  }

  return currentSource;
}

function applyLinuxComputerUseRendererAvailabilityPatch(currentSource) {
  let patchedSource = currentSource;
  let platformPredicateChanged = false;
  let availabilityChanged = false;
  let availabilityGateFound = false;

  const computerUseFeatureNeedle = "featureName:`computer_use`";
  const hasComputerUseAvailabilityGate = () =>
    currentSource.includes(computerUseFeatureNeedle) &&
    (currentSource.includes("isComputerUseAvailable") || currentSource.includes("1506311413"));
  const availabilityAlreadyPatched = () =>
    /featureName:`computer_use`[\s\S]{0,1200}?let ([A-Za-z_$][\w$]*)=[A-Za-z_$][\w$]*&&[A-Za-z_$][\w$]*&&\([A-Za-z_$][\w$]*===`linux`\|\|[A-Za-z_$][\w$]*&&\([A-Za-z_$][\w$]*\|\|[A-Za-z_$][\w$]*\)\),[A-Za-z_$][\w$]*=\1&&![A-Za-z_$][\w$]*&&\([A-Za-z_$][\w$]*===`linux`\|\|[A-Za-z_$][\w$]*\.enabled\)&&![A-Za-z_$][\w$]*\.isLoading/.test(patchedSource) ||
    /featureName:`computer_use`[\s\S]{0,1800}?isComputerUseFeatureEnabled:([A-Za-z_$][\w$]*)===`linux`\|\|[A-Za-z_$][\w$]*\.enabled,isComputerUseFeatureLoading:\1!==`linux`&&[A-Za-z_$][\w$]*\.isLoading,isComputerUseGateEnabled:\1===`linux`\|\|[A-Za-z_$][\w$]*,isHostCompatiblePlatform:\1===`linux`\|\|[A-Za-z_$][\w$]*\(\1\),isHostLocal:/.test(patchedSource) ||
    patchedSource.includes(availabilityPatch) ||
    patchedSource.includes(currentAvailabilityPatch);

  const findPlatformVarForAvailabilityGate = (offset, platformLoadingVar) => {
    const lookback = patchedSource.slice(Math.max(0, offset - 900), offset);
    const loadingFirst = new RegExp(String.raw`\{isLoading:${platformLoadingVar},platform:([A-Za-z_$][\w$]*)\}=`);
    const platformFirst = new RegExp(String.raw`\{platform:([A-Za-z_$][\w$]*),isLoading:${platformLoadingVar}\}=`);
    return lookback.match(loadingFirst)?.[1] ?? lookback.match(platformFirst)?.[1] ?? null;
  };

  const platformPredicateNeedle = "function hae(e){return e===`macOS`||e===`windows`}";
  const platformPredicatePatch =
    "function hae(e){return e===`macOS`||e===`windows`||e===`linux`}";
  const currentPlatformPredicateNeedle =
    /function ([A-Za-z_$][\w$]*)\(([A-Za-z_$][\w$]*)\)\{return \2===`macOS`\|\|\2===`windows`\}/g;
  const currentPlatformPredicatePatch = (_, fnName, platformVar) => {
    platformPredicateChanged = true;
    return `function ${fnName}(${platformVar}){return ${platformVar}===\`macOS\`||${platformVar}===\`windows\`||${platformVar}===\`linux\`}`;
  };
  if (patchedSource.includes(platformPredicateNeedle)) {
    patchedSource = patchedSource.split(platformPredicateNeedle).join(platformPredicatePatch);
    platformPredicateChanged = true;
  }
  patchedSource = patchedSource.replace(currentPlatformPredicateNeedle, currentPlatformPredicatePatch);

  const availabilityNeedle =
    "let m=a&&i&&s===`electron`&&u&&(c||p),h=m&&!c&&f.enabled&&!f.isLoading,g=m&&f.isLoading,_=m&&(c||f.isLoading),v;";
  const availabilityHostLocalLinuxPatch =
    "let m=a&&i&&s===`electron`&&(l===`linux`||u&&(c||p)),h=m&&!c&&(l===`linux`||f.enabled)&&!f.isLoading,g=m&&l!==`linux`&&f.isLoading,_=m&&(c||l!==`linux`&&f.isLoading),v;";
  const availabilityPatch =
    "let m=a&&(i||l===`linux`)&&s===`electron`&&(l===`linux`||u&&(c||p)),h=m&&!c&&(l===`linux`||f.enabled)&&!f.isLoading,g=m&&l!==`linux`&&f.isLoading,_=m&&(c||l!==`linux`&&f.isLoading),v;";
  if (patchedSource.includes(availabilityHostLocalLinuxPatch)) {
    patchedSource = patchedSource.split(availabilityHostLocalLinuxPatch).join(availabilityPatch);
    availabilityChanged = true;
  }
  if (patchedSource.includes(availabilityNeedle)) {
    patchedSource = patchedSource.split(availabilityNeedle).join(availabilityPatch);
    availabilityChanged = true;
  }

  const currentAvailabilityNeedle =
    "let _=a&&i&&l&&(o||m),v=_&&!o&&p.enabled&&!p.isLoading,y=_&&p.isLoading,b=_&&(o||p.isLoading),x;";
  const currentAvailabilityPatch =
    "let _=a&&i&&(c===`linux`||l&&(o||m)),v=_&&!o&&(c===`linux`||p.enabled)&&!p.isLoading,y=_&&c!==`linux`&&p.isLoading,b=_&&(o||c!==`linux`&&p.isLoading),x;";
  if (patchedSource.includes(currentAvailabilityNeedle)) {
    patchedSource = patchedSource.split(currentAvailabilityNeedle).join(currentAvailabilityPatch);
    availabilityChanged = true;
  }

  const currentHookAvailabilityPattern =
    /let ([A-Za-z_$][\w$]*)=([A-Za-z_$][\w$]*)&&([A-Za-z_$][\w$]*)&&([A-Za-z_$][\w$]*)&&\(([A-Za-z_$][\w$]*)\|\|([A-Za-z_$][\w$]*)\),([A-Za-z_$][\w$]*)=\1&&!\5&&([A-Za-z_$][\w$]*)\.enabled&&!\8\.isLoading,([A-Za-z_$][\w$]*)=\1&&\8\.isLoading,([A-Za-z_$][\w$]*)=\1&&\(\5\|\|\8\.isLoading\),([A-Za-z_$][\w$]*);/g;
  patchedSource = patchedSource.replace(
    currentHookAvailabilityPattern,
    (
      match,
      availabilityVar,
      enabledVar,
      isHostLocalVar,
      rolloutVar,
      platformLoadingVar,
      supportedPlatformVar,
      availableVar,
      featureQueryVar,
      fetchingVar,
      loadingVar,
      resultVar,
      offset,
    ) => {
      const contextStart = Math.max(0, offset - 900);
      const context = patchedSource.slice(contextStart, offset + match.length);
      if (!context.includes(computerUseFeatureNeedle)) {
        return match;
      }
      availabilityGateFound = true;
      const platformVar = findPlatformVarForAvailabilityGate(offset, platformLoadingVar);
      if (platformVar == null) {
        return match;
      }
      availabilityChanged = true;
      return `let ${availabilityVar}=${enabledVar}&&${isHostLocalVar}&&(${platformVar}===\`linux\`||${rolloutVar}&&(${platformLoadingVar}||${supportedPlatformVar})),${availableVar}=${availabilityVar}&&!${platformLoadingVar}&&(${platformVar}===\`linux\`||${featureQueryVar}.enabled)&&!${featureQueryVar}.isLoading,${fetchingVar}=${availabilityVar}&&${platformVar}!==\`linux\`&&${featureQueryVar}.isLoading,${loadingVar}=${availabilityVar}&&(${platformLoadingVar}||${platformVar}!==\`linux\`&&${featureQueryVar}.isLoading),${resultVar};`;
    },
  );

  const currentObjectAvailabilityPattern =
    /([A-Za-z_$][\w$]*)=([A-Za-z_$][\w$]*)\(\{enabled:([A-Za-z_$][\w$]*),isComputerUseFeatureEnabled:([A-Za-z_$][\w$]*)\.enabled,isComputerUseFeatureLoading:\4\.isLoading,isComputerUseGateEnabled:([A-Za-z_$][\w$]*),isHostCompatiblePlatform:([A-Za-z_$][\w$]*)\(([A-Za-z_$][\w$]*)\),isHostLocal:([A-Za-z_$][\w$]*),isPlatformLoading:([A-Za-z_$][\w$]*),windowType:`electron`\}\)/g;
  patchedSource = patchedSource.replace(
    currentObjectAvailabilityPattern,
    (
      match,
      resultVar,
      helperVar,
      enabledVar,
      featureQueryVar,
      rolloutVar,
      platformPredicateVar,
      platformVar,
      isHostLocalVar,
      platformLoadingVar,
      offset,
    ) => {
      const contextStart = Math.max(0, offset - 900);
      const context = patchedSource.slice(contextStart, offset + match.length);
      if (!context.includes(computerUseFeatureNeedle)) {
        return match;
      }
      availabilityGateFound = true;
      availabilityChanged = true;
      return `${resultVar}=${helperVar}({enabled:${enabledVar},isComputerUseFeatureEnabled:${platformVar}===\`linux\`||${featureQueryVar}.enabled,isComputerUseFeatureLoading:${platformVar}!==\`linux\`&&${featureQueryVar}.isLoading,isComputerUseGateEnabled:${platformVar}===\`linux\`||${rolloutVar},isHostCompatiblePlatform:${platformVar}===\`linux\`||${platformPredicateVar}(${platformVar}),isHostLocal:${isHostLocalVar},isPlatformLoading:${platformLoadingVar},windowType:\`electron\`})`;
    },
  );

  if (availabilityChanged || availabilityAlreadyPatched()) {
    return patchedSource;
  }

  if (hasComputerUseAvailabilityGate() || availabilityGateFound) {
    console.warn(
      "WARN: Could not find Computer Use renderer availability gate — skipping Linux Computer Use UI availability patch",
    );
    return currentSource;
  }

  return platformPredicateChanged ? patchedSource : currentSource;
}

function applyX11ComputerUseSettingsRowPatch(currentSource) {
  const marker = "codexLinuxComputerUseTakeoverProvider";
  const staleMarker = "codexLinuxX11Plugin";
  const lookupNeedle = "let m=p,h;";
  const staleAvailableOnlyLookup = "let m=p,codexLinuxX11Plugin=X(d.availablePlugins,`codex-computer-use-x11`,f),h;";
  const staleInstalledFirstLookup = "let m=p,codexLinuxX11Plugin=X(d.installedPlugins??[],`codex-computer-use-x11`,f)??X(d.availablePlugins,`codex-computer-use-x11`,f),h;";
  const x11ProviderLookup = "((d.installedPlugins??[]).find(e=>(e.plugin?.name??e.name)===`codex-computer-use-x11`||(e.plugin?.id??e.id??``).split(`@`)[0]===`codex-computer-use-x11`||e.marketplaceName===`codex-computer-use-x11`)??d.availablePlugins.find(e=>(e.plugin?.name??e.name)===`codex-computer-use-x11`||(e.plugin?.id??e.id??``).split(`@`)[0]===`codex-computer-use-x11`||e.marketplaceName===`codex-computer-use-x11`))";
  const unsafeMarketplaceFilteredLookupPatch = `let m=p,codexLinuxComputerUseTakeoverProvider=X(d.installedPlugins??[],\`codex-computer-use-x11\`,f)??X(d.availablePlugins,\`codex-computer-use-x11\`,f),h;/*${COMPUTER_USE_PROVIDER_TAKEOVER_MARKER}*/`;
  const lookupPatch = `let m=p,codexLinuxComputerUseTakeoverProvider=${x11ProviderLookup},h;/*${COMPUTER_USE_PROVIDER_TAKEOVER_MARKER}*/`;
  const rowNeedle = "w.push(r)}if(g!=null){";
  const bundledRowConditionNeedle = "if(r.available&&m!=null){";
  const simpleTakeoverPrefix = "if(codexLinuxComputerUseTakeoverProvider!=null&&!w.some(e=>e.plugin===codexLinuxComputerUseTakeoverProvider)){w.push({plugin:codexLinuxComputerUseTakeoverProvider,title:`X11 Computer Use`,description:`Standalone X11/EWMH desktop control tools`,codexLinuxComputerUseProviderShim:!0})}if(false&&r.available&&m!=null){";
  const staleRowPatch = "w.push(r)}if(codexLinuxX11Plugin!=null)w.push({plugin:codexLinuxX11Plugin,title:`X11 Computer Use`,description:`Standalone X11/EWMH desktop control tools`});if(g!=null){";
  const rowPatch = "w.push(r)}if(g!=null){";
  const simpleRowPatch = simpleTakeoverPrefix;
  const memoizedBundledConditionNeedle = ",T=e,r.available&&m!=null){";
  const memoizedBundledConditionPatch = ",T=e,false&&r.available&&m!=null){";
  const memoizedTailNeedle = "}else S=t[21],T=t[22];let E,D;";
  const staleMemoizedTailPatch = "}else S=t[21],T=t[22];codexLinuxX11Plugin!=null&&!S.some(e=>e.plugin===codexLinuxX11Plugin)&&S.push({plugin:codexLinuxX11Plugin,title:`X11 Computer Use`,description:`Standalone X11/EWMH desktop control tools`});let E,D;";
  const unsafeMemoizedUnavailableItemsPatch = "}else S=t[21],T=t[22];codexLinuxComputerUseTakeoverProvider!=null&&!S.some(e=>e.plugin===codexLinuxComputerUseTakeoverProvider)&&S.push({plugin:codexLinuxComputerUseTakeoverProvider,title:`X11 Computer Use`,description:`Standalone X11/EWMH desktop control tools`,codexLinuxComputerUseProviderShim:!0});codexLinuxComputerUseTakeoverProvider==null&&!S.some(e=>e.id===`codex-computer-use-x11-unavailable`)&&S.push({id:`codex-computer-use-x11-unavailable`,title:`X11 Computer Use`,description:`X11 provider takeover is enabled but codex-computer-use-x11 is not installed or available`,codexLinuxComputerUseProviderUnavailable:!0});let E,D;";
  const memoizedTailPatch = "}else S=t[21],T=t[22];S=S??[];T=T??[];codexLinuxComputerUseTakeoverProvider!=null&&!S.some(e=>e.plugin===codexLinuxComputerUseTakeoverProvider)&&(S=[...S,{plugin:codexLinuxComputerUseTakeoverProvider,title:`X11 Computer Use`,description:`Standalone X11/EWMH desktop control tools`,codexLinuxComputerUseProviderShim:!0}]);codexLinuxComputerUseTakeoverProvider==null&&!T.some(e=>e.id===`codex-computer-use-x11-unavailable`)&&(T=[...T,{id:`codex-computer-use-x11-unavailable`,title:`X11 Computer Use`,description:`X11 provider takeover is enabled but codex-computer-use-x11 is not installed or available`,codexLinuxComputerUseProviderUnavailable:!0}]);let E,D;";

  if (currentSource.includes(marker)) {
    const upgradedSource = currentSource
      .replace(unsafeMarketplaceFilteredLookupPatch, lookupPatch)
      .replace(unsafeMemoizedUnavailableItemsPatch, memoizedTailPatch)
      .replace(
        "if(codexLinuxComputerUseTakeoverProvider!=null){w.push({plugin:codexLinuxComputerUseTakeoverProvider,title:`X11 Computer Use`,description:`Standalone X11/EWMH desktop control tools`,codexLinuxComputerUseProviderShim:!0})}else{w.push({id:`codex-computer-use-x11-unavailable`,title:`X11 Computer Use`,description:`X11 provider takeover is enabled but codex-computer-use-x11 is not installed or available`,codexLinuxComputerUseProviderUnavailable:!0})}if(false&&r.available&&m!=null){",
        simpleTakeoverPrefix,
      );
    return upgradedSource;
  }

  let migratedSource = currentSource;
  if (migratedSource.includes(staleAvailableOnlyLookup) || migratedSource.includes(staleInstalledFirstLookup) || migratedSource.includes(staleMarker)) {
    migratedSource = migratedSource
      .replace(staleAvailableOnlyLookup, lookupPatch)
      .replace(staleInstalledFirstLookup, lookupPatch)
      .replace(staleRowPatch, rowPatch)
      .replace(staleMemoizedTailPatch, memoizedTailPatch)
      .replace(memoizedBundledConditionNeedle, memoizedBundledConditionPatch);
    if (migratedSource !== currentSource) {
      return migratedSource;
    }
  }

  if (currentSource.includes(lookupNeedle) && currentSource.includes(bundledRowConditionNeedle)) {
    return currentSource.replace(lookupNeedle, lookupPatch).replace(bundledRowConditionNeedle, simpleRowPatch);
  }
  if (currentSource.includes(lookupNeedle) && currentSource.includes(memoizedTailNeedle)) {
    return currentSource
      .replace(lookupNeedle, lookupPatch)
      .replace(memoizedBundledConditionNeedle, memoizedBundledConditionPatch)
      .replace(memoizedTailNeedle, memoizedTailPatch);
  }

  if (
    currentSource.includes("settings.computerUse.anyApp") ||
    (currentSource.includes("computer-use") && currentSource.includes("settings.computerUse"))
  ) {
    console.warn(
      "WARN: Could not find X11 Computer Use settings row insertion point — skipping settings row patch",
    );
  }

  return currentSource;
}

function applyLinuxComputerUseInstallFlowPatch(currentSource) {
  const availabilityNeedle =
    "ne=f({featureName:`computer_use`,hostId:t}),re=!ne.isLoading&&ne.enabled,";
  const availabilityPatch =
    "ne=f({featureName:`computer_use`,hostId:t}),re=!ne.isLoading&&ne.enabled||navigator.userAgent.includes(`Linux`),";
  const currentAvailabilityPattern =
    /([A-Za-z_$][\w$]*)=([A-Za-z_$][\w$]*)\(\{featureName:`computer_use`,hostId:([^}]+)\}\),([^;]{0,300}?)([A-Za-z_$][\w$]*)=!\1\.isLoading&&\1\.enabled,/g;

  let changed = false;
  let patchedSource = currentSource;

  if (patchedSource.includes(availabilityNeedle)) {
    patchedSource = patchedSource.split(availabilityNeedle).join(availabilityPatch);
    changed = true;
  }

  patchedSource = patchedSource.replace(
    currentAvailabilityPattern,
    (_, queryVar, queryFn, hostExpr, between, availableVar) => {
      changed = true;
      return `${queryVar}=${queryFn}({featureName:\`computer_use\`,hostId:${hostExpr}}),${between}${availableVar}=!${queryVar}.isLoading&&${queryVar}.enabled||navigator.userAgent.includes(\`Linux\`),`;
    },
  );

  if (changed) {
    return patchedSource;
  }

  if (/=[^=]+\.isLoading&&[^=]+\.enabled\|\|navigator\.userAgent\.includes\(`Linux`\),/.test(currentSource)) {
    return currentSource;
  }

  if (currentSource.includes("featureName:`computer_use`")) {
    console.warn(
      "WARN: Could not find Computer Use install flow gate — skipping Linux Computer Use install flow patch",
    );
  }

  return currentSource;
}

module.exports = {
  COMPUTER_USE_UI_ENV_VAR,
  COMPUTER_USE_UI_SETTINGS_KEY,
  COMPUTER_USE_PROVIDER_TAKEOVER_MARKER,
  buildComputerUseProviderDiagnostics,
  applyLinuxComputerUseFeaturePatch,
  applyLinuxComputerUseInstallFlowPatch,
  applyLinuxComputerUsePluginGatePatch,
  applyLinuxComputerUseRendererAvailabilityPatch,
  applyX11ComputerUseSettingsRowPatch,
  isComputerUseUiEnabled,
  resolveComputerUseProviderRows,
};
