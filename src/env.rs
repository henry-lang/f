use std::collections::HashMap;

use crate::{parser::Expression, interpreter::Value};

pub struct Function<'a> {
    args: usize,
    body: FunctionBody<'a>
}

impl<'a> Function<'a> {
    pub fn new(args: usize, body: impl Into<FunctionBody<'a>>) -> Self {
        Self {args, body: body.into()}
    }

    pub fn args(&self) -> usize {
        self.args
    }
}

pub enum FunctionBody<'a> {
    Normal(Expression<'a>),
    System(SystemFunction),
}

impl<'a> From<Expression<'a>> for FunctionBody<'a> {
    fn from(expr: Expression<'a>) -> Self {
        Self::Normal(expr)
    }
}

impl From<SystemFunction> for FunctionBody<'_> {
    fn from(func: SystemFunction) -> Self {
        Self::System(func)
    }
}

type SystemFunction = fn(Vec<Expression>) -> Value;
type Environment<'a> = HashMap<&'a str, Function<'a>>;


macro_rules! default_env {
    ($(($n:literal,$a:literal,$func:tt)),*) => {
        pub fn default_env<'a>() -> Environment<'a> {
            let mut env = HashMap::new();
            $(
                #[allow(unused_parens)]
                let func: SystemFunction = $func; 
                env.insert($n, Function::new($a, func))
            ),*;
            env
        }
    };
}

default_env! [
    ("+", 2, (|_| {
        Value::Num(100)
    }))
];