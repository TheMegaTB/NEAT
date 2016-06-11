/// A global definition whether or not double precision should be used.
///
/// = f32 for single, = f64 for double precision mode
/// Even though it theoretically enhances precision it shouldn't make the training faster/more precise.
/// In reality it makes it slower because it needs to calculate with more data
#[cfg(feature = "double_precision")]
pub type Float = f64;
#[cfg(feature = "single_precision")]
pub type Float = f32;

/// ID for identifying a genome uniquely across the whole population
pub type GID = usize;
/// ID for identifying a node. These ID's are local to a node
pub type NID = usize;
/// A link consisting of a source and a target node in a network.
pub type Link = (NID, NID);


// Probabilities
// GENE
pub const GENE_WEIGHT_MERGE_PROB: Float = 0.5;
pub const GENE_DISABLE_MERGE_PROB: Float = 0.5;

pub const GENE_MUT_RESET: Float = 0.1;
pub const GENE_MUT_STRENGTH: Float = 0.1;

// NETWORK
pub const NETWORK_MUT_ADD_GENE: Float = 0.4;
pub const NETWORK_MUT_ADD_NODE: Float = 0.4;
pub const NETWORK_MUT_MUTATE_GENE: Float = 0.2;
// mutate node
pub const NETWORK_MUT_CROSSOVER: Float = 0.1;
