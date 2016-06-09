use std::collections::HashMap;

use GID;
use NID;
use Node;
use Gene;
use Float;

/// Structure representing a network or lifeform inside the population
/// The nodes with the NIDs from 0 to x represent the inputs where x is the number of inputs
/// The nodes with the NIDs from nodes.len()-x to nodes.len() represent the outputs where x is the number of outputs
#[derive(Debug)]
pub struct Network {
    /// HashMap that contains the genes and their respective GIDs
    genome: HashMap<GID, Gene>,
    /// Nodes of the network that are connected via links defined in the genome
    pub nodes: Vec<Node>,
    /// List of node IDs that are the outputs of the network
    outputs: Vec<NID>
}

impl Network {
    pub fn new_empty(inputs: usize, outputs: usize, gid_counter: &mut GID) -> Network {
        Network {
            genome: (0..inputs).flat_map(|i| {
                (inputs..inputs+outputs).map(|o| {
                    Gene::new_random(gid_counter, i, o, false)
                }).collect::<Vec<_>>()
            }).collect(),
            nodes: (0..inputs+outputs).map(|i| {
                Node::new(i)
            }).collect(),
            outputs: (inputs..inputs+outputs).collect()
        }
    }
}

fn steep_sigmoid(x: Float) -> Float {
    1.0 / ( 1.0 + (-4.9 * x).exp())
}

#[test]
fn sigmoid() {
    assert_eq!(steep_sigmoid(0.25), 0.77294225)
}

#[test]
fn gid_increases() {
    let mut gid_counter = 0;
    Network::new_empty(5, 5, &mut gid_counter);
    assert!(0 != gid_counter);
    assert_eq!(gid_counter, 25);
}

#[test]
fn nid_is_unique() {
    let mut gid_counter = 0;
    let mut nids = Vec::with_capacity(10);
    for node in Network::new_empty(5, 5, &mut gid_counter).nodes {
        assert!(!nids.contains(&node.id));
        nids.push(node.id);
    }
}
