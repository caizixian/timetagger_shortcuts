use rand::distr::{Alphanumeric, SampleString};

pub(crate) fn generate_key(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
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
            assert!(c.is_alphabetic());
        }
    }
}
