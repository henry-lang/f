use ansi_term::Color::{Blue, Red};

use crate::span::Span;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    General(String),
    Spanned(String, Span),
}

impl Error {
    pub fn message(&self) -> &String {
        match self {
            Self::General(msg) | Self::Spanned(msg, _) => msg,
        }
    }

    pub fn log(&self, file: &str) {
        println!("{}: {}", Red.bold().paint("error"), self.message());

        if let Self::Spanned(_, span) = self {
            let line_num = file[..span.start].chars().filter(|x| *x == '\n').count();
            let offset = file[..span.start]
                .chars()
                .rev()
                .enumerate()
                .find(|(_, c)| *c == '\n')
                .unwrap_or((span.start, '\n'))
                .0;
            let padding = ((((line_num as f64).log10()) as usize) + 4) + offset;

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
    }

    pub fn log_and_exit(&self, file: &str) -> ! {
        self.log(file);
        std::process::exit(1)
    }
}
