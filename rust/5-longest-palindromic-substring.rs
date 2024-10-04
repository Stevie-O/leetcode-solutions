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

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest = Palindrome { first: 0, last: 0 };
        let mut longest_len = longest.len();
        let s_bytes = s.as_bytes();
        //let mut palindromes = Vec<Palindrome>::with_capacity(s.len() + s.len() / 2);
        /* this is an interesting problem.
         * it's very simple and obvious to brute-force it, but that would be slow...
         *
         * What kind of assumptions can we make?
         * - Under what circumstances can palindromes _overlap_?
         * - If we can determine circumstances under which palindromes _cannot_ overlap, can we leverage Boyer-Moore somehow?
         * - Can we leverage Boyer-Moore somehow?
         */

         /* Also, from a design perspective, there are *two* different kinds of palindromes: 
          * - Ones that are reflections around a central letter (the most common kind): Bob, racecar, tenet, level, Aha
          * - Ones that are actually fully mirrored: Abba, sees, Dennis sinned, Warsaw was raw
          */

        let mut prev_ch : u8 = 0;
        for (idx, &ch) in s_bytes.iter().enumerate() {
            //palindromes.push( Palindrome { idx, idx });
            let mut pal = Palindrome { first: idx, last: idx };
            let new_size = pal.grow(&s_bytes);
            if (new_size > longest_len) {
                longest = pal;
                longest_len = new_size;
            }
            if (idx > 0 && ch == prev_ch) {
                //palindromes.push( Palindrome { idx - 1, idx });
                let mut pal = Palindrome { first: idx - 1, last: idx };
                let new_size = pal.grow(&s_bytes);
                if (new_size > longest_len) {
                    longest = pal;
                    longest_len = new_size;
                }
            }
            prev_ch = ch;
        }

/*
        for i in 0..palindromes.len() {
            let mut pal = &mut palindromes[i];
            while (!pal.try_grow)
        }*/

        String::from_utf8((&s_bytes[ (longest.first) ..= (longest.last) ]).to_vec()).unwrap()      
        
    }
}
