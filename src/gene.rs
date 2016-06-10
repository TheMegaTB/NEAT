use rand::{thread_rng, Rng};

use NID;
use Float;
use Link;

/// Struct that represents a gene which in turn represents a connection/link inside a network
#[derive(Debug)]
pub struct Gene {
    /// Whether or not this gene has been disabled
    pub disabled: bool,
    /// The multiplier that is applied for data passing through this link
    weight: Float,
    /// Endpoints of the link
    pub link: Link
}

impl Gene {
    pub fn new_random(src: NID, dest: NID, disabled: bool) -> Gene {
        Gene {
            disabled: disabled,
            weight: thread_rng().gen::<Float>()*2.0 - 1.0,  // TODO: Improve this w/ a thread wide generator (aka species wide)
            link: (src, dest)
        }
    }

    pub fn evaluate(&self, input: Float) -> Float {
        input * self.weight
    }
}
