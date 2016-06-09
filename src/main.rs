#![allow(dead_code)]
extern crate rand;

mod type_def;
pub use type_def::*;

mod node;
pub use node::Node;

mod gene;
pub use gene::Gene;

mod network;
pub use network::Network;

mod species;
pub use species::Species;

mod population;
pub use population::Population;

fn main() {
    println!("Hello world!");
}
