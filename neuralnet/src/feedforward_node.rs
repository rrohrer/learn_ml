use crate::node::Node;

/// The type of activation function that the feedforward node will use.
enum Activation {
    ReLU,
    SoftMax,
}

struct FeedForwardNone {
    activation: Activation,
    input_size: usize,
    output_size: usize,
    // Node Parameters
    weights: Vec<f32>,     // output_size * input_size
    biases: Vec<f32>,      // output_size
    activations: Vec<f32>, // output_size
    // Gradients used during training
    activation_gradients: Vec<f32>,
    // These two gradients are used to accumulate loss gradients
    weight_gradients: Vec<f32>,
    biases_gradients: Vec<f32>,
    // this is used to store temporary values in a single backpropagation pass
    input_gradients: Vec<f32>,
    // last input is used to compute loss gradients with respect to the wieghts
    // durring the backpropagation
    last_input: Vec<f32>,
}
