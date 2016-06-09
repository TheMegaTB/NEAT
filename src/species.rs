use Network;

/// A niche that contains multiple networks to protect changes that are less performant at first
#[derive(Debug)]
pub struct Species {
    networks: Vec<Network>
}

impl Species {
    pub fn new() -> Species {
        Species {
            networks: Vec::new()
        }
    }

    pub fn from(networks: Vec<Network>) -> Species {
        Species {
            networks: networks
        }
    }
}
