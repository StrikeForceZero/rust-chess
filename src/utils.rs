pub struct CustomStructIterator<'a, T> {
    pub data: &'a T,
    pub index: usize,
}

pub struct CustomStructIteratorMut<'a, T> {
    pub data: &'a mut T,
    pub index: usize,
}
