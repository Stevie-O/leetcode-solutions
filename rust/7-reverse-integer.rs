struct DigitIterator(i32);

impl Iterator for DigitIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 { return None; }
        let dig = (self.0 % 10);
        self.0 = self.0 / 10;
        Some(dig)
    }
}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // special case
        if x == 0 { return x }
        // okay, so:
        // 1. no 64-bit arithmetic allowed (not sure how they'd enforce this, but whatever)
        // 2. if the answer is out-of-bounds for an i32, return 0
        
        // so there's only one number that can be represented as a negative but not a positive,
        // and that is i32::MIN, which is -2_147_483_648
        // the reverse of that would be -8_463_847_412
        // which is not a valid i32, so it's impossible to have that happen.
        // this means that I can extract the sign
    
        let sgn = x.signum();
        let x = x.abs();
        let mut ans : i32 = 0;
        for dig in DigitIterator(x) {
            // max value is i32::MAX
            // if ans is greater than i32::MAX/10, then multiplying by 10 will overflow
            if ans > (i32::MAX / 10) { return 0; }
            ans = ans * 10;
            // if ans is greater than i32::MAX-dig, then ans+dig is greater than i32::MAX
            // and adding dig will overflow
            if ans > i32::MAX - dig { return 0; }
            ans = ans + dig;
        }

        return ans * sgn;
    }
}
