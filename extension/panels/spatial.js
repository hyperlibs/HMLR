// HMLR 3D Spatial HUD & SpatialEdgeDB Panel Controller
const port = chrome.runtime?.connect?.({ name: 'hmlr-spatial-panel' });
const tbody = document.getElementById('spatial-body');
const cellsVal = document.getElementById('db-cells');
const mortonVal = document.getElementById('morton-eff');
const photonsVal = document.getElementById('active-photons');
const physicsVal = document.getElementById('physics-time');

port?.onMessage?.addListener((msg) => {
  if (msg.type === 'HMLR_SPATIAL_EVENT' && msg.detail) {
    const { id, type, pos, rot, scale, edge_db, total_photons, physics_step_ms } = msg.detail;

    if (edge_db) {
      if (cellsVal) cellsVal.textContent = edge_db.total_cells?.toLocaleString() || cellsVal.textContent;
      if (mortonVal) mortonVal.textContent = `${(edge_db.morton_efficiency || 98.4).toFixed(1)}%`;
    }
    if (total_photons && photonsVal) photonsVal.textContent = total_photons.toLocaleString();
    if (physics_step_ms && physicsVal) physicsVal.textContent = `${physics_step_ms.toFixed(2)} ms`;

    if (id && tbody) {
      let row = document.getElementById(`node-${id}`);
      if (!row) {
        row = document.createElement('tr');
        row.id = `node-${id}`;
        tbody.prepend(row);
      }
      const kindClass = type === 'SpatialEdgeDB' ? 'type-cell' : type === 'PerspectiveCamera' ? 'type-pin' : 'type-3d';
      row.innerHTML = `
        <td><code>${id}</code></td>
        <td><span class="${kindClass}">${type || 'Node3D'}</span></td>
        <td><span class="coord-val">(${pos?.join?.(', ') || '0,0,0'})</span></td>
        <td>(${rot?.join?.(', ') || '0,0,0'})</td>
        <td>[${scale?.join?.(', ') || '1,1,1'}]</td>
        <td>physics: true, active: true</td>
      `;
    }
  }
});
