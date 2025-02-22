use std::rc::Rc;

use rand::{rngs::StdRng, Rng};

use crate::excercise::random::pick_proportionally;

use super::{random::Prob, ExcerciseFactory};

pub struct ExcerciseSet {
    factories: Vec<(u32, Box<dyn ExcerciseFactory>)>,
}

impl ExcerciseSet {
    pub fn from<const N: usize>(factories: [(u32, Box<dyn ExcerciseFactory>); N]) -> Self {
        assert_ne!(N, 0);
        Self {
            factories: factories.into(),
        }
    }

    pub fn choose_random<'a>(&'a mut self, rnd: &mut StdRng) -> &'a mut dyn ExcerciseFactory {
        // TODO: pick index proportionally
        let random_idx = rnd.random_range(0..self.factories.len());
        &mut *(self.factories[random_idx].1)
    }
}
