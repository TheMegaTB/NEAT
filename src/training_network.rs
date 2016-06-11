use neatwork::{Float, EvaluationError, Network};
use Score;

#[derive(Debug)]
pub struct TrainingNetwork {
    network: Network,
    score: Option<Score>
}

impl TrainingNetwork {
    pub fn new(network: Network) -> TrainingNetwork {
        TrainingNetwork {
            network: network,
            score: None
        }
    }

    pub fn evaluate(&mut self, inputs: &Vec<Float>) -> Result<Vec<Float>, EvaluationError> {
        self.network.evaluate(inputs)
    }
}
