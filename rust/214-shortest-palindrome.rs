// this was actually the first several tries
//	- once I got it compiling, I failed to handle the empty string correctly
//	- then it turns out that I'm only allowed to prepend characters to the beginning of the string
//  - then it was giving the right answers but I exceeded the maximum runtime when given an "a" followed by 41999 "b"s

struct Palindrome {
    first: usize,
    last:  usize,
}

impl Palindrome {
    pub fn len(&self) -> usize { (self.last - self.first) + 1 }
    pub fn grow(&mut self, string: &[u8]) -> usize {
        // we can't grow if there's no room to grow
        while self.first > 0 && self.last < string.len() - 1
            && (string[self.first-1] == string[self.last + 1]) {
            
            self.first -= 1;
            self.last += 1;
        }
        self.len()
    }
}

struct Solution {}

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.len() == 0 { return s; }
        let mut longest = Palindrome { first: 0, last: 0 };
        let mut longest_len = longest.len();
        let s_bytes = s.as_bytes();
		// so my solution here is based on my "Longest Palindrome" solution
		// here, we're looking for the longest palindrome that either starts or ends at the edge of the string

        let mut prev_ch : u8 = 0;
        for (idx, &ch) in s_bytes.iter().enumerate() {
            let mut pal = Palindrome { first: idx, last: idx };
            let new_size = pal.grow(&s_bytes);
            if new_size > longest_len  // it beats out our existing best palindrome
				&& (pal.first == 0 /*|| pal.last == s_bytes.len() - 1 */)     // AND the only reason it couldn't be bigger is because we ran off the end of the string
			{
                longest = pal;
                longest_len = new_size;
            }
            if idx > 0 && ch == prev_ch {
                let mut pal = Palindrome { first: idx - 1, last: idx };
                let new_size = pal.grow(&s_bytes);
                if (new_size > longest_len) && (pal.first == 0 /*|| pal.last == s_bytes.len() - 1 */) {
                    longest = pal;
                    longest_len = new_size;
                }
            }
            prev_ch = ch;
        }
		
		// degenerate case: s is already a palindrome
		if longest_len == s_bytes.len() { return s; }
		
		// the new length will be (s_bytes.len() + s_bytes.len() - longest_len)
		let new_size = 2 * s_bytes.len() - longest_len;
		let mut palindrome_buf : Vec<u8> = Vec::with_capacity(new_size);
		if longest.first == 0 {
			palindrome_buf.extend( (&s_bytes[ (longest.last+1).. ]).iter().rev() );
		}
		palindrome_buf.extend_from_slice( &s_bytes );
		if longest.first > 0 {
			palindrome_buf.extend( (&s_bytes[ 0 .. (longest.first) ]).iter().rev() );
		}
		
        String::from_utf8(palindrome_buf).unwrap()
    }
}
