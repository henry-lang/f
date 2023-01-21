use crate::span::Span;
use ansi_term::Color::{Blue, Red};
use std::borrow::Cow;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    General(Cow<'static, str>),
    Spanned(Cow<'static, str>, Span),
}

impl Error {
    pub fn message(&self) -> &Cow<'static, str> {
        match self {
            Self::General(msg) | Self::Spanned(msg, _) => msg,
        }
    }

    pub fn log_and_exit(&self, file: &str) -> ! {
        println!("{}: {}", Red.bold().paint("error"), self.message().as_ref());

        if let Self::Spanned(_, span) = self {
            let line_num = file[..span.0].chars().filter(|x| *x == '\n').count();
            let offset = file[..span.0]
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == '\n')
                .next()
                .unwrap_or((span.0, '\n'))
                .0;
            let padding = (line_num.checked_ilog10().unwrap_or(0) + 4) as usize + offset;

            println!(
                "{} {} {}",
                Blue.bold().paint((line_num + 1).to_string()),
                Blue.bold().paint("|"),
                file.lines().nth(line_num).unwrap()
            );
            println!(
                "{}{}",
                " ".repeat(padding),
                Red.bold().paint("^".repeat(span.len()))
            );
        }

        std::process::exit(1)
    }
}
