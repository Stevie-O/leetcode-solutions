struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        // an array with fewer than 2 elements is always sorted
        if arr.len() < 2 { return 0; }

        // hmm... an interesting idea...
        // so the subarray in question must be contiguous.
        // possibilities:
        // 1. already sorted (answer is 0 elements need to be removed)
        // 2. out-of-order at the front
        // 3. out-of-order at the end
        // 4. out-of-order in the middle

        // there's some special handling needed for a strictly-decreasing sequence
        // at the beginning of the array

        let arr_slice = arr.as_slice();

        let increasing_segments = {
            let initial_state = (Vec::new(), arr_slice[0], (0, 1));
            let (mut vec, _, range) = arr_slice.iter().enumerate().skip(1).fold(initial_state,
                |mut state, (index, &next)| {
                    //println!("considering: index={index}, next={next}, prev={}", state.1);
                    if state.1 <= next {
                    //    println!("in order");
                        // nondecreasing
                        state.2.1 = index + 1;
                    } else {
                      //  println!("out of order");
                        // whoops, we went backwards!
                        // first, limit ourselves to two segments.
                        // only the first and last segments matter for any of the math
                        if state.0.len() >= 2 { state.0.pop(); }
                        state.0.push(state.2);
                        state.2 = (index, index + 1);
                    }
                    state.1 = next;
                    //println!("new state: {state:?}");
                    state
                });
            if vec.len() >= 2 { vec.pop(); }
            vec.push(range);
            vec
        };
        
        println!("increasing_segments: {increasing_segments:?}");

        let ans = match increasing_segments[..] {
            [] => unreachable!(),
            [_] => 0, // array is already sorted, that was easy
            [first, .., last] => {
                let first = first.0 .. first.1;
                let last = last.0 .. last.1;
                println!("first = {first:?}, last = {last:?}");
                println!("{:?} <-> {:?}", &arr_slice[first.clone()], &arr_slice[last.clone()]);
                println!("check 1: {} <= {}", arr_slice[first.end - 1], arr_slice[last.start]);
                println!("check 2: {} <= {}", arr_slice[last.end - 1], arr_slice[first.start]);
                if arr_slice[first.end - 1] <= arr_slice[last.start] {
                    // deleting those middle segments makes it sorted
                    last.start - first.end
                }
                else if arr_slice[last.end - 1] < arr_slice[first.start] {
                    // okay, first and last segments are strictly  out of order with regard to each other
                    // we can only keep first or last, so discard the shorter of the two
                    if first.len() < last.len() {
                        // remove [0..last.start] 
                        arr_slice[0..last.start].len()
                    } else {
                        // remove [first.end..
                        arr_slice[first.end..].len()
                    }
                }
                else {
                    // the nastiest case: the 'first' and 'last' sections overlap somewhat
                    // so we need to delete 0 or more elements from the END of 'first'
                    // and 0 or more elements from the BEGINNING of 'last'
                    // (but for a total of at least 1 element

                    // there might a way to do this in O(log n) time -- the problem seems similar
                    // to the "find median of two sorted arrays" problem -- but the fact is that
                    // this is O(n log n) time which is probably fine
                    let mut best_answer = arr_slice.len();
                    println!("best answer: {best_answer}");
                    for first_keep in first.clone() {
                        let first_keep = first_keep + 1;
                        let val = arr_slice[first_keep - 1];
                        let last_discard_point = last.start + &arr_slice[last.clone()].partition_point(|&n| n < val);
                        let total_discard_count = last_discard_point - (first_keep);
                        println!("considering: {:?}, {:?} ({total_discard_count})", &arr_slice[0..first_keep], &arr_slice[last_discard_point..]);
                        best_answer = std::cmp::min(best_answer, total_discard_count); 
                    }
                    best_answer
                }
            },
        };
        ans as i32
   }
}

fn main() {
    for input in [
        //vec![1,2,3,10,4,2,3,5],
        //vec![2,2,2,1,1,1],
        vec![13,0,14,7,18,18,18,16,8,15,20],
    ]
    {
        println!("input: {input:?}");
        let ans = Solution::find_length_of_shortest_subarray(input);
        println!("-> {ans}");
    }
}
