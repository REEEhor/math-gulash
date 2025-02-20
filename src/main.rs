use excercise::{
    extract::div_squares::DivSquaresFactory, random::SymbolsGenerator, ExcerciseFactory,
};
use rand::prelude::*;
pub mod excercise;
mod expression;
mod input;
pub mod simplification;
use input::*;

fn main() {
    let seed: u64 = get_number("Zadej seed");
    let mut rnd = StdRng::seed_from_u64(seed);

    let mut excercise_factory = DivSquaresFactory {
        symbols_generator: SymbolsGenerator::new(1, 2),
        p_number_part: 0.8_f64,
        p_number_fraction: 0.0_f64,
        p_number_and_var_fused: 1.0_f64,
        var_exponent_range: 1..=20,
        number_part_range: 1..=20,
        p_should_tell_via_formula: 1_f64,
    };

    for idx in 0.. {
        println!("\n\n{idx}: ");
        excercise_factory.generate(&mut rnd).do_excercise();
    }
}
