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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_numeric_string_length() {
        let s = generate_numeric_string(10);
        assert_eq!(s.len(), 10);
    }

    #[test]
    fn test_generate_numeric_string_is_numeric() {
        let s = generate_numeric_string(20);
        assert!(s.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_numeric_string_zero_length() {
        let s = generate_numeric_string(0);
        assert_eq!(s, "");
    }

    #[test]
    fn test_generate_numeric_string_various_lengths() {
        for len in [1, 5, 50, 100].iter() {
            let s = generate_numeric_string(*len);
            assert_eq!(s.len(), *len);
            assert!(s.chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_generate_numeric_string_uniqueness() {
        let s1 = generate_numeric_string(16);
        let s2 = generate_numeric_string(16);
        // It's possible for them to be equal, but highly unlikely
        assert_ne!(s1, s2);
    }
}
