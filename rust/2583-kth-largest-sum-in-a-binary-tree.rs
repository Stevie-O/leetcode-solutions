// 2583. Kth Largest Sum in a Binary Tree

struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        // oof, nasty.
        let mut heap = BinaryHeap::new();
        let mut row : Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let fake_root = TreeNode { val: -1, left: root, right: None };
        row.push(Rc::new(RefCell::new(fake_root)));
        while row.len() > 0 {
            let mut next_row : Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            let mut sum : i64 = 0;
            for item in row {
                let item = item.borrow();
                sum += item.val as i64;
                if let Some(child) = &item.left { next_row.push(child.clone()); }
                if let Some(child) = &item.right { next_row.push(child.clone()); }
            }
            // weird behavior: BinaryHeap is documented as a MAX-HEAP
            // but into_sorted_vec returns elements in increasing order
            heap.push(Reverse(sum));
            row = next_row;
        }

        println!("heap: {:?}", heap.clone().into_sorted_vec().into_iter().map(|rn| rn.0).collect::<Vec<_>>());
        let k = k as usize;
        heap.into_sorted_vec().into_iter().map(|rn| rn.0).skip(k-1).nth(0).unwrap_or(-1)
    }
}
enum NextChild { Root, Left, Right }
fn build_tree(input: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root : Option<Rc<RefCell<TreeNode>>> = None;
    let mut row : Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut next_row : Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut next_child = NextChild::Root;
    // man, this is really messy
    let mut row_iter = row.iter();
    let mut cur_parent : Option<Rc<RefCell<TreeNode>>> = None;
    for num in input {
        let new_node = 
                if num == i32::MAX { None } 
                else { Some(Rc::new(RefCell::new(TreeNode::new(num)))) }
                ;
        next_row.push(new_node.clone());
        if match next_child {
                NextChild::Root => {
                    root = new_node.clone();
                    next_child = NextChild::Left;
                    true
                },
                NextChild::Left => {
                    let mut par = cur_parent.as_ref().unwrap().borrow_mut();
                    par.left = new_node;
                    next_child = NextChild::Right;
                    false
                }
                NextChild::Right => {
                    let mut par = cur_parent.as_ref().unwrap().borrow_mut();
                    par.right = new_node;
                    next_child = NextChild::Left;
                    true
                }
            } {
            // starting a new node
            if let Some(next_parent) = row_iter.next() {
                cur_parent = next_parent.clone();
            } else {
                // starting a new ROW!
                // shift over
                (row, next_row) = (next_row, Vec::new());
                row_iter = row.iter();
                cur_parent = row_iter.next().unwrap().clone();
            }
         }
    }
    root
}
fn main() {
    for (treedef, k) in [ 
            (vec![5,8,9,2,1,3,7,4,6], 2)
        ]
    {
        // gotta move everything out of testcase at once
        println!("input: {:?}, k={}:", treedef, k);
        let ans = Solution::kth_largest_level_sum(build_tree(treedef), k);
        println!("--> answer: {}", ans);
    }
}
