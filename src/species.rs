use Network;

/// A niche that contains multiple networks to protect changes that are less performant at first
pub struct Species {
    networks: Vec<Network>
}
