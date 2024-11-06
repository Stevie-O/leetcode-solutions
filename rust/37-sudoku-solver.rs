struct Solution{}

use std::fmt::{Formatter, Debug};

// 37. Sudoku Solver
// Sudoku is rather interesting, structurally:
// - there are 9x9 = 81 squares in the grid
// - there are 9x (3x3) subgrids
// - because every number is placed exactly once in each row, column, and subgrid,
//      - for each cell, we need to track which of the 9 digits can be placed in that cell
//      - for each digit,
//              for each row, column, and subgrid, whe need to track which of the 9 squares in that row/column/subgrid
//              that digit can be placed in
//
// so: i16s abound.  I use i16 so I can use a negative number (either -1 or i16::MIN) to represent a digit that has been placed already

// this is for 9x9 sudoku
// the 6x6 sudoku that Puzzle Page uses is 6 symbols, box width 3, box height 2

// Terms:
//      GRID    = the entire puzzle area.  Always square, as all ROWs and all COLUMNs must be equal length.
//      SYMBOL  = the symbols that can be placed into the GRID.  For traditional Sudoku, these are digits (normally 1-9).
//                The internal implementation assigns each SYMBOLS a number from 0..NUM_SYMBOLS and operates exlusively on that number.
//      CELL    = an element of the GRID that contains exactly one SYMBOL at the end.
//      ROW     = A horizontal run of CELLs. Size is equal to the number of SYMBOLs.
//      COLUMN  = A vertical run of CELLs. Size is equal to the number of SYMBOLs. 
//      BOX     = A rectangular subdivision of the grid. Size is equal to the number of SYMBOLs.
//                Boxes do not overlap and collectively cover the entirety of the grid.
//                As a consequence of this, the the total number of BOXes is always equal to the total number of SYMBOLs
//      REGION  = a ROW, COLUMN, or BOX.
//                Each REGION contains every SYMBOL exactly once.


// BOX indices are laid out left-to-right, top-to-bottom
// so for a typical 9x9 Sudoku

const NUM_SYMBOLS : usize = 9;
const BOX_WIDTH : usize = 3;
const BOX_HEIGHT : usize = 3;
// PlacementMask is used in two different ways, which relies on the fact that every symbol must occur exactly once in each
// (row, column, box).
// When associated with a grid cell, indicates _which symbol is placed in that cell_ or _which symbols may be stored placed in that cell_
// When associated with a symbol and region, indicates _which position in that region that symbol is placed_ or _which positions in that region that symbol may be placed_.
//      specifically:
//          if negative, equal to i32::MIN plus the symbol or position index
//          if positive, a bitmask where (1<<n) is set if (symbol/position) N is a candiate.
type PlacementMaskType = i16;

const NUM_REGION_TYPES : usize = 3; // rows, columns, boxes

// INVARIANTS. DO NOT CHANGE.
const GRID_SIZE : usize = NUM_SYMBOLS * NUM_SYMBOLS;
const REGION_SIZE : usize = NUM_SYMBOLS;  // number of cells in each region (row, column, or box)
const NUM_REGIONS : usize = NUM_SYMBOLS;  // number of regions of each type
// need a static assertion: BOX_WIDTH * BOX_HEIGHT is equal to REGION_SIZE
// as long as all regions are the same size, this assertion is sufficient to
// prove several other things that absolutely need to be true
const BOXES_PER_ROW : usize = REGION_SIZE / BOX_WIDTH;

// each cell has NUM_CONFLICTS cells that it conflicts with
// [MAYBE] itself (this might make some of the solver logic easier)
// the (BOX_SIZE-1) other cells in the same box
// the (ROW_WIDTH-BOX_WIDTH) cells in the same row but not the same box
// the (COL_HEIGHT-BOX_HEIGHT) cells in the same column but not the same box
const NUM_CONFLICTS : usize = (REGION_SIZE - 1) + (REGION_SIZE - BOX_WIDTH) + (REGION_SIZE - BOX_HEIGHT);


// .0 = region ID (0..NUM_REGIONS), .1 = index (0..REGION_SIZE) within that region
type RegionIdAndLocation = (usize, usize);

type RegionCellMap = [[usize; NUM_SYMBOLS]; NUM_REGIONS];

struct SudokuSchemeGenerator {}

