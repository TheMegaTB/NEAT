use Float;

/// Node inside a network that is just there to wrap around the sigmoid function and eventually other ones later
#[derive(Debug, RustcDecodable, RustcEncodable, Clone, PartialEq)]
pub struct Node {
    /// Flag to define whether or not the node has been executed (in the current 'round')
    pub executed: bool,
    /// A list of inputs that are all summed upon evaluation
    pub inputs: Vec<Float>,
    /// Final output value of node after evaluate() is called
    pub output: Float
}

impl Node {
    /// Initializes a single Node instance
    pub fn new() -> Node {
        Node {
            executed: false,
            inputs: Vec::new(),
            output: 0.0
        }
    }

    /// Creates a vector of nodes instances w/ a length of 'amount'
    pub fn multiple_new(amount: usize) -> Vec<Node> {
        (0..amount).map(|_| {
            Node::new()
        }).collect()
    }

    /// Starts the evaluation of the node returning the result
    pub fn evaluate(&mut self) -> Float {
        if !self.executed {
            let input_sum = self.inputs.iter().fold(0.0, |acc, input| {
                acc + input
            });
            self.executed = true;
            self.inputs.clear();
            // self.output = steep_sigmoid(input_sum);
            self.output = relu(input_sum);
        }
        self.output
    }

    /// Reset the node for a new calculation within the current instance (recurrent data is kept)
    pub fn reset(&mut self) {
        self.executed = false;
    }
}

fn relu(x: Float) -> Float {
    if x > 0.0 { x } else { 0.0 }
}

/// Steepened sigmoid function
fn steep_sigmoid(x: Float) -> Float {
    1.0 / ( 1.0 + (-4.9 * x).exp())
}

#[test]
fn sigmoid() {
    if cfg!(feature = "single_precision") {
        assert_eq!(steep_sigmoid(0.25), 0.77294225)
    } else if cfg!(feature = "double_precision") {
        assert_eq!(steep_sigmoid(0.25), 0.7729422593967386);
    }
}

#[test]
fn evaluate_empty() {
    // Sigmoid of 0 is 0.5
    assert_eq!(Node::new().evaluate(), 0.5);
}
