// HMLR Agentic Telemetry Panel Controller
const port = chrome.runtime?.connect?.({ name: 'hmlr-telemetry-panel' });
const streamEl = document.getElementById('mx-stream');

port?.onMessage?.addListener((msg) => {
  if (msg.type === 'HMLR_DIAG_EVENT' && msg.detail) {
    const { code, severity, message, fix } = msg.detail;
    const diagLine = `\n@diag ${code} ${severity} "${message}" fix="${fix || 'Auto-heal'}"`;
    streamEl.textContent += diagLine;
    streamEl.scrollTop = streamEl.scrollHeight;
  }
});
