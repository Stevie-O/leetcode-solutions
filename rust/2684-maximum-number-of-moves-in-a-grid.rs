// 2684. maximum number of moves in a grid

// this version works with the initially failing test case (I think it was an unsigned integer wraparound bug before)
// but it fails with a different one: [[137,112,78,67],[76,65,122,135]]
// ahh, an off-by-one error means it will never move from the 2nd row (index 1) up to the first (index 0)

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
                            let first_row = if rown > 0 { rown - 1 } else { rown };
                            let last_row = if rown + 1 < num_rows { rown + 1 } else { rown };
                            (first_row..=last_row)
                                .filter_map(move |target_row| 
                                    if value < grid_ref[target_row][coln]
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
