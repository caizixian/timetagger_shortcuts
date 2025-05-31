use rand::prelude::*;

static KEY_ALPHABET: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn generate_key(len: u32) -> String {
    // If the size of the alphabet is N
    // We essentially need to generate an N-based number with len digits
    // Which means we need to generate a number in [0, N^len)
    let upper_bound = KEY_ALPHABET.len().pow(len);
    let mut rng = rand::rng();
    let mut numerical_representation = rng.random_range(0..upper_bound);
    let alphabet_size = KEY_ALPHABET.len();
    // Now convert to string
    let mut chars = vec![];
    for _ in 0..len {
        chars.push(KEY_ALPHABET[numerical_representation % alphabet_size]);
        numerical_representation /= alphabet_size;
    }
    String::from_utf8(chars).expect("A valid key should be generated from an ASCII alphabet")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_length() {
        let key = generate_key(8);
        assert_eq!(key.len(), 8);
    }

    #[test]
    fn test_key_alphabet() {
        let key = generate_key(8);
        for c in key.chars() {
            let alphabet: String = String::from_utf8_lossy(KEY_ALPHABET).to_string();
            assert!(alphabet.contains(c));
        }
    }
}
