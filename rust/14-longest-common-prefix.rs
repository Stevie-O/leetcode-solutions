struct Solution {}
impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.len() == 1 { return std::mem::replace(&mut strs[0], Default::default()); }
        strs.sort();
        //if strs[0].len() == 1 { return std::mem::replace(&mut strs[0], Default::default()); }

        let common_length =
            strs[0].chars()
            .zip(strs[strs.len()-1].chars())
            .take_while(|(ch1, ch2)| ch1 == ch2)
            .count()
            ;
        
        strs[0].chars().take(common_length).collect()
    }
}

fn main() {
    for input in [ 
            vec!["a", "b"],
        ]
    {
        // gotta move everything out of testcase at once
        println!("input: {:?}:", input);
        let ans = Solution::longest_common_prefix(input.into_iter().map(|s| String::from(s)).collect());
        println!("--> answer: {}", ans);
    }
}
