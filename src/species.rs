use TrainingNetwork;

/// A niche that contains multiple networks to protect changes that are less performant at first
#[derive(Debug)]
pub struct Species {
    networks: Vec<TrainingNetwork>
}

impl Species {
    pub fn new() -> Species {
        Species {
            networks: Vec::new()
        }
    }

    pub fn from(networks: Vec<TrainingNetwork>) -> Species {
        Species {
            networks: networks
        }
    }
}
