use std::cmp::max;
use rand::{thread_rng, Rng};

use neatwork::{Float, EvaluationError, Network, NID, GID, Gene, Node};
use trainer::{Score, TrainingParameters, Probability};

#[derive(Debug, Clone)]
pub struct TrainingNetwork {
    pub network: Network,
    pub score: Score,
    global_rank: usize
}

impl TrainingNetwork {
    pub fn new(network: Network) -> TrainingNetwork {
        TrainingNetwork {
            network: network,
            score: 0.0,
            global_rank: 0
        }
    }

    pub fn evaluate(&mut self, inputs: &Vec<Float>) -> Result<Vec<Float>, EvaluationError> {
        self.network.evaluate(inputs)
    }




    pub fn get_weight_of(&self, other_gene: &Gene) -> Option<Float> {
        for gene in self.network.genome.iter() {
            if gene == other_gene {
                return Some(gene.weight)
            }
        }

        None
    }

    pub fn is_compatible_with(&self, other: &TrainingNetwork) -> bool {
        const C1: f64 = 1.0;
        const C2: f64 = 0.4;
        const DELTA_MAX: f64 = 3.0;

        let mut d = 0;
        let mut w = 0.0;
        let n = max(self.network.genome.len(), other.network.genome.len()) as f64;

        for gene in self.network.genome.iter() {
            match other.get_weight_of(gene) {
                Some(weight) => w += (gene.weight - weight).abs(),
                None => d += 1

            }
        }

        w /= (self.network.genome.len() - d) as f64;

        d as f64 * C1/n + w * C2 < DELTA_MAX
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


    pub fn crossover(&self, other: &TrainingNetwork, self_is_fitter: bool) -> TrainingNetwork {
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

        TrainingNetwork::new(child)
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
    }

    pub fn calculate_score<F>(&mut self, eval_closure: &F) -> Score where F : Fn(&mut TrainingNetwork) -> Score {
        let score = (eval_closure)(self);
        self.network.reset();
        self.score = score;
        score
    }
}
