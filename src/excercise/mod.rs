use rand::{rngs::StdRng, Rng};
pub mod extract;
pub mod random;

pub trait ExcerciseFactory {
    fn generate(&mut self, rnd: &mut StdRng) -> Box<dyn Excercise>;
    fn combine<EFactory: ExcerciseFactory + 'static>(
        self,
        other: EFactory,
    ) -> ExcerciseFactoryCombination
    where
        Self: 'static + Sized,
    {
        ExcerciseFactoryCombination {
            factories: vec![Box::new(self), Box::new(other)],
        }
    }
}

pub trait Excercise {
    fn do_excercise(&self);
}

pub struct ExcerciseFactoryCombination {
    factories: Vec<Box<dyn ExcerciseFactory>>,
}

impl ExcerciseFactory for ExcerciseFactoryCombination {
    fn generate(&mut self, rnd: &mut StdRng) -> Box<dyn Excercise> {
        let random_idx = rnd.random_range(0..self.factories.len());
        self.factories[random_idx].generate(rnd)
    }
}
