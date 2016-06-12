#![allow(dead_code)]
extern crate neatwork;
extern crate rand;
extern crate time;

use rand::{Rng, thread_rng};

mod training_network;
pub use training_network::TrainingNetwork;

mod species;
pub use species::Species;

mod population;
pub use population::Population;

mod trainer;
pub use trainer::{Trainer, Score};

const RUNS: u16 = 500;

fn main() {
    println!("Hello world!");
    let mut trainer = Trainer::new(3, 1, |net| {
        let scores = (0..RUNS).fold(0.0, |acc, _| {
            let mut rng = thread_rng();
            let input = (rng.gen::<bool>() as u8, rng.gen::<bool>() as u8);
            let target_result = input.0 ^ input.1;
            let score = match net.evaluate(&vec![input.0 as f64, input.1 as f64, 1.0]) {
                Ok(result) => {
                        acc + (result[0] - target_result as f64).abs()
                },
                Err(_) => {
                    println!("error");
                    0.0
                }
            };
            net.network.reset();
            score
        });
        // println!("{:?}", scores/ RUNS as f64);

        // 1.0 / (scores / RUNS as f64 + 0.000000001)
        -scores / RUNS as f64
    });

    let start_time = time::precise_time_s();
    for i in 0..100 {
        println!("{:?}: {:?}", i, trainer.next());
    }

    let mut net = trainer.get_best_network();
    let correct_counter = (0..RUNS).fold(0, |acc, _| {
        let mut rng = thread_rng();
        let input = (rng.gen::<bool>() as u8, rng.gen::<bool>() as u8);
        let target_result = input.0 ^ input.1;
        let score = match net.evaluate(&vec![input.0 as f64, input.1 as f64, 1.0]) {
            Ok(result) => {
                println!("{:?} -> {}", if result[0] > 0.5 {1} else {0}, target_result);
                if (result[0] - target_result as f64).abs() < 0.5 {
                    acc + 1
                } else { acc }
            },
            Err(_) => acc
        };
        net.reset();
        score
    });
    println!("{:?} out of {:?} test ({}).", correct_counter, RUNS, correct_counter as f64/RUNS as f64);
    println!("The whole training took {} seconds.", time::precise_time_s() - start_time);
}
