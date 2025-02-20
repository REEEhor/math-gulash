use std::{
    io::{self, Write},
    ops::RangeInclusive,
};

use colored::Colorize;
use rand::Rng;

use crate::{
    excercise::{
        random::{pow_simplified, random_mult_term, Prob, SymbolsGenerator},
        Excercise, ExcerciseFactory,
    },
    expression::{display, Expr},
    simplification::{simplify_mult_div::SimplifyMultDiv, Simplification},
};

pub struct AddSubSquared {
    a: Expr,
    b: Expr,
    //
    a2: Expr,
    has_minus: bool,
    two_ab: Expr,
    b2: Expr,
    //
    should_tell_via_formula: bool,
}

impl Excercise for AddSubSquared {
    fn do_excercise(&self) {
        if self.should_tell_via_formula {
            println!("Rozložte na součin pomocí vzorce:");
        } else {
            println!("Rozložte na součin:");
        }

        let sign: char = if self.has_minus { '-' } else { '+' };
        let expr_str = format!(
            "{} {} {} + {}",
            self.a2.disp(),
            sign,
            self.two_ab.disp(),
            self.b2.disp(),
        );
        let result_str = format!(
            "({} {} {}){}",
            self.a.disp(),
            sign,
            self.b.disp(),
            display::to_super(2)
        );

        println!("  {}  =  ?", expr_str.cyan());
        print!("Waiting for enter... ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();

        println!("Correct result:");
        println!("  {}  =  {}", expr_str.cyan(), result_str.red());
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
}

impl ExcerciseFactory for AddSubSquaredFactory {
    fn generate(&mut self, rnd: &mut rand::prelude::StdRng) -> Box<dyn Excercise> {
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

        let raw_two_ab =
            Expr::Multiplication([Expr::Number(2), Expr::clone(&a), Expr::clone(&b)].into());
        let simplified_two_ab = SimplifyMultDiv
            .simplify_recursive(&raw_two_ab)
            .expect("This sould never fail... oh well");
        let two_ab = simplified_two_ab.unwrap_or(raw_two_ab);

        let has_minus = rnd.random_bool(self.p_minus_sign);

        Box::new(AddSubSquared {
            a,
            b,
            a2,
            has_minus,
            two_ab,
            b2,
            should_tell_via_formula: rnd.random_bool(self.p_should_tell_via_formula),
        })
    }
}
