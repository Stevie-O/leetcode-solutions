// numbers as a sum of squares, huh
// well it's _guaranteed_ to work, since 1 is a square, but...
// will a greedy algorithm work, I wonder?
// NO. the first example case proves it:
//  12 = 9 + 1 + 1 + 1
//  12 = 4 + 4 + 4
// so let's try some dynamic programming, shall we?

// man, I wish I had isqrt

use std::collections::HashMap;
trait IntegerSquareRoot {
    fn integer_sqrt(self) -> Self;
}

#[allow(non_snake_case)]
impl IntegerSquareRoot for u32 {
    fn integer_sqrt(self) -> Self {
       
    let mut L : Self; // = 0;
	let mut M;
	let mut R : Self; //= self + 1;

    let log2 = self.ilog2();
   if log2 <= 0 {
		return self; // if log2 <= 0, then y==0 or y==1, which means sqrt(y)==y
   } else {
/* consider this:
     sqrt(y) = y^0.5 = b^(log_b(y)/2))
     LZCNT/BSR can easily give us floor(log2(y))
     let's define "flog2(x)" == "hibit(x)" == floor(log_2(x))
		 
consider these properties:
2**(hibit(x)) <= x
2**(hibit(x)+1) > x

if the 2nd property isn't obvious:
hibit(x) is the power of the highest bit set in x.
	e.g. hibit(0b1) = 0, hibit(0b10) = 1, flog2(0b11) = 1, 
	hibit(0b100) = 2, hibit(0b111) = 2, hibit(0b1000) = 3.
			
Basically, it is one less than the number of bits required to store _x_.
			
2**(hibit(x)) is the smallest value z such that hibit(z) = hibit(x); that is,
	they require the same number of bits to store.

Since 2**(hibit(x) + 1) is twice as large, it requires one additional bit to store, compared
to 2**(hibit(x)).  However, we already established that x and 2**(hibit(x)) require the same
storage space, so 2**(hibit(x) + 1) must be greater than x.

let's say hibit(y) is even:
 	first off, 2**(hibit(y)) is a perfect square -- (2**(hibit(y)/2)) ** 2
	2**(hibit(y)+2) is also a perfect square -- (2**(hibit(y)/2 + 1)) ** 2
	Since 2**(hibit(y)+2) > y (in fact, 2**(hibit(y)+2) > 2y),
		then 2**(hibit(y)/2 + 1) > sqrt(2y) > sqrt(y)

	This gives us lower- and upper-bounds on sqrt(y): 2**(hibit(y)/2) <= sqrt(y) < 2**(hibit(y)/2 + 1)

let's say hibit(y) is odd:
	yuck, this is nasty.  there's a sqrt(2) here, but I _should_ be able to shave off a step or two.

		*/

		let log2_sqrt = log2 >> 1;
		L = 1 << log2_sqrt;
		let cand_r = 1 << (log2_sqrt + 2);
		//if (cand_r < R) 
		R = cand_r;
	}
	
	//int steps = 0;

	while L != R - 1
	{
		//steps++;
		M = (L + R) / 2;

        if M.checked_mul(M).is_some_and(|ans| ans <= self) {
            L = M;
        } else {
            R = M;
        }
	}

	//Console.WriteLine("steps: {0}", steps);
	//return (L, steps);

        L
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
struct NumSquaresSolver(HashMap<u32, u32>);

impl NumSquaresSolver {
    fn new() -> Self { Default::default() }
    fn try_num_squares(&mut self, n : u32, limit: u32) -> Option<u32> {
        // this little wrapper makes the logic inside _core simpler to follow
        match self.try_num_squares_core(n, limit) {
            Some(ans) if ans <= limit => Some(ans),
            _ => None,
        }
    }
    fn try_num_squares_core(&mut self, n : u32, mut limit: u32) -> Option<u32> {
        if n > 0 && limit == 0 { return None; }
        // for 0, 1, 2, 3: it's a sum of n '1's
        if n < 4 { return Some(n); }
        // if we already know the answer, then cool
        if let Some(&cached) = self.0.get(&n) {
            eprintln!("try_num_squares({n}, limit = {limit}) => {cached} (cached)");
            return Some(cached);
        }

        // find out the square root of n
        let sqrt = n.integer_sqrt();
        eprintln!("sqrt({}) -> {} -> {}", n, sqrt, sqrt * sqrt);
        if n == sqrt * sqrt { 
            self.0.insert(1, 1);
            return Some(1);
        }
        if limit == 1 { return None; }
        let _input_limit = limit; 
        
        // okay, so there's definitely an optimization somewhere in here, though
        // I can't quite see what it _is_ just yet
        // okay I've got it

        let mut _best_square = None;
        let mut best_answer = None;
        for square in (1..=sqrt).rev().map(|x| x * x) {
            // here's the optimization: if we remove @square, we aren't removing
            // any squares GREATER than @square from @n
            // if @limit copies of @square do not add up to at least @n,
            // then not only will @square not lead to a valid solution,
            // no value less than @square will, either, so we can stop here.
            if (n / square) > limit { break; }
            eprintln!("trying {n} - {square}");
            let answer = self.try_num_squares(n - square, limit - 1);
            eprintln!("try_num_squares({n} - {square} = {}, limit = {}) => {:?}",
                        n - square, limit - 1, answer);
            if let Some(answer) = answer {
                let answer = 1 + answer; // add in the s'square' we removed
                _best_square = Some(square);
                best_answer = Some(answer);
                limit = answer;
            }
        }
        if let Some(answer) = best_answer {
            eprintln!("try_num_squares({n}, limit={_input_limit}) = {answer} (removed: {})", _best_square.unwrap());
            self.0.insert(n, answer);
            Some(answer)
        } else {
            eprintln!("try_num_squares({n}, limit={_input_limit}) = no answer");
            None
        }
    }
}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut solver = NumSquaresSolver::new();
        solver.try_num_squares(n as u32, n as u32).unwrap() as i32
    }
}

struct Solution{}
fn main() {
    for input in [
                //12,
                207,
            ]
    {
        println!("input: {}", input);
        let ans = Solution::num_squares(input);
        println!("-> answer: {ans}");
    }
}
