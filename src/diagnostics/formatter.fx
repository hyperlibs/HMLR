// HMLR Diagnostics: AI Self-Healing Diagnostic Formatter
// Formats deterministic @diag FX-XXXX records in pure .mx syntax

let format_diag code severity message line col fix =
  let header = "@diag " + code + " " + severity + " \"" + message + "\"" in
  let with_line = if line > 0 then header + " line=" + int_to_string line else header in
  let with_col = if col > 0 then with_line + " col=" + int_to_string col else with_line in
  let with_fix = if string_length fix > 0 then with_col + " fix=\"" + fix + "\"" else with_col in
  with_fix

let create_syntax_error code msg line col fix =
  format_diag code "error" msg line col fix

let create_boundary_error code msg fix =
  format_diag code "error" msg 0 0 fix

let create_velocity_warning code msg latency fix =
  format_diag code "warning" msg 0 0 fix

// Format batch diagnostic stream
let format_diagnostics_stream diags =
  diags
  |> map_list (fun d -> format_diag d.code d.severity d.message d.line d.col d.fix)
  |> join_lines "\n"
