mod env;
mod interpreter;
mod error;
mod parser;
mod span;
mod tokenizer;

use parser::parse_expr;
use tokenizer::tokenize;

fn main() {
    const CODE: &str = "#comment!!\nn";
    let tokens = tokenize(CODE).unwrap_or_else(|err| err.log_and_exit(CODE));
    let expr = parse_expr(&mut tokens.iter(), &vec![], &Default::default())
        .unwrap_or_else(|err| err.log_and_exit(CODE));

    println!("{:?}", expr,);
}
