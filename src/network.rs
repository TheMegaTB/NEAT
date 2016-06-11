use GID;
use NID;
use Node;
use Gene;
use Float;
use Link;

#[derive(Debug)]
pub enum EvaluationError {
    InputSizeMismatch,
    Unknown
}

#[derive(Debug)]
pub enum MutationError {
    GeneNotExistent
}

type Genome = Vec<Gene>;

/// Structure representing a network or lifeform inside the population
///
/// The nodes with the NIDs from 0 to x represent the inputs where x is the number of inputs
/// The nodes with the NIDs from nodes.len()-x to nodes.len() represent the outputs where x is the number of outputs
#[derive(Debug)]
pub struct Network {
    /// HashMap that contains the genes and their respective GIDs
    genome: Genome,
    /// Nodes of the network that are connected via links defined in the genome
    /// The index defines the NID (it will never change as there are only nodes added, never removed)
    pub nodes: Vec<Node>,
    /// Amount of nodes starting from zero that are the inputs of the network
    inputs: usize,
    /// List of NIDs that are the outputs of the network
    outputs: Vec<NID>,
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

    fn add_node(&mut self, gene_id: GID) -> Result<(), MutationError>{
        let (link, weight) = match self.genome.get_mut(gene_id) {
            Some(gene) => (gene.link, gene.weight),
            None => { return Err(MutationError::GeneNotExistent) }
        };

        // Add link
        let node_id = self.nodes.len();
        self.nodes.push(Node::new());
        // Add node
        self.genome.push(Gene::with_weight(link.0, node_id, false, 1.0));
        self.genome.push(Gene::with_weight(node_id, link.1, false, weight));

        self.genome[gene_id].disable(); // No match required as the match at the beginning would have returned if the gene doesn't exist
        Ok(())
    }

    /// Function to list all dependencies that are required for a node.
    ///
    /// It returns two vectors where the first one consist of the GIDs of recurring connections (returning to the node itself)
    /// The second vector is a list of GIDs that are non-recurring connections
    fn get_node_dependencies(&self, node: NID) -> (Vec<GID>, Vec<GID>) {
        self.genome.iter().enumerate().fold((Vec::new(), Vec::new()), |mut acc, (i, gene)| {
            if gene.link.1 == node && !gene.disabled {
                if gene.link.0 == gene.link.1 {
                    acc.0.push(i);
                } else {
                    acc.1.push(i);
                }
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
    fn recursive_calc_node(&mut self, node_id: NID) -> Float {
        // Get the IDs of all connections this node depends on
        let dependencies = self.get_node_dependencies(node_id);

        // Check if there are any dependencies and prevent unnecessary calculations
        if dependencies.1.len() > 0 {
            // Get all connections that this node depends on
            let dependend_links = dependencies.1.iter().map(|gene_id| {
                self.genome.get(*gene_id).expect("Gene disappeared!").link
            }).collect::<Vec<_>>();

            // Calculate the values of the nodes that are on the other end of the connection
            for link in dependend_links.iter() {
                self.recursive_calc_node(link.0); //TODO prevent infinitie loop (3->4 and 4->3)
            }

            // Push the outputs through the genes (apply weights) and insert them into the target/current node
            for (gene_id, link) in dependencies.1.iter().zip(dependend_links.iter()) {
                self.process_gene(*link, node_id, *gene_id);
            }
        }

        let (evaluated, out) = {
            let node = self.nodes.get_mut(node_id).expect("Node disappeared!");

            // Either evaluate the node or grab its current output value (-> dont re-evaluate and waste resources)
            (!node.executed,
            if node.executed {
                node.output
            } else {
                node.evaluate()
            })
        };

        // Only 'execute' recurring connections once (when the node got evaluated)
        if evaluated {
            for recurring_gene_id in dependencies.0.iter() {
                let link = self.genome.get(*recurring_gene_id).expect("Gene disappeared!").link;
                self.process_gene(link, node_id, *recurring_gene_id);
            }
        }

        out
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
            input_node.inputs.push(inputs[input_id])
        }

        // Recursively calculate the output nodes and all their dependencies
        let outputs = self.outputs.clone(); // This assumes that outputs is NEVER modified whilst this function runs
        let output_values = outputs.iter().map(|output_id| {
            self.recursive_calc_node(*output_id)
        }).collect();

        for node in self.nodes.iter_mut() {
            node.reset();
        }

        Ok(output_values)
    }

    /// Reset the network fully by removing all remaining recurrent data and resetting all states.
    pub fn reset(&mut self) {
        for node in self.nodes.iter_mut() {
            node.inputs.clear();
            node.reset();
        }
    }
}

#[test]
fn dependency() {
    let net = Network::new_empty(5, 1);
    assert_eq!(net.get_node_dependencies(5), (Vec::new(), vec![0, 1, 2, 3, 4]));
}

#[test]
fn add_node() {
    let mut net = Network::new_empty(5, 1);
    let gene_count = net.genome.len();
    let node_count = net.nodes.len();

    net.add_node(0).unwrap(); // Add a new node between node 0 (first input) and 5 (output)

    assert_eq!(net.genome.len(), gene_count+2);
    assert_eq!(net.nodes.len(), node_count+1);
    assert!(net.nodes.get(6).is_some());
}

#[test]
fn short_term_memory() {
    let mut net = Network::new_empty(1, 1);
    net.genome.push(Gene::random(0, 0, false));
    let _ = net.evaluate(&vec![0.5]);
    assert!(net.nodes[0].inputs.len() == 1);
}
