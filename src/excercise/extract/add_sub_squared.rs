use std::{
    io::{self, Write},
    ops::{RangeBounds, RangeInclusive},
};

use colored::Colorize;
use rand::{rngs::StdRng, Rng};

use crate::{
    excercise::{
        random::{pow_simplified, random_item_from, random_mult_term, Prob, SymbolsGenerator},
        Excercise, ExcerciseFactory,
    },
    expression::{display, ops::exp, Expr},
    input::{get_input_map, wait_for_enter},
    simplification::{self, simplify_mult_div::SimplifyMultDiv, Simplification},
};

struct CorrectResult {
    a: Expr,
    b: Expr,
}

// a b

// a2      +-2ab   b2
// correct correct correct
// correct invalid correct

enum MiddleTermPosition {
    First,
    Middle,
    Last,
}

pub struct AddSubSquared {
    a: Expr,
    b: Expr,
    //
    a2: Expr,
    b2: Expr,
    //
    has_minus: bool,
    correct_middle_term: Expr,
    //
    middle_term_position: MiddleTermPosition,
    //
    invalid_middle_term: Option<Expr>,
    //
    should_tell_via_formula: bool,
}

impl Excercise for AddSubSquared {
    fn do_excercise(&self) {
        println!("Rozložte na součin pomocí vzorce:");

        let middle_term = self
            .invalid_middle_term
            .as_ref()
            .unwrap_or(&self.correct_middle_term)
            .clone()
            .maybe_wrap_in_minus(self.has_minus);
        let a2 = self.a2.clone();
        let b2 = self.b2.clone();

        use crate::expression::shorthands::*;
        let assignment_expr = match self.middle_term_position {
            MiddleTermPosition::First => add([middle_term, a2, b2]),
            MiddleTermPosition::Middle => add([a2, middle_term, b2]),
            MiddleTermPosition::Last => add([a2, b2, middle_term]),
        };

        let assignment_expr_str = format!("{}", assignment_expr.disp());
        let expected_answer = self.invalid_middle_term.is_none();
        let result_str = format!(
            "({} {} {}){}",
            self.a.disp(),
            if self.has_minus { '-' } else { '+' },
            self.b.disp(),
            display::to_super(2)
        );

        println!("  {}  =  ?", assignment_expr_str.cyan());

        let user_answer = get_input_map("Jde rozložit na součin?", [("ano", true), ("ne", false)]);

        match (expected_answer, user_answer) {
            (true, true) => {
                println!("{} Výraz lze rozložit:", "Správně!".green());
                println!(
                    "  {}  =  {}",
                    assignment_expr_str.cyan(),
                    result_str.green()
                );
            }
            (true, false) => {
                println!("{}. Výraz lze rozložit:", "Špatně".red());
                println!("  {}  =  {}", assignment_expr_str.cyan(), result_str.red());
            }
            (false, true) => {
                println!("{}. Výraz nelze rozložit.", "Špatně".red());
            }
            (false, false) => {
                println!("{} Výraz nelze rozložit.", "Správně!".green());
            }
        }

        println!();
        wait_for_enter("Dej enter pro další příklad");
    }
}

pub struct AddSubSquaredFactory {
    pub symbols_generator: SymbolsGenerator,
    pub p_number_part: Prob,
    pub p_number_fraction: Prob,
    pub p_number_and_var_fused: Prob,
    pub var_exponent_range: RangeInclusive<u32>,
    pub number_part_range: RangeInclusive<u32>,
    pub p_var_in_denominator: Prob,
    pub p_minus_sign: Prob,
    pub p_should_tell_via_formula: Prob,
    pub p_swapped_middle_term: Prob,
    //
    pub p_invalid_middle_term: Prob,
}

impl AddSubSquaredFactory {
    fn generate_mult_term(
        &self,
        rnd: &mut StdRng,
        available_symbols: &[char],
        must_contain_var: bool,
    ) -> Expr {
        random_mult_term(
            rnd,
            available_symbols,
            must_contain_var,
            self.p_number_part,
            self.p_number_fraction,
            self.p_number_and_var_fused,
            self.p_var_in_denominator,
            self.var_exponent_range.clone(),
            self.number_part_range.clone(),
        )
    }

