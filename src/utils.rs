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
