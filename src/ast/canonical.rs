use core::num;
use std::ops::Index;

#[derive(Eq, PartialEq)]
pub struct NumberFraction {
    top: u32,
    bottom: u32,
}

impl NumberFraction {
    fn whole_number(number: u32) -> Self {
        Self {
            top: number,
            bottom: 1,
        }
    }

    fn new(top: u32, bottom: u32) -> Self {
        Self { top, bottom }
    }
}

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

#[derive(PartialEq, Eq)]
pub struct CanonicalTerm {
    is_positive: bool,
    number_frac: NumberFraction,
    var_exp_map: VarExpMap,
}
