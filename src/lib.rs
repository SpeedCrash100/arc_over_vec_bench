pub type DataType = u8;

/// Generates a vector with size `size` elements
pub fn generate_vec(size: usize) -> Vec<DataType> {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    (0..size).map(|_| rng.gen_range(0..255)).collect()
}
