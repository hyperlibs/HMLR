// HMLR Compiler: Tokenizer State Machine
// Written in pure .fx functional syntax

// Token Type Definitions
let tok_directive tag = ("DIRECTIVE", tag)
let tok_ident id = ("IDENT", id)
let tok_number n = ("NUMBER", n)
let tok_string s = ("STRING", s)
let tok_symbol sym = ("SYMBOL", sym)
let tok_eof = ("EOF", "")

// State Machine transitions
let is_whitespace ch =
  match ch with
  | " " -> true
  | "\t" -> true
  | "\n" -> true
  | "\r" -> true
  | _ -> false

let is_digit ch =
  match ch with
  | "0" -> true | "1" -> true | "2" -> true | "3" -> true | "4" -> true
  | "5" -> true | "6" -> true | "7" -> true | "8" -> true | "9" -> true
  | _ -> false

let is_symbol ch =
  match ch with
  | "|" -> true | ">" -> true | "-" -> true | "=" -> true
  | "(" -> true | ")" -> true | "{" -> true | "}" -> true
  | "+" -> true | "*" -> true | "/" -> true | "," -> true
  | ":" -> true | "@" -> true
  | _ -> false

// Scan next token step
let scan_step input pos =
  if pos >= string_length input then
    Ok (tok_eof, pos)
  else
    let ch = char_at input pos in
    if is_whitespace ch then
      scan_step input (pos + 1)
    else if ch == "@" then
      let tag = scan_word input (pos + 1) in
      Ok (tok_directive tag, pos + string_length tag + 1)
    else if is_symbol ch then
      Ok (tok_symbol ch, pos + 1)
    else
      let word = scan_word input pos in
      Ok (tok_ident word, pos + string_length word)

// Pipeline token stream processor
let tokenize source =
  source
  |> scan_all 0
