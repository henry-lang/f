use crate::{
    error::{Error, Result},
    span::{Span, Spanned},
};

#[derive(Clone, Debug)]
pub enum Token<'a> {
    Name(&'a str),
    Num(u64), // Only natural number support for now
    Arrow,
}

impl Token<'_> {
    pub fn kind(&self) -> TokenKind {
        TokenKind::from(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Name,
    Num,
    Arrow,
}

impl From<&Token<'_>> for TokenKind {
    fn from(token: &Token) -> Self {
        match token {
            Token::Name(_) => Self::Name,
            Token::Num(_) => Self::Num,
            Token::Arrow => Self::Arrow,
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Name => "name",
                Self::Num => "string",
                Self::Arrow => "<arrow>",
            }
        )
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Spanned<Token>>> {
    let mut tokens = vec![];
    let mut chars = src.chars().enumerate().peekable();

    while let Some((i, c)) = chars.next() {
        tokens.push(match c {
            '0'..='9' => {
                let mut end = i + 1;
                while chars.next_if(|(_, next)| !next.is_whitespace()).is_some() {
                    end += 1;
                }

                src[i..end]
                    .parse::<u64>()
                    .map(|n| Spanned::new(Token::Num(n), Span(i, end)))
                    .or_else(|_| {
                        Err(Error::Spanned(
                            "invalid number literal".into(),
                            Span(i, end),
                        ))
                    })?
            }

            ' ' | '\t' | '\n' | '\r' => continue,

            '-' => {
                match chars.peek() {
                    Some((_, '>')) => {
                        let _ = chars.next();
                        Spanned::new(Token::Arrow, Span(i, i + 2))
                    }
                    Some((_, ' ' | '\t' | '\n' | '\r')) => {
                        let _ = chars.next();
                        Spanned::new(Token::Name("-"), Span(i, i + 1))
                    }
                    _ => panic!(), // Bad but whatever
                }
            }

            '#' => {
                while chars.next_if(|&(_, c)| c != '\n').is_some() {}
                continue;
            }

            _ => {
                let mut end = i + 1;

                while let Some(_) = chars.next_if(|(_, next)| !next.is_whitespace()) {
                    end += 1;
                }

                Spanned::new(Token::Name(&src[i..end]), Span(i, end))
            }
        });
    }

    Ok(tokens)
}
