use std::fmt;
use crate::{
    env::{Environment, FunctionBody},
    error::{Error, Result},
    parser::Expression,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(u64),
    Bool(bool),
    Nothing
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{}", n),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nothing => write!(f, "none")
        }?;
        Ok(())
    }
}

fn eval_(expr: &Expression, env: &Environment, args: &Vec<Value>) -> Result<Value> {
    match expr {
        Expression::App(name, params) => {
            let func = env.get(name).unwrap();

            let eager_eval = || {
                params
                    .iter()
                    .map(|e| eval_(e, env, args))
                    .collect::<Result<Vec<_>>>()
            };

            match func.body() {
                FunctionBody::Normal(expr) => eval_(expr, env, &eager_eval()?),
                FunctionBody::System(func) => func(&eager_eval()?),
                FunctionBody::LazySystem(func) => func(params, eval_, env, args),
            }
        }
        Expression::Arg(idx) => Ok(args[*idx].clone()),
        Expression::Literal(value) => Ok(value.clone()),
        Expression::Temp => Err(Error::General(
            "attemped to evaluate temp expr: this is a BUG".into(),
        )),
    }
}

pub fn eval(expr: &Expression, env: &Environment) -> Result<Value> {
    eval_(expr, env, &vec![])
}
