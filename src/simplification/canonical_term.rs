use crate::expression::number_fraction::NumberFraction;

use super::var_exp_map::VarExpMap;

pub struct CanonicalMultTerm {
    pub vars: VarExpMap,
    pub number_part: NumberFraction,
}

impl CanonicalMultTerm {
    pub fn new() -> Self {
        Self {
            vars: VarExpMap::new(),
            number_part: NumberFraction::whole_number(1),
        }
    }

    pub fn contains_no_vars(&self) -> bool {
        self.vars.0.iter().all(|exp| *exp == 0)
    }
}
