#![allow(dead_code)]
extern crate neatwork;
extern crate rand;

mod species;
mod trainer;
mod training_network;

use trainer::{TrainingParameters, Trainer};

fn main() {
    println!("Hello world!");
    let runs = 4;
    let parameters = TrainingParameters {
        population_size: 150,
        cull_percentage: 0.5,
        crossover_probability: 0.75,
        add_gene_probability: 0.03,
        add_node_probability: 0.05,
        mutate_gene_probability: 0.9,
        gene_enable_probability: 0.4,
        gene_disable_probability: 0.2
    };

    let mut trainer = Trainer::new(parameters, 3, 1, |net| {
        let scores = (0..runs).fold(0.0, |acc, i| {
            let input = (i % 2, (i / 2) % 2);
            let target_result = input.0 ^ input.1;
            let score = match net.evaluate(&vec![input.0 as f64/10.0, input.1 as f64/10.0, 1.0]) {
                Ok(result) => {
                        // println!("{:?} -> {} (guess: {})", input, target_result, result[0]);
                        // if (result[0] - target_result as f64).abs() < 0.5 {
                        //     acc + 1
                        // } else { acc }
                        acc - (result[0] - target_result as f64).abs()
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

    for i in 0..2000 {
        trainer.next();
        let mut net = trainer.get_best_network();
        println!("Generation {}: {:?}", i, net.score);
        // if net.score > -1.0 {
        if i == 1999 {
            net.network.reset();
            let scores = (0..runs).fold(0.0, |acc, i| {
                let input = (i % 2, (i / 2) % 2);
                let target_result = input.0 ^ input.1;
                let score = match net.network.evaluate(&vec![input.0 as f64/10.0, input.1 as f64/10.0, 1.0]) {
                    Ok(result) => {
                            println!("{:?} -> {} (guess: {})", input, target_result, result[0]);
                            println!("dist: {:?}", (result[0] - target_result as f64).abs());
                            // if (result[0] - target_result as f64).abs() < 0.5 {
                            //     acc + 1
                            // } else { acc }
                            acc - (result[0] - target_result as f64).abs()
                    },
                    Err(_) => {
                        println!("error");
                        acc
                    }
                };
                net.network.reset();
                score
            });
            println!("Score: {:?} / {}", scores, runs);
            let size = net.network.get_size();
            println!("Size: {} genes and {} nodes", size.0, size.1);
            println!(" WOHOOO IT LEARNED XOR!!!! (in generation {})", i);
            break
        }
    }
}
