#![allow(dead_code)]
extern crate neatwork;
extern crate rand;

use rand::{thread_rng, Rng};

mod species;
mod trainer;
mod training_network;

use trainer::{TrainingParameters, Trainer};

fn main() {
    println!("Hello world!");
    let runs = 4;
    let parameters = TrainingParameters {
        population_size: 15,
        cull_percentage: 0.5,
        crossover_probability: 0.75,
        add_gene_probability: 0.03,
        add_node_probability: 0.05,
        mutate_gene_probability: 0.9,
        gene_enable_probability: 0.4,
        gene_disable_probability: 0.2,
        staleness_maximum: 15
    };

    let mut trainer = Trainer::new(parameters, 3, 1, |net| {
        let scores = (0..runs).fold(0.0, |acc, i| {
            // let input = (i % 2, (i / 2) % 2);
            // let target_result = input.0 ^ input.1;
            // match net.reset_and_evaluate(&vec![input.0 as f64, input.1 as f64, 1.0]) {
            //     Ok(result) => {
            //             let tmp = (result[0] - target_result as f64).abs();
            //             acc - tmp.powi(3)
            //     },
            //     Err(_) => {
            //         println!("error");
            //         acc
            //     }
            // }


            let max_pos: f64 = 10.0 + thread_rng().gen::<f64>() * 60.0;
            let a: f64 = thread_rng().gen::<f64>() * 5.0 + 5.0;
            let mut t: f64 = 0.0;
            let mut closed = true;
            net.network.reset();
            while closed && max_pos - 0.5 * a * t * t > 0.0 {
                match net.evaluate(&vec![max_pos / 100.0, a / 100.0, t / 10.0]) {
                    Ok(result) => {
                            if result[0] > 0.5 {
                                closed = false;
                            }
                    },
                    Err(_) => {
                        println!("error");
                    }
                }
                t += 0.1;
            }
            let pos = if max_pos - 0.5 * a * t * t > 0.0 {(max_pos - 0.5 * a * t * t) / max_pos} else {999999999999.0};
            acc - pos
        });
        scores / runs as f64
    });

    for i in 0..50000 {
        // println!("{:?}", trainer.next());
        trainer.next();
        let mut net = trainer.get_best_network();
        // println!("Generation {}: {:?}", i, net.score);
        // println!("{:?}, {:?}", net.network.genome.len(), net.network.nodes.len());

        if i % 1000 == 0 {
            println!("{:?}", i);
            // let scores = (0..runs).fold(0.0, |acc, i| {
            //     let input = ((i % 2), (i / 2) % 2);
            //     let target_result = input.0 ^ input.1;
            //     match net.reset_and_evaluate(&vec![input.0 as f64, input.1 as f64, 1.0]) {
            //         Ok(result) => {
            //                 println!("{:?} -> {} (guess: {} / {})", input, target_result, result[0].round(), result[0]);
            //                 println!("dist: {:?}", (result[0] - target_result as f64).abs());
            //                 let tmp = (result[0] - target_result as f64).abs();
            //                 acc - tmp.powi(2)
            //         },
            //         Err(_) => {
            //             println!("error");
            //             acc
            //         }
            //     }
            // });
            // println!("Score: {:?} for {} runs", scores, runs);
            // let size = net.network.get_size();
            // println!("Size: {} genes and {} nodes", size.0, size.1);
            // println!(" WOHOOO IT LEARNED XOR!!!! (in generation {})", i);

            for i in 0..10 {
                let max_pos: f64 = 10.0 + thread_rng().gen::<f64>() * 60.0;
                let a: f64 = thread_rng().gen::<f64>() * 5.0 + 5.0;//9.18;
                let mut t: f64 = 0.0;
                let mut closed = true;
                net.network.reset();
                while closed && max_pos - 0.5 * a * t * t > 0.0 {
                    t += 0.1;
                    match net.evaluate(&vec![max_pos / 100.0, a / 100.0, t / 10.0]) {
                        Ok(result) => {
                                if result[0] > 0.5 {
                                    closed = false;
                                }
                        },
                        Err(_) => {
                            println!("error");
                        }
                    }
                }
                if max_pos - 0.5 * a * t * t > 0.0 {
                    println!("{:?}", (max_pos - 0.5 * a * t * t) / max_pos);
                }
                else {
                    println!("died: {}", (max_pos - 0.5 * a * t * t) / max_pos);
                };
            }
            println!("-------");
        }
    }
}
