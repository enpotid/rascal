#[derive(Debug)]
pub struct Cursor {
    pub line_lengths: Vec<usize>,
    pub row: usize,
    pub column: usize,
}

impl Cursor {
    pub fn del_char(&mut self) {
        self.line_lengths[self.row] -= 1;
        self.column -= 1;
    }

    pub fn new_char(&mut self) {
        self.line_lengths[self.row] += 1;
        self.column += 1;
    }

    pub fn del_line(&mut self) {
        self.column = self.line_lengths[self.row - 1];
        self.line_lengths[self.row - 1] += self.line_lengths.remove(self.row);
        self.row -= 1;
    }

    pub fn new_line(&mut self) {
        self.line_lengths
            .insert(self.row + 1, self.line_lengths[self.row] - self.column);
        self.line_lengths[self.row] = self.column;
        self.row += 1;
        self.column = 0;
    }

    pub fn prev_char(&mut self) {
        if self.column == 0 {
            if self.row != 0 {
                self.row -= 1;
                self.column = self.line_lengths[self.row];
            }
        } else {
            self.column -= 1;
        }
    }

    pub fn next_char(&mut self) {
        if self.column >= self.line_lengths[self.row] {
            self.row = (self.row + 1).min(self.line_lengths.len() - 1);
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    pub fn prev_line(&mut self) {
        self.row = self.row.max(1) - 1;
        self.column = self.line_lengths[self.row].min(self.column);
    }

    pub fn next_line(&mut self) {
        self.row = (self.row + 1).min(self.line_lengths.len() - 1);
        self.column = self.line_lengths[self.row].min(self.column);
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            line_lengths: vec![0],
            row: 0,
            column: 0,
        }
    }
}
