// 27. Remove Element

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut write_ptr : usize = 0;
        for read_ptr in 0..nums.len() {
            if nums[read_ptr] != val {
                if read_ptr != write_ptr { nums[write_ptr] = nums[read_ptr]; }
                write_ptr += 1;
            }
        }
        write_ptr as i32        
    }
}
