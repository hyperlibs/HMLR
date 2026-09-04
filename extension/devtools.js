/**
 * HMLR DevTools Entrypoint
 * Registers the 4 inspector panels in Chromium / Firefox DevTools.
 */

chrome.devtools.panels.create(
  '⚡ HMLR Signals',
  'icons/icon16.png',
  'panels/signals.html',
  function (panel) {
    // Panel initialized
  }
);

chrome.devtools.panels.create(
  '🚀 Hypermedia Audit',
  'icons/icon16.png',
  'panels/auditor.html',
  function (panel) {
    // Panel initialized
  }
);

chrome.devtools.panels.create(
  '📐 3D Spatial HUD',
  'icons/icon16.png',
  'panels/spatial.html',
  function (panel) {
    // Panel initialized
  }
);

chrome.devtools.panels.create(
  '🤖 Agentic .mx',
  'icons/icon16.png',
  'panels/telemetry.html',
  function (panel) {
    // Panel initialized
  }
);
