// this is still unsolved
struct Solution {}

use std::ops::Range;
use std::cmp::Ordering;
use std::fmt::Debug;

// okay, so f64 is a little bit hostile here...
/*
#[derive(Copy, Clone, Eq, Ord, Debug)]
struct HalfInteger<T>(T, bool);

impl<T: Display> Display for HalfInteger<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)?;
        if self.1 {
            f.write_str(".5")?;
        }
    }
}

impl<T> From<T> for HalfInteger {
    fn 
}

impl<T> HalfInteger<T> {
}
*/

// find the median of the concatenation of n1 and n2,
// given that n1 and n2 are both sorted ascending AND every element of n2
// is greater than every element of n1
fn find_median_concat_arrays(n1: &[i32], n2: &[i32]) -> f64 {
    let total_len = n1.len() + n2.len();
    let median_index = (total_len - 1) / 2;

    let a = if median_index < n1.len() { n1[median_index] }
            else { n2[median_index - n1.len()] };

    match (total_len % 2) == 0 {
        false => a as f64,
        true => {
            let b = if median_index + 1 < n1.len() { n1[median_index + 1] }
             else { n2[median_index + 1 - n1.len()] };
            ((a + b) as f64) / 2.0
        },
    }
}

fn binary_search_range<T : Debug + Copy + Into<U>, U: Debug + PartialEq + PartialOrd>(s : &[T], target: U) -> Range<usize> {
    // given that @s is sorted
    // find the range of values EQUAL TO @target
    // if the specified value does not exist in the slice, return an empty range (end==start)
    // where start is equal to the position where the specified value WOULD belong.
    let start = 
        s.binary_search_by(|&item| 
            match item.into().partial_cmp(&target).unwrap() {
                Ordering::Less => Ordering::Less,
                Ordering::Greater | Ordering::Equal => Ordering::Greater
            }
        )
        .err()
        .unwrap();
    // degenerate case 1: target is less than every element of s
    // degenerate case 2: target is greater than every element of s
    // not-so-degenerate cases: s[start] < target (which means @target) does not exist in the slice
    if start == 0 || start == s.len() || s[start].into() < target {
        Range { start: start, end : start }
    } else {
        assert!(s[start].into() == target, "start = {}, s[start] = {:?}, target = {:?}", start, s[start], target);
        // okay s[start] == target, unless binary_seach somehow went awry
        let end =
            s[start..]
            .binary_search_by(|&item|
                match item.into().partial_cmp(&target).unwrap() {
                    Ordering::Less | Ordering::Equal => Ordering::Less,
                    Ordering::Greater => Ordering::Greater
                }
            )
            .err()
            .unwrap()
            ;
        Range { start, end }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // make 'n1' the longer array and 'n2' the shorter one
        let (mut n1, mut n2) = if nums1.len() >= nums2.len() { (nums1.as_slice(), nums2.as_slice()) }
                        else { (nums2.as_slice(), nums1.as_slice()) };
        let total_len = n1.len() + n2.len();
        // the median will be greater than exactly[1] floor(target_lt / 2) elements across
        // both arrays
        // [1] the problem statement doesn't say whether or not every array value is unique.
        //     
        let target_lt = total_len / 2;
        // okay let's consider:
        // first, n1.len() is always greater than zero (problem constraints
        // have at least one element, and n1 is the longest array)
        // lens: 1 -> halflen=0 and the median is at index 1
        // 2 -> halflen=1 and the median is the mean of [0] and [1]
        // 3 -> halflen=1 and the median is [1]
        // 4 -> halflen=2 and the median is the mean of [1] and [2]
        // 5 -> halflen=2 and the median is [2]
        let n1_halflen = (n1.len() - 1) / 2;
        let mut median : f64 = match n1.len() % 2 == 0 {
            false => n1[n1_halflen] as f64,
            true => ((n1[n1_halflen] + n1[n1_halflen + 1]) as f64) / 2.0,
        };
        // degenerate case: one array is empty, in which case the answer is the median of the
        // longer array.
        if n2.len() == 0 { println!("degenerate case 1: only one array"); return median; }
        // okay, we know that n1.len() and n2.len() are both greater than zero.
        // check for the next degenerate case (example 2), where the arrays could simply be
        // concatenated.
        if n1[n1.len()-1] <= n2[0] {
            return find_median_concat_arrays(n1, n2);
        } else if n2[n2.len() - 1] <= n1[0] {
            return find_median_concat_arrays(n2, n1);
        }

        let mut median_pos = binary_search_range(n2, median);
        println!("median_pos = {:?}", median_pos);
        assert!(median_pos.len() <= 1, "actual len = {}", median_pos.len());
        let mut num_lt = 
        
        0.0
    }
}

fn main() {
    for testcase in [
            //[vec![1,2], vec![3,4]],
            //[vec![3,4], vec![1,2]],
            //[vec![1,3], vec![2]],
            [vec![1,3], vec![2,4]],
        ]
    {
        // gotta move everything out of testcase at once
        let [array_n, array_m] = testcase;
        println!("finding median of {:?} and {:?}:", array_n, array_m);
        let ans = Solution::find_median_sorted_arrays(array_n, array_m);
        println!("--> answer: {}", ans);
    }
}