impl SudokuSchemeGenerator {
    const fn compute_rows() -> RegionCellMap {
        // this can be dramatically reworked if we ever get a const version of array::from_fn()
        // https://gendignoux.com/blog/2024/06/17/const-array-from-fn.html
         let mut out = [[0; NUM_SYMBOLS]; NUM_REGIONS];
         let mut row = 0;
         let mut grid_cell = 0;
         while row < NUM_REGIONS {
             let mut col = 0;
             while col < NUM_SYMBOLS {
                 out[row][col] = grid_cell;
                 grid_cell += 1;
                 col += 1;
             }
             row += 1;
         }
         out
    }
    const fn compute_cols() -> RegionCellMap {
         let mut out = [[0; NUM_SYMBOLS]; NUM_REGIONS];
         let mut col = 0;
         while col < NUM_REGIONS {
             let mut grid_cell = col;
             let mut row = 0;
             while row < NUM_SYMBOLS {
                 out[row][col] = grid_cell;
                 row += 1;
                 grid_cell += NUM_SYMBOLS; // aka ROW_WIDTH
             }
             col += 1;
         }
         out
    }
    const fn compute_boxes() -> RegionCellMap {
         let mut out = [[0; NUM_SYMBOLS]; NUM_REGIONS];
         // grid cell index of top-left corner of box
         let mut box0_grid_cell = 0;
         // grid row index of top-left corner of box
         //let mut box0_row = 0;
         
         // grid column index of top-left corner of box
         let mut box0_col = 0;
         // box number (index into @out)
         let mut boxn = 0;
         while boxn < NUM_REGIONS {
             let mut grid_cell = box0_grid_cell;
             let mut box_row = 0; // row offset of current cell from box0_row
             let mut box_col = 0; // col offset of current cell from box0_col
             let mut box_cell = 0; // index into out
             while box_cell < NUM_SYMBOLS {
                 out[boxn][box_cell] = grid_cell;
                 grid_cell += 1;
                 box_col += 1;
                 if box_col >= BOX_WIDTH {
                     box_col = 0;
                     box_row += 1;
                     grid_cell += REGION_SIZE /* aka ROW_WIDTH aka ROW_SIZE */
                                  - BOX_WIDTH;
                 }
                 box_cell += 1;
             }
             assert!(box_row == BOX_HEIGHT);
             // move to the next box
             boxn += 1;
             box0_col += BOX_WIDTH;
             box0_grid_cell += BOX_WIDTH;
             if box0_col >= NUM_SYMBOLS {
                box0_col = 0;
                // box0_grid_cell is already correct
                // turns out I didn't need this
                //box0_row += BOX_HEIGHT;
             }
         }
         out
    }
    // computes all of the cells that are mutually exclusive with the cell at (row, col)
    const fn compute_cell_conflicts(row: usize, col: usize) -> [usize; NUM_CONFLICTS] {
        let mut out = [0; NUM_CONFLICTS];
        let mut out_index = 0;
        // find out where the box starts
        let box0_row = (row / BOX_HEIGHT) * BOX_HEIGHT;
        let box0_col = (col / BOX_WIDTH)  * BOX_WIDTH;
        let mut box_row = 0;
        let mut box_col = 0;
        let mut grid_cell = box0_row * REGION_SIZE /* aka ROW_WIDTH */ + box0_col;
        
        // some designs are simpler if we include the cell itself in the list
        // if we're doing that, ensure it is always the FIRST ARRAY ELEMENT,
        // so for logic that wants to exclude it, it can just use [1..]
        // out[out_index] = grid_cell; out_index += 1;
        
        while box_row < BOX_HEIGHT {
            if !(box0_row + box_row == row && box0_col + box_col == col) {
                out[out_index] = grid_cell; out_index += 1;
            }
            grid_cell += 1;
            box_col += 1;
            if box_col >= BOX_WIDTH {
                box_col = 0;
                grid_cell += REGION_SIZE /* aka ROW_WIDTH */ - BOX_WIDTH;
                box_row += 1;
            }
        }
        //assert_eq!(out_index, REGION_SIZE - 1);
        assert!(out_index == REGION_SIZE - 1);

        // okay, add all cells from the same row that aren't in the same box
        // (this helpfuly also excludes the grid cell at [row, col])
        let mut row_col = 0;
        let mut grid_cell = row * REGION_SIZE /* aka ROW_WIDTH */;
        // cover all columns to the left of the box
        while row_col < box0_col {
            out[out_index] = grid_cell; out_index += 1;
            grid_cell += 1;
            row_col += 1;
        }
        // skip past the box
        row_col += BOX_WIDTH;
        grid_cell += BOX_WIDTH;
        // cover all columns to the right of the box
        while row_col < REGION_SIZE /* aka ROW_WIDTH */ {
            out[out_index] = grid_cell; out_index += 1;
            grid_cell += 1;
            row_col += 1;
        }

        // now add all cells from the same column that aren't in the same box
        let mut col_row = 0;
        let mut grid_cell = col;
        while col_row < box0_row {
            out[out_index] = grid_cell; out_index += 1;
            grid_cell += REGION_SIZE /* aka ROW_WIDTH */;
            col_row += 1;
        }
        col_row += BOX_HEIGHT;
        grid_cell += REGION_SIZE * BOX_HEIGHT;
        while col_row < REGION_SIZE {
            out[out_index] = grid_cell; out_index += 1;
            grid_cell += REGION_SIZE /* aka ROW_WIDTH */;
            col_row += 1;
        }
        //assert_eq!(out_index, NUM_CONFLICTS);
        assert!(out_index == NUM_CONFLICTS);
        out
    }

