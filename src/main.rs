#![allow(dead_code)]
extern crate rand;
extern crate rustc_serialize;

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
    let mut net = Network::new_empty(1, 1);
    println!("{:?}", net.evaluate(&vec![0.5]));
    net.add_node_in_gene(0).unwrap();
    println!("{:?}", net);
    // println!("");
    // println!("{:?}", net.crossover(&Network::new_empty(1, 1), true).unwrap());
}

// IDEA:
// Calculation is done from the outputs to the inputs
// Each node that is connected to a output gets evaluated
// To evaluate those nodes every connected node needs to be evaluated and so on
// A node may only be evaluated once to reduce unnecessary calculations
// This goes on recursively until all nodes are evaluated
// If there is a link to a node itself a unresolvable cycle dependency is created and needs to be caught
// by ignoring it as a dependency but using it when writing the output (e.g. write the output back into the input for next round)
