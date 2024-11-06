// 38. Count and Say (look-and-say sequence).
// this version doesn't quite work

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
        if self.buffered_digit.is_some() { return self.buffered_digit.take(); }
        if let &[first, ref rest @ ..] = self.slice {
            // okay, there's at least one element left
            let mut count : u8 = 1;
            self.slice = rest;
            // let's see how many there are...
            // this didn't work :()
            //while let &[ _ @ first, rest @ .. ] = self.slice {
            while let &[ next, ref rest @ .. ] = self.slice {
                if next == first {
                    count += 1;
                    self.slice = rest;
                } else { break; }
            }
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
