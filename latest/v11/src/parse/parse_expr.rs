use super::{ parse_value, Parser };

pub fn parse_expr(parser: &mut Parser) {
  loop {
    println!("Parsed value: {:?}", parse_value(parser));
    println!();
    parser.skip_spaces();
    match parser.current() {
      '\n' => {
        // Look for a token that completes the expression, or end it.
        // This will be complicated
        // Example:
        // 1 expression: `let x = 1\n    .to_string()`
        // 2 expressions: `let x = 1\nlet y = 2`
        todo!()
      },
      _ => syntax_err!("unexpected token {:?}", parser.current(); parser)
    }
  }
}
