use sha2::{Digest, Sha256};

pub fn hash_64bit_numbers(numbers: &[u64]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    for &number in numbers {
        let bytes = number.to_be_bytes(); // Convert the number to bytes in big-endian format
        hasher.update(&bytes);
    }

    hasher.finalize().to_vec() // Return the resulting hash as a byte vector
}
