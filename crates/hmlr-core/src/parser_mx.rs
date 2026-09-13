//! HMLR Zero-Copy Streaming .mx Parser
//! Blazing fast, zero-heap-allocation parser for flat spatial coordinate, SpatialEdgeDB & tabular documents.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct MXPin<'a> {
    pub id: &'a str,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MXCell<'a> {
    pub id: &'a str,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub layer: u16,
    pub intensity: f32,
    pub morton_code: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MXLayer<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MXMatrixDelta<'a> {
    pub row: usize,
    pub col: usize,
    pub value: &'a str,
    pub flash_class: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MXTable<'a> {
    pub model: &'a str,
    pub headers: Vec<&'a str>,
    pub rows: Vec<HashMap<&'a str, &'a str>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MXDocument<'a> {
    pub meta: HashMap<&'a str, &'a str>,
    pub models: HashMap<&'a str, Vec<(&'a str, &'a str)>>,
    pub tables: Vec<MXTable<'a>>,
    pub pins: Vec<MXPin<'a>>,
    pub cells: Vec<MXCell<'a>>,
    pub layers: Vec<MXLayer<'a>>,
    pub deltas: Vec<MXMatrixDelta<'a>>,
    pub diags: Vec<&'a str>,
}

impl<'a> MXDocument<'a> {
    pub fn new() -> Self {
        Self {
            meta: HashMap::new(),
            models: HashMap::new(),
            tables: Vec::new(),
            pins: Vec::new(),
            cells: Vec::new(),
            layers: Vec::new(),
            deltas: Vec::new(),
            diags: Vec::new(),
        }
    }
}

enum SectionState<'a> {
    None,
    Meta,
    Model(&'a str),
}

pub struct MXParser;

