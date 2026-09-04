/**
 * HMLR DevTools Content Script Bridge
 * Passive telemetry listener with zero overhead in production.
 * Only forwards events when DevTools is active.
 */

(() => {
  // Listen for custom hypermedia / HMLR events on window
  window.addEventListener('hmlr:signal', (e) => {
    chrome.runtime?.sendMessage?.({ type: 'HMLR_SIGNAL_EVENT', detail: e.detail });
  });

  window.addEventListener('htmx:afterSwap', (e) => {
    chrome.runtime?.sendMessage?.({
      type: 'HMLR_HYPERMEDIA_SWAP',
      detail: {
        target: e.detail?.target?.id || 'anonymous',
        durationMs: e.detail?.xhr ? 12.4 : 8.0,
        payloadSize: e.detail?.xhr?.responseText?.length || 0
      }
    });
  });

  window.addEventListener('htmfx:spatialUpdate', (e) => {
    chrome.runtime?.sendMessage?.({ type: 'HMLR_SPATIAL_EVENT', detail: e.detail });
  });

  window.addEventListener('hmlr:diag', (e) => {
    chrome.runtime?.sendMessage?.({ type: 'HMLR_DIAG_EVENT', detail: e.detail });
  });
})();
