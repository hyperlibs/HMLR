// HMLR Agentic Telemetry Panel Controller
const streamEl = document.getElementById('mx-stream');
const btnClear = document.getElementById('btn-clear');
const btnCopy = document.getElementById('btn-copy');

btnClear?.addEventListener('click', () => {
  if (streamEl) streamEl.textContent = '';
});

btnCopy?.addEventListener('click', () => {
  if (streamEl) {
    navigator.clipboard?.writeText(streamEl.textContent || '');
    btnCopy.textContent = 'Copied!';
    setTimeout(() => { btnCopy.textContent = 'Copy .mx Stream'; }, 1500);
  }
});

// Passive message listener
if (typeof chrome !== 'undefined' && chrome.runtime?.onMessage) {
  chrome.runtime.onMessage.addListener((msg) => {
    if (msg.type === 'HMLR_DIAG_EVENT' && msg.detail && streamEl) {
      const { code, severity, message, fix } = msg.detail;
      const diagLine = `\n@diag ${code} ${severity} "${message}" fix="${fix || 'Auto-heal'}"`;
      streamEl.textContent += diagLine;
      streamEl.scrollTop = streamEl.scrollHeight;
    }
  });
}
