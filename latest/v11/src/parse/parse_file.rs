use crate::parse::parse_expr;

use super::{ Parser, parse_function };

/// Parses the items in the provided file content
pub fn parse_file(file_name: &str, data: &str) {
  let mut parser = Parser::new(file_name, data);

  loop {
    parser.skip_whitespaces();
    if !parser.current().is_ascii_alphabetic() {
      syntax_err!("invalid or unexpected token {:?}", parser.current(); parser);
    }
    let word = parser.parse_ascii_word();
    match word.as_str() {
      "const" => {
        parser.next_token();
        // HINT: maybe we should match `{` or `[` to implement destructuring,
        // and if any alphabetic character is found, then call `parser.parse_word()`
        let ident = parser.expect_word();

        parser.next_token();
        // parser.expect(':');

        parser.expect('=');
        parser.next_token();
        let value = parse_expr(&mut parser);
        println!("{:#?}", value);
        todo!()
      },
      "fun" => {
        parser.skip_whitespaces();
        parse_function(&mut parser);
        todo!();
      }
      _ => syntax_err!("unexpected token {word:?} here"; parser)
    }
  }
}
