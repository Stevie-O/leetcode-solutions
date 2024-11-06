// 38. Count and Say (look-and-say sequence).
// i'm not sure exactly how this one differs materially -- it doesn't edit self.slice inside the while loop, but I don't see why that would affect matters -- but this one works and the other doesn't

struct Solution{}

struct CountAndSayIterator<'a> {
    slice: &'a [u8],
    buffered_digit: Option<u8>,
}

impl<'a> CountAndSayIterator<'a> {
    pub fn new(slice: &'a [u8]) -> Self { 
        CountAndSayIterator {
            slice: slice,
            buffered_digit: None,
        }
    }
}

impl<'a> Iterator for CountAndSayIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        // if we've buffered a digit, take it
        //println!("buffered_digit: {:?} seq: {:?}", self.buffered_digit, self.slice);
        if self.buffered_digit.is_some() { return self.buffered_digit.take(); }
        if let &[first, ref rest @ ..] = self.slice {
            let mut rest = rest; // error[E0658]: mutable by-reference bindings are experimental
            // okay, there's at least one element left
            let mut count : u8 = 1;
            // let's see how many there are...
            // this didn't work :()
            //while let &[ _ @ first, rest @ .. ] = self.slice {
            while let &[ next, ref maybe_rest @ .. ] = rest {
                if next == first {
                    count += 1;
                    rest = maybe_rest;
                } else { break; }
            }
            self.slice = rest;

            //println!("count: {count} first: {first} rest: {:?}", rest);
            self.buffered_digit = Some(first);
            self.slice = rest;
            Some(count)
        } else {
            None
        }
    }
}
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut sequence = vec![1_u8];
        for _ in 1..n {
            sequence = CountAndSayIterator::new(&sequence).collect();
        }
        sequence.into_iter().map(|x| format!("{x}")).collect::<Vec<_>>().join("").to_string()

    }
}
fn main() {
    let mut seq = vec![1_u8];
    println!("{:?}", seq);
    for _ in 1..=30 {
        seq = CountAndSayIterator::new(&seq).collect();
        println!("{:?}", seq);
    }
    /*
    for input in [
            //vec![[57,213,269,42,135,1,130,16,5,283,181,46,56,30],[170,260,222,279,276,50,182,264,256,255,233,82,300,19],[300,68,261,241,96,205,1,254,201,95,124,132,40,220],[212,38,50,38,176,271,109,27,83,195,165,7,180,224],[238,106,44,264,157,180,94,92,119,142,59,269,275,189],[35,34,226,241,45,195,200,178,104,92,255,170,49,91],[94,137,145,78,49,243,263,113,230,144,249,192,36,276],[271,241,12,126,232,192,294,212,239,199,49,106,177,104],[243,206,191,14,110,62,137,184,185,12,259,217,285,209],[124,197,299,299,4,36,245,160,182,300,7,170,50,67],[77,39,189,185,277,223,122,181,55,97,97,85,215,287],[124,42,198,38,204,87,172,9,102,232,112,71,186,296],[13,101,254,158,289,112,89,172,194,269,27,171,149,223]]
            vec![[137,112,78,67],[76,65,122,135]]
        ]
    {
        // gotta move everything out of testcase at once
        println!("input: {:?}", input);
        let input = input.into_iter().map(Vec::from).collect();
        let ans = Solution::max_moves(input);
        println!("--> answer: {:?}", ans);
    }
    */
}
