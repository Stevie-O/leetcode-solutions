// 2684. maximum number of moves in a grid

definitely is getting the wrong answer for the supplied test case: 57-213-222-241-276 

use std::collections::HashSet;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let num_cols = grid[0].len();
        let num_rows = grid.len();
        let mut longest_dist = [0].repeat(num_rows);
        for coln in (0..(num_cols-1)).rev() {
            let new_longest_dist = grid.iter().enumerate()
                .map(|(rown, row)|
                        (rown-1..=rown+1)
                            .filter_map(|target_row| 
                                    if (0..num_rows).contains(&target_row)
                                        && grid[target_row][coln+1] > row[coln]
                                    { Some(longest_dist[target_row] + 1) }
                                    else { None }
                            )
                            .max()
                            .unwrap_or(0)
                )
                .collect();
            longest_dist = new_longest_dist;
        }
        longest_dist.into_iter().max().unwrap()
    }
}

[
	[57,213,269,42,135,1,130,16,5,283,181,46,56,30],
	[170,260,222,279,276,50,182,264,256,255,233,82,300,19],
	[300,68,261,241,96,205,1,254,201,95,124,132,40,220],
	[212,38,50,38,176,271,109,27,83,195,165,7,180,224],
	[238,106,44,264,157,180,94,92,119,142,59,269,275,189],
	[35,34,226,241,45,195,200,178,104,92,255,170,49,91],
	[94,137,145,78,49,243,263,113,230,144,249,192,36,276],
	[271,241,12,126,232,192,294,212,239,199,49,106,177,104],
	[243,206,191,14,110,62,137,184,185,12,259,217,285,209],
	[124,197,299,299,4,36,245,160,182,300,7,170,50,67],
	[77,39,189,185,277,223,122,181,55,97,97,85,215,287],
	[124,42,198,38,204,87,172,9,102,232,112,71,186,296],
	[13,101,254,158,289,112,89,172,194,269,27,171,149,223]
]
