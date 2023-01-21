use std::collections::HashMap;

use crate::{parser::Expression, interpreter::Value};

pub struct Function<'a> {
    args: usize,
    body: FunctionBody<'a>
}

impl Function<'_> {
    pub fn new<'a>(args: usize, body: impl Into<FunctionBody<'a>>) -> Self {
        Self {args, body: body.into()}
    }

    pub fn args(&self) -> usize {
        self.args
    }
}

enum FunctionBody<'a> {
    Normal(Expression<'a>),
    System(SystemFunction),
}

impl From<Expression<'_>> for FunctionBody<'_> {
    fn from(expr: Expression<'_>) -> Self {
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

fn default_env<'a>() -> Environment<'a> {
    let mut env = HashMap::new();

    env.insert("+", Function::new(2, |add| {
        Value::Num(0) // Temporary
    }));

    env
}