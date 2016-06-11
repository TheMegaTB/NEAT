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
pub use network::{Network, EvaluationError};
