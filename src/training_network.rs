use rand::{thread_rng, Rng};

use neatwork::{Float, EvaluationError, Network, NID, GID, Gene, Node};
use trainer::{Score, TrainingParameters, Probability};

#[derive(Debug, Clone)]
pub struct ScoredTrainingNetwork {
    pub network: Network,
    pub score: Score,
    global_rank: usize
}

#[derive(Debug, Clone)]
pub struct UnscoredTrainingNetwork {
    pub network: Network
}


impl ScoredTrainingNetwork {
    pub fn evaluate(&mut self, inputs: &Vec<Float>) -> Result<Vec<Float>, EvaluationError> {
        self.network.evaluate(inputs)
    }

    pub fn reset_and_evaluate(&mut self, inputs: &Vec<Float>) -> Result<Vec<Float>, EvaluationError> {
        self.network.reset();
        self.network.evaluate(inputs)
    }

    pub fn is_compatible_with(&self, other: &ScoredTrainingNetwork) -> bool {
        const C1: f64 = 1.0;
        const C2: f64 = 0.4;
        const DELTA_MAX: f64 = 3.0;

        let (net1, net2) = if self.network.genome.len() > other.network.genome.len() { (&self, &other) } else { (&other, &self) };

        let n = net1.network.genome.len() as f64;

        let (d, mut w) = net1.network.genome.iter().fold((0, 0.0), |(mut d, mut w), gene| {
            match net2.get_weight_of(gene) {
                Some(weight) => w += (gene.weight - weight).abs(),
                None => d += 1
            };
            (d, w)
        });

        let d = net2.network.genome.iter().fold(d, |d, gene| {
            match net1.get_weight_of(gene) {
                Some(_) => d,
                None => d + 1
            }
        });

        w /= (net1.network.genome.len() - d) as f64;

        d as f64 * C1/n + w * C2 < DELTA_MAX
    }

    pub fn crossover(&self, other: &ScoredTrainingNetwork, self_is_fitter: bool) -> UnscoredTrainingNetwork {
        if self.network.inputs != other.network.inputs || self.network.outputs.len() != other.network.outputs.len() {
            panic!("IO Size mismatch on crossover")
        }

        let (mut child, other) = if self_is_fitter { (self.network.clone(), other) } else { (other.network.clone(), self) };

        for gene in child.genome.iter_mut() {
            match other.network.genome.iter().find(|other_gene| other_gene == &gene) {
                Some(other_gene) => {
                    gene.merge(other_gene)
                },
                None => {}
            }
        }

        UnscoredTrainingNetwork::new(child)
    }

    pub fn get_weight_of(&self, other_gene: &Gene) -> Option<Float> {
        for gene in self.network.genome.iter() {
            if gene == other_gene {
                return Some(gene.weight)
            }
        }

        None
    }
}

impl UnscoredTrainingNetwork {
    pub fn new(network: Network) -> UnscoredTrainingNetwork {
        UnscoredTrainingNetwork {
            network: network
        }
    }

    pub fn evaluate(&mut self, inputs: &Vec<Float>) -> Result<Vec<Float>, EvaluationError> {
        self.network.evaluate(inputs)
    }

    pub fn reset_and_evaluate(&mut self, inputs: &Vec<Float>) -> Result<Vec<Float>, EvaluationError> {
        self.network.reset();
        self.network.evaluate(inputs)
    }

    fn add_connection(&mut self, src: NID, dest: NID, weight: Option<Float>) {
        let weight = match weight {
            Some(w) => w,
            None => thread_rng().gen::<Float>()*2.0 - 1.0
        };

        let new_gene = Gene::with_weight(src, dest, false, weight);

        if match self.network.genome.iter_mut().find(|gene| gene == &&new_gene) {
            Some(gene) => {
                gene.enable();
                false
            },
            None => true
        } {
            self.network.genome.push(new_gene);
        };
    }

