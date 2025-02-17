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

    use expression::test_helpers::*;
    let expr = mult(&[
        num(324),
        vexp('a', 5),
        vexp('b', 3),
        var('y'),
        vexp('d', 9),
        add(&[num(2), var('x')]),
        vexp('e', 8),
    ]);
    println!("{}", expr.disp());
    //
    println!();
    //
    let expr = mult(&[
        num(324),
        vexp('a', 5),
        vexp('b', -23),
        var('y'),
        vexp('d', 999),
        add(&[num(2), var('x')]),
        vexp('e', 87),
    ]);
    println!("{}", expr.disp());
    //
    println!();
    //
    let expr = add(&[
        mult(&[num(2), var('a')]),
        mult(&[
            num(324),
            var('y'),
            vexp('d', 9),
            add(&[num(2), var('x')]),
            vexp('e', 8),
        ]),
    ]);
    println!("{}", expr.disp());
    //
    println!();
    //
    let expr1 = add(&[var('a'), neg(var('b'))]).pow(2);
    let expr2 = add(&[
        var('a').pow(2),
        neg(mult(&[num(2), var('a'), var('b')])),
        var('b').pow(2),
    ]);
    println!("{} = {}", expr1.disp(), expr2.disp());
    //
}
