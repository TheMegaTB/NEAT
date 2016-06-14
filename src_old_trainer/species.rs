use TrainingNetwork;

/// A niche that contains multiple networks to protect changes that are less performant at first
#[derive(Debug)]
pub struct Species {
    pub networks: Vec<TrainingNetwork>,
    pub protection: u16
}

impl Species {
    pub fn new() -> Species {
        Species {
            networks: Vec::new(),
            protection: 0
        }
    }

    pub fn from(networks: Vec<TrainingNetwork>) -> Species {
        Species {
            networks: networks,
            protection: 0
        }
    }

    pub fn from_network_with_protection(network: TrainingNetwork, protection: u16) -> Species {
        Species {
            networks: vec!(network),
            protection: protection
        }
    }
}
