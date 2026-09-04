//! HMLR Native Runtime Tests (Rust Cargo Test Harness)
//! Zero-Node, pure native test verification.

use hmlr_core::{MXParser, HIREnv, HIREvaluator, HIRNode, HIRVal};
use std::fs;
use std::path::{Path, PathBuf};

fn get_workspace_file(rel_path: &str) -> String {
    let mut p = PathBuf::from(rel_path);
    if !p.exists() {
        // Fallback for when running from crate directory
        p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../").join(rel_path);
    }
    fs::read_to_string(&p).unwrap_or_else(|_| panic!("Failed to read {:?}", p))
}

#[test]
fn test_parse_manifest_mx() {
    let content = get_workspace_file("MANIFEST.mx");
    let doc = MXParser::parse(&content);

    assert_eq!(doc.meta.get("project"), Some(&"HMLR"));
    assert_eq!(doc.meta.get("runtime"), Some(&"hmlr@1.0"));
    assert!(doc.tables.len() >= 2, "Should parse SubsystemScope and BusProtocol tables");

    let scope_table = doc.tables.iter().find(|t| t.model == "SubsystemScope").unwrap();
    assert!(scope_table.rows.len() >= 6, "Must contain all 6 core subsystems");
}

#[test]
fn test_parse_hmlr_mx() {
    let content = get_workspace_file("hmlr.mx");
    let doc = MXParser::parse(&content);

    assert_eq!(doc.meta.get("project"), Some(&"HMLR"));
    assert_eq!(doc.meta.get("toolchain"), Some(&"Rust_Cargo_Wasm"));
    assert!(doc.tables.iter().any(|t| t.model == "ArchitectureLayer"));
    assert!(doc.tables.iter().any(|t| t.model == "ProhibitedPattern"));
}

#[test]
fn test_parse_grammar_and_codes_mx() {
    let grammar_content = get_workspace_file("src/syntax/grammar.mx");
    let doc_grammar = MXParser::parse(&grammar_content);
    assert!(doc_grammar.tables.iter().any(|t| t.model == "GrammarProduction"));

    let codes_content = get_workspace_file("src/diagnostics/codes.mx");
    let doc_codes = MXParser::parse(&codes_content);
    assert!(doc_codes.tables.iter().any(|t| t.model == "DiagnosticSpec"));
}

#[test]
fn test_hir_evaluator() {
    let mut env = HIREnv::new();
    // let x = 20 in x + 22
    let prog = HIRNode::Let {
        name: "x".to_string(),
        val: Box::new(HIRNode::Const(HIRVal::Number(20.0))),
        body: Box::new(HIRNode::BinOp {
            op: "+".to_string(),
            left: Box::new(HIRNode::Var("x".to_string())),
            right: Box::new(HIRNode::Const(HIRVal::Number(22.0))),
        }),
    };

    let result = HIREvaluator::eval(&prog, &mut env).expect("Evaluation must succeed");
    assert_eq!(result, HIRVal::Number(42.0));
}

#[test]
fn test_hir_result_monad() {
    let mut env = HIREnv::new();
    let ok_node = HIRNode::ResultOk(Box::new(HIRNode::Const(HIRVal::Number(100.0))));
    let check_op = HIRNode::Apply {
        fn_name: "is_ok".to_string(),
        arg: Box::new(ok_node),
    };

    let result = HIREvaluator::eval(&check_op, &mut env).expect("Eval check");
    assert_eq!(result, HIRVal::Bool(true));
}
