use core::fmt;
use std::any;
use std::collections::{BTreeMap, HashMap};
use std::io::{self, Write};
use std::iter::Inspect;
use std::ops::RangeInclusive;
use std::str::FromStr;

use colored::Colorize;

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

pub fn get_input_map<const N: usize, T: Clone>(prompt: &str, mapping: [(&str, T); N]) -> T {
    let options_str = mapping
        .iter()
        .map(|t| t.0)
        .fold(String::new(), |mut acc, curr| {
            let is_first_iter = acc.is_empty();
            acc.push_str(&format!("{}'{curr}'", if is_first_iter {""} else {", "} ));
            acc
        });
    let expected_str = format!("jedno z: {options_str}");

    let map = HashMap::<&str, T>::from(mapping);

    get_input(prompt, Some(&expected_str), |user_input| {
        map.get(user_input).cloned()
    })
}

pub fn wait_for_enter(prompt: &str) {
    get_input(prompt, Some("enter"), |_| Some(()))
}

pub fn get_number<Num: FromStr>(question: &str) -> Num {
    get_input(question, Some("číslo"), |input| input.parse::<Num>().ok())
}

pub fn get_input<Res, Parser: Fn(&str) -> Option<Res>>(
    question: &str,
    expected: Option<&str>,
    parser_fn: Parser,
) -> Res {
    println!("{question}");
    if let Some(expected) = expected {
        print!("{}", format!("(očekávám {expected}): ").bright_black());
    }
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Něco se pokazilo :(");

    let input = line.trim();
    parser_fn(input).unwrap_or_else(|| get_input(question, expected, parser_fn))
}
