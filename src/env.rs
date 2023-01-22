use std::collections::HashMap;

use crate::{interpreter::Value, parser::Expression, error::{Error, Result}};

pub struct Function<'a> {
    args: usize,
    body: FunctionBody<'a>,
}

impl<'a> Function<'a> {
    pub fn new(args: usize, body: impl Into<FunctionBody<'a>>) -> Self {
        Self {
            args,
            body: body.into(),
        }
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

pub type SystemFunction = fn(Vec<Value>) -> Result<Value>;
pub type Environment<'a> = HashMap<&'a str, Function<'a>>;

macro_rules! extract_args {
    ($params:expr,$($variant:ident),+) => {{
        let mut counter = 0;
        let mut next = || {
            counter += 1;
            counter
        };
        (
            $(
                if let Value::$variant(x) = $params[next() - 1] { x } else {
                    return Err(Error::General(format!("wrong argument type for index {}", counter).into()));
                },
            )+
        )
    }};
}

macro_rules! default_env {
    ($(($n:literal,$a:literal,$func:tt)),+) => {
        pub fn default_env<'a>() -> Environment<'a> {
            let mut env = Environment::new();
            $(
                #[allow(unused_parens)]
                let func: SystemFunction = $func;
                env.insert($n, Function::new($a, func));
            )+
            env
        }
    };
}

default_env![
    ("+", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Num(lhs + rhs))
    })),
    ("-", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Num(lhs - rhs))
    }))
];
