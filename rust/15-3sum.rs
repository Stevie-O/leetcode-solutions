// 15. 3Sum (working solution)
// this one's a bit slower, though
// I wonder if I could optimize it a bit by finding a partition such that 
struct Solution{}

use std::collections::HashSet;

// looks like LeetCode hasn't updated to Rust 1.82.0 as of 2024-10-24. this is not very surprising, since Rust 1.82.0 came out exactly 1 week ago (2024-10-17)
trait NoneOr<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> NoneOr<T> for Option<T> {
     #[must_use]
     #[inline]
     fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            None => true,
            Some(val) => f(val),
        }
     }
}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
        // Notice that the solution set must not contain duplicate triplets.

        // okay so under the original problem definition, the order of the input array _does not matter_:
        // 1. The output only depends on the *values*, not the original indices.
        // 2. Therefore, swapping two elements in the input array does not affect the output.
        // 3. All reorderings can be expressed as a series of swaps.

        nums.sort();
        let nums = nums; // no more mut
        
        eprintln!("sorted_nums = {:?}", nums);

        // degenerate case check: trivially unsolvable
        if nums[0] > 0 || nums[nums.len() - 1] < 0 {
            // either every number in the input is positive, or every number in the input is negative
            // either way, there are no solutions to nums[i]+nums[j]+nums[i] == 0, even without the restriction that i!=j, i!=k, j!=k
            return Vec::new();
        }

        // no duplicate entries allowed
        let mut seen = HashSet::new();

        // so now that nums is sorted, let's consider the meaningful possibilities:
        //  nums[i] + nums[j] + nums[k] == 0
        // if we start by restricting ourselves such that i<j<k -- since the solution is isomorphic up to the six permutations of three elements -- we have:
        //              everything can start with
        //      nums[i] == 0,         nums[j] == 0, nums[k] == 0
        //      nums[i] <  0,         nums[j] == 0, nums[k] >  0
        //      nums[i] <  0,         nums[j] <  0, nums[k] >  0   for when nums[k] is very large
        //      nums[i] <  0,         nums[j] >  0, nums[k] >  0   for when nums[i] is a large negative value
        
        // so the ONLY way for nums[i] OR nums[k] to be zero is if all three are zero, which means there are least three zeroes in the input

        let first_nonneg = nums.partition_point(|&x| x<0);
        assert!(first_nonneg < nums.len()); // this should have been guaranteed by the trivially-unsolvable checks above
        let zero_count = (&nums[first_nonneg..]).iter().copied()
            .take_while(|&x| x == 0)
            .take(4) // 4 instead of 3 to detect lots of zeroes
            .count();
        if  zero_count >= 3
        {
            // this should be (0,0,0)
            seen.insert( [nums[first_nonneg], nums[first_nonneg+1], nums[first_nonneg+2]] );
        }

        // at this point, we have accounted for the scenario where nums[i]==0, nums[j]==0, and nums[k]==0.
        // therefore, from here on out, the following inequalities hold for all cases we're interested in:
        //          if i<j<k, nums[i] < nums[j] <= nums[k] OR nums[i] <= nums[j] < nums[k]
        //          if i<k,   nums[i] <    0    < nums[k]
        //      (nums[j] could be any sign: positive, negative, or zero)

        let i_maximum_excl = first_nonneg;
        let k_minimum_incl =
            if zero_count > 3 {
                    // hrm, there might be a lot of zeroes. do a binary search
                    first_nonneg + &nums[first_nonneg..].partition_point(|&x| x==0)
            } else {

                    first_nonneg + zero_count
            };
            
        eprintln!("len = {}, zero_count = {zero_count}, i_maximum_excl = {i_maximum_excl}, k_minimum_incl = {k_minimum_incl}", nums.len());
        eprintln!("possible nums[i]: {:?}", &nums[0..i_maximum_excl]);
        eprintln!("possible nums[k]: {:?}", &nums[k_minimum_incl..]);

        // I don't think it's possible to do much better than O(n^2) on this one
        let mut i_range = 0..i_maximum_excl;
        let mut prev_i = None;
        
        let max_nums_k = nums[nums.len()-1];

        'outer: while let Some(i) = i_range.find(|&i| prev_i.is_none_or(|pi| pi < nums[i])) {
            prev_i = Some(nums[i]);
            
            let mut j_range = (i+1)..nums.len() - 1; // note that this only goes up to nums.len()-2
            let mut prev_j = None;

            while let Some(j) = j_range.find(|&j| prev_j.is_none_or(|pj| pj < nums[j])) {
                prev_j = Some(nums[j]);

                let target_value = 0 - (nums[i] + nums[j]);
                
                eprintln!("[i={}]={}, [j={}]={}, looking for {}", i, nums[i], j, nums[j], target_value);
                
                // quick O(1) check to see if it's even possible
                if target_value > max_nums_k { continue; }

                let k_start = std::cmp::max(k_minimum_incl, j + 1);

                // binary_search is underspecified when the target value occurs multiple times in the array
                // (this actually makes perfect sense, given how the algorithm is normally implemented.)
                // i had this tedious copy-and-paste binary_search_by routine I wrote and then I
                // then discovered there's a method that gives exactly what I need: partition_point

                let k = k_start + (&nums[k_start..]).partition_point(|&item| item < target_value);
                assert!(k == nums.len() || nums[k] >= target_value);
                if k < nums.len() {
                    // okay, there are items in the range [k_start..] that are greater than or equal to target_value
                    if target_value < nums[k] {
                        // the original logic here was obviously incorrect, because it failed a test case
                        // target_value < nums[k]
                        // -(nums[i] + nums[j]) < nums[k]
                        // -nums[i] < nums[j] + nums[k]
                        // AHA. the trick is: is k equal to k_start?
                        // if k is greater than k_start, then it's possible that there's a
                        // j'>j that pairs with a k' that k_start <= k' < k
                        // but if k == k_start, then there is such k'

                        // so here's the deal: nums[k] > target_value
                        // -(nums[i] + nums[j]) < nums[k]
                        // -nums[i] - nums[j] < nums[k]
                        // -nums[i] < nums[j] + nums[k]

                        // because the list is sorted, and we're starting with the normalizion of i<j<k,
                        // for any later j', k' (j < j' < k < k'):
                        //      nums[j] <= nums[j']
                        //      nums[k] <= nums[k']
                        // nums[j] + nums[k] <= nums[j'] + nums[k']
                        // thus
                        // -nums[i] < nums[j'] + nums[k']
                        //
                        // Therefore, no pair of indices j<j'<k' will solve for nums[i], and we can give up on this value of 'i'.

                        // I'm pretty sure there are some circumstances under which we can abandon all subsequent 'i' values, too, but I can't think of what those might be.
                        if k == k_start {
                            eprintln!("giving up on i={i}, j={j}, k={k} ({}, {}, {})", nums[i], nums[j], nums[k]);
                            break;
                        }
                    } else {
                        // okay, we found a winner
                        /*
                        for idx in [(i,j,k),(i,k,j),(j,i,k),(j,k,i),(k,i,j),(k,j,i)] {
                            seen.insert( [nums[idx.0], nums[idx.1], nums[idx.2] ]);
                        }*/
                        // okay I misread the instructions
                        // 
                        eprintln!("found solution: {i}, {j}, {k} -> [{}, {}, {}]", nums[i], nums[j], nums[k]);
                        seen.insert([nums[i],nums[j],nums[k]]);
                    }
                }
            } // 'j' loop
        } // 'i' loop
        seen.into_iter().map(Vec::from).collect()
    }
}

fn main() {
    for input in [ 
            vec![34,55,79,28,46,33,2,48,31,-3,84,71,52,-3,93,15,21,-43,57,-6,86,56,94,74,83,-14,28,-66,46,-49,62,-11,43,65,77,12,47,61,26,1,13,29,55,-82,76,26,15,-29,36,-29,10,-70,69,17,49],
            //vec![-82, -11, -6, 13, 69]
        ]
    {
        // gotta move everything out of testcase at once
        println!("input: {:?}", input);
        let ans = Solution::three_sum(input);
        println!("--> answer: {:?}", ans);
    }
}
