use rustc_serialize::json::{EncoderError, DecoderError, self};
use rand::Rng;

pub type Float = f64;
pub type Location = (usize, usize);
const PREALLOC_IO: usize = 576;
pub const INPUT_LENGTH: usize = 5761;//1729;
pub const OUTPUT_LENGTH: usize = 576;
const GRADIENT: Float = 1.0;

#[derive(Debug)]
pub enum Error {
    InputLengthMismatch
}

impl Error {
    fn print(&self) -> String {
        match *self {
            Error::InputLengthMismatch => {
                "Input length mismatched expected length.".to_string()
            }
        }
    }
}

enum Interpolation {
    Sigmoid,
    Linear(Float)
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Neuron {
    pub bias: Float,
    pub weights: Vec<Float>
}

impl Neuron {
    pub fn new(bias: Float) -> Neuron {
        Neuron {
            bias: bias,
            weights: Vec::with_capacity(PREALLOC_IO)
        }
    }

    pub fn new_random<R: Rng>(weight_count: usize, rng: &mut R) -> Neuron {
        Neuron {
            bias: rng.next_f64() * 2.0 - 1.0,//TODO: Set some random bias //rng.next_f64()*weight_count as f64,
            weights: (0..weight_count).map(|_| rng.next_f64() * 2.0 - 1.0).collect()
        }
    }

    fn calculate(&self, inputs: &Vec<Float>, interpolation: Interpolation) -> Float {
        let weighted_sum = inputs.iter().zip(self.weights.iter())
            .fold(self.bias, |weighted_sum, (input, weight)| {
                weighted_sum + input * weight
            });
        match interpolation {
            Interpolation::Sigmoid => {
                1f64 / (1f64 + (-weighted_sum).exp())
            },
            Interpolation::Linear(gradient) => {
                gradient * weighted_sum
            }
        }
    }
}

#[derive(Clone)]
pub struct NeuralNetwork {
    hidden: Vec<Vec<Neuron>>
}

impl NeuralNetwork {
    pub fn new_random<R: Rng>(assembly: Vec<usize>, rng: &mut R) -> NeuralNetwork {
        NeuralNetwork {
            hidden: assembly.windows(2).map(|window| {
                (0..window[1]).map(|_| Neuron::new_random(window[0], rng)).collect()
            }).collect()
        }
    }

    pub fn calculate(&self, input: Vec<Float>, gradient: Float) -> Result<Vec<Float>, Error> {
        if !(input.len() == self.hidden[0][0].weights.len()) {
            Err(Error::InputLengthMismatch)
        } else {
            Ok(self.hidden.iter().enumerate().fold(input, |input, (index, layer)| {
                layer.iter().map(|neuron| {
                    neuron.calculate(&input,
                        if index == (self.hidden.len()-1) {
                            Interpolation::Linear(gradient)
                        } else {
                            Interpolation::Sigmoid
                        })
                }).collect()
            }))
        }
    }

    pub fn mutate<R: Rng>(&mut self, amount: f32, strength: f32, rng: &mut R) {
        for layer in self.hidden.iter_mut() {
            for neuron in layer.iter_mut() {
                if rng.next_f32() < amount {
                    neuron.bias += (rng.next_f64() * 2.0 - 1.0) * strength as f64 * 0.001;
                    if neuron.bias > 1.0 {
                        neuron.bias = 100.0;
                    } else if neuron.bias < -1.0 {
                        neuron.bias = -100.0;
                    }
                }
                for weight in neuron.weights.iter_mut() {
                    if rng.next_f32() < amount {
                        *weight += (rng.next_f64() * 2.0 - 1.0) * strength as f64;
                        if *weight > 1.0 {
                            *weight = 1.0;
                        } else if *weight < -1.0 {
                            *weight = -1.0;
                        }
                    }
                }
            }
        }
    }

    pub fn encode(&self) -> Result<String, EncoderError> { json::encode(&self.hidden) }

    pub fn decode(input: String) -> Result<NeuralNetwork, DecoderError> {
        json::decode(&input).map(|hidden_layer|
            NeuralNetwork {
                hidden: hidden_layer
            }
        )
    }
}
