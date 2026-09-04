//! HMLR (HyperMedia Language Runtime) Core Library
//! Zero-dependency, pure Rust native runtime kernel.

pub mod parser_mx;
pub mod hir;
pub mod diagnostics;
pub mod installer;

pub use parser_mx::{MXDocument, MXParser, MXPin, MXTable};
pub use hir::{HIREnv, HIREvaluator, HIRNode, HIRVal};
pub use diagnostics::{Diagnostic, DiagnosticEngine};
pub use installer::{BrowserInfo, HMLRProvisioner, SystemEnvironmentReport};
