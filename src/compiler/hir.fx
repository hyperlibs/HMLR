// HMLR Compiler: High-level Intermediate Representation (HIR)
// Pure .fx lowering and optimization pass

// HIR Opcode Constructors
let hir_const val = ("HIR_CONST", val)
let hir_var name = ("HIR_VAR", name)
let hir_let name val body = ("HIR_LET", name, val, body)
let hir_lambda param body = ("HIR_LAMBDA", param, body)
let hir_apply fn arg = ("HIR_APPLY", fn, arg)
let hir_binop op left right = ("HIR_BINOP", op, left, right)
let hir_match expr arms = ("HIR_MATCH", expr, arms)

// AST to HIR Lowering
let lower_to_hir ast_node =
  match ast_node with
  | ("LitNumber", n) -> hir_const ("number", n)
  | ("LitString", s) -> hir_const ("string", s)
  | ("LitBool", b) -> hir_const ("bool", b)
  | ("LitUnit", ()) -> hir_const ("unit", ())
  | ("Ident", name) -> hir_var name
  | ("Let", name, val_expr, body_expr) ->
      let hir_v = lower_to_hir val_expr in
      let hir_b = lower_to_hir body_expr in
      hir_let name hir_v hir_b
  | ("Lambda", param, body) ->
      let hir_b = lower_to_hir body in
      hir_lambda param hir_b
  | ("Apply", fn, arg) ->
      let hir_f = lower_to_hir fn in
      let hir_a = lower_to_hir arg in
      hir_apply hir_f hir_a
  | _ -> hir_const ("unknown", ())

// Optimization Pass: Constant folding
let optimize_hir hir_node =
  match hir_node with
  | ("HIR_BINOP", "+", ("HIR_CONST", ("number", a)), ("HIR_CONST", ("number", b))) ->
      hir_const ("number", a + b)
  | ("HIR_BINOP", "*", ("HIR_CONST", ("number", a)), ("HIR_CONST", ("number", b))) ->
      hir_const ("number", a * b)
  | other -> other

// Pipeline compilation flow
let compile_source ast =
  ast
  |> lower_to_hir
  |> optimize_hir
