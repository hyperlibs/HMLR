// HMLR Signals Panel Controller
const filterInput = document.getElementById('filter-input');
const tbody = document.getElementById('signals-body');

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
      const { id, value, subscribers, hz } = msg.detail;
      let row = document.getElementById(`sig-${id}`);
      if (!row) {
        row = document.createElement('tr');
        row.id = `sig-${id}`;
        tbody.prepend(row);
      }
      row.innerHTML = `
        <td><code>${id}</code></td>
        <td><strong>${typeof value === 'object' ? JSON.stringify(value) : value}</strong></td>
        <td><span class="badge-sub">${subscribers || 1} elements</span></td>
        <td><span class="badge-hz">${hz || 60} Hz</span></td>
        <td>Just now</td>
      `;
    }
  });
}
