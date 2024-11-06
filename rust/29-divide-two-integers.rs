// 29. Divide Two Integers
// gives wrong answer on i32::MIN divided by -1
// somehow the answer reported is "-2147483648" which does not seem to be possible

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let final_sign = dividend.signum() * divisor.signum();
        let divisor = divisor.abs();

        let mut quotient : i32 = 0;
        let mut dividend = if dividend == i32::MIN {
                            // remove one 'divisor' from @dividend
                            quotient += 1;
                            (dividend + divisor).abs()
                        } else { dividend.abs() };

        if dividend < divisor { return quotient * final_sign; }

        // find log2 of dividend and divisor
        let num_ilog2 = dividend.ilog2();
        let den_ilog2 = divisor.ilog2();
        // the above dividend<divisor check should guarantee that num_ilog2 >= den_ilog2
        let mut div_bit : i32 = (1 << (num_ilog2 - den_ilog2));
        let mut divisor  = divisor << (num_ilog2 - den_ilog2);

        while div_bit > 0 {
            if dividend >= divisor  {
                quotient += div_bit;
                dividend -= divisor;
            }
            divisor >>= 1;
            div_bit >>= 1;
        }

        // final value in divisor is the REMAINDER
        quotient * final_sign
    }
}
