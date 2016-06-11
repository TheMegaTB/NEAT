use rand::{thread_rng, Rng};
use NID;
use Float;
use Link;

/// Struct that represents a gene which in turn represents a connection/link inside a network
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Gene {
    /// Whether or not this gene has been disabled
    pub disabled: bool,
    /// The multiplier that is applied for data passing through this link
    pub weight: Float,
    /// Endpoints of the link
    pub link: Link
}

impl PartialEq for Gene {
    fn eq(&self, other: &Gene) -> bool {
        self.link == other.link
    }

    fn ne(&self, other: &Gene) -> bool {
        !self.eq(other)
    }
}

impl Gene {
    pub fn random(src: NID, dest: NID, disabled: bool) -> Gene {
        Gene {
            disabled: disabled,
            weight: thread_rng().gen::<Float>()*2.0 - 1.0,  // TODO: Improve this w/ a thread wide generator (aka species wide)
            link: (src, dest)
        }
    }

    pub fn with_weight(src: NID, dest: NID, disabled: bool, weight: Float) -> Gene {
        Gene {
            disabled: disabled,
            weight: weight,
            link: (src, dest)
        }
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn evaluate(&self, input: Float) -> Float {
        input * self.weight
    }
}
