use crate::node::Node;
pub struct Model {
    name: String,
    nodes: Vec<Box<dyn Node>>,
}

impl Model {
    pub fn new(name: &str) -> Self {
        Model {
            name: String::from(name),
            nodes: Vec::new(),
        }
    }
}
