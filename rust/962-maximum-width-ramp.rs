// problem 962 (Maximum Width Ramp)
// first attempt. failed with Time Limit Exceeded
// max input length is 50,000. since we're O(n^2) 

impl Solution {
    fn max_width_ramp_REVERSE_SLOW(nums: Vec<i32>) -> i32 {
        // try every possible ramp size IN REVERSE
        for longest_ramp in (1..nums.len()).rev() {
            for i in 0 .. (nums.len() - longest_ramp) {
                if nums[i] <= nums[i + longest_ramp] { 
                    return longest_ramp as i32;
                }
            }
        }
        0
    }
    fn max_width_ramp_SLOW(nums: Vec<i32>) -> i32 {
        let mut longest_ramp : usize = 0;
        for i in 0 .. nums.len()-1 {
            for j in i+longest_ramp+1 .. nums.len() {
                if (nums[i] <= nums[j]) {
                    longest_ramp = j - i;
                }
            }
        }
        longest_ramp as i32
    }
}
