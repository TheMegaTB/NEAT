#![allow(dead_code)]
use num_cpus;
use std::cmp::Ordering;
use rand::Rng;
use std::thread;
use pbr::ProgressBar;
use std::sync::{Arc, Mutex};
use nn::NeuralNetwork;
use freefall;

pub struct TrainerConfig {
    /// Amount of clients that one given generation consists of
    generation_size: usize,
    /// Amount of survivors that get mutated to the next generation
    survivor_count: usize,
    /// Amount of games a client takes place in to calculate the avg. score
    games_per_client: usize,
    /// Percentage how many parameters should be changed while mutating
    mutation_amount: f32,
    /// Value by what the weights should be updated
    mutation_strength: f32,
    /// Structure of the NeuralNetwork
    structure: Vec<usize>
}

impl TrainerConfig {
    pub fn build_new_trainer<R: Rng>(&self, rng: &mut R) -> Trainer {
        println!("Generating random networks . . .");
        let mut pb = ProgressBar::new(self.generation_size);
        let t = Trainer {
            survivor_count: self.survivor_count,
            games_per_client: self.games_per_client,
            mutation_amount: self.mutation_amount,
            mutation_strength: self.mutation_strength,
            current_generation:
                TrainerConfig::generate_random_generation(&self.structure, self.generation_size, rng, &mut pb)
        };
        t
    }

    pub fn new() -> TrainerConfig {
        TrainerConfig {
            generation_size: 100, //1600
            survivor_count: 20, //200
            games_per_client: 75,
            mutation_amount: 0.50, //Percentage
            mutation_strength: 0.129,
            structure: vec![3, 60, 100, 100, 60, 20, 1]
        }
    }

    fn generate_random_generation<R: Rng>(structure: &Vec<usize>, size: usize, rng: &mut R, p: &mut ProgressBar) -> Vec<NeuralNetwork> {
        (0..size).map(|_| {
            p.inc();
            NeuralNetwork::new_random(structure.clone(), rng)
        }).collect()
    }
}

pub struct Trainer {
    survivor_count: usize,
    games_per_client: usize,
    mutation_amount: f32,
    mutation_strength: f32,
    current_generation: Vec<NeuralNetwork>,
}

impl Trainer {
    pub fn step<R: Rng>(&mut self, rng: &mut R) -> NeuralNetwork {

        let pb = ProgressBar::new(self.current_generation.len()*self.games_per_client);
        let pb = Arc::new(Mutex::new(pb));

        let generation_size = self.current_generation.len();
        let current_generation = self.current_generation.clone();
        self.current_generation = Vec::with_capacity(generation_size);

        let contestants_per_thread: usize = (generation_size as f32 / num_cpus::get() as f32).ceil() as usize;
        let mut threads = Vec::with_capacity(self.current_generation.len() / contestants_per_thread);
        current_generation.into_iter().enumerate().fold( Vec::new(), |mut contestants, (index, contestant)| {
            if (index + 1) % contestants_per_thread == 0 {
                let pb = pb.clone();
                let games_per_client = self.games_per_client.clone();
                contestants.push(contestant);
                threads.push(thread::spawn(move || {
                    contestants.into_iter().map(move |contestant| {
                        let score_sum = (0..games_per_client).map(|_| {
                            let score = freefall::ff(&contestant, false);
                            pb.lock().unwrap().inc();
                            score
                        }).fold(0, |acc, score| score + acc );
                        let avg_score = score_sum as f32 / games_per_client as f32;

                        (contestant, avg_score)
                    }).collect::<Vec<_>>()
                }));
                Vec::new()
            } else {
                contestants.push(contestant);
                contestants
            }
        });

        let mut results = threads.into_iter().flat_map(|thread_handle| {
            match thread_handle.join().ok() {
                Some(contestants) => {
                    contestants
                }, None => {
                    println!("ERROR - Couldn't join thread handle!");
                    Vec::new()
                }
            }
        }).collect::<Vec<_>>();

        results.sort_by(|a, b| if a.1 < b.1 {Ordering::Less} else {Ordering::Greater});

        println!("");
        println!("Best average score of generation is: {}", results[0].1);

        let best_nn = results[0].0.clone();

        let mutation_per_survivor = (generation_size - self.survivor_count) / self.survivor_count;
        self.current_generation = results.into_iter().take(self.survivor_count).flat_map(|survivor| {
            let survivor = survivor.0;
            let mut mutations = (0..mutation_per_survivor).map(|_| {
                let mut mutation = survivor.clone();
                mutation.mutate(self.mutation_amount, self.mutation_strength, rng);
                mutation
            }).collect::<Vec<_>>();

            mutations.push(survivor);
            mutations
        }).collect::<Vec<_>>();

        best_nn
    }
}
