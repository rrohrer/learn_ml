use crate::node::Node;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::cell::RefCell;

pub struct Model {
    name: String,
    nodes: Vec<RefCell<Box<dyn Node>>>,
    rng: SmallRng,
}

impl Model {
    /// Create a new model with a given name.
    pub fn new(name: &str, seed: u64) -> Self {
        Model {
            name: String::from(name),
            nodes: Vec::new(),
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    /// Adds a node to the graph, and returns its index.
    pub fn add_node(&mut self, node: Box<dyn Node>) -> usize {
        let result = self.nodes.len();
        self.nodes.push(RefCell::new(node));
        result
    }

    /// create an edge connecting two nodes
    pub fn create_edge(&mut self, src: usize, dst: usize) {
        self.nodes[src].borrow_mut().add_subsequent_node(dst);
        self.nodes[dst].borrow_mut().add_antecent_node(src);
    }

    /// initialize all of the nodes in the model, using the given seed
    pub fn init(&mut self) {
        for node in &mut self.nodes {
            node.borrow_mut().init(self.rng.clone());
        }
    }

    /// retuns the name of the model.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Create a model using the given optimizer
    pub fn train<Optimizer>(optimizer: Optimizer) {}
}
