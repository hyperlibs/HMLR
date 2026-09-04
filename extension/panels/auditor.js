// HMLR Hypermedia Auditor Panel Controller
const port = chrome.runtime?.connect?.({ name: 'hmlr-auditor-panel' });
const tbody = document.getElementById('audits-body');
const latencyEl = document.getElementById('avg-latency');
const purityEl = document.getElementById('js-purity');

port?.onMessage?.addListener((msg) => {
  if (msg.type === 'HMLR_HYPERMEDIA_SWAP' && msg.detail) {
    const { target, durationMs, payloadSize } = msg.detail;
    const row = document.createElement('tr');
    const isFast = durationMs < 50;
    row.innerHTML = `
      <td>#${target}</td>
      <td>${durationMs.toFixed(1)} ms</td>
      <td>${payloadSize} B</td>
      <td>100% (Pure HTML)</td>
      <td><span class="${isFast ? 'passed' : 'warning'}">${isFast ? 'PASS' : 'WARN (>50ms)'}</span></td>
    `;
    tbody.prepend(row);
  }
});
