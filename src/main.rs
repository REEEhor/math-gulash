use excercise::{
    excercise_set::{self, ExcerciseSet},
    extract::{add_sub_squared::AddSubSquaredFactory, diff_squares::DiffSquaresFactory},
    random::SymbolsGenerator,
    ExcerciseFactory,
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

    let excercise_factory_1 = DiffSquaresFactory {
        symbols_generator: SymbolsGenerator::new(1, 2),
        p_number_part: 0.8_f64,
        p_number_fraction: 0.3_f64,
        p_number_and_var_fused: 0.4_f64,
        p_var_in_denominator: 0.15_f64,
        var_exponent_range: 1..=20,
        number_part_range: 1..=20,
        p_should_tell_via_formula: 1_f64,
        p_invalid: 0.5_f64,
    };

    let excercise_factory_2 = AddSubSquaredFactory {
        symbols_generator: SymbolsGenerator::new(1, 2),
        p_number_part: 0.8_f64,
        p_number_fraction: 0.05_f64,
        p_number_and_var_fused: 0.95_f64,
        p_var_in_denominator: 0.0_f64,
        var_exponent_range: 1..=10,
        number_part_range: 1..=10,
        p_should_tell_via_formula: 1_f64,
        p_minus_sign: 0.5_f64,
        p_invalid_middle_term: 0.5_f64,
        p_swapped_middle_term: 0.1_f64,
    };

    let mut excercise_set = ExcerciseSet::from([
        (1, Box::new(excercise_factory_1)),
        (1, Box::new(excercise_factory_2)),
    ]);

    for idx in 1.. {
        println!("\n=======================================================================");
        println!("{idx}: ");
        excercise_set
            .choose_random(&mut rnd)
            .generate(&mut rnd)
            .do_excercise();
    }
}
