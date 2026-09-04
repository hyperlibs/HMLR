//! HMLR Native CLI Binary
//! Ultra-fast native CLI executing .fx, parsing .mx documents, and provisioning the universal ecosystem runtime.

use hmlr_core::{
    DiagnosticEngine, HIREnv, HIREvaluator, HIRNode, HIRVal, HMLRProvisioner, MXParser,
};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 { &args[1] } else { "help" };

    match command {
        "doctor" | "status" => {
            println!("🔍 Inspecting HMLR Ecosystem Environment...\n");
            let report = HMLRProvisioner::inspect_environment();

            println!("@meta");
            println!("  engine: \"HMLR Native Runtime\"");
            println!("  version: \"1.0.0\"");
            println!("  ambient_installed: {}", report.runtime_installed);
            println!("  in_system_path: {}", report.in_system_path);
            println!("  install_location: \"{}\"", report.runtime_path.display());
            println!("");
            println!("@model BrowserInspection");
            println!("  browser: string");
            println!("  installed: boolean");
            println!("  extension_provisioned: boolean");
            println!("  status: string");
            println!("");
            println!("## BrowserInspection | @model BrowserInspection");
            println!("| browser | installed | extension_provisioned | status |");
            println!("| --- | --- | --- | --- |");

            for b in &report.browsers {
                let status = if !b.installed {
                    "NOT_DETECTED"
                } else if b.extension_installed {
                    "READY_ACTIVE"
                } else {
                    "NEEDS_INSTALLATION"
                };
                println!(
                    "| {} | {} | {} | {} |",
                    b.name, b.installed, b.extension_installed, status
                );
            }
            println!("");
            if !report.runtime_installed || report.browsers.iter().any(|b| b.installed && !b.extension_installed) {
                println!("💡 Tip: Run `hmlr setup` to provision ambient runtime and browser extensions globally.");
            } else {
                println!("✅ All ecosystem runtimes and DevTools extensions are fully synchronized.");
            }
        }

        "setup" | "install" => {
            println!("⚙️ Provisioning HMLR Universal Ambient Runtime & Extensions...\n");
            let current_exe = env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("hmlr.exe"));
            let ext_src = if Path::new("extension").exists() {
                Some(Path::new("extension"))
            } else if Path::new("../../extension").exists() {
                Some(Path::new("../../extension"))
            } else {
                None
            };

            match HMLRProvisioner::install_system(&current_exe, ext_src) {
                Ok(_) => {
                    let install_dir = HMLRProvisioner::get_default_install_dir();
                    println!("✅ Ambient Runtime Installed:");
                    println!("   Binary Location:    {}", install_dir.join("bin").display());
                    println!("   Extension Location: {}", install_dir.join("extension").display());
                    println!("\n🌐 Browser Extension Provisioning:");
                    println!("   To load in Chrome / Edge / Brave:");
                    println!("   1. Navigate to chrome://extensions (or edge://extensions)");
                    println!("   2. Enable 'Developer mode'");
                    println!("   3. Click 'Load unpacked' and select: {}", install_dir.join("extension").display());
                    println!("\n   To load in Firefox:");
                    println!("   1. Navigate to about:debugging#/runtime/this-firefox");
                    println!("   2. Click 'Load Temporary Add-on' and select: {}", install_dir.join("extension").join("manifest.json").display());
                }
                Err(e) => {
                    eprintln!("❌ Installation error: {}", e);
                    std::process::exit(1);
                }
            }
        }

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
HyperMedia Language Runtime (HMLR) — Native Universal Engine v1.0.0

Usage:
  hmlr doctor             Inspect system PATH, browser DevTools extensions, and ambient runtime
  hmlr setup              Provision global runtime binary and multi-browser DevTools extensions
  hmlr parse <file.mx>    Parse a .mx spatial/tabular document into AST
  hmlr eval               Execute sample .fx functional HIR pipeline
  hmlr diag <file.mx>     Run AI self-healing diagnostic inspection on .mx document
  hmlr help               Show this help message
"#
            );
        }
    }
}
