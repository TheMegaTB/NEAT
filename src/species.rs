use rand::{thread_rng, Rng};

use training_network::{ScoredTrainingNetwork, UnscoredTrainingNetwork};
use trainer::{TrainingParameters, Probability};
use trainer::Score;

/// A niche that contains multiple networks to protect changes that are less performant at first
#[derive(Debug)]
pub struct Species {
    pub networks: Vec<ScoredTrainingNetwork>,
    pub score: Score
}

impl Species {
    pub fn new() -> Species {
        Species {
            networks: Vec::new(),
            score: 0.0
        }
    }

    pub fn from(networks: Vec<ScoredTrainingNetwork>) -> Species {
        Species {
            networks: networks,
            score: 0.0
        }
    }

    pub fn calculate_score(&mut self) -> Score {
        let score = self.networks.iter().fold(0.0, |acc, net| { acc + net.score }) / self.networks.len() as Score;
        self.score = score;
        score
    }

    pub fn breed(&mut self, parameters: &TrainingParameters) -> UnscoredTrainingNetwork {
        let mut net = if thread_rng().gen::<Probability>() < parameters.crossover_probability {
            let parent1 = &self.networks[thread_rng().gen_range(0, self.networks.len())];
            let parent2 = &self.networks[thread_rng().gen_range(0, self.networks.len())];
            parent1.crossover(&parent2, parent1.score > parent2.score)
        } else {
            UnscoredTrainingNetwork::new(self.networks[thread_rng().gen_range(0, self.networks.len())].network.clone())
        };

        net.mutate(parameters);

        net
    }

    pub fn cull(&mut self, percentage: Probability) {
        self.networks.sort_by(|a, b| // Sort so that the net w/ the highest score is at index 0
            b.score.partial_cmp(&a.score).expect("F64 score comparison failed")
        );

        let mut resulting_size = (self.networks.len() as Probability * percentage) as usize;
        if resulting_size == 0 { resulting_size = 1 };

        self.networks.truncate(resulting_size);
    }
}
