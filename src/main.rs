mod env;
mod error;
mod interpreter;
mod parser;
mod span;
mod tokenizer;

use std::io::Write;

use parser::{parse_expr, parse_file};
use tokenizer::tokenize;

use crate::interpreter::eval;

fn main() {
    let mut env = env::default_env();
    let tokens = tokenize("fac n -> if = n 0 1 * n fac - n 1\nfib n -> if = n 0 1 if = n 1 1 + fib - n 1 fib - n 2").unwrap();
    println!("{tokens:?}");
    parse_file(&tokens, &mut env)
        .unwrap_or_else(|err| err.log_and_exit("fac n -> if = n 0 1 * n fac - n 1"));
    println!("{env:?}");
    let mut code = String::new();
    loop {
        print!("> ");
        code.clear();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut code).unwrap();
        let tokens = tokenize(&code);
        match tokens {
            Ok(tokens) => {
                let expr = parse_expr(&mut tokens.iter(), &vec![], &env);
                match expr {
                    Ok(expr) => {
                        println!("{expr:?}");
                        let start = std::time::Instant::now();
                        let eval = eval(&expr, &env);
                        match eval {
                            Ok(eval) => {
                                let ns = start.elapsed().as_nanos();
                                println!("time: {ns}ns");
                                println!("{eval:?}");
                            }
                            Err(err) => {
                                err.log(&code);
                                continue;
                            }
                        }
                    }
                    Err(err) => {
                        err.log(&code);
                        continue;
                    }
                }
            }
            Err(err) => {
                err.log(&code);
                continue;
            }
        }
    }
}
