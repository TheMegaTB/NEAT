use rustc_serialize::json;

use {
    GID,
    NID,
    Node,
    Gene,
    Float,
    Link
};

#[derive(Debug)]
pub enum EvaluationError {
    InputSizeMismatch,
    Unknown
}

pub type Genome = Vec<Gene>;

/// Structure representing a network or lifeform inside the population
///
/// The nodes with the NIDs from 0 to x represent the inputs where x is the number of inputs
/// The nodes with the NIDs from nodes.len()-x to nodes.len() represent the outputs where x is the number of outputs
#[derive(Debug, RustcDecodable, RustcEncodable, Clone, PartialEq)]
pub struct Network {
    /// HashMap that contains the genes and their respective GIDs
    pub genome: Genome,
    /// Nodes of the network that are connected via links defined in the genome
    /// The index defines the NID (it will never change as there are only nodes added, never removed)
    pub nodes: Vec<Node>,
    /// Amount of nodes starting from zero that are the inputs of the network
    pub inputs: usize,
    /// List of NIDs that are the outputs of the network
    pub outputs: Vec<NID>
}

impl Network {
    pub fn new_empty(inputs: usize, outputs: usize) -> Network {
        Network {
            genome: (0..inputs).flat_map(|i| {
                (inputs..inputs+outputs).map(|o| {
                    Gene::random(i, o, false)
                }).collect::<Vec<_>>()
            }).collect(),
            nodes: Node::multiple_new(inputs+outputs),
            inputs: inputs,
            outputs: (inputs..inputs+outputs).collect()
        }
    }

    fn import(data: String) -> Result<Network, json::DecoderError> {
        json::decode(&data)
    }

    fn export(&self) -> Result<String, json::EncoderError> {
        json::encode(self)
    }

    /// Function to list all dependencies that are required for a node.
    fn get_node_dependencies(&self, node: NID) -> Vec<GID> {
        self.genome.iter().enumerate().fold(Vec::new(), |mut acc, (i, gene)| {
            if gene.link.1 == node && !gene.disabled {
                acc.push(i);
            }
            acc
        })
    }

    /// Execute a gene => grab the output of the src, evaluate the gene and add the resulting value to the target nodes input
    fn process_gene(&mut self, link: Link, node_id: NID, gene_id: GID) {
        if link.1 != node_id { panic!("Link target does not match current node ({}): {} -> {}", node_id, link.0, link.1); }
        let output = self.nodes.get(link.0).expect("Node disappeared!").output;
        self.nodes.get_mut(link.1).expect("Node disappeared!").inputs.push(
            self.genome.get(gene_id).expect("Gene disappeared!").evaluate(output)
        );
    }

    /// Calculate a node and all its dependencies
    fn recursive_calc_node(&mut self, node_id: NID, visited: &mut Vec<NID>) -> Float {
        let executed = self.nodes.get_mut(node_id).expect("Node disappeared!").executed;
        if visited.contains(&node_id) || executed {
            self.nodes.get_mut(node_id).expect("Node disappeared!").output
        } else {
            // Add node id to viseted nodes
            visited.push(node_id);

            // Get the IDs of all connections this node depends on
            let dependencies = self.get_node_dependencies(node_id);

            // Check if there are any dependencies and prevent unnecessary calculations
            if dependencies.len() > 0 {
                // Get all connections that this node depends on
                let dependend_links = dependencies.iter().map(|gene_id| {
                    self.genome.get(*gene_id).expect("Gene disappeared!").link
                }).collect::<Vec<_>>();

                // Calculate the values of the nodes that are on the other end of the connection
                for link in dependend_links.iter() {
                    self.recursive_calc_node(link.0, visited); //TODO prevent infinite loop (3->4 and 4->3)
                }

                // Push the outputs through the genes (apply weights) and insert them into the target/current node
                for (gene_id, link) in dependencies.iter().zip(dependend_links.iter()) {
                    self.process_gene(*link, node_id, *gene_id);
                }
            }
            self.nodes.get_mut(node_id).expect("Node disappeared!").evaluate()
        }
    }

    pub fn get_size(&self) -> (GID, NID) {
        let non_disabled_genes = self.genome.iter().fold(0, |acc, gene| {
            if gene.disabled {
                acc
            } else {
                acc + 1
            }
        });
        (non_disabled_genes, self.nodes.len())
    }

    /// Evaluate the network with some input data.
    ///
    /// This might eventually leave some remaining recurrent data in the network behind for the next evaluation.
    pub fn evaluate(&mut self, inputs: &Vec<Float>) -> Result<Vec<Float>, EvaluationError> {
        if !(inputs.len() == self.inputs) {
            return Err(EvaluationError::InputSizeMismatch);
        }

        // Fill in all the inputs
        for input_id in 0..self.inputs {
            let input_node = self.nodes.get_mut(input_id).expect("Input node disappeared!");
            input_node.inputs.push(inputs[input_id]);
            // input_node.output = inputs[input_id];
        }

        // Recursively calculate the output nodes and all their dependencies
        let outputs = self.outputs.clone(); // This assumes that outputs is NEVER modified whilst this function runs
        let output_values = outputs.iter().map(|output_id| {
            // self.recursive_calc_node(*output_id)
            self.recursive_calc_node(*output_id, &mut Vec::new())
        }).collect();

        // Reset the 'executed' flag for all nodes
        for node in self.nodes.iter_mut() {
            node.reset();
        }

        Ok(output_values)
    }

    /// Reset the network fully by removing all remaining recurrent data and resetting all states.
    pub fn reset(&mut self) {
        for node in self.nodes.iter_mut() {
            node.output = 0.0;
            node.reset();
        }
    }
}


#[test]
fn dependency() {
    let net = Network::new_empty(5, 1);
    assert_eq!(net.get_node_dependencies(5), vec![0, 1, 2, 3, 4]);
}

#[test]
fn persistent_results() {
    let mut net = Network::new_empty(1, 1);
    let res1 = net.evaluate(&vec![0.5]).unwrap();
    let res2 = net.evaluate(&vec![0.5]).unwrap();
    assert_eq!(res1, res2);
}

#[test]
fn short_term_memory() {
    let mut net = Network::new_empty(1, 1);
    net.genome.push(Gene::random(0, 0, false));
    let res1 = net.evaluate(&vec![0.5]).unwrap();
    let res2 = net.evaluate(&vec![0.5]).unwrap();
    assert!(res1 != res2);
}

#[test]
fn recursive_evaluation() {
    let mut net = Network::new_empty(1, 1);
    let res = net.nodes[1].evaluate();
    net.nodes[1].reset();
    assert!(res != net.evaluate(&vec![0.5]).unwrap()[0]);
}
