use std::mem::swap;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {

    fn add_two_numbers_core(mut it1: &mut ListNode, mut it2: &mut ListNode) {

        let mut carry : i32 = 0;
        // here are the rules:
        // it1 and it2 currently point to a dummy element that's the first one
        while let (Some(a), Some(b)) = (it1.next.as_deref_mut(), it2.next.as_deref_mut()) {
            let sum = a.val + b.val + carry;
            let digit_sum;
            if (sum >= 10) {
                digit_sum = sum - 10;
                carry = 1;
            } else {
                digit_sum = sum;
                carry = 0;
            }

            a.val = digit_sum;
            it1 = it1.next.as_mut().unwrap();
            it2 = it2.next.as_mut().unwrap();
        }

        // okay, either it1.next is None or it2.next is None, or BOTH
        // if it2.next is not None, swap them. otherwise, the remainder of the digits to be processed, if any, already hang off of it1.next
        //if let Some(_) = it2.next {
        if it2.next.is_some() {
            std::mem::swap( &mut it1.next, &mut it2.next );
        }

        // see if there's any carry to be had
        while carry != 0 {
            if let Some(a) = it1.next.as_deref_mut() {
                // there are more digits in the number
                a.val = a.val + carry;
                if (a.val >= 10) {
                    a.val -= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }
                it1 = it1.next.as_mut().unwrap();
            } else {
                it1.next = Some(Box::new(ListNode::new(carry)));
                carry = 0;
            }
        }
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut carry : i32 = 0;
        let mut l1wrap = ListNode { next: l1, val: 0 };
        let mut l2wrap = ListNode { next: l2, val: 0 };

        // Rust *hates* linked lists, because memory safety enforcement (specifically, ownership and aliasing restrictions)
        // on linked lists is nigh-impossible.
        // However, if you're not _building_ a list, life is a lot simpler.

        Self::add_two_numbers_core(&mut l1wrap, &mut l2wrap);

        l1wrap.next
    }
}