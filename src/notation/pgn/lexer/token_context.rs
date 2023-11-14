#[derive(Debug, Default)]
pub struct TokenContext<'a> {
    data: &'a str,
    line_ix: usize,
    col_ix: usize,
}

impl<'a> TokenContext<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }
    pub(crate) fn update(&mut self, line_ix: usize, col_ix: usize) {
        self.line_ix = line_ix;
        self.col_ix = col_ix;
    }
}
