struct Solution{}

// 16. 3Sum Closest
// given an array and a target sum,
// find the closest value to target_sum you can get for any three distinct indices
// note that ([3, 3, 3], 9) has an exact answer with indices 0, 1, 2
struct NonCopy<T>(T);
use core::ops::{Add, Sub};

impl Solution {
    // panics if n contains fewer than 2 elements.
    // otherwise,
    // given a SORTED slice and a target sum,
    // returns the value X such that:
    //  1. there exists at least one i, j (i!=j) such that nums[i] + nums[j] == X
    //  2. for all i, j (i!=j) in 0..nums.len(), abs(nums[i] + nums[j] - target) >= abs(X - target)
    // put another way,
    fn pair_sum_closest<T : Copy + Ord + Add<Output=T> + Sub<Output=T> + Default + std::fmt::Debug + std::fmt::Display>(nums: &[T], target: T) -> T {
        assert!(nums.len() >= 2, "pair_sum_closest: input slice is too small (len={})", nums.len());
        let mut best_sum = nums[0] + nums[1];
        let mut best_dist = if best_sum < target { target - best_sum } else { best_sum - target };
        // if the first two elements somehow give a perfect answer, well, 
        let mut prev_j_val : Option<T> = None;

        // i kept using the wrong slice
        let mut nums = NonCopy(nums);

        while /*nums.0.len() >= 2 && */ let [j_val, rest @ .. ] = nums.0 {
            if rest.is_empty() { break; }
            let j_val = *j_val;
            //let _ : () = j_val;
            if prev_j_val != Some(j_val) {
                prev_j_val = Some(j_val);
                let target_numsk = target - j_val;

                let search_result = rest.binary_search(&target_numsk);
                match search_result {
                    Ok(k) => {
                        // perfect match!
                        assert_eq!(j_val + rest[k], target);
                        return target;
                    }
                    Err(k) => {
                        // well, let's see how close we got, I guess
                        let slice_start = if k > 0 { k - 1 } else { k };
                        let slice_end = if k + 1 < rest.len() { k + 1 } else { k };
                        for &k_val in &rest[slice_start .. slice_end] {
                            let new_sum = j_val + k_val;
                            let new_dist = if new_sum < target { target - new_sum } else { new_sum - target };
                            if new_dist < best_dist { (best_sum, best_dist) = (new_sum, new_dist); }
                        }
                    }
                }
            }
            nums = NonCopy(rest);
        }
        best_sum
    }
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let nums = nums; // no more mut
        let mut best_sum = nums[0] + nums[1] + nums[2];
        let mut best_dist = target.abs_diff(best_sum);
        let mut previous_i_val = nums[0] - 1;

        let mut num_slice = nums.as_slice();
        while let [ i_val, ref rest @ .. ] = *num_slice {
            if rest.len() < 2 { break; }
            //let i_val = *i_val;
            // if the same number occurs multiple times, don't check it again
            if i_val != previous_i_val {
                previous_i_val = i_val;

                let target_subsum = target - i_val;
                let found_subsum = Self::pair_sum_closest(rest, target_subsum);
                // if we found an exact match, we can stop here
                if found_subsum == target_subsum { return target; }
                let found_diff = target_subsum.abs_diff(found_subsum);
                if found_diff < best_dist {  (best_sum, best_dist) = (found_subsum + i_val, found_diff); }
            }
            num_slice = rest;
        }

        best_sum
    }
}

fn main() {
    for input in [
            (vec![-2,-1,1,4],0)
        ]
    {
        // gotta move everything out of testcase at once
        println!("input: {:?}", input);
        let (v, t) = input;
        let ans = Solution::three_sum_closest(v, t);
        println!("--> answer: {:?}", ans);
    }
}
