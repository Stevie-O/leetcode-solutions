// 16. 3Sum closest


impl Solution {
    // panics if n contains fewer than 2 elements.
    // otherwise,
    // given a SORTED slice and a target sum,
    // returns the value X such that:
    //  1. there exists at least one i, j (i!=j) such that nums[i] + nums[j] == X
    //  2. for all i, j (i!=j) in 0..nums.len(), abs(nums[i] + nums[j] - target) >= abs(X - target)
    // put another way,
    fn pair_sum_closest<T : Copy + Ord + Add + Sub + Default>(nums: &[T], target: T) -> T {
        assert!(nums.len() >= 2, "pair_sum_closest: input slice is too small (len={})", nums.len());
        let mut best_sum = nums[0] + nums[1];
        let mut best_dist = if best_sum < target { target - best_sum } else { best_sum - target };
        // if the first two elements somehow give a perfect answer, well, 
        if best_dist == <T as Default>::default() { return best_sum; }
        for i in 0..nums.len() - 1 {
            let target_numsj = target - nums[i];
            let search_result = nums[i+1..].binary_search((|&numsj| numsj.cmp(target_numsj));
            match pos {
                Ok(j) => {
                    // perfect match!
                    assert_eq!(nums[i] + nums[i+1+j], target);
                    return target;
                }
                Err(j_offs) => {
                    let j = i + 1 + j_offs;
                    // well, let's see how close we got, I guess
                    let slice_start = if j_offs > 0 { j - 1 } else { j };
                    let slice_end = if j + 1 < nums.len() { j + 1 } else { j };
                    for try_j in nums[slice_start .. slice_end ] {
                        let new_sum = nums[i] + nums[j];
                        let new_dist = if new_sum < target { target - new_sum } else { new_sum - target };
                        if new_dist < best_dist { (best_sum, best_dist) = (new_sum, new_dist); }
                    }
                }
            }
        }
        best_sum
    }
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut best_sum = nums[0] + nums[1] + nums[2];
        let mut best_dist = target.abs_diff(&best_sum);
        let num_slice = 
        for (i, i_val) in &nums[0..nums.len()-2].into_iter().copied().enumerate(()) {
            let subsum_target = target - i_val;
            let found_subsum = Self::pair_sum_closest()
        }
    }
}
