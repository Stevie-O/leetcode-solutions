#[allow(dead_code)]
struct Solution{}

use std::fmt::{Formatter, Display, Debug};
use std::ops::{BitOr, BitOrAssign};

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
const REGION_TYPE_NAMES : [&str; NUM_REGION_TYPES] = ["Row", "Col", "Box"];

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

struct DebugRegionType(usize);
impl DebugRegionType {
    pub fn with_id(self, id: usize) -> DebugRegion { DebugRegion(self.0, id) }
}
impl Debug for DebugRegionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(REGION_TYPE_NAMES[self.0])
    }
}

struct DebugRegion(usize, usize);
impl DebugRegion {
    pub fn with_index(self, index: usize) -> DebugRegionCell { DebugRegionCell(self, index) }
}
impl Debug for DebugRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(REGION_TYPE_NAMES[self.0]).field(&self.1).finish()
    }
}

struct DebugRegionCell(DebugRegion, usize);
impl Debug for DebugRegionCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}[{}]", self.0, self.1)
    }
}


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

const REGION_CELLS : [RegionCellMap; NUM_REGION_TYPES] = [
                            SudokuSchemeGenerator::compute_rows(),
                            SudokuSchemeGenerator::compute_cols(),
                            SudokuSchemeGenerator::compute_boxes(),
                        ];

// CELL_CONFLICTS[n] (n is 0..GRID_SIZE) contains the grid cell indices for every cell that conflicts with that cell
const CELL_CONFLICTS : [[usize; NUM_CONFLICTS]; GRID_SIZE] = SudokuSchemeGenerator::compute_grid_conflicts();

// a PlacementMask value that means "we haven't ruled anything out"
const PLACEMENT_MASK_ANY : PlacementMaskType = (1 << NUM_SYMBOLS) - 1;

#[derive(Copy, Clone, PartialEq, Eq)]
enum SolveStatus {
    Solved(usize),
    Unsolved(UnsolvedMask),
}

impl Debug for SolveStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Solved(index) => f.debug_tuple("Solved").field(index).finish(),
            Self::Unsolved(mask) => {
                f.write_str("Unsolved(")?;
                f.debug_set().entries(mask.items()).finish()?;
                f.write_str(")")
            }
        }
    }
}

use SolveStatus::{Solved, Unsolved};


/// PlacementMask and UnsolvedMask are really similar:
//  - PlacementMask is used to track EITHER:
//      "which symbols MIGHT be in this cell" -or- which symbol this cell contains
//  or  "which cells MIGHT this symbol be in" -or- which cell contains this symbol
//  - UnsolvedMask is used to track "which symbols are unsolved in this region" or "which regions are unsolved in this grid")
// the main difference is that it is an ERROR for PlacementMask to reduce to zero,
//      because that would mean the puzzle is unsolvable.
// in contrast, it's appropriate (and good!) for UnsolvedMask to be zero, which simply means "fully solved".
//  - TouchedMask is like UnsolvedMask but it starts _empty_ and then you add elements

#[derive(Copy, Clone, PartialEq, Eq)]
struct UnsolvedMask(PlacementMaskType);
impl Default for UnsolvedMask { fn default() -> Self { UnsolvedMask(PLACEMENT_MASK_ANY) } }
impl UnsolvedMask {
    /// Returns true if all items are solved
    pub fn is_solved(&self) -> bool { self.0 == 0 }
    /// Returns true if the specified item is solved
    pub fn is_item_solved(&self, value: usize) -> bool { (self.0 & (1 << value)) == 0 }
    /// Returns true if the specified item is still unsolved
    pub fn is_item_unsolved(&self, value: usize) -> bool { (self.0 & (1 << value)) != 0 }
    /// Marks the specified item as solved. Returns true if that was the last item (so now is_solved() is true)
    pub fn mark_item_solved(&mut self, value: usize) -> bool { self.0 &= !(1 << value); self.0 == 0 }
    /// Returns an iterator that yields the list of unsolved items at the time of the call
    pub fn items(&self) -> impl Iterator<Item = usize> { self.0.bit_iter().map(|x| x as usize) }
}
impl Debug for UnsolvedMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.0 == 0 {
            write!(f, "FullySolved")
        } else {
            f.write_str("Unsolved(")?;
            // keeping this here for future rustc diagnostic output improvements:
            // f.debug_set(PlacementMaskBitIterator(self.0)).finish()?;
            f.debug_set().entries(self.items()).finish()?;
            f.write_str(")")
        }
    }
}

