use crate::node::Node;
use rand::rngs::SmallRng;
use std::fs::File;
use std::io::Read;

const DIMENSIONS: usize = 28 * 28;

#[derive(Default)]
pub struct MnistNode {
    rng: Option<SmallRng>,
    images_path: String,
    labels_path: String,
    images_file: Option<File>,
    labels_file: Option<File>,
    antecedents: Vec<usize>,
    subsequents: Vec<usize>,
    label: [f32; 10],
    data: Vec<f32>,
    image_count: Option<usize>,
}

impl Node for MnistNode {
    fn init(&mut self, rng: SmallRng) {
        self.rng = Some(rng);

        self.images_file = Some(File::open(&self.images_path).unwrap());
        self.labels_file = Some(File::open(&self.labels_path).unwrap());

        self.read_mnist_header();
    }
    fn forward(&mut self, input: &[f32]) {}
    fn reverse(&mut self, gradients: &[f32]) {}
    fn add_antecent_node(&mut self, index: usize) {
        self.antecedents.push(index);
    }
    fn add_subsequent_node(&mut self, index: usize) {
        self.subsequents.push(index);
    }
}

impl MnistNode {
    pub fn new(images_path: &str, labels_path: &str) -> Self {
        MnistNode {
            rng: None,
            images_path: String::from(images_path),
            labels_path: String::from(labels_path),
            ..Default::default()
        }
    }

    pub fn label(&self) -> [f32; 10] {
        self.label
    }

    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }

    pub fn read_next(&mut self) {
        let mut raw_buffer = [0; DIMENSIONS];
        // read the next image from the input file.
        self.images_file
            .as_mut()
            .unwrap()
            .read(&mut raw_buffer)
            .unwrap();

        // normalize each pixel between 0 and 1
        self.data = raw_buffer.into_iter().map(|x| x as f32 / 255.0).collect();

        // read the label byte in.
        let mut label_buffer = [0; 1];
        self.labels_file
            .as_mut()
            .unwrap()
            .read(&mut label_buffer)
            .unwrap();

        // Store the label as a 1-hot encoding so we can use it in loss calculations.
        let mut label = [0.0; 10];
        label[label_buffer[0] as usize] = 1.0;
        self.label = label;
    }

    fn read_mnist_header(&mut self) {
        let images = self.images_file.as_mut().unwrap();
        if read_be_u32(images).unwrap() != 2051 {
            panic!("Images file is malformed!");
        }

        let labels = self.labels_file.as_mut().unwrap();
        if read_be_u32(labels).unwrap() != 2049 {
            panic!("Labels file is malformed!");
        }

        let image_count = read_be_u32(images).unwrap();
        let label_count = read_be_u32(labels).unwrap();
        if image_count != label_count {
            panic!("There were not the same number of input images and input labels!");
        }

        let image_rows = read_be_u32(images).unwrap();
        let image_columns = read_be_u32(images).unwrap();
        if image_rows != 28 || image_columns != 28 {
            panic!("Image from MNIST dataset are always 28x28! This is not valid data!");
        }

        self.image_count = Some(image_count as usize);
        println!("Successfully loaded {} images from file.", image_count);
    }
}

/// helper to read big endian u32s from a file.
fn read_be_u32(file: &mut dyn Read) -> std::io::Result<u32> {
    let mut buffer = [0; 4];
    file.read(&mut buffer)?;
    Ok(u32::from_be_bytes(buffer))
}
