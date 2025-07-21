#[derive(Debug, Default)]
pub struct Cursor {
    pub row: usize,
    pub column: usize,
}

impl Cursor {
    pub fn next_line(&mut self) {
        self.row += 1;
    }

    pub fn prev_line(&mut self) {
        self.row = self.row.max(1) - 1;
    }
}