// much like TouchedMask, but used STRICTLY for internal consistency checking purposes
#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct UnsolvedMaskBuilder(PlacementMaskType);

impl UnsolvedMaskBuilder {
    pub fn mark_item_unsolved(&mut self, value : usize) { self.0 |= 1 << value; }
}

impl BitOr for UnsolvedMaskBuilder {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { UnsolvedMaskBuilder(self.0 | rhs.0) }
}

impl BitOrAssign for UnsolvedMaskBuilder {
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

// these impls aren't part of TouchedMask
impl From<UnsolvedMaskBuilder> for UnsolvedMask {
    fn from(item: UnsolvedMaskBuilder) -> Self { UnsolvedMask(item.0) }
}

impl BitOrAssign<UnsolvedMask> for UnsolvedMaskBuilder {
    fn bitor_assign(&mut self, rhs: UnsolvedMask) { self.0 |= rhs.0; }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct TouchedMask(PlacementMaskType);

impl TouchedMask {
    pub fn touch_item(&mut self, value : usize) { self.0 |= 1 << value; }
    pub fn items(&self) -> impl Iterator<Item = usize> { self.0.bit_iter().map(|x| x as usize) }
}

impl BitOr for TouchedMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { TouchedMask(self.0 | rhs.0) }
}

impl BitOrAssign for TouchedMask {
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl Debug for TouchedMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.0 == 0 {
            write!(f, "Untouched")
        } else {
            f.write_str("Touched(")?;
            f.debug_set().entries(self.items()).finish()?;
            f.write_str(")")
        }
    }
}

#[derive(Copy, Clone)]
struct PlacementMask(PlacementMaskType);
impl Default for PlacementMask { fn default() -> Self { PlacementMask(PLACEMENT_MASK_ANY) } }

#[allow(dead_code)]
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
    pub fn decode(&self) -> SolveStatus {
        if self.0 < 0 { Solved( (self.0 - PlacementMaskType::MIN) as usize ) }
        else { Unsolved(UnsolvedMask(self.0)) }
    }
    pub fn solved_value(&self) -> Option<usize> {
        if self.0 < 0 { Some( (self.0 - PlacementMaskType::MIN) as usize ) }
        else { None }
    }
    pub fn candidate_mask(&self) -> Option<PlacementMaskType> {
        if self.0 >= 0 { Some(self.0) }
        else { None }
    }
    pub fn solve(&mut self, value: usize) -> TouchedMask {
        if /* unlikely */ let Some(solved_value) = self.solved_value() {
            eprintln!("WARNING: somehow we were asked to solve an already-Solved({solved_value}) as {value}");
            // we want to always panic because the code that uses this routine
            // should never encounter this situation
            /*if solved_value == value {
                TouchedMask(0)
            } else {*/
                panic!("attempt to solve already-solved placement");
            //}
        } else {
            let mask = 1 << value;
            let was_removed = (self.0 & mask) != 0;
            if !was_removed { panic!("invalid solution attempt"); }
            let ret_mask = self.0 & !mask;
            self.0 = (value as PlacementMaskType) + PlacementMaskType::MIN;
            TouchedMask(ret_mask)
        }
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

// bit_iter() returns a sequence of u32s containing the bits that are set in the number
trait IntoBitIterator { type BitIteratorType : Iterator<Item = u32>; fn bit_iter(self) -> Self::BitIteratorType; }
struct BitIterator<T>(T);
// I learned how to do this from from_str_radix_int_impl
macro_rules! into_bit_iterator_impl {
    ($($num_type:ty)*) => {$(
        impl Iterator for BitIterator<$num_type> {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 0 { None }
                else {
                    let bitnum = self.0.trailing_zeros();
                    self.0 &= self.0 - 1;
                    Some(bitnum)
                }
            }
        }
        impl IntoBitIterator for $num_type {
            type BitIteratorType = BitIterator<$num_type>;
            fn bit_iter(self) -> Self::BitIteratorType {
                BitIterator(self)
            }
        }
    )*}
}
// mod.rs line 1379
into_bit_iterator_impl! { isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128 }

