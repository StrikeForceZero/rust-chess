pub struct CustomStructIterator<'a, T> {
    pub data: &'a T,
    pub index: usize,
}

impl<T> CustomStructIterator<'_, T> {
    pub const fn from(data: &T) -> CustomStructIterator<T> {
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

impl<T> CustomStructIteratorMut<'_, T> {
    pub const fn from(data: &mut T) -> CustomStructIteratorMut<T> {
        Self {
            data,
            index: 0,
        }
    }
}
