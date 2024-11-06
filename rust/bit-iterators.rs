// allow "x.bit_iter()" for numeric types to return the bit numbers (with LSB=0) that are set in x
// this is after I learned a bit -- it uses a single generic type for all iterator implementations

// bit_iter() returns a sequence of u32s containing the bits that are set in the number
trait IntoBitIterator { type BitIteratorType : Iterator<Item = u32>; fn bit_iter(self) -> Self::BitIteratorType; }
struct BitIterator<T>(T);
macro_rules! define_bit_iterator {
    ($num_type:ty) => {
        impl Iterator for BitIterator<$num_type> {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 0 { None }
                else {
                    let bitnum = self.0.trailing_zeros();
                    self.0 &= self.0 - 1;
                    Some(bitnum)
                }
            }
        }
        impl IntoBitIterator for $num_type {
            type BitIteratorType = BitIterator<$num_type>;
            fn bit_iter(self) -> Self::BitIteratorType {
                BitIterator(self)
            }
        }
    }
}
define_bit_iterator!(i16);
define_bit_iterator!(u16);
define_bit_iterator!(u128);

fn main() {
   println!("bit iterator on 0xFF_i16: {:?}", (0xFF_i16).bit_iter().collect::<Vec<_>>());
   println!("bit iterator on 0xFF_u16: {:?}", (0xFF_u16).bit_iter().collect::<Vec<_>>());
   println!("bit iterator on 0x4000_i16: {:?}", (0x4000_i16).bit_iter().collect::<Vec<_>>());
   println!("bit iterator on 0x8000_u16: {:?}", (0x8000_u16).bit_iter().collect::<Vec<_>>());
}
