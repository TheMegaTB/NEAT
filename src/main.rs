#![allow(dead_code)]
extern crate rustc_serialize;
extern crate num_cpus;
extern crate pbr;
extern crate rand;

mod nn;
mod trainer;
mod freefall;

use trainer::*;
use nn::NeuralNetwork;

use std::io::prelude::*;
use std::fs::File;

// fn main() {
//     let mut rng = rand::thread_rng();
//     let mut trainer = TrainerConfig::new().build_new_trainer(&mut rng);
//     let mut generation_index = 0;
//     loop {
//         println!(" ");
//         println!(" ");
//         println!("Running generation #{}", generation_index);
//         let best_nn = trainer.step(&mut rng);
//
//
//         let mut buffer = File::create("best.nn").unwrap();
//         buffer.write_fmt(format_args!("{}", best_nn.encode().unwrap())).unwrap();
//
//         generation_index += 1;
//     }
// }

fn main() {
    println!("{}", freefall::ff(&NeuralNetwork::new_random(vec![3, 3, 1], &mut rand::thread_rng()), true));
}
