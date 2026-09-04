/**
 * HMLR DevTools Background Service Worker
 * Targeted Tab Multiplexer: Activates telemetry ONLY on tabs where DevTools is open.
 */

const activeTabConnections = new Map();

chrome.runtime.onConnect.addListener(port => {
  let boundTabId = null;

  const extensionListener = (message) => {
    if (message.name === 'init' && message.tabId) {
      boundTabId = message.tabId;
      activeTabConnections.set(boundTabId, port);
      // Explicitly notify the specific content script that DevTools is inspecting it
      chrome.tabs.sendMessage(boundTabId, { type: 'HMLR_ACTIVATE_TAB', tabId: boundTabId }).catch(() => {});
    }
  };

  port.onMessage.addListener(extensionListener);

  port.onDisconnect.addListener(() => {
    if (boundTabId) {
      activeTabConnections.delete(boundTabId);
      // Notify the specific tab content script to deactivate telemetry listeners
      chrome.tabs.sendMessage(boundTabId, { type: 'HMLR_DEACTIVATE_TAB', tabId: boundTabId }).catch(() => {});
    }
  });
});

// Relay messages from content script to the corresponding DevTools panel port
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (sender.tab && sender.tab.id) {
    const tabId = sender.tab.id;
    if (activeTabConnections.has(tabId)) {
      activeTabConnections.get(tabId).postMessage(request);
    }
  }
  sendResponse({ status: 'relayed' });
  return true;
});
