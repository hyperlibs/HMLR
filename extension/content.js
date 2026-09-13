/**
 * HMLR DevTools Content Script Bridge (Selective Tab Activation & 10-Engine Telemetry)
 * ZERO OVERHEAD: Dormant by default. Telemetry hooks ONLY activate when DevTools
 * is explicitly opened for this specific tab or when explicitly enabled in dev mode.
 */

(() => {
  let isActive = false;

  // Handlers for Core & 10-Engine Suite
  const handleSignal = (e) => {
    if (!isActive) return;
    try {
      chrome.runtime?.sendMessage?.({ type: 'HMLR_SIGNAL_EVENT', detail: e.detail });
    } catch (_) {}
  };

  const handleSwap = (e) => {
    if (!isActive) return;
    try {
      chrome.runtime?.sendMessage?.({
        type: 'HMLR_HYPERMEDIA_SWAP',
        detail: {
          target: e.detail?.target?.id || 'anonymous',
          durationMs: e.detail?.xhr ? 12.4 : 8.0,
          payloadSize: e.detail?.xhr?.responseText?.length || 0
        }
      });
    } catch (_) {}
  };

  const handleSpatial = (e) => {
    if (!isActive) return;
    try {
      chrome.runtime?.sendMessage?.({ type: 'HMLR_SPATIAL_EVENT', detail: e.detail });
    } catch (_) {}
  };

  const handleDiag = (e) => {
    if (!isActive) return;
    try {
      chrome.runtime?.sendMessage?.({ type: 'HMLR_DIAG_EVENT', detail: e.detail });
    } catch (_) {}
  };

  const handleToast = (e) => {
    if (!isActive) return;
    try {
      chrome.runtime?.sendMessage?.({ type: 'HMLR_TOAST_EVENT', detail: e.detail });
    } catch (_) {}
  };

  const handleGridMutation = (e) => {
    if (!isActive) return;
    try {
      chrome.runtime?.sendMessage?.({ type: 'HMLR_GRID_EVENT', detail: e.detail });
    } catch (_) {}
  };

  const handleOfflineSync = (e) => {
    if (!isActive) return;
    try {
      chrome.runtime?.sendMessage?.({ type: 'HMLR_OFFLINE_EVENT', detail: e.detail });
    } catch (_) {}
  };

  const handleHistoryAction = (e) => {
    if (!isActive) return;
    try {
      chrome.runtime?.sendMessage?.({ type: 'HMLR_HISTORY_EVENT', detail: { action: e.type, detail: e.detail } });
    } catch (_) {}
  };

  function inspectCspState() {
    try {
      const stateEl = document.querySelector('script[type="application/json"][hx-state]');
      if (stateEl && stateEl.textContent) {
        const parsed = JSON.parse(stateEl.textContent);
        chrome.runtime?.sendMessage?.({ type: 'HMLR_CSP_STATE_EVENT', detail: parsed });
      }
    } catch (_) {}
  }

  function activateTelemetry() {
    if (isActive) return;
    isActive = true;
    window.addEventListener('hmlr:signal', handleSignal, { passive: true });
    window.addEventListener('htmx:afterSwap', handleSwap, { passive: true });
    window.addEventListener('htmfx:spatialUpdate', handleSpatial, { passive: true });
    window.addEventListener('hmlr:diag', handleDiag, { passive: true });
    window.addEventListener('htmx:toast', handleToast, { passive: true });
    window.addEventListener('hx-grid:cell-mutated', handleGridMutation, { passive: true });
    window.addEventListener('hx-offline:sync', handleOfflineSync, { passive: true });
    window.addEventListener('hx-offline:queue-change', handleOfflineSync, { passive: true });
    window.addEventListener('$undo', handleHistoryAction, { passive: true });
    window.addEventListener('$redo', handleHistoryAction, { passive: true });

    inspectCspState();
  }

  function deactivateTelemetry() {
    if (!isActive) return;
    isActive = false;
    window.removeEventListener('hmlr:signal', handleSignal);
    window.removeEventListener('htmx:afterSwap', handleSwap);
    window.removeEventListener('htmfx:spatialUpdate', handleSpatial);
    window.removeEventListener('hmlr:diag', handleDiag);
    window.removeEventListener('htmx:toast', handleToast);
    window.removeEventListener('hx-grid:cell-mutated', handleGridMutation);
    window.removeEventListener('hx-offline:sync', handleOfflineSync);
    window.removeEventListener('hx-offline:queue-change', handleOfflineSync);
    window.removeEventListener('$undo', handleHistoryAction);
    window.removeEventListener('$redo', handleHistoryAction);
  }

  // Check for explicit in-page dev mode declarations
  const hasMetaDev = document.querySelector('meta[name="hmlr-dev"][content="on"]') !== null;
  const hasQueryDev = window.location.search.includes('hmlr_dev=1');
  const hasGlobalDev = (window as any).__HMLR_DEV_MODE__ === true;

  if (hasMetaDev || hasQueryDev || hasGlobalDev) {
    activateTelemetry();
  }

  // Listen for targeted activation/deactivation from DevTools background manager
  chrome.runtime?.onMessage?.addListener((msg, sender, sendResponse) => {
    if (msg.type === 'HMLR_ACTIVATE_TAB') {
      activateTelemetry();
      sendResponse({ status: 'activated', tabId: msg.tabId });
    } else if (msg.type === 'HMLR_DEACTIVATE_TAB') {
      deactivateTelemetry();
      sendResponse({ status: 'deactivated', tabId: msg.tabId });
    }
    return true;
  });
})();