impl Debug for PlacementMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { self.decode().fmt(f) }
}

type SymPlacement = [[PlacementMask; NUM_REGIONS]; NUM_REGION_TYPES];

#[derive(Clone)]
struct SudokuSolver {
    // grid[n] either contains the symbol placed in grid cell #n (numbered across/down), or the set of _possible_ symbols that may be placed in grid cell #n
    grid: [PlacementMask; GRID_SIZE],
    // sym_placement[n] is placement info for symbol #n
    //      sym_placement[n][rty][rid] is either the index (0..REGION_SIZE) that symbol #n is placed in inside region type-#rty number-#rid
    //                                 or the set of _possible_ indices for that symbol in that region
    //      e.g. sym_placement[2][0][1] is either the column number, or the set of possible column numbers, that symbol #2 may occur in for row (type #0) #1
    //      e.g. sym_placement[0][1][2] is either the row number, or the set of possible row numbers, that symbol #0 may occur in for column (type #1) #2
    sym_placement: [SymPlacement; NUM_SYMBOLS],
    // unsolved_region_cells[rty][id] gives a mask of which cells in region type rty, region id id, are unsolved
    unsolved_region_cells: [[UnsolvedMask; NUM_REGIONS]; NUM_REGION_TYPES],
    // unsolved_region_syms[rty][id] gives a mask of which symbols in region type rty, region id id, are unsolved
    unsolved_region_syms: [[UnsolvedMask; NUM_REGIONS]; NUM_REGION_TYPES],
    // unsolved_regions[n] is the set of regions of type #n that have unsolved cells
    //  for example, if unsolved_regions[0] is 0b1001, then rows (type #0) #0 and #3 have unsolved grid cells
    //  if unsolved_regions[0] is 0, *all* of unsolved_regions should be 0, which should mean that all grid cells are solved
    unsolved_regions: [UnsolvedMask; NUM_REGION_TYPES],
    sym_placements_left: [usize; NUM_SYMBOLS], // [sym] = number of MISSING copies of @sym from the board
    // unsolved_symbols is the set of symbols that have been placed fewer than NUM_REGIONS times (aka sym_placements_left[n] > 0)
    unsolved_symbols: UnsolvedMask,     // if this is zero, then the entire grid should be solved
    
    // these fields track what changes have been made to @grid so we can figure out which 
    touched_grid: [TouchedMask; GRID_SIZE],
    touched_cells: u128,
    touched_syms:  TouchedMask,

    dead: bool,
}

