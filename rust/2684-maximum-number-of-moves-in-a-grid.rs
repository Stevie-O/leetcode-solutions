// 2684. maximum number of moves in a grid

this version processess in a forward direction
still getting the wrong answer for the supplied teswt case: 57-213-222-241-276 

use std::collections::HashSet;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let num_cols = grid[0].len();
        let num_rows = grid.len();
        let mut longest_chain : Vec<_> = 
            grid.iter().enumerate().map(|(rown, row)| (rown, row[0], 0)).collect();
        for coln in 1..num_cols {
            let new_longest_chain : Vec<_> = 
                longest_chain.iter()
                    .flat_map(|&(rown, value, moves)|
                        {
                            let grid_ref = &grid;
                            (rown-1..=rown+1)
                                .filter_map(move |target_row| 
                                    if (0..num_rows).contains(&target_row) && value < grid_ref[target_row][coln]
                                    { Some( (target_row, grid_ref[target_row][coln], moves+1) )}
                                    else { None }
                            )
                        }
                    )
                    .collect();
                if new_longest_chain.len() == 0 { break; } // dead-ended
                longest_chain = new_longest_chain;
                longest_chain.sort();
                longest_chain.dedup();
        }
        longest_chain.into_iter().map(|(_, _, moves)| moves).max().unwrap_or(0)
    }
}
