use species::Species;
use training_network::TrainingNetwork;
use neatwork::Network;

const POPULATION_SIZE: usize = 150;
pub type Score = f64;
pub type Probability = f32;

pub struct TrainingParameters {
    pub population_size: usize,
    pub cull_percentage: Probability,
    pub crossover_probability: Probability,
    pub add_gene_probability: Probability,
    pub add_node_probability: Probability,
    pub mutate_gene_probability: Probability,
    pub gene_enable_probability: Probability,
    pub gene_disable_probability: Probability
}

pub struct Trainer<F> where F: Fn(&mut TrainingNetwork) -> Score {
    parameters: TrainingParameters,
    species: Vec<Species>,
    eval_closure: F
}

impl<F> Trainer<F> where F : Fn(&mut TrainingNetwork) -> Score {
    pub fn new(parameters: TrainingParameters, inputs: usize, outputs: usize, closure: F) -> Trainer<F> {
        Trainer {
            species: vec![Species::from(
                (0..parameters.population_size).map(|_| {
                    TrainingNetwork::new(Network::new_empty(inputs, outputs))
                }).collect()
            )],
            parameters: parameters,
            eval_closure: closure
        }
    }

    fn calculate_scores(&mut self) {
        for species in self.species.iter_mut() {
            for network in species.networks.iter_mut() {
                network.calculate_score(&self.eval_closure);
            }
        }
    }

    pub fn get_best_network(&mut self) -> TrainingNetwork {
        let mut current_species_id = 0;
        let mut current_network_id = 0;
        let mut current_score = 0.0;
        self.calculate_scores();
        for (species_id, species) in self.species.iter_mut().enumerate() {
            for (network_id, network) in species.networks.iter_mut().enumerate() {
                if network.score > current_score {
                    current_species_id = species_id;
                    current_network_id = network_id;
                    current_score = network.score;
                }
            }
        }
        self.species[current_species_id].networks[current_network_id].clone()
    }

    fn delete_weak_species(&mut self) {
        let total_average_score = self.get_total_avg_score();
        let mut dead_species_ids = Vec::new();
        for (species_id, species) in self.species.iter_mut().enumerate() {
            species.calculate_score(&self.eval_closure); // This is overkill. A re-estimation from all the net scores would be sufficient instead of a full re-evaluation of every network
            let breed = species.score / total_average_score * self.parameters.population_size as f64;
            if breed < 1.0 {
                dead_species_ids.push(species_id);
            }
        }
        for dead_species_id in dead_species_ids.into_iter().rev() {
            self.species.swap_remove(dead_species_id);
        }
    }

    fn get_total_avg_score(&mut self) -> Score {
        let mut total_average_score = 0.0;
        for species in self.species.iter_mut() {
            total_average_score += species.calculate_score(&self.eval_closure);
        }
        total_average_score
    }

    fn add_to_population(&mut self, child: TrainingNetwork) {
        for species in self.species.iter_mut() {
            if species.networks[0].is_compatible_with(&child) {
                species.networks.push(child);
                return
            }
        }
        self.species.push(Species::from(vec!(child)));
    }

    fn next_generation(&mut self) {
        for species in self.species.iter_mut() {
            species.calculate_score(&self.eval_closure); // Calculate it so that cull can use it
            species.cull(self.parameters.cull_percentage);
        };
        self.delete_weak_species(); // This recalculates the species scores implicitly
        let tas = self.get_total_avg_score();
        let mut children = Vec::new();
        {
            let parameters = &self.parameters;
            for species in self.species.iter_mut() {
                let breed = (species.score / tas * self.parameters.population_size as f64) as usize - 1;
                children.append(&mut (1..breed).map(|_| {
                    species.breed(parameters)
                }).collect());
                species.cull(0.0);
            }
        }
        // insert random while loop here
        for child in children.into_iter() {
            self.add_to_population(child);
        }

        // println!("Amount of species: {}", self.species.len());
    }
}

impl<F> Iterator for Trainer<F> where F : Fn(&mut TrainingNetwork) -> Score {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        self.next_generation();
        Some(())
    }
}
