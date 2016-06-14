use neatwork::{Float, EvaluationError, Network};
use Score;

#[derive(Debug, Clone)]
pub struct TrainingNetwork {
    pub network: Network,
    pub score: Option<Score>
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
