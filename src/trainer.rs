use rand::{thread_rng, Rng};
use TrainingNetwork;
use Population;
use Species;
use neatwork::{GID, NID};

const POPULATION_SIZE: usize = 150;
pub type Score = f64;
pub type Stats = Vec<(Score, (GID, NID, usize))>;

pub struct Trainer<F> where F: Fn(&mut TrainingNetwork) -> Score {
    pub population: Population, //TODO make private
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
                net.network.reset();
                net.score = Some(score);
                score
            }
        }
    }

    pub fn get_stats(&mut self) -> Stats {
        (0..self.population.species.len()).fold(Vec::new(), |mut species_scores, species_id| {
            let species_size = self.population.species[species_id].networks.len();
            let score = (0..species_size).fold(0.0, |acc, net_id| {
                acc + self.get_score((species_id, net_id))
            }) / species_size as f64;
            let size = (0..species_size).fold((0, 0, 0), |(acc1, acc2, acc3), net_id| {
                let size = self.population.species[species_id].networks[net_id].network.get_size();
                (acc1 + size.0, acc2 + size.1, acc3 + size.2)
            });

            species_scores.push((score, (size.0 / species_size, size.1 / species_size, size.2 / species_size)));
            species_scores
        })
    }

    pub fn mutate_networks(&mut self) {
        let mut to_move = Vec::new();
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
                    network.score = None;
                    (
                        network.network.mutate(&other_network.network, net_score > other_net_score),
                        if net_score > other_net_score { other_network_id } else { network_id }
                    )
                };
                match mutation_result.1 {
                    Some(new_net) => {
                        let new_net = TrainingNetwork::new(new_net);
                        let species_nets = &mut self.population.species[species_id].networks;
                        species_nets[worse_net_id] = new_net;
                    },
                    None => {
                        if mutation_result.0 > 0 {
                            to_move.push((species_id, network_id, mutation_result.0));
                        }
                    }
                }
            }
        }

        for &(species_id, network_id, protection) in to_move.iter().rev() {
            let net = {
                let species = &mut self.population.species[species_id];
                if species.networks.len() == 1 { continue; }
                species.networks[network_id].clone()
            };
            self.population.species.push(Species::from_network_with_protection(net, protection));
        }
    }

    pub fn get_best_network(&mut self) -> TrainingNetwork {
        let mut current_species_id = 0;
        let mut current_network_id = 0;
        for species_id in 0..self.population.species.len() {
            let species_size = self.population.species[species_id].networks.len();
            for network_id in 0..species_size {
                if self.get_score((current_species_id, current_network_id)) < self.get_score((species_id, network_id)) {
                    current_species_id = species_id;
                    current_network_id = network_id;
                }
            }
        }
        // println!("Fitness of best network: {:?}", self.get_score((current_species_id, current_network_id)));
        self.population.species[current_species_id].networks[current_network_id].clone()
    }

    pub fn speciation(&mut self) {
        let mut to_move = Vec::new();
        let mut empty_species = Vec::new();

        for species_id in 0..self.population.species.len() {
            let species = &mut self.population.species[species_id];
            let protection_worn_out = species.protection == 1;
            if species.protection > 0 { species.protection -= 1; }
            if protection_worn_out || species.protection == 0 {
                let species_size = species.networks.len();
                let mut moved_network_amount = 0;
                for network_id in ( if protection_worn_out {0} else {1} )..species_size {
                    if protection_worn_out || !species.networks[0].network.is_compatible_with(&species.networks[network_id].network) {
                        to_move.push((species_id, network_id));
                        moved_network_amount+=1;
                    }
                }
                if moved_network_amount == species_size { empty_species.push(species_id); }
            }
        }

        for &(species_id, network_id) in to_move.iter().rev() {
            let net = self.population.species[species_id].networks.swap_remove(network_id);
            for other_species_id in 0..self.population.species.len() {
                if species_id != other_species_id {
                    if net.network.is_compatible_with(&self.population.species[other_species_id].networks[0].network) {
                        self.population.species[other_species_id].networks.push(net.clone());
                        break;
                    }
                }
            }
            self.population.species.push(Species::from(vec!(net)));
        }

        for species in empty_species.into_iter().rev() {
            self.population.species.swap_remove(species);
        }
    }
}

impl<F> Iterator for Trainer<F> where F : Fn(&mut TrainingNetwork) -> Score {
    type Item = Stats;
    fn next(&mut self) -> Option<Self::Item> {
        self.mutate_networks();
        self.speciation();
        Some(self.get_stats())
    }
}
