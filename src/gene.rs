use rand::{thread_rng, Rng};

use GID;
use NID;
use Float;
use Link;

/// Struct that represents a gene which in turn represents a connection/link inside a network
#[derive(Debug)]
pub struct Gene {
    // The unique ID of a gene. This is necessary for crossover. See GID for more information
    // id: GID, => now stored in the network genome hashmap rather than the gene itself
    /// Whether or not this gene has been disabled
    disabled: bool,
    /// The multiplier that is applied for data passing through this link
    weight: Float,
    /// Endpoints of the link
    link: Link
}

impl Gene {
    pub fn new_random(gid_counter: &mut GID, src: NID, dest: NID, disabled: bool) -> (GID, Gene) {
        *gid_counter += 1;
        (*gid_counter - 1, Gene {
            disabled: disabled,
            weight: thread_rng().gen::<Float>()*2.0 - 1.0,  // TODO: Improve this w/ a thread wide generator (aka species wide)
            link: (src, dest)
        })
    }
}

#[test]
fn gid_increases() {
    let mut gid_counter = 0;
    Gene::new_random(&mut gid_counter, 1, 2, false);
    assert!(0 != gid_counter);
    assert_eq!(gid_counter, 1);
}
