/// The main trait for a `Node` in the Network Graph.
pub trait Node {
    /// Nodes need to be set to an initial state, and need to be reset.
    fn init(&mut self);

    /// Durring forward propagation nodes transform input data and feed
    /// results to all subsequent nodes.
    fn forward(&mut self, input: &[f32]);

    /// Durring reverse propagation nodes recieve loss gradients to its
    /// previous outputs and compute gradients with respect to each
    /// tunable parameter.
    fn reverse(&mut self, gradients: &[f32]);

    /// If this Node has tunable parameters, it should override this.
    /// This returns the number of parameters a Node has that can be tuned.
    fn parameter_count(&self) -> usize {
        0
    }

    /// Get a parameter by the index.
    fn parameter(&mut self, index: usize) -> Option<&mut f32> {
        None
    }

    /// Access the loss gradient with respect to the parameter index.
    fn gradient(&mut self, index: usize) -> Option<&mut f32> {
        None
    }

    /// return the human readable name of this node.
    fn name(&self) -> String {
        String::from("Default Node Name")
    }
}
