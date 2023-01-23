use crate::{
    env::{Environment, FunctionBody},
    error::{Error, Result},
    parser::Expression,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(u64),
    Bool(bool),
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
        Expression::Temp => Err(Error::General("tried to evaluate temp expr".into())),
    }
}

pub fn eval(expr: &Expression, env: &Environment) -> Result<Value> {
    eval_(expr, env, &vec![])
}
