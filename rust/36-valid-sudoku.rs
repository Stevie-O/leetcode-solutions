type DigitUseMap = [u16; 9];
#[derive(Default)]
struct SudokuBoard {
    rows: DigitUseMap,
    cols: DigitUseMap,
    sqrs: DigitUseMap,
}
impl SudokuBoard {
    fn find_square(row : usize, col : usize) -> usize {
        let srow = row / 3;
        let scol = col / 3;
        srow * 3 + scol
    }
    pub fn try_add_digit(&mut self, row : usize, col : usize, digit: char) -> bool {
        let digit = (digit.to_digit(10).unwrap() - 1) as u8;
        let sqr = Self::find_square(row, col);
        let digmask = 1_u16 << digit;
        if ( (self.rows[row] & digmask) == 0 &&
             (self.cols[col] & digmask) == 0 &&
             (self.sqrs[sqr] & digmask) == 0){
            self.rows[row] |= digmask;
            self.cols[col] |= digmask;
            self.sqrs[sqr] |= digmask;
            true
        } else {
            false
        }
    }
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut sudoku = SudokuBoard::default();
        board.iter().enumerate().all(
            |(row, cols)|
            cols.iter().enumerate()
            .filter(|(_, &digit)| match digit { '1'..='9' => true, _ => false })
            .all(
                |(col, &digit)|
                sudoku.try_add_digit(row, col, digit)
            )
        )
    }
}
