use crate::{
    env::{Environment, Function, Symbol},
    error::{Error, Result},
    interpreter::Value,
    tokenizer::{Token, TokenKind},
};

#[derive(Debug)]
pub enum Expression {
    App(Symbol, Vec<Expression>), // Maybe use arena allocator for better cache locality
    Arg(usize),
    Literal(Value),
    Temp,
}

pub fn parse_file<'a>(tokens: &[Token], env: &mut Environment) -> Result<()> {
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        let Token::Decl(name, _) = token else {
            Err(Error::Spanned(
                format!("expected declaration found {}", token.kind()),
                token.span(),
            ))?
        };

        let mut args = vec![];
        while let Some(Token::Name(name, _)) = tokens.next_if(|t| t.kind() == TokenKind::Name) {
            args.push(*name);
        }

        let next = tokens.next();
        // TODO: Rewrite this
        if next.is_none() {
            return Err(Error::General("expected arrow, found <eof>".into()));
        } else if next.unwrap().kind() != TokenKind::Arrow {
            return Err(Error::Spanned(
                format!("expected arrow, found {}", next.unwrap().kind()),
                next.unwrap().span(),
            ));
        } else {
            // TODO: make this better
            env.insert_function(name, Function::new(args.len(), Expression::Temp));
            let expr = parse_expr(&mut tokens, &args, env)?;
            env.insert_function(name, Function::new(args.len(), expr));
        }
    }

    Ok(())
}

pub fn parse_expr<'a>(
    tokens: &mut impl Iterator<Item = &'a Token<'a>>,
    args: &Vec<&str>,
    env: &Environment,
) -> Result<Expression> {
    let expr = match tokens
        .next()
        .ok_or_else(|| Error::General("expected expression, found <eof>".into()))?
    {
        Token::Name(name, span) => {
            if let Some(idx) = args.iter().position(|&a| a == *name) {
                Expression::Arg(idx)
            } else if let Some((symbol, func)) = env.get_entry(name) {
                let mut app_args = Vec::with_capacity(func.args());
                for _ in 0..func.args() {
                    app_args.push(parse_expr(tokens, args, env)?);
                }

                Expression::App(symbol, app_args)
            } else {
                Err(Error::Spanned(
                    format!("cannot find function or local {name}"),
                    span.clone(),
                ))?
            }
        }

        Token::Num(num, _) => Expression::Literal(Value::Num(*num)),
        Token::String(str, _) => Expression::Literal(Value::String(str.to_string())),
        token => Err(Error::Spanned(
            format!("unexpected token {}", token.kind()),
            token.span(),
        ))?,
    };
    Ok(expr)
}