    pub fn add_node_in_gene(&mut self, gene_id: GID) {
        let (link, weight) = match self.network.genome.get_mut(gene_id) {
            Some(gene) => (gene.link, gene.weight),
            None => { panic!("Gene non existent") }
        };

        let node_id = self.network.nodes.len();
        self.network.nodes.push(Node::new());

        self.add_connection(link.0, node_id, Some(1.0));
        self.add_connection(node_id, link.1, Some(weight));

        self.network.genome[gene_id].disable();
    }

    pub fn mutate(&mut self, parameters: &TrainingParameters) {
        if thread_rng().gen::<Probability>() < parameters.add_gene_probability {
            let src = thread_rng().gen_range(0, self.network.nodes.len());
            let dest = thread_rng().gen_range(0, self.network.nodes.len());
            self.add_connection(src, dest, None);
        }
        if thread_rng().gen::<Probability>() < parameters.add_node_probability {
            let gene_id = thread_rng().gen_range(0, self.network.genome.len());
            self.add_node_in_gene(gene_id);
        }
        if thread_rng().gen::<Probability>() < parameters.mutate_gene_probability {
            let gene_id = thread_rng().gen_range(0, self.network.genome.len());
            self.network.genome[gene_id].mutate();
        }
        if thread_rng().gen::<Probability>() < parameters.gene_enable_probability {
            let gene_id = thread_rng().gen_range(0, self.network.genome.len());
            self.network.genome[gene_id].enable();
        }
        if thread_rng().gen::<Probability>() < parameters.gene_disable_probability {
            let gene_id = thread_rng().gen_range(0, self.network.genome.len());
            self.network.genome[gene_id].disable();
        }
    }

    pub fn calculate_score<F>(mut self, eval_closure: &F) -> ScoredTrainingNetwork where F : Fn(&mut UnscoredTrainingNetwork) -> Score {
        let score = (eval_closure)(&mut self);
        ScoredTrainingNetwork {
            network: self.network,
            score: score,
            global_rank: 0
        }
    }
}

fn add_node() {
    let mut net = UnscoredTrainingNetwork::new(Network::new_empty(5, 1));
    let gene_count = net.network.genome.len();
    let node_count = net.network.nodes.len();

    net.add_node_in_gene(0); // Add a new node between node 0 (first input) and 5 (output)

    assert_eq!(net.network.genome.len(), gene_count+2);
    assert_eq!(net.network.nodes.len(), node_count+1);
    assert!(net.network.nodes.get(6).is_some());
}

#[test]
#[should_panic]
fn crossover_io_size_mismatch() {
    let net1 = UnscoredTrainingNetwork::new(Network::new_empty(5, 1)).calculate_score(&(|_| 0.0));
    let net2 = UnscoredTrainingNetwork::new(Network::new_empty(5, 2)).calculate_score(&(|_| 0.0));
    net1.crossover(&net2, false);
}

#[test]
fn compatibility() {
    let net1 = UnscoredTrainingNetwork::new(Network::new_empty(1, 1)).calculate_score(&(|_| 0.0));
    let net2 = UnscoredTrainingNetwork::new(Network::new_empty(1, 1)).calculate_score(&(|_| 0.0));
    let net3 = UnscoredTrainingNetwork::new(Network::new_empty(9, 8)).calculate_score(&(|_| 0.0));
    assert!(net1.is_compatible_with(&net2));
    assert!(!net1.is_compatible_with(&net3));
}

#[test]
fn dedup_genome() {
    let mut net = UnscoredTrainingNetwork::new(Network::new_empty(5, 1));
    let genome_length = net.network.genome.len();
    net.add_connection(2, 2, None);
    assert_eq!(net.network.genome.len(), genome_length+1);
    net.add_connection(2, 2, None);
    assert_eq!(net.network.genome.len(), genome_length+1);
}

#[test]
fn reenabling_gene() {
    let mut net = UnscoredTrainingNetwork::new(Network::new_empty(5, 1));
    let link = net.network.genome[0].link;
    net.network.genome[0].disable();
    net.add_connection(link.0, link.1, None);
    assert!(!net.network.genome[0].disabled);
}
