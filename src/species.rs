use rand::{thread_rng, Rng};

use training_network::{TrainingNetwork};
use trainer::{TrainingParameters, Probability};
use trainer::Score;

/// A niche that contains multiple networks to protect changes that are less performant at first
#[derive(Debug)]
pub struct Species {
    pub networks: Vec<TrainingNetwork>,
    score: Option<Score>
}

impl Species {
    pub fn new() -> Species {
        Species {
            networks: Vec::new(),
            score: None
        }
    }

    pub fn from(networks: Vec<TrainingNetwork>) -> Species {
        Species {
            networks: networks,
            score: None
        }
    }

    pub fn get_score(&mut self) -> Score {
        match self.score {
            Some(score) => score,
            None => {
                let score = self.networks.iter().fold(0.0, |acc, net| { acc + net.score.expect("No score there...") }) / self.networks.len() as Score;
                self.score = Some(score);
                score
            }
        }
    }

    pub fn breed(&mut self, parameters: &TrainingParameters) -> TrainingNetwork {
        let mut net = if thread_rng().gen::<Probability>() < parameters.crossover_probability {
            let parent1 = &self.networks[thread_rng().gen_range(0, self.networks.len())];
            let parent2 = &self.networks[thread_rng().gen_range(0, self.networks.len())];
            parent1.crossover(&parent2, parent1.score > parent2.score)
        } else {
            self.networks[thread_rng().gen_range(0, self.networks.len())].clone()
        };

        net.mutate(parameters);

        net
    }

    pub fn cull(&mut self, percentage: Probability) {
        self.networks.sort_by(|a, b| // Sort so that the net w/ the highest score is at index 0
            b.score.expect("No score there...")
                .partial_cmp(&a.score.expect("No score there...")).expect("F64 score comparison failed")
        );

        let mut resulting_size = (self.networks.len() as Probability * percentage) as usize;
        if resulting_size == 0 { resulting_size = 1 };

        self.networks.truncate(resulting_size);
    }

    pub fn reset(&mut self) {
        self.score = None;
    }
}
