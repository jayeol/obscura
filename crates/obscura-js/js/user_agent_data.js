"use strict";

// Keep navigator.userAgentData aligned with the active User-Agent and platform
// globals injected by ObscuraJsRuntime before __obscura_init() runs. This lives
// outside bootstrap.js so the compatibility repair stays small and auditable.
(function installUserAgentDataPatch() {
  function chromeVersionParts() {
    var ua = String(globalThis.navigator && globalThis.navigator.userAgent || "");
    var match = /Chrome\/(\d+(?:\.\d+)*)/.exec(ua);
    var full = match ? match[1] : "145.0.0.0";
    var major = full.split(".")[0] || "145";
    return { full: full, major: major };
  }

  function brands(major) {
    return [
      { brand: "Google Chrome", version: major },
      { brand: "Chromium", version: major },
      { brand: "Not;A=Brand", version: "24" },
    ];
  }

  function fullVersionList(full, major) {
    return [
      { brand: "Google Chrome", version: full },
      { brand: "Chromium", version: full },
      { brand: "Not;A=Brand", version: "24.0.0.0" },
    ];
  }

  function platform() {
    return globalThis.__obscura_ua_platform || "Windows";
  }

  function platformVersion() {
    return globalThis.__obscura_ua_platform_version || "15.0.0";
  }

  function mobile() {
    return /Mobile|Android|iPhone|iPad/i.test(String(globalThis.navigator && globalThis.navigator.userAgent || ""));
  }

  function buildUserAgentData() {
    var v = chromeVersionParts();
    return {
      get brands() { return brands(v.major); },
      get mobile() { return mobile(); },
      get platform() { return platform(); },
      getHighEntropyValues: function getHighEntropyValues(_hints) {
        var current = chromeVersionParts();
        return Promise.resolve({
          architecture: "x86",
          bitness: "64",
          brands: brands(current.major),
          fullVersionList: fullVersionList(current.full, current.major),
          mobile: mobile(),
          model: "",
          platform: platform(),
          platformVersion: platformVersion(),
          uaFullVersion: current.full,
        });
      },
      toJSON: function toJSON() {
        return { brands: brands(chromeVersionParts().major), mobile: mobile(), platform: platform() };
      },
    };
  }

  var getter = function userAgentDataGetter() { return buildUserAgentData(); };
  try { if (typeof _markNative === "function") _markNative(getter); } catch (_) {}
  Object.defineProperty(globalThis.navigator, "userAgentData", {
    get: getter,
    enumerable: true,
    configurable: true,
  });
})();
