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
};

pub struct DivSquares {
    a: Expr,
    b: Expr,
    a2: Expr,
    b2: Expr,
    should_tell_via_formula: bool,
}

impl Excercise for DivSquares {
    fn do_excercise(&self) {
        if self.should_tell_via_formula {
            println!("Rozložte na součin pomocí vzorce:");
        } else {
            println!("Rozložte na součin:");
        }

        let expr_str = format!("{} - {}", self.a2.disp(), self.b2.disp());
        let result_str = format!("({0} - {1}) * ({0} + {1})", self.a.disp(), self.b.disp());

        println!("  {}  =  ?", expr_str.cyan());
        print!("Waiting for enter... ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();

        println!("\nCorrect result:");
        println!("  {}  =  {}", expr_str.cyan(), result_str.red());
    }
}

pub struct DivSquaresFactory {
    pub symbols_generator: SymbolsGenerator,
    pub p_number_part: Prob,
    pub p_number_fraction: Prob,
    pub p_number_and_var_fused: Prob,
    pub var_exponent_range: RangeInclusive<i32>,
    pub number_part_range: RangeInclusive<u32>,
    pub p_should_tell_via_formula: Prob,
}

impl ExcerciseFactory for DivSquaresFactory {
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
            self.var_exponent_range.clone(),
            self.number_part_range.clone(),
        );

        let a2 = pow_simplified(rnd, a.clone(), 2).expect("This sould never fail... oh well");
        let b2 = pow_simplified(rnd, b.clone(), 2).expect("This sould never fail... oh well");

        Box::new(DivSquares {
            a,
            b,
            a2,
            b2,
            should_tell_via_formula: rnd.random_bool(self.p_should_tell_via_formula),
        })
    }
}
