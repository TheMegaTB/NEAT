use rand::{thread_rng, Rng};
use {
    NID,
    Float,
    Link
};

const GENE_WEIGHT_MERGE_PROB: Float = 0.5;
const GENE_DISABLE_MERGE_PROB: Float = 0.5;

const GENE_MUT_RESET: Float = 0.1;
const GENE_MUT_STRENGTH: Float = 0.1; //0.5; //100% = 1.0

/// Struct that represents a gene which in turn represents a connection/link inside a network
#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
pub struct Gene {
    /// Whether or not this gene has been disabled
    pub disabled: bool,
    /// The multiplier that is applied for data passing through this link
    pub weight: Float,
    /// Endpoints of the link
    pub link: Link
}

impl Gene {
    fn random_weight() -> Float {
        thread_rng().gen::<Float>()*2.0 - 1.0
    }

    pub fn random(src: NID, dest: NID, disabled: bool) -> Gene {
        Gene {
            disabled: disabled,
            weight: Gene::random_weight(),  // TODO: Improve this w/ a thread wide generator (aka species wide)
            link: (src, dest)
        }
    }

    pub fn mutate(&mut self) {
        if thread_rng().gen::<Float>() < GENE_MUT_RESET {
            self.weight = Gene::random_weight();
        } else {
            // println!("{} =>", self.weight);
            self.weight += Gene::random_weight() * GENE_MUT_STRENGTH;
            // println!("{}", self.weight);
        }
    }

    pub fn with_weight(src: NID, dest: NID, disabled: bool, weight: Float) -> Gene {
        Gene {
            disabled: disabled,
            weight: weight,
            link: (src, dest)
        }
    }

    pub fn merge(&mut self, other: &Gene) {
        if !other.disabled && thread_rng().gen::<Float>() > GENE_WEIGHT_MERGE_PROB {
            self.weight = other.weight;
        }
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn enable(&mut self) {
        self.disabled = false;
    }

    pub fn evaluate(&self, input: Float) -> Float {
        input * self.weight
    }
}

impl PartialEq for Gene {
    fn eq(&self, other: &Gene) -> bool {
        self.link == other.link
    }

    fn ne(&self, other: &Gene) -> bool {
        !self.eq(other)
    }
}

#[test]
fn mutation() {
    let mut gene = Gene::random(1, 2, false);
    let old_weight = gene.weight;
    gene.mutate();
    assert!(gene.weight != old_weight);
}
