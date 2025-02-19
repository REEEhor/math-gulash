use core::fmt;
use std::any;
use std::io::{self, Write};
use std::ops::RangeInclusive;
use std::str::FromStr;

pub type Result<T> = anyhow::Result<T>;

pub trait InOut {
    fn read_line(&mut self) -> Result<String>;
    fn write_line(&mut self, text: &str);
}

impl dyn InOut {
    fn wait_for_enter(&mut self, prompt: &str) {
        self.write_line(prompt);
        self.read_line();
    }
}

pub fn wait_for_enter(prompt: &str) {
    get_input(prompt, Some("stiskni enter"), |_| Some(()))
}

pub fn get_number<Num: FromStr>(question: &str) -> Num {
    get_input(question, Some("číslo"), |input| input.parse::<Num>().ok())
}

pub fn get_input<Res, Parser: Fn(&str) -> Option<Res>>(
    question: &str,
    expected: Option<&str>,
    parser_fn: Parser,
) -> Res {
    println!("Prompt: {question}");
    if let Some(expected) = expected {
        print!("(Očekávám {expected}): ");
    }
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Něco se pokazilo :(");

    let input = line.trim();
    parser_fn(input).unwrap_or_else(|| get_input(question, expected, parser_fn))
}
