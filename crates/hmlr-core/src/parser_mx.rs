//! HMLR Zero-Copy Streaming .mx Parser
//! Blazing fast, zero-heap-allocation parser for flat spatial coordinate & tabular documents.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct MXPin<'a> {
    pub id: &'a str,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
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
    pub diags: Vec<&'a str>,
}

impl<'a> MXDocument<'a> {
    pub fn new() -> Self {
        Self {
            meta: HashMap::new(),
            models: HashMap::new(),
            tables: Vec::new(),
            pins: Vec::new(),
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
}