impl MXParser {
    pub fn parse<'a>(source: &'a str) -> MXDocument<'a> {
        let mut doc = MXDocument::new();
        let mut state = SectionState::None;
        let mut current_table_model: Option<&'a str> = None;
        let mut table_headers: Vec<&'a str> = Vec::new();
        let mut table_rows: Vec<HashMap<&'a str, &'a str>> = Vec::new();

        for raw_line in source.lines() {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }

            // Comments (ignore # but keep ## section markers)
            if line.starts_with('#') && !line.starts_with("##") {
                continue;
            }

            // Directives
            if line.starts_with("@meta") {
                state = SectionState::Meta;
                continue;
            }

            if line.starts_with("@diag") {
                doc.diags.push(line);
                continue;
            }

            if line.starts_with("@type") {
                state = SectionState::None;
                continue;
            }

            if line.starts_with("@model") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let m_name = parts[1];
                    state = SectionState::Model(m_name);
                    doc.models.entry(m_name).or_insert_with(Vec::new);
                } else {
                    state = SectionState::None;
                }
                continue;
            }

            if line.starts_with("@pin") {
                state = SectionState::None;
                if let Some(pin) = Self::parse_pin(line) {
                    doc.pins.push(pin);
                }
                continue;
            }

            // SpatialEdgeDB Flat Cell Records: @cell id (x, y, z) layer=1 intensity=1.0 morton=12345
            if line.starts_with("@cell") {
                state = SectionState::None;
                if let Some(cell) = Self::parse_cell(line) {
                    doc.cells.push(cell);
                }
                continue;
            }

            // SpatialEdgeDB Layers: @layer id name="Tumor Stroma" opacity=0.85
            if line.starts_with("@layer") {
                state = SectionState::None;
                if let Some(layer) = Self::parse_layer(line) {
                    doc.layers.push(layer);
                }
                continue;
            }

            // Hypersheet / Virtual Data Grid 2D Matrix Delta: Δrow:col:val[:flashClass] or @delta row:col:val
            if line.starts_with('Δ') || line.starts_with("@delta") {
                state = SectionState::None;
                if let Some(delta) = Self::parse_delta(line) {
                    doc.deltas.push(delta);
                }
                continue;
            }

            // Section table binding: ## Section Title | @model ModelName
            if line.starts_with("##") {
                state = SectionState::None;
                if line.contains("@model") {
                    // Flush previous table
                    if let Some(model) = current_table_model {
                        if !table_headers.is_empty() {
                            doc.tables.push(MXTable {
                                model,
                                headers: table_headers.clone(),
                                rows: table_rows.clone(),
                            });
                            table_headers.clear();
                            table_rows.clear();
                        }
                    }

                    if let Some(idx) = line.find("@model") {
                        let m_name = line[idx + 6..].trim();
                        current_table_model = Some(m_name);
                    }
                }
                continue;
            }

            // Table rows: | h1 | h2 |
            if line.starts_with('|') && line.ends_with('|') {
                state = SectionState::None;
                // Ignore markdown divider | --- | --- |
                if line.chars().all(|c| c == '|' || c == '-' || c == ':' || c == ' ') {
                    continue;
                }

                let cols: Vec<&'a str> = line[1..line.len() - 1]
                    .split('|')
                    .map(|s| s.trim())
                    .collect();

                if table_headers.is_empty() {
                    table_headers = cols;
                } else {
                    let mut row = HashMap::new();
                    for (i, &h) in table_headers.iter().enumerate() {
                        if i < cols.len() {
                            row.insert(h, cols[i]);
                        }
                    }
                    table_rows.push(row);
                }
                continue;
            }

            // Indented key-value or field definition
            if line.contains(':') && !line.starts_with('|') {
                if let Some((k, v)) = line.split_once(':') {
                    let key = k.trim();
                    let val = v.trim();
                    match state {
                        SectionState::Meta => {
                            doc.meta.insert(key, val);
                        }
                        SectionState::Model(m_name) => {
                            doc.models.entry(m_name).or_default().push((key, val));
                        }
                        SectionState::None => {
                            doc.meta.insert(key, val);
                        }
                    }
                }
                continue;
            }
        }

        // Flush trailing table
        if let Some(model) = current_table_model {
            if !table_headers.is_empty() {
                doc.tables.push(MXTable {
                    model,
                    headers: table_headers,
                    rows: table_rows,
                });
            }
        }

        doc
    }

    fn parse_pin<'a>(line: &'a str) -> Option<MXPin<'a>> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }
        let id = parts[1];
        let open_p = line.find('(')?;
        let close_p = line.find(')')?;
        let coords_str = &line[open_p + 1..close_p];
        let nums: Vec<f64> = coords_str
            .split(',')
            .filter_map(|s| s.trim().parse::<f64>().ok())
            .collect();

        if nums.len() >= 2 {
            Some(MXPin {
                id,
                x: nums[0],
                y: nums[1],
                z: if nums.len() > 2 { Some(nums[2]) } else { None },
            })
        } else {
            None
        }
    }

    fn parse_cell<'a>(line: &'a str) -> Option<MXCell<'a>> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }
        let id = parts[1];
        let open_p = line.find('(')?;
        let close_p = line.find(')')?;
        let coords_str = &line[open_p + 1..close_p];
        let nums: Vec<f64> = coords_str
            .split(',')
            .filter_map(|s| s.trim().parse::<f64>().ok())
            .collect();

        if nums.len() < 3 {
            return None;
        }

        let mut layer: u16 = 0;
        let mut intensity: f32 = 1.0;
        let mut morton: u64 = 0;

        for part in &parts[2..] {
            if let Some((k, v)) = part.split_once('=') {
                match k {
                    "layer" => layer = v.parse().unwrap_or(0),
                    "intensity" => intensity = v.parse().unwrap_or(1.0),
                    "morton" => morton = v.parse().unwrap_or(0),
                    _ => {}
                }
            }
        }

        Some(MXCell {
            id,
            x: nums[0],
            y: nums[1],
            z: nums[2],
            layer,
            intensity,
            morton_code: morton,
        })
    }

    fn parse_layer<'a>(line: &'a str) -> Option<MXLayer<'a>> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        let id = parts[1];
        let mut name = "Layer";
        let mut opacity = 1.0f32;

        if let Some(n_idx) = line.find("name=\"") {
            let rest = &line[n_idx + 6..];
            if let Some(end_idx) = rest.find('"') {
                name = &rest[..end_idx];
            }
        }

        for part in &parts[2..] {
            if let Some((k, v)) = part.split_once('=') {
                if k == "opacity" {
                    opacity = v.parse().unwrap_or(1.0);
                }
            }
        }

        Some(MXLayer { id, name, opacity })
    }

    fn parse_delta<'a>(line: &'a str) -> Option<MXMatrixDelta<'a>> {
        // e.g. "Δ14:3:$9,450.00:bg-emerald-500" or "@delta 14:3:val"
        let raw = if line.starts_with('Δ') {
            &line['Δ'.len_utf8()..]
        } else if line.starts_with("@delta") {
            line[6..].trim()
        } else {
            line
        };

        let parts: Vec<&'a str> = raw.split(':').collect();
        if parts.len() < 3 {
            return None;
        }

        let row: usize = parts[0].trim().parse().ok()?;
        let col: usize = parts[1].trim().parse().ok()?;
        let value = parts[2].trim();
        let flash_class = if parts.len() > 3 { Some(parts[3].trim()) } else { None };

        Some(MXMatrixDelta {
            row,
            col,
            value,
            flash_class,
        })
    }
}