impl SudokuSolver {
    fn check_consistency(&self) {
        let mut found_unsolved_region_cells = [[UnsolvedMaskBuilder::default(); NUM_REGIONS]; NUM_REGION_TYPES];
        let mut found_unsolved_region_syms = [[UnsolvedMaskBuilder::default(); NUM_REGIONS]; NUM_REGION_TYPES];
        let mut found_unsolved_regions = [UnsolvedMaskBuilder::default(); NUM_REGION_TYPES];
        let mut found_sym_placements_left = [NUM_SYMBOLS; NUM_SYMBOLS];
        let mut found_unsolved_symbols = UnsolvedMaskBuilder::default();
        
        for grid_cell in 0..GRID_SIZE {
            let cell_loc = CELL_LOCATIONS[grid_cell];
            match self.grid[grid_cell].decode() {
                Solved(symbol) => {
                    found_sym_placements_left[symbol] -= 1;
                    
                    // this grid cell is solved
                    for rty in 0..NUM_REGION_TYPES {
                        let (rgn_id, cell_index) = cell_loc[rty];
                        // this cell should be marked as 'solved' in unsolved_region_cells
                        assert!(self.unsolved_region_cells[rty][rgn_id].is_item_solved(cell_index),
                            "grid cell #{grid_cell} ({:?}) is solved, but unsolved_region_cells for type #{rty} ({}) #{rgn_id} does not reflect this", cell_loc, REGION_TYPE_NAMES[rty]);
                        // this symbol should be marked as 'solved' in unsolved_region_syms
                        assert!(self.unsolved_region_syms[rty][rgn_id].is_item_solved(symbol),
                            "grid cell #{grid_cell} ({:?}) is solved as #{symbol}, but unsolved_region_syms for type #{rty} ({}) #{rgn_id} does not reflect this", cell_loc, REGION_TYPE_NAMES[rty]);
                        // this symbol should correctly be marked as solved in sym_placement
                        assert_eq!(self.sym_placement[symbol][rty][rgn_id].decode(), Solved(cell_index),
                            "grid cell #{grid_cell} ({:?}) is solved as #{symbol}, but sym_placement for type #{rty} ({}) #{rgn_id} does not reflect this", cell_loc, REGION_TYPE_NAMES[rty]);
                    }
                },
                Unsolved(symbol_mask) => {
                    // this grid cell is unsolved
                    for rty in 0..NUM_REGION_TYPES {
                        let (rgn_id, cell_index) = cell_loc[rty];
                        // this cell is unsolved
                        found_unsolved_region_cells[rty][rgn_id].mark_item_unsolved(cell_index);
                        found_unsolved_regions[rty].mark_item_unsolved(rgn_id);
                        found_unsolved_region_syms[rty][rgn_id] |= symbol_mask;
                        found_unsolved_symbols |= symbol_mask;
                        // this cell should be marked as 'unsolved' in unsolved_region_cells
                        assert!(self.unsolved_region_cells[rty][rgn_id].is_item_unsolved(cell_index),
                            "grid cell #{grid_cell} ({:?}) is unsolved, but unsolved_region_cells for type #{rty} ({}) #{rgn_id} does not reflect this", cell_loc, REGION_TYPE_NAMES[rty]);
                        for symbol in symbol_mask.items() {
                            // this symbol should be marked as 'unsolved' in unsolved_region_syms
                            assert!(self.unsolved_region_syms[rty][rgn_id].is_item_unsolved(symbol),
                                "grid cell #{grid_cell} ({:?}) is solved as #{symbol}, but unsolved_region_syms for type #{rty} ({}) #{rgn_id} does not reflect this", cell_loc, REGION_TYPE_NAMES[rty]);
                            // this grid cell is a candidate solution for this symbol
                            assert!(self.sym_placement[symbol][rty][rgn_id].is_candidate(cell_index),
                                "grid cell #{grid_cell} ({:?}) may contain #{symbol}, but sym_placement for type #{rty} ({}) #{rgn_id} does not reflect this", cell_loc, REGION_TYPE_NAMES[rty]);
                        }
                    }
                }
            }
        } // for grid_cell
        
        // okay, let's check some stuff
        let found_unsolved_region_cells =
            found_unsolved_region_cells.map(|r| r.map(|c| c.into()));
        assert_eq!(self.unsolved_region_cells, found_unsolved_region_cells);

        let found_unsolved_region_syms =
            found_unsolved_region_syms.map(|r| r.map(|c| c.into()));
        assert_eq!(self.unsolved_region_syms,  found_unsolved_region_syms);

        let found_unsolved_regions =
            found_unsolved_regions.map(|r| r.into());
        assert_eq!(self.unsolved_regions, found_unsolved_regions);

        assert_eq!(self.sym_placements_left, found_sym_placements_left);
        
        let found_unsolved_symbols = found_unsolved_symbols.into();
        assert_eq!(self.unsolved_symbols, found_unsolved_symbols);
    }
}

struct CellLocationsDebug {}
impl Debug for CellLocationsDebug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f)?;
        
        for rty in 0..NUM_REGION_TYPES {
            writeln!(f, "Region type {}:", REGION_TYPE_NAMES[rty])?;
            let mut col = 0;
            for grid_cell in 0..GRID_SIZE {
                let (region_id, cell_index) = CELL_LOCATIONS[grid_cell][rty];
                // "Xyz(N)[Y]"
                //  123456789
                // add space on both sides
                write!(f, "{:^11}", format!("{:?}", DebugRegion(rty, region_id).with_index(cell_index)))?;
                col += 1;
                if col >= REGION_SIZE {
                    col = 0;
                    writeln!(f)?;
                } else {
                    write!(f, "|")?;
                }
            }
        }
        Ok(())
    }
}

#[allow(dead_code)]
fn debug_cell_metadata() {
    println!("Cell locations: {:?}", CellLocationsDebug{});
}