    fn random_invalid_middle_term(
        &self,
        rnd: &mut StdRng,
        a: Expr,
        b: Expr,
        available_symbols: &[char],
    ) -> Expr {
        use crate::expression::shorthands::*;

        let raw_terms = match rnd.random_range(1..=5) {
            1 => mult([a, b]),         // Forgot to multiply by 2
            2 => mult([num(4), a, b]), // Multiplied by 4 instead of 2
            3 if get_any_symbol_from(&a).is_some() => mult([num(2), a]), // Forgot to multiply by `b`
            4 if get_any_symbol_from(&b).is_some() => mult([num(2), b]), // Forgot to multiply by `a`
            _ => {
                // The middle term will contain var^2
                let random_symbol = get_any_symbol_from(&a)
                    .or_else(|| get_any_symbol_from(&b))
                    .expect("At least one of the terms should contain a variable");
                mult([num(2), a, b, var(random_symbol)])
            }
        };

        let simplification = SimplifyMultDiv
            .simplify_recursive(&raw_terms)
            .expect("Unexpected error while generating a term");
        let term = simplification.unwrap_or(raw_terms);

        term
    }

    fn random_middle_term(
        &self,
        rnd: &mut StdRng,
        a: Expr,
        b: Expr,
        available_symbols: &[char],
    ) -> Expr {
        use crate::expression::shorthands::*;
        let raw_terms = mult([num(2), a, b]);
        let simplification = SimplifyMultDiv
            .simplify_recursive(&raw_terms)
            .expect("Unexpected error while generating a term");
        let term = simplification.unwrap_or(raw_terms);

        term
    }
}

fn get_any_symbol_from(expr: &Expr) -> Option<char> {
    match expr {
        Expr::Multiplication(exprs) => exprs.iter().find_map(get_any_symbol_from),
        Expr::Division { lhs, rhs } => {
            get_any_symbol_from(lhs).or_else(|| get_any_symbol_from(rhs))
        }
        Expr::UnaryMinus(expr) => get_any_symbol_from(expr),
        Expr::Variable { symbol } => Some(*symbol),
        Expr::Exp { base, exp } => get_any_symbol_from(base),
        _ => None,
    }
}

impl ExcerciseFactory for AddSubSquaredFactory {
    fn generate(&mut self, rnd: &mut StdRng) -> Box<dyn Excercise> {
        let available_symbols: Vec<char> = self
            .symbols_generator
            .random_symbols(rnd)
            .iter()
            .copied()
            .collect();
        let which_term_must_have_vars: bool = rnd.random_bool(0.5);

        let a = self.generate_mult_term(rnd, &available_symbols, !which_term_must_have_vars);
        let b = self.generate_mult_term(rnd, &available_symbols, which_term_must_have_vars);

        let a2 = pow_simplified(rnd, a.clone(), 2).expect("This should never fail... oh well");
        let b2 = pow_simplified(rnd, b.clone(), 2).expect("This should never fail... oh well");

        let correct_middle_term =
            self.random_middle_term(rnd, a.clone(), b.clone(), &available_symbols);

        let is_invalid = rnd.random_bool(self.p_invalid_middle_term);
        let invalid_middle_term = is_invalid.then(|| {
            self.random_invalid_middle_term(rnd, a.clone(), b.clone(), &available_symbols)
        });

        let middle_term_position = if rnd.random_bool(self.p_swapped_middle_term) {
            if rnd.random_bool(0.5_f64) {
                MiddleTermPosition::First
            } else {
                MiddleTermPosition::Last
            }
        } else {
            MiddleTermPosition::Middle
        };

        Box::new(AddSubSquared {
            a,
            b,
            a2,
            has_minus: rnd.random_bool(self.p_minus_sign),
            correct_middle_term,
            b2,
            should_tell_via_formula: rnd.random_bool(self.p_should_tell_via_formula),
            invalid_middle_term,
            middle_term_position,
        })
    }
}
