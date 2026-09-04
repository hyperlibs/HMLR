// HMLR Compiler: Immutable AST & Pattern Matching
// Written in pure .fx functional syntax

// AST Node Constructors
let ast_lit_num n = ("LitNumber", n)
let ast_lit_str s = ("LitString", s)
let ast_lit_bool b = ("LitBool", b)
let ast_lit_unit = ("LitUnit", ())
let ast_ident name = ("Ident", name)

let ast_let name value body =
  ("Let", name, value, body)

let ast_lambda param body =
  ("Lambda", param, body)

let ast_apply fn arg =
  ("Apply", fn, arg)

let ast_pipe left right =
  // Desugars pipeline into forward application
  ast_apply right left

let ast_result_ok val =
  ("ResultOk", val)

let ast_result_err err =
  ("ResultErr", err)

let ast_match expr arms =
  ("Match", expr, arms)

// Pattern matching evaluation kernel
let eval_ast node env =
  match node with
  | ("LitNumber", n) -> Ok n
  | ("LitString", s) -> Ok s
  | ("LitBool", b) -> Ok b
  | ("LitUnit", ()) -> Ok ()
  | ("Ident", name) -> lookup_env env name
  | ("Let", name, val_expr, body_expr) ->
      match eval_ast val_expr env with
      | Ok v ->
          let next_env = extend_env env name v in
          eval_ast body_expr next_env
      | Err e -> Err e
  | ("Apply", fn_expr, arg_expr) ->
      match eval_ast fn_expr env with
      | Ok fn_val ->
          match eval_ast arg_expr env with
          | Ok arg_val -> apply_closure fn_val arg_val
          | Err e -> Err e
      | Err e -> Err e
  | ("ResultOk", inner) ->
      match eval_ast inner env with
      | Ok v -> Ok (ast_result_ok v)
      | Err e -> Err e
  | ("ResultErr", inner) ->
      match eval_ast inner env with
      | Ok v -> Ok (ast_result_err v)
      | Err e -> Err e
  | _ -> Err ("Unknown AST node kind")