    const fn compute_grid_conflicts() -> [[usize; NUM_CONFLICTS]; GRID_SIZE] {
        let mut out = [[0; NUM_CONFLICTS]; GRID_SIZE];
        let mut row = 0;
        let mut grid_cell_num = 0;
        while row < NUM_REGIONS {
            let mut col = 0;
            while col < NUM_REGIONS {
                out[grid_cell_num] = Self::compute_cell_conflicts(row, col);
                grid_cell_num += 1;
                col += 1;
            }
            row += 1;
        }
        //assert_eq!(grid_cell_num, GRID_SIZE);
        assert!(grid_cell_num == GRID_SIZE);
        out
    }
    
    const fn compute_grid_cell_locations() -> [[RegionIdAndLocation; NUM_REGION_TYPES]; GRID_SIZE] {
        let mut out = [[(0, 0); NUM_REGION_TYPES]; GRID_SIZE];
        let mut row = 0;
        let mut band_box0 = 0;  // box number at column 0 of current row
        let mut band_boxy =  0; // 'y' coordinate in current band
        let mut grid_cell = 0; // current grid cell
        while row < NUM_REGIONS {
            let mut col = 0;    // current column of grid (0..REGION_SIZE)
            let mut col_box = 0; // index of box within current band (so the current box is band_box0 + col_box) (0..BOXES_PER_ROW)
            let mut box_col = 0; // current column in current box (0..BOX_WIDTH)
            while col < NUM_REGIONS {
                // there are three region types we have to compute for: rows, columns, and boxes
                // compute box ID
                let cell_boxn = band_box0 + col_box;
                // compute index within that box
                let box_cell_index = band_boxy * BOX_WIDTH + box_col;
                
                out[grid_cell] = 
                    [   
                        // row ID and index within that row -- easy
                        (row, col),
                        // column ID and index within that column -- also easy
                        (col, row),
                        // box ID and index within that box -- less easy
                        (cell_boxn, box_cell_index),
                    ];
                grid_cell += 1;
                
                // advance one
                col += 1;
                box_col += 1;
                if box_col >= BOX_WIDTH {
                    box_col = 0;
                    col_box += 1;
                }
            }
            row += 1;
            band_boxy += 1;
            if band_boxy >= BOX_HEIGHT {
                band_box0 += BOXES_PER_ROW;
                band_boxy = 0;
            }
        }
        //assert_eq!(grid_cell, GRID_SIZE);
        assert!(grid_cell == GRID_SIZE);
        out
    }
}


// several constants to deal with

// CELL_LOCATIONS[x] maps grid cell #x to:
//          [0] - (row index, cell index within that row)
//          [1] - (column index, cell index within that column)
//          [2] - (box index, cell index within that box)
const CELL_LOCATIONS : [[RegionIdAndLocation; NUM_REGION_TYPES]; GRID_SIZE] = SudokuSchemeGenerator::compute_grid_cell_locations();

