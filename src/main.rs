mod env;
mod error;
mod interpreter;
mod parser;
mod span;
mod tokenizer;

use env::{default_env, FunctionBody};
use error::Error;
use rustyline::Editor;
use std::env::args;

use parser::{parse_expr, parse_file};
use tokenizer::tokenize;

use crate::error::Result;
use crate::{env::Environment, interpreter::Value};

pub fn repl() -> rustyline::Result<()> {
    let env = env::default_env();
    let mut editor = Editor::<()>::new()?;

    println!("REPL: {} functions loaded", env.len());

    fn run(line: &str, env: &Environment) -> Result<Value> {
        let tokens = tokenize(line)?;
        let ast = parse_expr(&mut tokens.iter(), &vec![], env)?;
        interpreter::eval(&ast, env)
    }

    while let Ok(line) = editor.readline(">> ") {
        let run = run(&line, &env);
        match run {
            Ok(run) => println!("{:?}", run),
            Err(err) => err.log(&line),
        }
    }

    Ok(())
}

fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let file_name = args().nth(1).unwrap_or_default();

    if file_name.is_empty() {
        repl()?;
    } else {
        let file = std::fs::read_to_string(&file_name)
            .map_err(|_| Error::General(format!("could not load file {}", file_name)))
            .unwrap_or_else(|err| err.log_and_exit(""));
        
        let tokens = tokenize(&file).unwrap_or_else(|err| err.log_and_exit(&file));
        let mut env = env::default_env();
        parse_file(&tokens, &mut env).unwrap_or_else(|err| err.log_and_exit(&file));
        
        let main = env.get("main").unwrap_or_else(|| Error::General("no main function found in file".into()).log_and_exit(&file));

        match main.body() {
            FunctionBody::Normal(expr) => {
                interpreter::eval(expr, &env).unwrap_or_else(|err| err.log_and_exit(&file));
            },
            FunctionBody::System(_) | FunctionBody::LazySystem(_) => unreachable!()
        }

        
    }

    Ok(())
}
