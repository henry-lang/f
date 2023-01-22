use crate::{
    env::{Environment, Function},
    error::{Error, Result},
    interpreter::Value,
    span::Spanned,
    tokenizer::{Token, TokenKind},
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression<'a> {
    App(&'a str, Vec<Expression<'a>>), // Maybe use arena allocator for better cache locality
    Arg(usize),
    Literal(Value),
}

// A file is basically just a sequence of functions which are loaded into the REPL.
pub fn parse_file(tokens: &[Spanned<Token>], env: &mut Environment) -> Result<()> {
    let mut tokens = tokens.iter();

    while let Some(Spanned { value: token, span }) = tokens.next() {
        let Token::Name(name) = token else {
            Err(Error::Spanned(format!("expected name found {}", token.kind()).into(), *span))?
        };

        
    }

    Ok(())
}

pub fn parse_expr<'a>(
    tokens: &mut impl Iterator<Item = &'a Spanned<Token<'a>>>,
    args: &Vec<&str>,
    funcs: &HashMap<&str, Function>,
) -> Result<Expression<'a>> {
    Ok(
        match tokens
            .next()
            .ok_or(Error::General("expected expression, found <eof>".into()))?
        {
            Spanned {
                value: Token::Name(name),
                span,
            } => {
                if let Some(idx) = args.iter().position(|a| a == name) {
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
                        *span,
                    ))?
                }
            }
            Spanned {
                value: Token::Num(num),
                ..
            } => Expression::Literal(Value::Num(*num)),
            Spanned { value: token, span } => Err(Error::Spanned(
                format!("unexpected token {}", token.kind()).into(),
                *span,
            ))?,
        },
    )
}
