//! HMLR Native CLI Binary
//! Ultra-fast native CLI executing .fx and parsing .mx documents with zero-Node overhead.

use hmlr_core::{MXParser, DiagnosticEngine, HIREnv, HIREvaluator, HIRNode, HIRVal};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 { &args[1] } else { "help" };

    match command {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Usage: hmlr parse <file.mx>");
                std::process::exit(1);
            }
            let path = &args[2];
            let content = fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("Error reading file {}: {}", path, e);
                std::process::exit(1);
            });
            let doc = MXParser::parse(&content);
            println!("Parsed .mx Document (Zero-Copy):");
            println!("  Meta fields: {}", doc.meta.len());
            println!("  Models: {:?}", doc.models.keys());
            println!("  Tables: {}", doc.tables.len());
            for table in &doc.tables {
                println!("    - Table @model {}: {} rows", table.model, table.rows.len());
            }
            println!("  Pins: {}", doc.pins.len());
            println!("  Diagnostics: {}", doc.diags.len());
        }
        "diag" => {
            if args.len() < 3 {
                eprintln!("Usage: hmlr diag <file.mx>");
                std::process::exit(1);
            }
            let path = &args[2];
            let content = fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("Error reading file {}: {}", path, e);
                std::process::exit(1);
            });
            let doc = MXParser::parse(&content);
            let diags = DiagnosticEngine::validate_document(&doc);
            if diags.is_empty() {
                println!("✅ No diagnostic errors or boundary violations detected.");
            } else {
                for d in &diags {
                    println!("{}", DiagnosticEngine::format_diag(d));
                }
            }
        }
        "eval" => {
            let mut env = HIREnv::new();
            let prog = HIRNode::Let {
                name: "x".to_string(),
                val: Box::new(HIRNode::Const(HIRVal::Number(21.0))),
                body: Box::new(HIRNode::BinOp {
                    op: "*".to_string(),
                    left: Box::new(HIRNode::Var("x".to_string())),
                    right: Box::new(HIRNode::Const(HIRVal::Number(2.0))),
                }),
            };
            match HIREvaluator::eval(&prog, &mut env) {
                Ok(val) => println!("Evaluation Result: {:?}", val),
                Err(err) => eprintln!("Evaluation Error: {}", err),
            }
        }
        "help" | _ => {
            println!(
                r#"
HyperMedia Language Runtime (HMLR) — Native Rust Engine v1.0.0

Usage:
  hmlr parse <file.mx>    Parse a .mx spatial/tabular document into AST
  hmlr eval               Execute sample .fx functional HIR pipeline
  hmlr diag <file.mx>     Run AI self-healing diagnostic inspection on .mx document
  hmlr help               Show this help message
"#
            );
        }
    }
}
