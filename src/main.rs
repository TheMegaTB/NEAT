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

const RUNS: u16 = 10;

fn main() {
    println!("Hello world!");
    let mut trainer = Trainer::new(3, 1, |net| {
        let scores = (0..RUNS).fold(0, |acc, i| {
            let input = (i % 2, (i / 2) % 2);
            let target_result = input.0 ^ input.1;
            let score = match net.evaluate(&vec![input.0 as f64/10.0, input.1 as f64/10.0, 1.0]) {
                Ok(result) => {
                        if (result[0] - target_result as f64).abs() < 0.5 {
                            acc + 1

                        } else { acc }
                },
                Err(_) => {
                    println!("error");
                    acc
                }
            };
            net.network.reset();
            score
        });
        scores as f64
    });

    let start_time = time::precise_time_s();
    // let mut i1 = 0;
    for i1 in 0..100 {
        // i1 += 1;
        println!("----------------------------------- GENERATION {} ----------------------------------- ", i1);
        let net = trainer.get_best_network();
        let stats = trainer.next();
        // println!("{:?}", stats);
        let species_amount = stats.unwrap().len();
        let population_size = trainer.population.species.iter().fold(0, |acc, species| {
            species.networks.len() + acc
        });
        println!("Amount of species: {:?}", species_amount);
        println!("Size of population: {:?}", population_size);
        println!("Average amount of networks per species: {}", population_size / species_amount);
        // trainer.population.species[0].networks.push(net);




    let mut net = trainer.get_best_network();




    let scores = (0..RUNS).fold(0, |acc, i| {
        // let mut rng = thread_rng();
        let input =  (i % 2, (i / 2) % 2);  //(rng.gen::<bool>() as u8, rng.gen::<bool>() as u8);
        let target_result = input.0 ^ input.1;
        let score = match net.evaluate(&vec![input.0 as f64/10.0, input.1 as f64/10.0, 1.0]) {
            Ok(result) => {
                    if (result[0] - target_result as f64).abs() < 0.5 {
                        // println!("{:?}", input);
                        acc + 1
                    } else {
                        // println!("{:?}", input);
                        acc
                    }
                    // acc + (result[0] - target_result as f64).abs()
            },
            Err(_) => {
                println!("error");
                acc
            }
        };
        net.network.reset();
        score
    });

    println!("Result: {:?}", scores as f64 /RUNS as f64);
}
    println!("The whole training took {} seconds.", time::precise_time_s() - start_time);
}
