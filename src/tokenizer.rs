use crate::{
    error::{Error, Result},
    span::Span,
};

#[derive(Clone, Debug)]
pub enum Token<'a> {
    Decl(&'a str, Span),
    Name(&'a str, Span),
    Num(u64, Span), // Only natural number support for now
    Arrow(Span),
}

impl Token<'_> {
    pub fn span(&self) -> Span {
        match self {
            Self::Decl(_, s) | Self::Name(_, s) | Self::Num(_, s) | Self::Arrow(s) => s.clone(),
        }
    }

    pub fn kind(&self) -> TokenKind {
        TokenKind::from(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Decl,
    Name,
    Num,
    Arrow,
}

impl From<&Token<'_>> for TokenKind {
    fn from(token: &Token) -> Self {
        match token {
            Token::Decl(_, _) => Self::Decl,
            Token::Name(_, _) => Self::Name,
            Token::Num(_, _) => Self::Num,
            Token::Arrow(_) => Self::Arrow,
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Decl => "declaration",
                Self::Name => "name",
                Self::Num => "string",
                Self::Arrow => "<arrow>",
            }
        )
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
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
                    .map(|n| Token::Num(n, i..end))
                    .map_err(|_| Error::Spanned("invalid number literal".into(), i..end))?
            }

            ' ' | '\t' | '\n' | '\r' => continue,

            '-' => {
                match chars.peek() {
                    Some((_, '>')) => {
                        let _ = chars.next();
                        Token::Arrow(i..i + 2)
                    }
                    Some((_, ' ' | '\t' | '\n' | '\r')) => {
                        let _ = chars.next();
                        Token::Name("-", i..i + 1)
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

                while chars.next_if(|(_, next)| !next.is_whitespace()).is_some() {
                    end += 1;
                }
                
                if c == '\\' {
                    Token::Decl(&src[i + 1..end], i + 1..end)
                } else {
                    Token::Name(&src[i..end], i..end)
                }
            }
        });
    }

    Ok(tokens)
}
