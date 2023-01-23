use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    interpreter::Value,
    parser::Expression,
};

pub struct Function<'a> {
    args: usize,
    body: FunctionBody<'a>,
}

impl std::fmt::Debug for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("args", &self.args)
            .finish()
    }
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

    pub fn body(&self) -> &FunctionBody {
        &self.body
    }
}

pub enum FunctionBody<'a> {
    Normal(Expression<'a>),
    System(SystemFunction),
    LazySystem(LazySystemFunction),
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

impl From<LazySystemFunction> for FunctionBody<'_> {
    fn from(func: LazySystemFunction) -> Self {
        Self::LazySystem(func)
    }
}

pub type SystemFunction = fn(&[Value]) -> Result<Value>;
pub type LazySystemFunction = fn(
    &[Expression],
    fn(&Expression, &Environment, &Vec<Value>) -> Result<Value>,
    &Environment,
    &Vec<Value>,
) -> Result<Value>;
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
    ($(($t:ident,$name:literal,$num_args:literal,$func:tt)),+) => {
        pub fn default_env<'a>() -> Environment<'a> {
            let mut env = Environment::new();
            $(
                #[allow(unused_parens)]
                let func: $t = $func;
                env.insert($name, Function::new($num_args, func));
            )+
            env
        }
    };
}

#[rustfmt::skip]
default_env![
    (SystemFunction, "+", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Num(lhs + rhs))
    })),
    (SystemFunction, "-", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Num(lhs - rhs))
    })),
    (SystemFunction, "*", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Num(lhs * rhs))
    })),
    (SystemFunction, "/", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Num(lhs / rhs))
    })),
    (LazySystemFunction, "if", 3, (|params, eval, env, args| {
        let pred = extract_args!(&[eval(&params[0], env, args)?], Bool);

        Ok(if pred.0 {
            eval(&params[1], env, args)?
        } else {
            eval(&params[2], env, args)?
        })
    })),
    (SystemFunction, "=", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Bool(lhs == rhs))
    }))
];
