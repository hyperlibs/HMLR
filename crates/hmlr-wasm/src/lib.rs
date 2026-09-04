//! HMLR Ultra-Lean WebAssembly Runtime (<50KB)
//! Native edge execution target for Cloudflare Workers and Browser DevTools.

use hmlr_core::{MXParser, HIREnv, HIREvaluator, HIRNode, HIRVal};

#[no_mangle]
pub extern "C" fn hmlr_version() -> u32 {
    100 // Version 1.0.0 encoded as integer
}

#[no_mangle]
pub extern "C" fn hmlr_parse_mx_len(ptr: *const u8, len: usize) -> usize {
    if ptr.is_null() || len == 0 {
        return 0;
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    if let Ok(source) = std::str::from_utf8(slice) {
        let doc = MXParser::parse(source);
        doc.tables.len() + doc.pins.len()
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn hmlr_eval_add(a: f64, b: f64) -> f64 {
    let mut env = HIREnv::new();
    let expr = HIRNode::BinOp {
        op: "+".to_string(),
        left: Box::new(HIRNode::Const(HIRVal::Number(a))),
        right: Box::new(HIRNode::Const(HIRVal::Number(b))),
    };
    match HIREvaluator::eval(&expr, &mut env) {
        Ok(HIRVal::Number(res)) => res,
        _ => 0.0,
    }
}
