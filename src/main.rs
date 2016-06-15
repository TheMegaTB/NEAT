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
            match net.reset_and_evaluate(&vec![input.0 as f64, input.1 as f64, 1.0]) {
                Ok(result) => {
                        let tmp = (result[0] - target_result as f64).abs();
                        acc - tmp.powi(6)
                },
                Err(_) => {
                    println!("error");
                    acc
                }
            }
        });
        scores
    });

    for i in 0..10000 {
        println!("{:?}", trainer.next());
        let mut net = trainer.get_best_network();
        println!("Generation {}: {:?}", i, net.score);
        // println!("{:?}, {:?}", net.network.genome.len(), net.network.nodes.len());

        if i == 9999 {
            let scores = (0..runs).fold(0.0, |acc, i| {
                let input = ((i % 2), (i / 2) % 2);
                let target_result = input.0 ^ input.1;
                match net.reset_and_evaluate(&vec![input.0 as f64, input.1 as f64, 1.0]) {
                    Ok(result) => {
                            println!("{:?} -> {} (guess: {} / {})", input, target_result, result[0].round(), result[0]);
                            println!("dist: {:?}", (result[0] - target_result as f64).abs());
                            let tmp = (result[0] - target_result as f64).abs();
                            acc - tmp.powi(2)
                    },
                    Err(_) => {
                        println!("error");
                        acc
                    }
                }
            });
            println!("Score: {:?} for {} runs", scores, runs);
            let size = net.network.get_size();
            println!("Size: {} genes and {} nodes", size.0, size.1);
            println!(" WOHOOO IT LEARNED XOR!!!! (in generation {})", i);
            break
        }
    }
}
