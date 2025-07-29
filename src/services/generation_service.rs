use rand::{thread_rng, Rng};

pub fn generate_numeric_string(length: usize) -> String {
    let mut rng = thread_rng();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        let digit: u8 = rng.gen_range(0..=9); // Generate a random digit (0-9)
        result.push(char::from(b'0' + digit)); // Convert digit to char and append
    }
    result
}