struct GridDebug<'a>(&'a [PlacementMask; GRID_SIZE]);
impl<'a> Debug for GridDebug<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // max width is: "Unsolved({0,1,2,3,4,5,6,7,8})" = 29 characters
        // so let's pad that to 31
        let mut col = 0;
        f.write_str("\n")?; // fixup
        for item in self.0 {
            write!(f, "{:^31}", format!("{:?}", item))?;
            col += 1;
            if col >= REGION_SIZE {
                col = 0;
                f.write_str("\n")?;
            } else {
                f.write_str("|")?;
            }
        }
        Ok(())
    }
}

impl Debug for SudokuSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("SudokuSolver")
            // this... did not work out as planned.
            //.field("grid", &(self.grid.chunks(REGION_SIZE)))
            //.field("grid", &(self.grid.chunks(REGION_SIZE).collect::<Vec<_>>()))
            .field("grid", &GridDebug(&self.grid))
            .field("sym_placement", &self.sym_placement)
            .field("unsolved_region_cells", &self.unsolved_region_cells)
            .field("unsolved_regions", &self.unsolved_regions)
            .field("sym_placements_left", &self.sym_placements_left)
            .field("unsolved_symbols", &self.unsolved_symbols)
            .field("touched_grid", &self.touched_grid)
            .field("touched_cells", &self.touched_cells)
            .field("touched_syms", &self.touched_syms)
            .field("dead", &self.dead)
            .finish()
    }
}

