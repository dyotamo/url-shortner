use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_letters(length: usize) -> String {
    // Create a random number generator
    let rng = rand::thread_rng();

    // Define a distribution over alphanumeric characters
    let chars: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    chars
}
