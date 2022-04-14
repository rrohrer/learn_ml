mod mnist_node;
mod model;
mod node;

use mnist_node::MnistNode;
use model::Model;
use node::Node;
use rand::rngs::SmallRng;
use rand::SeedableRng;

fn main() {
    let mut model = Model::new("Silly Model", 0);
    let mut mnist = MnistNode::new(
        "/Users/ryanrohrer/code/mnist/train-images.idx3-ubyte",
        "/Users/ryanrohrer/code/mnist/train-labels.idx1-ubyte",
    );
    mnist.init(SmallRng::seed_from_u64(0));
    mnist.read_next();
    mnist.print_last_read();
    mnist.read_next();
    mnist.print_last_read();
    mnist.read_next();
    mnist.print_last_read();
    mnist.read_next();
    mnist.print_last_read();
    mnist.read_next();
    mnist.print_last_read();
    mnist.read_next();
    mnist.print_last_read();
    mnist.read_next();
    mnist.print_last_read();
    mnist.read_next();
    mnist.print_last_read();
}
