/**
 * HMLR Universal Multi-Browser Extension API Adapter
 * Standardizes API calls across Chromium (Chrome, Edge, Brave, Opera), Gecko (Firefox), and WebKit (Safari).
 */

const isFirefox = typeof browser !== 'undefined' && !!browser.runtime;
const isSafari = typeof safari !== 'undefined' || (typeof navigator !== 'undefined' && /^((?!chrome|android).)*safari/i.test(navigator.userAgent));
const webExtApi = typeof browser !== 'undefined' ? browser : (typeof chrome !== 'undefined' ? chrome : {});

export const HMLRBrowser = {
  isFirefox,
  isSafari,
  isChromium: !isFirefox && !isSafari,
  runtime: webExtApi.runtime || {},
  devtools: webExtApi.devtools || {},
  tabs: webExtApi.tabs || {},
  storage: webExtApi.storage?.local || webExtApi.storage || {}
};
