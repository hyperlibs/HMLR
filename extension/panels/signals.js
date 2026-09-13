// HMLR Signals & Scoped State Panel Controller
const filterInput = document.getElementById('filter-input');
const tbody = document.getElementById('signals-body');
const offlineVal = document.getElementById('count-offline');

filterInput?.addEventListener('input', (e) => {
  const query = e.target.value.toLowerCase();
  const rows = tbody.getElementsByTagName('tr');
  for (const row of rows) {
    const text = row.textContent.toLowerCase();
    row.style.display = text.includes(query) ? '' : 'none';
  }
});

// Passive message listener from content bridge
if (typeof chrome !== 'undefined' && chrome.runtime?.onMessage) {
  chrome.runtime.onMessage.addListener((msg) => {
    if (msg.type === 'HMLR_SIGNAL_EVENT' && msg.detail) {
      const { id, value, subscribers, hz, isScoped, scopeVar } = msg.detail;
      let row = document.getElementById(`sig-${id}`);
      if (!row) {
        row = document.createElement('tr');
        row.id = `sig-${id}`;
        tbody.prepend(row);
      }
      const scopeBadge = isScoped
        ? `<span class="badge-scope">hx-for (${scopeVar || 'item'})</span>`
        : '<span class="badge-sub">GLOBAL</span>';

      row.innerHTML = `
        <td><code>${id}</code></td>
        <td><strong>${typeof value === 'object' ? JSON.stringify(value) : value}</strong></td>
        <td><span class="badge-sub">${subscribers || 1} elements</span></td>
        <td><span class="badge-hz">${hz || 60} Hz</span></td>
        <td>${scopeBadge}</td>
      `;
    } else if (msg.type === 'HMLR_OFFLINE_EVENT' && msg.detail) {
      if (offlineVal && msg.detail.queueLength !== undefined) {
        offlineVal.textContent = `${msg.detail.queueLength} Pending`;
      }
    } else if (msg.type === 'HMLR_CSP_STATE_EVENT' && msg.detail) {
      // Ingest initial state from <script type="application/json" hx-state>
      for (const [k, v] of Object.entries(msg.detail)) {
        let row = document.getElementById(`sig-${k}`);
        if (!row) {
          row = document.createElement('tr');
          row.id = `sig-${k}`;
          tbody.appendChild(row);
        }
        row.innerHTML = `
          <td><code>${k}</code></td>
          <td><strong>${typeof v === 'object' ? JSON.stringify(v) : v}</strong></td>
          <td><span class="badge-sub">CSP Initial</span></td>
          <td><span class="badge-hz">0 Hz</span></td>
          <td><span class="badge-sub">hx-state</span></td>
        `;
      }
    }
  });
}
