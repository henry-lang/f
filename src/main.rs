mod env;
mod error;
mod interpreter;
mod parser;
mod span;
mod tokenizer;

use env::FunctionBody;
use error::{Error, UnwrapPretty};
use rustyline::Editor;
use std::{env::args, path::Path};

use parser::{parse_expr, parse_file};
use tokenizer::tokenize;

use crate::{env::Environment, error::Result, interpreter::Value};

fn load_file<P: AsRef<Path>>(
    path: P,
    env: &mut Environment,
) -> core::result::Result<(), (Error, String)> {
    let file = std::fs::read_to_string(&path).map_err(|_| {
        (
            Error::General(format!(
                "could not load file {}",
                path.as_ref().to_str().unwrap()
            )),
            "".to_string(),
        )
    })?;

    let tokens = tokenize(&file).map_err(|e| (e, file.clone()))?;
    parse_file(&tokens, env).map_err(|e| (e, file))?;
    Ok(())
}

pub fn repl() -> rustyline::Result<()> {
    let mut env = env::default_env();
    let mut editor = Editor::<()>::new()?;

    println!("repl: {} functions loaded", env.size());

    fn run(line: &str, env: &mut Environment) -> Result<Value> {
        let tokens = tokenize(line)?;
        let ast = parse_expr(&mut tokens.iter(), &vec![], env)?;
        interpreter::eval(&ast, env)
    }

    while let Ok(line) = editor.readline(">> ") {
        editor.add_history_entry(line.clone());
        if line.trim() == ":exit" {
            std::process::exit(1);
        }
        if line.starts_with(":load ") {
            let (_, path) = line.split_once(":load ").unwrap();
            load_file(path, &mut env).unwrap_or_else(|(err, file)| err.log(&file))
        } else {
            let run = run(&line, &mut env);
            match run {
                Ok(run) => println!("{}", run),
                Err(err) => err.log(&line),
            }
        }
    }

    Ok(())
}

fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let path = args().nth(1).unwrap_or_default();

    if path.is_empty() {
        repl()?;
    } else {
        let mut env = env::default_env();

        load_file(&path, &mut env).unwrap_or_else(|(err, file)| {
            err.log(&file);
            std::process::exit(1);
        });

        let main = env.get_function("main").unwrap_or_else(|| {
            Err(Error::General("no main function found in file".into())).unwrap_pretty(&path)
        });

        match main.body() {
            FunctionBody::Normal(expr) => {
                interpreter::eval(&expr, &env).unwrap_pretty(&path);
            }
            FunctionBody::System(_) | FunctionBody::LazySystem(_) => unreachable!(),
        }
    }

    Ok(())
}
