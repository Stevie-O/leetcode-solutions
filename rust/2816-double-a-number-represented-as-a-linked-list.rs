use std::mem::replace;

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

fn double_digit(node: &mut Box<ListNode>, carry : i32) -> i32 {
    let mut sum = node.val * 2 + carry;
    let out_carry : i32;
    if sum >= 10 {
        sum -= 10;
        out_carry = 1;
    } else {
        out_carry = 0;
    }
    node.val = sum;
    out_carry
}

impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head.unwrap();
        let mut node_stack : Vec<Box<ListNode>> = Vec::new();

        while let Some(_) = head.next {
            let tail = std::mem::replace(&mut head.next, None).unwrap();
            node_stack.push(head);
            head = tail;
        }

        let mut carry : i32 = 0;
        // while !node_stack.is_empty() 
        while let Some(mut newhead) = node_stack.pop() {
            carry = double_digit(&mut head, carry);
            //let mut newhead = node_stack.pop();
            std::mem::replace(&mut newhead.next, Some(head));
            head = newhead;
        }

        carry = double_digit(&mut head, carry);

        if carry > 0 {
            let mut newhead = Box::new(ListNode::new(carry));
            std::mem::replace(&mut newhead.next, Some(head));
            head = newhead;
        }

        Some(head)
    }
}
