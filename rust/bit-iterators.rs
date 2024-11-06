// allow "x.bit_iter()" for numeric types to return the bit numbers (with LSB=0) that are set in x
// this final version is based on from_str_radix_it_impl in mod.rs

// bit_iter() returns a sequence of u32s containing the bits that are set in the number
trait IntoBitIterator { type BitIteratorType : Iterator<Item = u32>; fn bit_iter(self) -> Self::BitIteratorType; }
struct BitIterator<T>(T);
// I learned how to do this from from_str_radix_int_impl
macro_rules! into_bit_iterator_impl {
    ($($num_type:ty)*) => {$(
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
    )*}
}
// mod.rs line 1379
into_bit_iterator_impl! { isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128 }

fn main() {
   println!("bit iterator on 0x40_i8: {:?}", (0x40_i8).bit_iter().collect::<Vec<_>>());
   println!("bit iterator on 0x80_u8: {:?}", (0x80_u8).bit_iter().collect::<Vec<_>>());
   println!("bit iterator on 0xFF_i16: {:?}", (0xFF_i16).bit_iter().collect::<Vec<_>>());
   println!("bit iterator on 0xFF_u16: {:?}", (0xFF_u16).bit_iter().collect::<Vec<_>>());
   println!("bit iterator on 0x4000_i16: {:?}", (0x4000_i16).bit_iter().collect::<Vec<_>>());
   println!("bit iterator on 0x8000_u16: {:?}", (0x8000_u16).bit_iter().collect::<Vec<_>>());
}
