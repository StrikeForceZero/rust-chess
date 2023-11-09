use sha2::{Sha256, Digest};

pub struct CustomStructIterator<'a, T> {
    pub data: &'a T,
    pub index: usize,
}

impl<'a, T> CustomStructIterator<'a, T> {
    pub const fn from(data: &'a T) -> CustomStructIterator<T> {
        Self {
            data,
            index: 0,
        }
    }
}

pub struct CustomStructIteratorMut<'a, T> {
    pub data: &'a mut T,
    pub index: usize,
}

impl<'a, T> CustomStructIteratorMut<'a, T> {
    pub fn from(data: &'a mut T) -> CustomStructIteratorMut<T> {
        Self {
            data,
            index: 0,
        }
    }
}


pub fn hash_64bit_numbers(numbers: &[u64]) -> Vec<u8> {
    let mut hasher = Sha256::new();

    for &number in numbers {
        let bytes = number.to_be_bytes(); // Convert the number to bytes in big-endian format
        hasher.update(&bytes);
    }

    hasher.finalize().to_vec() // Return the resulting hash as a byte vector
}
