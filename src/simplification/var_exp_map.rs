use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use super::Expr;
pub const NUM_OF_VARS: usize = ('z' as usize) - ('a' as usize) + 1;

#[derive(PartialEq, Eq)]
pub struct VarExpMap(pub [i32; NUM_OF_VARS]);

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
        let mut tops = Vec::new();
        let mut bottoms = Vec::new();

        for (idx, exponent) in self.0.iter().enumerate() {
            let symbol = (idx as u8 + 'a' as u8) as char;

            let new_expr = || Expr::Exp {
                base: Expr::Variable { symbol }.into(),
                exp: exponent.abs(),
            };
            match *exponent {
                1 => tops.push(Expr::Variable { symbol }),
                n if n.is_positive() => tops.push(new_expr()),
                n if n.is_negative() => bottoms.push(new_expr()),
                0 => {}
                _ => unreachable!(
                    "The value `{}` should have fallen into positive / negative / zero case",
                    exponent
                ),
            }
        }

        (tops, bottoms)
    }
}
