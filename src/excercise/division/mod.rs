use colored::*;
use rand::rngs::StdRng;
use rand::Rng;
use std::collections::VecDeque;

use crate::input::*;

/*
 * Represents the excercise:
 *      `base` : `divisor` = `result`
 */
pub struct Division {
    divisor: u64,
    result: u64,
}

impl Division {
    fn base(&self) -> u64 {
        self.divisor * self.result
    }
}

pub fn generate_division(rng: &mut StdRng) -> Division {
    Division {
        result: rng.random_range(25..99999999),
        divisor: rng.random_range(3..9),
    }
}

struct RemainderCalculation {
    subtractor: u32,
    bottom: u32,
}

struct SolutionState {
    assignment: Division,
    base_idx: u32,
    growing_result: Option<u32>,
    guess: Option<Digit>,
    remainders: Vec<RemainderCalculation>,
}

impl SolutionState {
    fn new(assignment: Division) -> Self {
        SolutionState {
            assignment,
            base_idx: 0,
            growing_result: None,
            guess: None,
            remainders: vec![],
        }
    }
}

enum PartOfSolution {
    PickBaseIndex { choices_count: u32 },
    GuessResult,
    CalculateMultiplication { progress: RightToLeftProgress },
    CalculateDifference { progress: RightToLeftProgress },
    Compare,
    BringDigitDown,
}

fn placeholder() -> ColoredString {
    "?".bold().cyan()
}

type Digit = u32;

struct RightToLeftProgress {
    content: VecDeque<Digit>,
}

impl RightToLeftProgress {
    fn new() -> Self {
        Self {
            content: VecDeque::new(),
        }
    }

    fn current_value(&self) -> Option<u32> {
        if self.content.is_empty() {
            return None;
        }
        let mut result = 0;

        for digit in self.content.iter().rev() {
            result *= 10;
            result += digit;
        }

        Some(result)
    }
}

fn print_state(in_progress: &PartOfSolution, state: &SolutionState) {
    use PartOfSolution as P;

    if let P::PickBaseIndex { choices_count } = in_progress {
        print!(" ");
        for _ in 0..*choices_count {
            print!("{}", placeholder())
        }
        println!()
    }

    print!(
        " {} : {} = ",
        state.assignment.base(),
        state.assignment.divisor,
    );
    if let Some(res) = state.growing_result {
        print!("{res}")
    }
    if let Some(guess) = state.guess {
        print!("{guess}");
    }
    if let P::GuessResult = in_progress {
        print!("{}", placeholder());
    }
    println!();

    let mut last_idx = 0;
    for (idx, remainder_calc) in state.remainders.iter().enumerate() {
        last_idx = idx;

        print!("{}", " ".repeat(idx));
        print!("-{}", remainder_calc.subtractor);
        println!();
        print!("{}", " ".repeat(idx + 1));
        print!("___");
        println!();
        print!("{}", " ".repeat(idx + 1));
        print!("{}", remainder_calc.bottom);
        println!();
    }
    last_idx += 1;
    let repeat_count = |prog: &RightToLeftProgress| last_idx - prog.content.len() - 1;

    if let P::CalculateMultiplication { progress } = in_progress {
        print!("{}-", " ".repeat(repeat_count(progress)));
        for digit in progress.content.iter() {
            print!("{}", char::from_digit(*digit, 10).unwrap())
        }
        print!("{}", placeholder());
        println!();
        print!("{}", " ".repeat(repeat_count(progress)));
        print!("___");
        println!();
    }

    if let P::CalculateDifference { progress } = in_progress {
        print!("{}", " ".repeat(repeat_count(progress)));
        print!("{}", placeholder());
        for digit in progress.content.iter() {
            print!("{}", char::from_digit(*digit, 10).unwrap())
        }
    }

    if let P::BringDigitDown = in_progress {
        print!("{}", placeholder());
    }
    println!();

    if let P::Compare = in_progress {
        // Do nothing
    }
}

/*
=======================
GuessResult
    53023 : 26 = ?

=======================
CalculareMultiplication
    53023 : 26 = 1
   - ?  (+ cycle)
   ___

=======================
CalculateDifference
    53023 : 26 = 1
   -26
   ___
    ?   (+ cycle)

=======================
Compare
    53023 : 26 = 1
   -26
   ___
    27      [Question what now ?]
=======================
BringDigitDown
    53023 : 26 = 2
   -52
   ____
     1?
=======================
... and so on

*/
pub fn solve_excercise(excercise: Division) {
    let mut state = SolutionState::new(excercise);

    // print_state(&PartOfSolution::PickBaseIndex, &state);

    print_state(&PartOfSolution::GuessResult, &state);

    let dividee = &format!("{}", state.assignment.base())[..=state.base_idx as usize];
    let question = format!(
        "Zadej kolikrát si myslíš, že se vejde {} do {}",
        state.assignment.divisor, dividee
    );
    let digit_guess = get_digit(&question);

    state.guess = Some(digit_guess);

    print_state(
        &PartOfSolution::CalculateMultiplication {
            progress: RightToLeftProgress::new(),
        },
        &state,
    );

    let _ =
        get_digit("Tenhle prompt se mi nechtěl psát, po tom co zadáš cifru, tak skončí program.");
}
