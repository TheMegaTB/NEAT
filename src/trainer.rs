use neatwork::Network;
use Population;

const POPULATION_SIZE: usize = 150;
pub type Score = f64;

pub struct Trainer<F> where F: Fn(&mut Network) -> Score {
    population: Population,
    eval_closure: F
}

impl<F> Trainer<F> where F : Fn(&mut Network) -> Score {
    pub fn new(inputs: usize, outputs: usize, closure: F) -> Trainer<F> {
        Trainer {
            population: Population::new(POPULATION_SIZE, inputs, outputs),
            eval_closure: closure
        }
    }
}

impl<F> Iterator for Trainer<F> where F : Fn(&mut Network) -> Score {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
