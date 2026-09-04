/**
 * HMLR DevTools Entrypoint
 * Registers the 4 inspector panels and initializes targeted tab inspection.
 */

const tabId = chrome.devtools.inspectedWindow.tabId;

// Establish background port for this specific inspected tab
const bgPort = chrome.runtime.connect({ name: `hmlr-devtools-${tabId}` });
bgPort.postMessage({ name: 'init', tabId });

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
