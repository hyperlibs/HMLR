/**
 * HMLR DevTools Background Service Worker
 * Multiplexes communication between inspected tabs and DevTools panels.
 */

const connections = new Map();

chrome.runtime.onConnect.addListener(port => {
  const extensionListener = (message, sender) => {
    if (message.name === 'init' && message.tabId) {
      connections.set(message.tabId, port);
      return;
    }
  };

  port.onMessage.addListener(extensionListener);

  port.onDisconnect.addListener(() => {
    for (const [tabId, p] of connections.entries()) {
      if (p === port) {
        connections.delete(tabId);
        break;
      }
    }
  });
});

// Relay messages from content script to devtools panel
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (sender.tab && sender.tab.id) {
    const tabId = sender.tab.id;
    if (connections.has(tabId)) {
      connections.get(tabId).postMessage(request);
    }
  }
  sendResponse({ status: 'ok' });
  return true;
});
