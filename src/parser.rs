use crate::{
    error::{Error, Result},
    span::Spanned,
    tokenizer::Token, env::Function,
};
use std::{collections::HashMap, iter::Enumerate};

#[derive(Debug)]
pub enum Expression<'a> {
    App(&'a str, Vec<Expression<'a>>), // Maybe use arena allocator for better cache locality
    Arg(usize),
    Num(u64),
}

pub fn parse_expr<'a>(
    tokens: &mut impl Iterator<Item = &'a Spanned<Token<'a>>>,
    args: &Vec<&'a str>,
    funcs: &HashMap<&str, Function<'a>>,
) -> Result<Expression<'a>> {
    Ok(
        match tokens.next().ok_or(Error::General(
            "expected expression, found <eof>".into(),
        ))? {
            &Spanned {
                value: Token::Name(name),
                span,
            } => {
                if let Some(idx) = args.iter().position(|&a| a == name) {
                    Expression::Arg(idx)
                } else if let Some(func) = funcs.get(name) {
                    let mut app_args = Vec::with_capacity(func.args());
                    for _ in 0..func.args() {
                        app_args.push(parse_expr(tokens, args, funcs)?);
                    }

                    Expression::App(name, app_args)
                } else {
                    Err(Error::Spanned(
                        format!("cannot find function or local {}", name).into(),
                        span,
                    ))?
                }
            }
            &Spanned {
                value: Token::Num(num),
                ..
            } => Expression::Num(num),
            &Spanned { value: token, span } => Err(Error::Spanned(
                format!("unexpected token {}", token.kind()).into(),
                span,
            ))?,
        },
    )
}
