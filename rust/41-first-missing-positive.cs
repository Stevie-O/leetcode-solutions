// 41. first missing positive
// got it in one :)


fn swap_chain(nums: &mut [i32], mut read_index : usize) {
    // if I wanted to be a bit more pure, I'd preserve the initial read_index and swap the final write_value
    let mut write_value = nums[read_index];
    loop {
        // if write_value is not positive, there is nothing (more) to do
        if write_value <= 0 { return; }
        let write_index = (write_value - 1) as usize;
        // if the number in question is already where it needs to be, there is nothing (more) to do
        if write_index == read_index { return; }
        // if the proper destination is outside of the array, there is nothing to do.
        if write_index >= nums.len() { return; }
        // we are going to read the value from <read_index> 
        let saved = nums[write_index];
        nums[write_index] = write_value;
        // now we've just read @saved in from @write_index
        // follow the chain
        (read_index, write_value) = (write_index, saved);
    }
}
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        // O(n) time, so a fixed number of passes: NO SORTING since all sorting is at least O(n log n)
        // O(1) space(!?!!)
        // is this really even possible?
        // well, with the definition here, I can destroy @nums
        // I'm sure it can't be done without O(n) writable space
        // okay, so let's ignore all values that are less than 1
        // oooh.
        let mut read_ptr : usize = 0;
        let mut write_ptr : usize = 0;
        // I wonder if this is actually O(n) time given the likely number of cache misses...
        // place every value that's in the range 1..=nums.len() into its correct index
        //     this is done one at a time.
        //     Every element in the array will trigger at most TWO reads in this loop (the array element itself, and if in-range,
        //          the array element with the index where that value belongs)
        //     Every element in the array will be written to AT MOST once
        // therefore, this is O(n + n + n) = O(3n) = O(n)
        for idx in 0..nums.len() {
            swap_chain(&mut nums, idx);
        }
        let nums_count = nums.len() as i32;
        // this is a single pass, so O(n) -> O(3n + n) = O(4n) = O(n)
        nums
            .into_iter() // convert to an iterator
            .enumerate() // enumerate it
            .find(|&(idx, val)| val - 1 != (idx as i32) ) // find the first element with the wrong value
            .map(|(idx, val)| (idx as i32) + 1) // and return the value that SHOULD be there
            .unwrap_or( nums_count + 1 )        // and if there is no such element, then it's equal to Vec::from(1..=nums.len()) in which case the next missing positive integer is nums.
    }
}
