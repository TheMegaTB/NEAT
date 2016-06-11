#![allow(dead_code)]
extern crate neatwork;

mod training_network;
pub use training_network::TrainingNetwork;

mod species;
pub use species::Species;

mod population;
pub use population::Population;

mod trainer;
pub use trainer::{Trainer, Score};


fn main() {
    println!("Hello world!");
    let trainer = Trainer::new(3, 1, |_net| {
        0.0
    });

    for stats in trainer {
        println!("{:?}", stats);
    }
}