// ROW_CELLS[n] (n is 0..REGION_SIZE) is the grid cell indices of every cell in that row
const ROW_CELLS : RegionCellMap = SudokuSchemeGenerator::compute_rows();
// COL_CELLS[n] (n is 0..REGION_SIZE) contains the grid cell indices of every cell in that column
const COL_CELLS : RegionCellMap = SudokuSchemeGenerator::compute_cols();
// BOX_CELLS[n] (n is 0..REGION_SIZE) contains the grid cell indices for every cell in that box
const BOX_CELLS : RegionCellMap = SudokuSchemeGenerator::compute_boxes();
// CELL_CONFLICTS[n] (n is 0..GRID_SIZE) contains the grid cell indices for every cell that conflicts with that cell
const CELL_CONFLICTS : [[usize; NUM_CONFLICTS]; GRID_SIZE] = SudokuSchemeGenerator::compute_grid_conflicts();

// a PlacementMask value that means "we haven't ruled anything out"
const PLACEMENT_MASK_ANY : PlacementMaskType = (1 << NUM_SYMBOLS) - 1;

#[derive(Copy, Clone)]
enum DecodedPlacementMask {
    Solved(usize),
    Unsolved(PlacementMaskType),
}

use DecodedPlacementMask::{Solved, Unsolved};

#[derive(Copy, Clone)]
struct PlacementMask(PlacementMaskType);
impl Default for PlacementMask { fn default() -> Self { PlacementMask(PLACEMENT_MASK_ANY) } }

impl PlacementMask {
    pub fn is_solved(&self) -> bool { self.0 < 0 }
    pub fn is_unsolved(&self) -> bool { self.0 >= 0 }
    pub fn is_unsolvable(&self) -> bool { self.0 == 0 }
    // Removes the specified candidate from the candidate list. 
    //      If the symbol in question is not already in the candidate list, returns Ok(false)
    //      If the symbol in question was removed from the candidate list, returns Ok(true)
    //      If the symbol in question was removed from the candidate list and the candidate list is now EMPTY (unsolvable), returns Err(()).
    pub fn remove_candidate(&mut self, value: usize) -> Result<bool, ()> {
        if self.0 < 0 { Ok(false) }
        else {
            if self.0 == 0 { panic!("self.0 is somehow zero already"); }
            let mask = (1 as PlacementMaskType) << value;
            let was_removed = (self.0 & mask) != 0;
            self.0 &= !mask;
            if self.0 > 0 { Ok(was_removed) }
            else { Err(()) }
        }
    }
    // Adds the specified candidate to the candidate list. Use only when trying to undo
    // a remove_candidate() that returned Ok(true)
    pub fn add_candidate(&mut self, value: usize) {
        assert!(self.0 < 0, "attempted to add candidate to already-solved tracker");
        self.0 |= (1 as PlacementMaskType) << value;
    }
    pub fn decode(&self) -> DecodedPlacementMask {
        if self.0 < 0 { Solved( (self.0 - PlacementMaskType::MIN) as usize ) }
        else { Unsolved(self.0) }
    }
    pub fn solved_value(&self) -> Option<usize> {
        if self.0 < 0 { Some( (self.0 - PlacementMaskType::MIN) as usize ) }
        else { None }
    }
    pub fn candidate_mask(&self) -> Option<PlacementMaskType> {
        if self.0 >= 0 { Some(self.0) }
        else { None }
    }
    pub fn solve(&mut self, value: usize) {
        if !self.is_candidate(value) { panic!("invalid solution attempt"); }
        self.0 = (value as PlacementMaskType) + PlacementMaskType::MIN;
    }
    pub fn is_candidate(&self, value: usize) -> bool {
        // very strictly speaking it's >= 0
        // but if self.0 is equal to zero, then it's actually unsolvable
        // I wonder if I should prevent that entirely, actually, using a NonZeroI16...
        if self.0 > 0 { 
            let mask = (1 as PlacementMaskType) << value;
            
            (self.0 & mask) != 0
        } else {
            false
        }
    }
}
struct PlacementMaskBitIterator(PlacementMaskType);
impl Iterator for PlacementMaskBitIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let bitnum = self.0.trailing_zeros();
            self.0 &= self.0 - 1; // this clears the lowest-order bit of self.0
            Some(bitnum)
        }
    }
}
impl Debug for PlacementMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.0 < 0 {
            f.debug_tuple("Solved").field(&(self.0 - PlacementMaskType::MIN)).finish()
        } else {
            f.write_str("Unsolved(")?;
            // keeping this here for future rustc diagnostic output improvements:
            // f.debug_set(PlacementMaskBitIterator(self.0)).finish()?;
            f.debug_set().entries(PlacementMaskBitIterator(self.0)).finish()?;
            f.write_str(")")
        }
    }
}

