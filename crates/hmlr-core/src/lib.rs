//! HMLR (HyperMedia Language Runtime) Core Library
//! Zero-dependency, pure Rust native runtime kernel.

pub mod parser_mx;
pub mod hir;
pub mod diagnostics;
pub mod installer;
pub mod spatial_inspector;

pub use parser_mx::{MXCell, MXDocument, MXLayer, MXMatrixDelta, MXParser, MXPin, MXTable};
pub use hir::{HIREnv, HIREvaluator, HIRNode, HIRVal};
pub use diagnostics::{Diagnostic, DiagnosticEngine};
pub use installer::{BrowserInfo, HMLRProvisioner, SystemEnvironmentReport};
pub use spatial_inspector::{SpatialEntityTelemetry, SpatialInspector, SpatialSnapshot};
