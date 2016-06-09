use Species;
use Network;
use GID;

/// The whole population of all lifeforms/networks/species
#[derive(Debug)]
pub struct Population {
    species: Vec<Species>,
    pub gid_counter: GID
}

impl Population {
    pub fn new(size: usize, inputs: usize, outputs: usize) -> Population {
        let mut gid_counter = 0;
        Population {
            species: vec![Species::from(
                (0..size).map(|_| {
                    Network::new_empty(inputs, outputs, &mut gid_counter)
                }).collect()
            )],
            gid_counter: gid_counter
        }
    }
}

#[test]
fn gid_increases() {
    let p = Population::new(1, 4, 1);
    assert_eq!(p.gid_counter, 4);
}
