// 951 Flip equivalent binary trees

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn sort_nodes(n1: Rc<RefCell<TreeNode>>, n2: Rc<RefCell<TreeNode>>) -> (i32, i32, Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>)
    {
        let n1_val = n1.borrow().val;
        let n2_val = n2.borrow().val;
        if n1_val <= n2_val { (n1_val, n2_val, n1, n2) }
        else { (n2_val, n1_val, n2, n1) }
    }
    
    fn children_equiv( 
            a_left : Option<Rc<RefCell<TreeNode>>>,
            a_right : Option<Rc<RefCell<TreeNode>>>,
            b_left :  Option<Rc<RefCell<TreeNode>>>,
            b_right: Option<Rc<RefCell<TreeNode>>>
        ) -> bool {
            // turns (None, Some(x)) into (Some(x), None); everything else remains unchanged
            fn sort_options<T>(o1: Option<T>, o2: Option<T>) -> (Option<T>, Option<T>) {
                if o1.is_none() && o2.is_some() { (o2, o1) } else { (o1, o2) }
            }

            let (a0, a1) = sort_options(a_left, a_right);
            let (b0, b1) = sort_options(b_left, b_right);

            match (a0, a1, b0, b1) {
                (None, None, None, None) => true, // both nodes are leaf nodes
                (Some(a), None, Some(b), None) => Self::flip_equiv( Some(a), Some(b) ),
                (Some(a0), Some(a1), Some(b0), Some(b1)) => {
                    let (a0n, a1n, a0, a1) = Self::sort_nodes(a0, a1);
                    let (b0n, b1n, b0, b1) = Self::sort_nodes(b0, b1);
                    if a0n == b0n && a1n == b1n {
                        // both children have the same value
                        Self::flip_equiv(Some(a0), Some(b0)) &&
                        Self::flip_equiv(Some(a1), Some(b1))
                    } else {
                        false
                    }
                }
                _ => false,
                //_ => panic!("sort_options produced invalid input"),
            }
    }

    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a_ref = a.borrow();
                let b_ref = b.borrow();
                if a_ref.val != b_ref.val { false }
                else {
                    let (a_left, a_right, b_left, b_right) =
                        (a_ref.left.clone(), a_ref.right.clone(),
                         b_ref.left.clone(), b_ref.right.clone());
                    drop(a_ref); drop(b_ref);
                    Self::children_equiv(a_left, a_right, b_left, b_right)
                }
            }
            (None, Some(_)) | (Some(_), None) => false,
        }

        
    }
}
