use std::ops::{Index, IndexMut};

use super::Expr;
const NUM_OF_VARS: usize = ('z' as usize) - ('a' as usize) + 1;

#[derive(PartialEq, Eq)]
pub struct VarExpMap([i32; NUM_OF_VARS]);

impl Index<char> for VarExpMap {
    type Output = i32;

    fn index(&self, index: char) -> &Self::Output {
        assert!(
            'a' <= index && index <= 'z',
            "The index has to be a lowercase letter, but is '{index}'."
        );
        let idx = (index as usize) - ('a' as usize);
        &self.0[idx]
    }
}

impl IndexMut<char> for VarExpMap {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        assert!(
            'a' <= index && index <= 'z',
            "The index has to be a lowercase letter, but is '{index}'."
        );
        let idx = (index as usize) - ('a' as usize);
        &mut self.0[idx]
    }
}

impl VarExpMap {
    pub fn new() -> Self {
        Self([0; NUM_OF_VARS])
    }

    pub fn partition_by_exp_sign(&self) -> (Vec<Expr>, Vec<Expr>) {
        let vars_to_power = self.0.iter().enumerate().filter_map(|(idx, exponent)| {
            if *exponent == 0 {
                None
            } else {
                let symbol = (idx as u8 + 'a' as u8) as char;
                Some((symbol, *exponent))
            }
        });

        let mut tops = Vec::new();
        let mut bottoms = Vec::new();

        for (symbol, exponent) in vars_to_power {
            let abs_exponent = exponent.unsigned_abs();
            let new_expr = || Expr::Exp {
                base: Expr::Variable { symbol }.into(),
                exp: Expr::Number(abs_exponent).into(),
            };
            match exponent {
                ..0 => tops.push(new_expr()),
                1.. => bottoms.push(new_expr()),
                0 => {}
            }
        }

        (tops, bottoms)
    }
}

pub struct CanonicalMultTerm {
    pub vars: VarExpMap,
    pub number: i32,
}

impl CanonicalMultTerm {
    pub fn new() -> Self {
        Self {
            vars: VarExpMap::new(),
            number: 1,
        }
    }

    pub fn contains_no_vars(&self) -> bool {
        self.vars.0.iter().all(|exp| *exp == 0)
    }
}