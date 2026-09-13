//! HMLR AI Self-Healing Diagnostics
//! Formats and validates @diag FX-XXXX records for ecosystem compliance.

use crate::parser_mx::MXDocument;

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub severity: &'static str,
    pub message: String,
    pub fix: &'static str,
}

pub struct DiagnosticEngine;

impl DiagnosticEngine {
    pub fn validate_document(doc: &MXDocument) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        // Check if tables have matching models
        for table in &doc.tables {
            if !doc.models.contains_key(table.model) {
                diags.push(Diagnostic {
                    code: "FX-1002",
                    severity: "warning",
                    message: format!("Table references model '{}' without declaration", table.model),
                    fix: "Add @model declaration for the table",
                });
            }
        }

        diags
    }

    /// Linter for HTMX 2 markup conventions
    pub fn validate_htmx_template(template_html: &str) -> Vec<Diagnostic> {
        let mut diags = Vec::new();

        if template_html.contains("hx-on:click") {
            diags.push(Diagnostic {
                code: "FX-0105",
                severity: "warning",
                message: "Unescaped 'hx-on:click' detected in HTMX 2 markup".to_string(),
                fix: "Use '@click' or 'hx-action' to prevent HTMX 2 window global scope evaluation leakage",
            });
        }

        diags
    }

    pub fn format_diag(diag: &Diagnostic) -> String {
        format!(
            "@diag {} {} \"{}\" fix=\"{}\"",
            diag.code, diag.severity, diag.message, diag.fix
        )
    }
}
