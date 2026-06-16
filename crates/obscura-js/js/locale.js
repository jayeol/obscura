"use strict";

// Keep navigator language surfaces aligned with runtime-injected locale globals.
// This stays outside bootstrap.js so the locale repair remains small and reviewable.
(function installLocalePatch() {
  function normalizeLocale(value) {
    var s = String(value || "en-US").trim();
    return s || "en-US";
  }

  function currentLocale() {
    return normalizeLocale(globalThis.__obscura_locale);
  }

  function currentLanguages() {
    var locale = currentLocale();
    var primary = locale.split("-")[0];
    if (primary && primary !== locale) return [locale, primary];
    return [locale];
  }

  var languageGetter = function languageGetter() { return currentLocale(); };
  var languagesGetter = function languagesGetter() { return currentLanguages(); };
  try {
    if (typeof _markNative === "function") {
      _markNative(languageGetter);
      _markNative(languagesGetter);
    }
  } catch (_) {}

  Object.defineProperty(globalThis.navigator, "language", {
    get: languageGetter,
    enumerable: true,
    configurable: true,
  });
  Object.defineProperty(globalThis.navigator, "languages", {
    get: languagesGetter,
    enumerable: true,
    configurable: true,
  });
})();