impl Default for SudokuSolver {
    fn default() -> Self {
        SudokuSolver {
            grid: [Default::default(); GRID_SIZE],
            sym_placement: [Default::default(); NUM_SYMBOLS],
            unsolved_region_cells: [[Default::default(); NUM_REGIONS]; NUM_REGION_TYPES],
            unsolved_region_syms: [[Default::default(); NUM_REGIONS]; NUM_REGION_TYPES],
            unsolved_regions: [Default::default(); NUM_REGION_TYPES],
            sym_placements_left: [NUM_REGIONS; NUM_SYMBOLS],
            unsolved_symbols: Default::default(),
            
            touched_grid: [Default::default(); GRID_SIZE],
            touched_cells: 0,
            touched_syms:  Default::default(),
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
                //println!("placing symbol #{sym} at row #{row}, col #{col}");
                solver.try_place_by_row_col(sym, row, col).unwrap();
            }
            col += 1;
            if col >= REGION_SIZE { 
                col = 0;
                row += 1;
            }
        }
        solver
    }
    
    pub fn is_solved(&self) -> bool { self.unsolved_symbols.is_solved() }

    // Attempt to place the symbol with ID @symbol (0 <= @symbol < NUM_SYMBOLS)
    // at grid position (row, col) (0 <= row, col < REGION_SIZE).
    // Returns an error if the specified symbol cannot be placed there.
    pub fn try_place_by_row_col(&mut self, symbol: usize, row: usize, col: usize) -> Result<(), PlaceError> {
        assert!(row < REGION_SIZE);
        assert!(col < REGION_SIZE);
        let grid_cell = row * REGION_SIZE + col;
        self.try_place_by_grid_cell(symbol, grid_cell)
    }

    // Attempt to place the symbol with ID @symbol (0 <= @symbol < NUM_SYMBOLS)
    // at grid cell @grid_cell (0 <= grid_cell < GRID_SIZE)
    // Returns an error if the specified symbol cannot be placed there.
    pub fn try_place_by_grid_cell(&mut self, symbol: usize, grid_cell: usize) -> Result<(), PlaceError> {
        assert!(grid_cell < GRID_SIZE);
        assert!(symbol < NUM_SYMBOLS);
        println!("placing symbol #{symbol} at grid cell #{grid_cell} ({:?}) which is curently {:?}", 
            CELL_LOCATIONS[grid_cell].iter().enumerate().map(
                |(rty, &(rid, cell_index))| DebugRegion(rty, rid).with_index(cell_index)).collect::<Vec<_>>(),
            self.grid[grid_cell].decode()
            );
        if self.dead { return Err("a previous try_place() failed in such a way that this object is no longer usable".into()); }
        if !self.grid[grid_cell].is_candidate(symbol) {
            return Err(format!("Symbol #{symbol} cannot be placed at grid cell #{grid_cell} -> {:?}", self.grid[grid_cell]));
        }
        
        // place the symbol in that cell
        self.grid[grid_cell].solve(symbol);
        // one fewer copy of @symbol left to be placed on the board!
        self.sym_placements_left[symbol] -= 1;
        if self.sym_placements_left[symbol] == 0 {
            println!("symbol #{symbol} is fully solved");
            if self.unsolved_symbols.mark_item_solved(symbol) {
                println!("all symbols have been solved");
            } else {
                println!("remaining symbols: {:?}", self.unsolved_symbols);
            }
        }

        let grid_cell_locs = CELL_LOCATIONS[grid_cell];
        for (rty, &(rgn_id, cell_index)) in grid_cell_locs.iter().enumerate() {
            if self.unsolved_region_cells[rty][rgn_id].mark_item_solved(cell_index) {
                println!("region type #{rty} ({}) ID #{rgn_id} is solved", REGION_TYPE_NAMES[rty]);
                self.unsolved_regions[rty].mark_item_solved(rgn_id);
            }
            self.unsolved_region_syms[rty][rgn_id].mark_item_solved(symbol);
        }
        
        // sanity check!
        //debug_assert_eq!(grid_cell_locs[0], (row, col));
        //debug_assert_eq!(grid_cell_locs[1], (col, row));
        // end sanity checks
        let sym_placement = &mut self.sym_placement[symbol];
        for (placement_map, (region_id, region_cell_index)) in 
            sym_placement.iter_mut().zip(grid_cell_locs.into_iter())
        {
            placement_map[region_id].solve(region_cell_index);
        }
        
        self.dead = true; // if an error occurs during placement, leave ourselves in a 'dead' state
        // look at all cells that conflict with (are same row, column, or box as) the one we just solved
        for &other_cell in CELL_CONFLICTS[grid_cell].iter() {
            let other_cell_locs = CELL_LOCATIONS[other_cell];
            let was_touched = self.grid[other_cell].remove_candidate(symbol)
                .map_err(|_| format!("Placing {symbol} at #{grid_cell} ({:?}) leaves grid cell #{other_cell} ({:?}) unsolvable", grid_cell_locs[0], other_cell_locs[0]))
                ?;
            if was_touched {
                // if we changed the possibility mask for this grid cell, make a note that
                // we touched that cell AND this symbol
                self.touched_cells |= 1_u128 << grid_cell;
                self.touched_grid[grid_cell].touch_item(symbol);
                self.touched_syms.touch_item(symbol);
                for (rty, (placement_map, (region_id, region_cell_index)))
                    in sym_placement.iter_mut().zip(other_cell_locs.into_iter()).enumerate()
                {
                    let removed = placement_map[region_id].remove_candidate(region_cell_index)
                        .map_err(|_| format!("Placing {symbol} at #{grid_cell} ({:?}) excludes that symbol from region type #{rty} #{region_id} index {region_cell_index}, which leaves no place to put that symbol in that region", grid_cell_locs[0]))
                        ?;
                    // this is an error
                    //assert!(removed, "Placing {symbol} at #{grid_cell} ({:?}) excludes that symbol from region type #{rty} #{region_id} index {region_cell_index}, but it was supposedly not a candidate there", grid_cell_locs[0]);
                }
            }
        }
        self.dead = false; // if we got here, everything worked
        self.check_consistency();
        Ok(())
    }
    
    /// Attempts to determine if there are any cells where there is only one possible value.
    // Returns true if any progress was made.
    pub fn solve_simple(&mut self) -> bool {
        // a prior step (either the previous solve iteration, or the initial placement of the givens)
        // will have narrowed down the possibilities for a puzzle.
        // touched_{cells,syms} are both bitmasks
        // touched_cells: bit n is set if grid cell #n's candidate mask has been narrowed down
        // touched_syms:  bit n is set if any of the possible positions for symbol #n has been ruled out
        let touched_grid  = std::mem::replace(&mut self.touched_grid, [Default::default(); GRID_SIZE]);
        let touched_cells = std::mem::replace(&mut self.touched_cells, 0);
        let touched_syms  = std::mem::replace(&mut self.touched_syms,  Default::default());
        if touched_cells == 0 { panic!("unsolvable, at least by this algorithm"); }
        let mut any_progress = false;
        // check if we've narrowed the set of possible symbols for a cell down to just one symbol
        // (this is actually pretty rare, in my experience).
        for touched_cell in touched_cells.bit_iter().map(|cell| cell as usize) {
            if let Some(mask) = self.grid[touched_cell].candidate_mask() {
                if mask.count_ones() == 1 {
                    // only one possible symbol left!
                    let symbol = mask.ilog2() as usize;
                    
                    println!("Grid cell #{touched_cell} ({:?}) only has one possible symbol left: {symbol}", 
                        CELL_LOCATIONS[touched_cell][0]
                            );
                    self.try_place_by_grid_cell(symbol, touched_cell).unwrap();
                    any_progress = true;
                }
            }
        }
        // it's possible that by narrowing the set of possible symbols for one cell, we can nail down a *different*
        // cell's symbol.  (This is much more common, in my experience.)
        // for example, if we removed '4' as a possible symbol from one cell,
        //      there might be only one '4' left in that same row, column, or box
        for touched_sym in touched_syms.0.bit_iter().map(|sym| sym as usize)
            //.filter(|&sym| self.sym_placements_left[sym] > 0)
        {
            if self.unsolved_symbols.is_item_solved(touched_sym) { continue; }
            // the Rust language design wants me to use iterators and not indices
            // but I'm pretty sure I can't do that here, because I would need to call methods on self
            // while I've borrowed &self.sym_placement
            // personal experience suggests that I have a lot of chains where "I ruled out 4 here, so 4 is there,
            // which means 3 is over there, which means 1 is over there".
            
            // the current design doesn't really handle that very efficiently.
            // I think that, to do it more efficiently, rather than separate touched_cells and touched_syms,
            // have a thing that tracks _which_ symbol was ruled out from _which_ cell.
            // then, when (say) we rule out symbol #3 from grid cell #53, we can simply check the candidate positions 
            //      for symbol #3 in the same row, column, and box as grid cell #53 (3 regions), rather than
            //      all rows, columns, and boxes (9 + 9 + 9 = 27 regions)
            for region_type_id in 0..NUM_REGION_TYPES {
                for region_id in 0..NUM_REGIONS {
                    /*
                    println!("considering: symbol #{} in {:?} -> {:?}", 
                        touched_sym, 
                        DebugRegionTypeAndId(region_type_id, region_id),
                        self.sym_placement[touched_sym][region_type_id][region_id]
                    );*/
                    if let Some(mask) = self.sym_placement[touched_sym][region_type_id][region_id].candidate_mask() {
                        if mask.count_ones() == 1 {
                            let region_cell_index = mask.ilog2() as usize;
                            let grid_cell = REGION_CELLS[region_type_id][region_id][region_cell_index];
                            println!("{:?} only has one possible position left for symbol #{touched_sym}: index {region_cell_index} = grid cell #{grid_cell}",
                                DebugRegion(region_type_id, region_id)
                            );
                            self.try_place_by_grid_cell(touched_sym, grid_cell).unwrap();
                            any_progress = true;
                        }
                    }
                }
            }
        }
        
        any_progress
    }
}

impl Solution {
    pub fn solve_sudoku(_board: &mut Vec<Vec<char>>) {
        unimplemented!()
    }
}

fn main() {
    //debug_cell_metadata(); return;
    for input in [
                [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]],
        ]
    {
        println!("input: {:?}", input);
        #[allow(unused_mut)]
        let mut solver = SudokuSolver::from(
                    input.into_iter().flat_map(|row| row.into_iter().map(|cell_str|
                            match cell_str.chars().next().unwrap() {
                                ch @ '1' ..= '9' => Some( (ch.to_digit(10).unwrap() - 1) as usize ),
                                _ => None,
                            }
                        )
                    )
                    );
        println!("solver: {:?}", solver);
        while solver.solve_simple() {
            println!("solved some more cells, trying again");
            println!("new solver: {:?}", solver);
        }
        //let ans = Solution::is_valid_sudoku(input.clone());
        //println!("--> is valid board?: {:?}", ans);
    }
}
