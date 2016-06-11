#![allow(dead_code)]
extern crate neatwork;
extern crate rand;
extern crate time;

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
    let mut trainer = Trainer::new(3, 1, |net| {
        let input = (0, 1);
        let target_result = input.0 ^ input.1;
        match net.evaluate(&vec![input.0 as f64, input.1 as f64, 1.0]) {
            Ok(result) => {
                let score = 1.0 / ((result[0] - target_result as f64).abs()+0.000000001);
                // println!("{:?}", score);
                score
            },
            Err(_) => 0.0
        }
    });

    let start_time = time::precise_time_s();
    for _ in 0..30 {
        trainer.next();
    }
    println!("The whole training took {} seconds.", time::precise_time_s() - start_time);
}
