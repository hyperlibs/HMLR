// HMLR Signals Panel Controller
const port = chrome.runtime?.connect?.({ name: 'hmlr-signals-panel' });
const tbody = document.getElementById('signals-body');

port?.onMessage?.addListener((msg) => {
  if (msg.type === 'HMLR_SIGNAL_EVENT' && msg.detail) {
    const { id, value, subscribers, hz } = msg.detail;
    let row = document.getElementById(`sig-${id}`);
    if (!row) {
      row = document.createElement('tr');
      row.id = `sig-${id}`;
      tbody.appendChild(row);
    }
    row.innerHTML = `
      <td>${id}</td>
      <td>${typeof value === 'object' ? JSON.stringify(value) : value}</td>
      <td>${subscribers || 1}</td>
      <td><span class="badge">${hz || 60} Hz</span></td>
    `;
  }
});
