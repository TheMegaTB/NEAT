use rand::{thread_rng, Rng};
use TrainingNetwork;
use Population;

const POPULATION_SIZE: usize = 150;
pub type Score = f64;

pub struct Trainer<F> where F: Fn(&mut TrainingNetwork) -> Score {
    population: Population,
    eval_closure: F
}

impl<F> Trainer<F> where F : Fn(&mut TrainingNetwork) -> Score {
    pub fn new(inputs: usize, outputs: usize, closure: F) -> Trainer<F> {
        Trainer {
            population: Population::new(POPULATION_SIZE, inputs, outputs),
            eval_closure: closure
        }
    }

    fn get_score(&mut self, network_id: (usize, usize)) -> Score {
        let net = &mut self.population.species[network_id.0].networks[network_id.1];
        match net.score {
            Some(score) => score,
            None => {
                let score = (&self.eval_closure)(net);
                net.score = Some(score);
                score
            }
        }
    }

    pub fn mutate_networks(&mut self) {
        for species_id in 0..self.population.species.len() {
            let species_size = self.population.species[species_id].networks.len();
            for network_id in 0..species_size {
                let other_network_id = thread_rng().gen_range(0, species_size);

                let other_net_score = self.get_score((species_id, other_network_id));
                let net_score = self.get_score((species_id, network_id));

                let (mutation_result, worse_net_id) = {
                    let species = &mut self.population.species[species_id];
                    let other_network = species.networks[other_network_id].clone();
                    let network = &mut species.networks[network_id];
                    (
                        network.network.mutate(&other_network.network, net_score > other_net_score),
                        if net_score > other_net_score { other_network_id } else { network_id }
                    )
                };
                match mutation_result {
                    Some(new_net) => {
                        let new_net = TrainingNetwork::new(new_net);
                        let species_nets = &mut self.population.species[species_id].networks;
                        species_nets[worse_net_id] = new_net;
                    },
                    None => {}
                }
            }
        }
    }
}

impl<F> Iterator for Trainer<F> where F : Fn(&mut TrainingNetwork) -> Score {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        self.mutate_networks();
        Some(())
    }
}
