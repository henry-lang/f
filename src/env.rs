use std::collections::HashMap;

use lasso::{Rodeo, Spur};

use crate::{
    error::{Error, Result},
    interpreter::{Value, ValueKind},
    parser::Expression,
};

pub type Symbol = Spur;

pub trait IntoSymbol: Sized {
    fn into_symbol(self, env: &Environment) -> Option<Symbol>;
}

impl IntoSymbol for &str {
    fn into_symbol(self, env: &Environment) -> Option<Symbol> {
        env.get_symbol(self)
    }
}

impl IntoSymbol for Symbol {
    fn into_symbol(self, _: &Environment) -> Option<Symbol> {
        Some(self)
    }
}

pub struct Environment {
    symbol_store: Rodeo<Symbol>,
    funcs: HashMap<Symbol, Function>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            symbol_store: Rodeo::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn insert_function(&mut self, name: &str, func: Function) {
        self.funcs
            .insert(self.symbol_store.get_or_intern(name), func);
    }

    pub fn get_symbol(&self, name: &str) -> Option<Symbol> {
        self.symbol_store.get(name)
    }

    pub fn get_function<I: IntoSymbol>(&self, name: I) -> Option<&Function> {
        let symbol = name.into_symbol(self)?;
        self.funcs.get(&symbol)
    }

    pub fn get_entry(&self, name: &str) -> Option<(Symbol, &Function)> {
        let symbol = self.symbol_store.get(name);
        let func = symbol.and_then(|s| self.funcs.get(&s));

        match (symbol, func) {
            (Some(symbol), Some(func)) => Some((symbol, func)),
            _ => None,
        }
    }

    pub fn size(&self) -> usize {
        self.funcs.len()
    }
}

pub struct Function {
    args: usize,
    body: FunctionBody,
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("args", &self.args)
            .finish()
    }
}

impl<'a> Function {
    pub fn new(args: usize, body: impl Into<FunctionBody>) -> Self {
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

pub enum FunctionBody {
    Normal(Expression),
    System(SystemFunction),
    LazySystem(LazySystemFunction),
}

impl From<Expression> for FunctionBody {
    fn from(expr: Expression) -> Self {
        Self::Normal(expr)
    }
}

impl From<SystemFunction> for FunctionBody {
    fn from(func: SystemFunction) -> Self {
        Self::System(func)
    }
}

impl From<LazySystemFunction> for FunctionBody {
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

macro_rules! extract_args {
    ($params:expr,$($variant:ident),+) => {{
        let mut counter = 0;
        let mut next = || {
            counter += 1;
            counter
        };
        (
            $(
                if let Value::$variant(x) = $params[next() - 1].clone() { x } else {
                    return Err(Error::General(format!("wrong argument type for index {}", counter)));
                },
            )+
        )
    }};
}

macro_rules! default_env {
    ($(($t:ident,$name:literal,$num_args:literal,$func:tt)),+) => {
        pub fn default_env() -> Environment {
            let mut env = Environment::new();
            $(
                #[allow(unused_parens)]
                let func: $t = $func;
                env.insert_function($name, Function::new($num_args, func));
            )+
            env
        }
    };
}

#[rustfmt::skip]
default_env![
    (SystemFunction, "print", 1, (|args| {
        println!("{}", args[0]);
        Ok(Value::Nothing)
    })),
    (SystemFunction, "true", 0, (|_| {
        Ok(Value::Bool(true))
    })),
    (SystemFunction, "false", 0, (|_| {
       Ok(Value::Bool(false))
    })),
    (SystemFunction, "+", 2, (|args| {
        match (&args[0], &args[1]) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a.to_owned() + b)),
            (a, b) => Err(Error::General(format!("types {} and {} cannot be added together", ValueKind::from(a), ValueKind::from(b))))
        }
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
    (SystemFunction, "%", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Num(lhs % rhs))
    })),
    (SystemFunction, "<", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Bool(lhs < rhs))
    })),
    (SystemFunction, ">", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Bool(lhs > rhs))
    })),
    (LazySystemFunction, "if", 3, (|params, eval, env, args| {
        let pred = extract_args!(&[eval(&params[0], env, &args)?], Bool);

        Ok(if pred.0 {
            eval(&params[1], env, &args)?
        } else {
            eval(&params[2], env, &args)?
        })
    })),
    (SystemFunction, "=", 2, (|args| {
        let (lhs, rhs) = extract_args!(args, Num, Num);

        Ok(Value::Bool(lhs == rhs))
    })),
    (SystemFunction, "none", 0, (|_| {
        Ok(Value::Nothing)
    })),
    (SystemFunction, "pair", 2, (|args| {
        Ok(Value::List(Vec::from([args[0].clone(), args[1].clone()])))
    })),
    (SystemFunction, "head", 1, (|args| {
        let list = extract_args!(args, List).0;
        Ok(list[0].clone())
    })),
    (SystemFunction, "tail", 1, (|args| {
        let list = extract_args!(args, List).0;
        Ok(Value::List(list[1..].iter().cloned().collect()))
    })),
    (SystemFunction, "fuse", 2, (|args| {
        match (args[0].clone(), args[1].clone()) {
            (Value::List(mut x), Value::List(mut y)) => Ok(Value::List({ x.append(&mut y); x })),
            (Value::List(mut x), y) => Ok(Value::List({ x.push(y); x })),
            (x, Value::List(mut y)) => Ok(Value::List({ let mut v = vec![x]; v.append(&mut y); v })),
            (x, y) => Ok(Value::List(vec![x, y])),
        }
    }))
];
