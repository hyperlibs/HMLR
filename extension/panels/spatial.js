// HMLR 3D Spatial HUD Panel Controller
const port = chrome.runtime?.connect?.({ name: 'hmlr-spatial-panel' });
const tbody = document.getElementById('spatial-body');

port?.onMessage?.addListener((msg) => {
  if (msg.type === 'HMLR_SPATIAL_EVENT' && msg.detail) {
    const { id, type, pos, rot, scale } = msg.detail;
    let row = document.getElementById(`node-${id}`);
    if (!row) {
      row = document.createElement('tr');
      row.id = `node-${id}`;
      tbody.appendChild(row);
    }
    row.innerHTML = `
      <td>${id}</td>
      <td>${type || 'Node3D'}</td>
      <td><span class="coord">(${pos?.join?.(', ') || '0,0,0'})</span></td>
      <td>(${rot?.join?.(', ') || '0,0,0'})</td>
      <td>[${scale?.join?.(', ') || '1,1,1'}]</td>
    `;
  }
});
