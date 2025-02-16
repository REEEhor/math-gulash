use core::fmt;
use std::io::{self, Write};
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn get_number_in_range<Num: fmt::Display + FromStr + PartialOrd>(
    question: &str,
    range: RangeInclusive<Num>,
) -> Num {
    let expected_str = format!("číslo {}-{}", range.start(), range.end());
    let parser_fn = |input: &str| match input.parse::<Num>() {
        Ok(number) => {
            if range.contains(&number) {
                println!(
                    "Číslo {number} není od {} do {}",
                    range.start(),
                    range.end()
                );
                Some(number)
            } else {
                None
            }
        }
        Err(_) => todo!(),
    };
    get_input(question, Some(&expected_str), parser_fn)
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