type SymPlacement = [[PlacementMask; NUM_REGIONS]; NUM_REGION_TYPES];

#[derive(Debug, Clone)]
struct SudokuSolver {
    grid: [PlacementMask; GRID_SIZE],
    sym_placement: [SymPlacement; NUM_SYMBOLS],
    touched_cells: u128,
    dead: bool,
}

impl Default for SudokuSolver {
    fn default() -> Self {
        SudokuSolver {
            grid: [Default::default(); GRID_SIZE],
            sym_placement: [Default::default(); NUM_SYMBOLS],
            touched_cells: 0,
            dead: false,
        }
    }
}

type PlaceError = String;

impl SudokuSolver {
    pub fn new() -> Self { Default::default() }
    
    pub fn from<I : Iterator<Item = Option<usize>>>(iterator: I) -> Self {
        let mut solver = Self::new();
        let mut row = 0;
        let mut col = 0;
        for cell_value in iterator {
            if let Some(sym) = cell_value {
                solver.try_place(sym, row, col).unwrap();
            }
            col += 1;
            if col >= REGION_SIZE { 
                col = 0;
                row += 1;
            }
        }
        solver
    }

    // Attempt to place the symbol with ID @symbol (0 <= @symbol < NUM_SYMBOLS)
    // at grid position (row, col) (0 <= row, col < REGION_SIZE).
    // Returns an error if the specified symbol cannot be placed there.
    pub fn try_place(&mut self, symbol: usize, row: usize, col: usize) -> Result<(), PlaceError> {
        assert!(symbol < NUM_SYMBOLS);
        assert!(row < REGION_SIZE);
        assert!(col < REGION_SIZE);
        if self.dead { return Err("a previous try_place() failed in such a way that this object is no longer usable".into()); }
        let grid_cell = row * REGION_SIZE + col;
        if !self.grid[grid_cell].is_candidate(symbol) {
            return Err(format!("Symbol {symbol} cannot be placed at row {row}, column {col} -> {:?}", self.grid[grid_cell]));
        }
        
        self.grid[grid_cell].solve(symbol);

        let grid_cell_locs = CELL_LOCATIONS[grid_cell];
        // sanity check!
        debug_assert_eq!(grid_cell_locs[0], (row, col));
        debug_assert_eq!(grid_cell_locs[1], (col, row));
        // end sanity checks
        let sym_placement = &mut self.sym_placement[symbol];
        for (placement_map, (region_id, region_cell_index)) in 
            sym_placement.iter_mut().zip(grid_cell_locs.into_iter())
        {
            placement_map[region_id].solve(region_cell_index);
        }

        self.dead = true; // if an error occurs during placement, leave ourselves in a 'dead' state
        for &other_cell in CELL_CONFLICTS[grid_cell].iter() {
            self.touched_cells |= 1_u128 << grid_cell;
            self.grid[other_cell].remove_candidate(symbol)
                .map_err(|_| String::from("Placing {symbol} at row {row}, column {col} leaves grid cell #{other_cell} unsolvable"))
                ?;
            let other_cell_locs = CELL_LOCATIONS[other_cell];
            for (placement_map, (region_id, region_cell_index))
                in sym_placement.iter_mut().zip(other_cell_locs.into_iter())
            {
                placement_map[region_id].remove_candidate(region_cell_index)
                    .map_err(|_| String::from("Placing {symbol} at row {row}, column {col} excludes that symbol from region {region_id} cell {cell_index}, which leaves no place to put that symbol in that region"))
                    ?;
            }
        }
        self.dead = false; // if we got here, everything worked
        Ok(())
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        unimplemented!()
    }
}

fn main() {
     for input in [
                [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]],
        ]
    {
        println!("input: {:?}", input);
        let solver = SudokuSolver::from(
                    input.into_iter().flat_map(|row| row.into_iter().map(|cell_str|
                            match cell_str.chars().next().unwrap() {
                                ch @ '1' ..= '9' => Some( (ch.to_digit(10).unwrap() - 1) as usize ),
                                _ => None,
                            }
                        )
                    )
                    );
        println!("solver: {:?}", solver);
        //let ans = Solution::is_valid_sudoku(input.clone());
        //println!("--> is valid board?: {:?}", ans);
    }
}
