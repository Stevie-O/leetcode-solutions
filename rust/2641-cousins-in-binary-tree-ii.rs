// 2641. Cousins in Binary Tree II
// solution based on my solutio to 2583. Kth Largest Sum in a Binary Tree

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
impl Solution {
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut row = Vec::new();
        row.push( (0_i32, [root.clone(), None]) );
        while row.len() > 0 {
            let mut row_sum : i32 = 0;
            // kind of annoyed at how many linear O(n) operations we're doing here
            // scan number one:
            let row_with_sums : Vec<_> = row
                .into_iter()
                .map(|(_, pair_array)| {
                    let pair_sum : i32 = pair_array.iter()
                        .filter_map(|item| item.as_ref() )
                        .map(|item| item.borrow().val)
                        .sum();
                    // dicey proposition #1: mutation of captured outer variable
                    // from closure
                    row_sum += pair_sum;
                    (pair_sum, pair_array)
                })
                .collect();
            // scan number two:
            let new_row = row_with_sums
                //.into_iter() -- does not work
                // using .into_iter() means that the pair_array
                // that backs the slice returned from the flat_map
                // callback DOES NOT LIVE LONG ENOUGH
                .iter()
                .flat_map(|(pair_sum, pair_array)| {
                    let slice = match pair_array {
                        [Some(_), Some(_)] => &pair_array[0..2],
                        [Some(_), None] => &pair_array[0..1],
                        [None, Some(_)] => &pair_array[1..2],
                        [None, None] => &pair_array[0..0], 
                    };
                    for child in slice {
                        let mut node = child.as_ref().unwrap().borrow_mut();
                        node.val = row_sum - pair_sum;
                    }
                    // new row is all children of current nodes
                    slice.into_iter().map(|c| {
                        // get to the node
                        let n = c.as_ref().unwrap().borrow();
                        (0_i32, [n.left.clone(), n.right.clone()])
                    })
                })
                .collect();
            row = new_row;
        }
        root
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
    for treedef in [ 
            vec![5,4,9,1,10,i32::MAX,7],
            vec![3,1,2],
        ]
    {
        // gotta move everything out of testcase at once
        println!("input: {:?}", treedef);
        let ans = Solution::replace_value_in_tree(build_tree(treedef));
        println!("--> answer: {:?}", ans);
    }
}
