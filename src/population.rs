use Species;
use Network;

/// The whole population of all lifeforms/networks/species
#[derive(Debug)]
pub struct Population {
    species: Vec<Species>
}

impl Population {
    pub fn new(size: usize, inputs: usize, outputs: usize) -> Population {
        Population {
            species: vec![Species::from(
                (0..size).map(|_| {
                    Network::new_empty(inputs, outputs)
                }).collect()
            )]
        }
    }
}
