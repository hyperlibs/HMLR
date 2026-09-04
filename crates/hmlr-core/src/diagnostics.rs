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

    pub fn format_diag(diag: &Diagnostic) -> String {
        format!(
            "@diag {} {} \"{}\" fix=\"{}\"",
            diag.code, diag.severity, diag.message, diag.fix
        )
    }
}
