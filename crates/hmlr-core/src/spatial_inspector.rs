//! HMLR Spatial & htmFX Telemetry Inspector
//! Passive inspection layer for htmFX 3D viewports, physics step timings, and particle telemetry.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SpatialEntityTelemetry {
    pub id: String,
    pub entity_type: String,
    pub position: [f64; 3],
    pub rotation_euler_deg: [f64; 3],
    pub scale: [f64; 3],
    pub physics_active: bool,
    pub linear_velocity: Option<[f64; 3]>,
    pub active_particles: Option<u32>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SpatialSnapshot {
    pub frame_id: u64,
    pub delta_time_ms: f64,
    pub active_cameras: usize,
    pub total_entities: usize,
    pub total_photons: u32,
    pub physics_step_ms: f64,
    pub entities: Vec<SpatialEntityTelemetry>,
}

pub struct SpatialInspector;

impl SpatialInspector {
    pub fn inspect_snapshot(snapshot: &SpatialSnapshot) -> String {
        let mut out = String::new();
        out.push_str("@meta\n");
        out.push_str(&format!("  inspector: \"HMLR htmFX Spatial HUD\"\n"));
        out.push_str(&format!("  frame_id: {}\n", snapshot.frame_id));
        out.push_str(&format!("  delta_time_ms: {:.2}\n", snapshot.delta_time_ms));
        out.push_str(&format!("  physics_step_ms: {:.2}\n", snapshot.physics_step_ms));
        out.push_str(&format!("  total_photons: {}\n", snapshot.total_photons));
        out.push_str("\n@model SpatialEntityStatus\n");
        out.push_str("  id: string\n");
        out.push_str("  entity_type: string\n");
        out.push_str("  position: string\n");
        out.push_str("  rotation: string\n");
        out.push_str("  physics: boolean\n");
        out.push_str("  particles: string\n");
        out.push_str("\n## SpatialEntityStatus | @model SpatialEntityStatus\n");
        out.push_str("| id | entity_type | position | rotation | physics | particles |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");

        for e in &snapshot.entities {
            let pos_str = format!("({:.1}, {:.1}, {:.1})", e.position[0], e.position[1], e.position[2]);
            let rot_str = format!("({:.0}°, {:.0}°, {:.0}°)", e.rotation_euler_deg[0], e.rotation_euler_deg[1], e.rotation_euler_deg[2]);
            let particles_str = e.active_particles.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string());
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                e.id, e.entity_type, pos_str, rot_str, e.physics_active, particles_str
            ));
        }

        out
    }
}
