use core::num;
use std::{
    collections::{HashSet, VecDeque},
    num::NonZero,
    ops::RangeInclusive,
    sync::Arc,
};

use rand::{
    distr::uniform::{SampleRange, SampleUniform},
    rngs::StdRng,
    Rng,
};

use crate::{
    expression::{error::EvalResult, number_fraction::NumberFraction, ops::mul, Expr},
    simplification::{
        simplify_exp::SimplifyExponentiation,
        var_exp_map::{VarExpMap, NUM_OF_VARS},
        Simplification,
    },
};

pub type Prob = f64;

pub struct SymbolsGenerator {
    min_vars_count: u32,
    max_vars_count: u32,
}

impl SymbolsGenerator {
    pub fn new(min_vars_count: u32, max_vars_count: u32) -> Self {
        assert!(min_vars_count <= max_vars_count);
        assert!(max_vars_count <= (NUM_OF_VARS as u32 - 2)); // `- 2` to exclude 'O' and 'L' symbols
        Self {
            min_vars_count,
            max_vars_count,
        }
    }

    pub fn random_symbols(&mut self, rnd: &mut StdRng) -> HashSet<char> {
        let desired_length = rnd.random_range(self.min_vars_count..=self.max_vars_count);
        let mut result = HashSet::new();

        while (result.len() as u32) < desired_length {
            let random_symbol = random_filter(rnd, 'a'..='z', |chr| *chr != 'o' && *chr != 'l');
            result.insert(random_symbol);
        }

        result
    }
}

pub fn random_filter<T: SampleUniform, Range: SampleRange<T> + Clone, Filter: Fn(&T) -> bool>(
    rnd: &mut StdRng,
    range: Range,
    is_ok: Filter,
) -> T {
    let mut loop_cnt = 0;
    loop {
        let random_value = rnd.random_range(range.clone());
        if is_ok(&random_value) {
            return random_value;
        }
        loop_cnt += 1;
        if loop_cnt > 5_000 {
            panic!("Tried too many times");
        }
    }
}

pub fn random_item_from<T: std::marker::Copy>(rnd: &mut StdRng, container: &[T]) -> T {
    if container.is_empty() {
        panic!("Cannot choose a random item from an empty container");
    }

    let range = 0..container.len();
    let random_idx = rnd.random_range(range);
    container[random_idx]
}

pub fn random_fraction(rnd: &mut StdRng, max_number: u32) -> NumberFraction {
    assert!(max_number >= 3);

    let top = rnd.random_range(1..max_number);
    let mut bottom = rnd.random_range(2..max_number);

    if top % bottom == 0 {
        bottom += 1;
    }

    let fraction = NumberFraction::new_in_base_form(top as i32, bottom as i32)
        .expect("The `bottom cannot be zero since it was generated in the range from 2..`");

    return fraction;
}

pub fn random_mult_term(
    rnd: &mut StdRng,
    available_symbols: &[char],
    has_to_contain_var: bool,
    p_number_part: Prob,
    p_number_fraction: Prob,
    p_number_and_var_fused: Prob,
    p_var_in_denominator: Prob,
    var_exponent_abs_range: RangeInclusive<u32>,
    number_part_range: RangeInclusive<u32>,
) -> Expr {
    assert!(
        !has_to_contain_var || !available_symbols.is_empty(),
        "Cannot generate if `available_symbols` is empty"
    );

    // Generate the variables part
    let mut vars_map = VarExpMap::new();
    //
    let vars_count_range = (has_to_contain_var as usize)..=available_symbols.len();
    let max_vars_count = rnd.random_range(vars_count_range);
    for _ in 0..max_vars_count {
        let symbol = random_item_from(rnd, available_symbols);
        let var_is_in_denominator = rnd.random_bool(p_var_in_denominator);
        let (exp_lo, exp_hi) = if var_is_in_denominator {
            let (lo, hi) = (
                *var_exponent_abs_range.start() as i32,
                *var_exponent_abs_range.end() as i32,
            );
            (-hi, -lo)
        } else {
            (
                *var_exponent_abs_range.start() as i32,
                *var_exponent_abs_range.end() as i32,
            )
        };
        vars_map[symbol] = rnd.random_range(exp_lo..=exp_hi);
    }
    //
    let (top_exprs, bottom_exprs) = vars_map.partition_by_exp_sign();
    let mut top_exprs: VecDeque<_> = top_exprs.into();
    let mut bottom_exprs: VecDeque<_> = bottom_exprs.into();

    // Decide whether the number part should be generated
    let has_no_vars = top_exprs.is_empty() && bottom_exprs.is_empty();
    let wanted_number_part = rnd.random_bool(p_number_part);
    let has_number_part = wanted_number_part || has_no_vars;

    if !has_number_part {
        return Expr::mult_div_from_exprs(top_exprs, bottom_exprs);
    }

    // Generate the number part
    let has_number_part_seperate = !rnd.random_bool(p_number_and_var_fused);
    let has_number_denominator = rnd.random_bool(p_number_fraction);
    let number_fraction = if !has_number_denominator {
        let random_number = rnd.random_range(number_part_range);
        NumberFraction::whole_number(random_number as i32)
    } else {
        random_fraction(rnd, *number_part_range.end())
    };
    //
    if has_number_part_seperate {
        let number_part_expr = Expr::from_number_fraction(number_fraction);
        let vars_part_expr = Expr::mult_div_from_exprs(top_exprs, bottom_exprs);
        if vars_part_expr.as_number() == Some(1) {
            return number_part_expr;
        }
        return mul(number_part_expr, vars_part_expr);
    }
    //
    if number_fraction.top > 1 {
        top_exprs.push_front(Expr::Number(number_fraction.top));
    }
    if number_fraction.bottom_u32() > 1 {
        bottom_exprs.push_front(Expr::Number(number_fraction.bottom_u32()));
    }
    return Expr::mult_div_from_exprs(top_exprs, bottom_exprs);
}

pub fn pow_simplified(rnd: &mut StdRng, expr: Expr, exponent: u32) -> EvalResult<Expr> {
    let mut expr = expr.pow(exponent as i32);

    while let Some(simplified_expr) = SimplifyExponentiation.simplify_once_recursive(&expr)? {
        expr = simplified_expr;
    }

    Ok(expr)
}
