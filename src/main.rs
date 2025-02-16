use rand::prelude::*;
mod expression;
mod input;
pub mod simplification;
use input::*;

fn main() {
    // let seed: u64 = get_number("Zadej seed");
    // let mut _rnd = StdRng::seed_from_u64(seed);
    // for _idx in 0.. {
    // }

    for divisor in 2_i32..=5_i32 {
        for i in -10_i32..=10_i32 {
            println!("{} mod {} = {}", i, divisor, i.rem_euclid(divisor));
        }
        println!("\n\n\n");
    }
}
