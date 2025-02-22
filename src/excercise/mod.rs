use std::rc::Rc;

use rand::{rngs::StdRng, Rng};
use random::Prob;
pub mod extract;
pub mod random;
pub mod excercise_set;

pub trait ExcerciseFactory {
    fn generate(&mut self, rnd: &mut StdRng) -> Box<dyn Excercise>;
}

pub trait Excercise {
    fn do_excercise(&self);
}

