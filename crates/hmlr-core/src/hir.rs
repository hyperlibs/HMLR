//! HMLR Intermediate Representation (HIR) & Functional Evaluation Engine
//! Pure immutable F#-inspired execution with pattern matching, Result<T,E>, and pipeline ops.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum HIRVal {
    Number(f64),
    Str(String),
    Bool(bool),
    Unit,
    Ok(Box<HIRVal>),
    Err(Box<HIRVal>),
    Tuple(Vec<HIRVal>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HIRNode {
    Const(HIRVal),
    Var(String),
    Let {
        name: String,
        val: Box<HIRNode>,
        body: Box<HIRNode>,
    },
    Apply {
        fn_name: String,
        arg: Box<HIRNode>,
    },
    BinOp {
        op: String,
        left: Box<HIRNode>,
        right: Box<HIRNode>,
    },
    If {
        cond: Box<HIRNode>,
        then_branch: Box<HIRNode>,
        else_branch: Box<HIRNode>,
    },
    ResultOk(Box<HIRNode>),
    ResultErr(Box<HIRNode>),
}

pub struct HIREnv {
    bindings: HashMap<String, HIRVal>,
}

impl HIREnv {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, val: HIRVal) {
        self.bindings.insert(name.to_string(), val);
    }

    pub fn get(&self, name: &str) -> Option<&HIRVal> {
        self.bindings.get(name)
    }
}

pub struct HIREvaluator;

impl HIREvaluator {
    pub fn eval(node: &HIRNode, env: &mut HIREnv) -> Result<HIRVal, String> {
        match node {
            HIRNode::Const(v) => Ok(v.clone()),
            HIRNode::Var(name) => env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Unbound variable '{}'", name)),
            HIRNode::Let { name, val, body } => {
                let v = Self::eval(val, env)?;
                let mut child_env = HIREnv {
                    bindings: env.bindings.clone(),
                };
                child_env.insert(name, v);
                Self::eval(body, &mut child_env)
            }
            HIRNode::Apply { fn_name, arg } => {
                let arg_val = Self::eval(arg, env)?;
                match fn_name.as_str() {
                    "is_ok" => match arg_val {
                        HIRVal::Ok(_) => Ok(HIRVal::Bool(true)),
                        _ => Ok(HIRVal::Bool(false)),
                    },
                    "unwrap" => match arg_val {
                        HIRVal::Ok(inner) => Ok(*inner),
                        HIRVal::Err(err) => Err(format!("Unwrap error: {:?}", err)),
                        other => Ok(other),
                    },
                    _ => Err(format!("Unknown native function '{}'", fn_name)),
                }
            }
            HIRNode::BinOp { op, left, right } => {
                let l = Self::eval(left, env)?;
                let r = Self::eval(right, env)?;
                match (l, r) {
                    (HIRVal::Number(a), HIRVal::Number(b)) => match op.as_str() {
                        "+" => Ok(HIRVal::Number(a + b)),
                        "-" => Ok(HIRVal::Number(a - b)),
                        "*" => Ok(HIRVal::Number(a * b)),
                        "/" => Ok(HIRVal::Number(a / b)),
                        "==" => Ok(HIRVal::Bool(a == b)),
                        "!=" => Ok(HIRVal::Bool(a != b)),
                        ">" => Ok(HIRVal::Bool(a > b)),
                        "<" => Ok(HIRVal::Bool(a < b)),
                        _ => Err(format!("Unsupported numeric binop '{}'", op)),
                    },
                    (HIRVal::Str(a), HIRVal::Str(b)) => match op.as_str() {
                        "+" => Ok(HIRVal::Str(format!("{}{}", a, b))),
                        "==" => Ok(HIRVal::Bool(a == b)),
                        _ => Err(format!("Unsupported string binop '{}'", op)),
                    },
                    (HIRVal::Bool(a), HIRVal::Bool(b)) => match op.as_str() {
                        "&&" => Ok(HIRVal::Bool(a && b)),
                        "||" => Ok(HIRVal::Bool(a || b)),
                        "==" => Ok(HIRVal::Bool(a == b)),
                        _ => Err(format!("Unsupported bool binop '{}'", op)),
                    },
                    _ => Err("Type mismatch in binary operation".to_string()),
                }
            }
            HIRNode::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let c = Self::eval(cond, env)?;
                match c {
                    HIRVal::Bool(true) => Self::eval(then_branch, env),
                    HIRVal::Bool(false) => Self::eval(else_branch, env),
                    _ => Err("Condition must evaluate to bool".to_string()),
                }
            }
            HIRNode::ResultOk(inner) => {
                let v = Self::eval(inner, env)?;
                Ok(HIRVal::Ok(Box::new(v)))
            }
            HIRNode::ResultErr(inner) => {
                let e = Self::eval(inner, env)?;
                Ok(HIRVal::Err(Box::new(e)))
            }
        }
    }
}
