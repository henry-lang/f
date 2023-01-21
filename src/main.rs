#![feature(type_alias_impl_trait)]
#![feature(anonymous_lifetime_in_impl_trait)]

mod env;
mod interpreter;
mod error;
mod parser;
mod span;
mod tokenizer;

use parser::parse_expr;
use tokenizer::tokenize;

fn main() {
    const CODE: &str = "+ + + 3 4 5 4";
    let env = env::default_env();
    let tokens = tokenize(CODE).unwrap_or_else(|err| err.log_and_exit(CODE));
    let expr = parse_expr(&mut tokens.iter(), &vec![], &env)
        .unwrap_or_else(|err| err.log_and_exit(CODE));

    println!("{:?}", expr,);
}
