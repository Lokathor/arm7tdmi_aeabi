#![allow(unused)]

extern crate proc_macro;
use std::collections::{hash_map::Entry, HashMap};

use proc_macro::{Ident, Literal, TokenStream, TokenTree};

/// This holds all the `extern "C"` declarations for the assembly code that the
/// crate can provide.
#[warn(missing_docs)]
#[allow(dead_code)]
mod fn_declarations;

const THE_CODE_BASE: &'static str = include_str!("the_code.s");
const FN_DECLARATIONS_BASE: &'static str = include_str!("fn_declarations.rs");

#[derive(Debug, Clone, Default)]
struct Args {
  section_prefix: String,
}

#[proc_macro]
pub fn generate_fns(token_stream: TokenStream) -> TokenStream {
  let mapping = parse_mapping(token_stream);
  println!("MAPPING: {mapping:?}");

  let args = Args {
    section_prefix: match mapping.get("section_prefix") {
      Some(section_prefix) => {
        let s = section_prefix.to_string();
        assert!(s.starts_with('"'), "section_prefix must be a string literal");
        assert!(s.ends_with('"'), "section_prefix must be a string literal");
        s
      }
      None => String::from(r#""""#),
    },
  };
  println!("ARGS: {args:?}");

  let section_replacement = {
    // `section_prefix` uses a string literal,
    // which has double quotes in it. We'll keep the opening one,
    // and match on that as part of the `replace`, then only the closing
    // double quote has to be popped away.
    let mut t = format!(r#".section {}"#, args.section_prefix);
    t.pop();
    t
  };

  let mut the_code = THE_CODE_BASE.to_string();
  let mut fn_declarations = FN_DECLARATIONS_BASE.to_string();
  for s in [&mut the_code, &mut fn_declarations] {
    // *s = s.replace("\r\n", "\n");
    *s = s.replace("libc_", "");
    *s = s.replace("aeabi_", "__aeabi_");
    *s = s.replace(r#".section ""#, section_replacement.as_str());
  }

  let out_string = format!(
    r#"
    #[cfg(not(target_feature="thumb"))]
    ::core::arch::global_asm!({the_code:?}, options(raw));

    #[cfg(target_feature="thumb")]
    ::core::arch::global_asm!(
      ".code 32",
      {the_code:?},
      ".code 16",
      options(raw)
    );

    {fn_declarations}
    "#
  );
  println!("===OUT_STRING===\n{out_string}===END===");

  out_string.parse().unwrap()
}

fn parse_mapping(token_stream: TokenStream) -> HashMap<String, Literal> {
  let mut hm = HashMap::new();
  let mut tt_iter = token_stream.into_iter();
  loop {
    let ident = match tt_iter.next() {
      Some(TokenTree::Ident(ident)) => ident,
      None => break,
      other => panic!("Expected Some(Ident), got {other:?}"),
    };
    let _eq = match tt_iter.next() {
      Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => punct,
      other => panic!("Expected Some(Punct(=)), got {other:?}"),
    };
    let literal = match tt_iter.next() {
      Some(TokenTree::Literal(literal)) => literal,
      other => panic!("Expected Some(Literal), got {other:?}"),
    };
    let ident_string = ident.to_string();
    match hm.entry(ident_string) {
      Entry::Occupied(_) => panic!("Identifier {ident} was specified twice!"),
      Entry::Vacant(ve) => ve.insert(literal),
    };
    let _comma = match tt_iter.next() {
      Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => punct,
      None => break,
      other => panic!("Expected Some(Punct(',')), got {other:?}"),
    };
  }
  hm
}
