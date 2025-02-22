use colored::*;
use std::{
    io::{self, Read, Write},
    ops::{Div, RangeInclusive},
};

use rand::{rngs::StdRng, Rng};

use crate::{
    excercise::{
        random::{pow_simplified, random_mult_term, Prob, SymbolsGenerator},
        Excercise, ExcerciseFactory,
    },
    expression::Expr,
    input::{get_input_map, wait_for_enter},
};

#[derive(Debug)]
pub struct DiffSquares {
    a: Expr,
    b: Expr,
    a2: Expr,
    b2: Expr,
    should_tell_via_formula: bool,
    //
    is_invalid: bool,
}

impl Excercise for DiffSquares {
    fn do_excercise(&self) {
        println!("Rozložte na součin pomocí vzorce:");

        let expected_answer = !self.is_invalid;

        let expr_str = format!(
            "{} {} {}",
            self.a2.disp(),
            if self.is_invalid { '+' } else { '-' },
            self.b2.disp(),
        );
        let result_str = format!("({0} - {1}) * ({0} + {1})", self.a.disp(), self.b.disp());

        println!("  {}  =  ?", expr_str.cyan());
        let user_answer = get_input_map("Jde rozložit na součin?", [("ano", true), ("ne", false)]);

        match (expected_answer, user_answer) {
            (true, true) => {
                println!("{} Výraz lze rozložit:", "Správně!".green());
                println!("  {}  =  {}", expr_str.cyan(), result_str.green());
            }
            (true, false) => {
                println!("{}. Výraz lze rozložit:", "Špatně".red());
                println!("  {}  =  {}", expr_str.cyan(), result_str.red());
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

#[derive(Clone)]
pub struct DiffSquaresFactory {
    pub symbols_generator: SymbolsGenerator,
    pub p_number_part: Prob,
    pub p_number_fraction: Prob,
    pub p_number_and_var_fused: Prob,
    pub var_exponent_range: RangeInclusive<u32>,
    pub number_part_range: RangeInclusive<u32>,
    pub p_var_in_denominator: Prob,
    pub p_should_tell_via_formula: Prob,
    pub p_invalid: Prob,
}

impl ExcerciseFactory for DiffSquaresFactory {
    fn generate(&mut self, rnd: &mut StdRng) -> Box<dyn Excercise> {
        let available_symbols: Vec<char> = self
            .symbols_generator
            .random_symbols(rnd)
            .iter()
            .copied()
            .collect();
        let which_term_must_have_vars: bool = rnd.random_bool(0.5);

        let a = random_mult_term(
            rnd,
            &available_symbols,
            !which_term_must_have_vars,
            self.p_number_part,
            self.p_number_fraction,
            self.p_number_and_var_fused,
            self.p_var_in_denominator,
            self.var_exponent_range.clone(),
            self.number_part_range.clone(),
        );
        let b = random_mult_term(
            rnd,
            &available_symbols,
            which_term_must_have_vars,
            self.p_number_part,
            self.p_number_fraction,
            self.p_number_and_var_fused,
            self.p_var_in_denominator,
            self.var_exponent_range.clone(),
            self.number_part_range.clone(),
        );

        let a2 = pow_simplified(rnd, a.clone(), 2).expect("This sould never fail... oh well");
        let b2 = pow_simplified(rnd, b.clone(), 2).expect("This sould never fail... oh well");

        Box::new(DiffSquares {
            a,
            b,
            a2,
            b2,
            should_tell_via_formula: rnd.random_bool(self.p_should_tell_via_formula),
            is_invalid: rnd.random_bool(self.p_invalid),
        })
    }
}
