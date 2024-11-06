// 23. Merge k sorted lists
// given a vector of linked lists, each of which is in sorted order,
// return a new linked list that contains the nodes from every input list, also in sorted order

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

/// Iterator for linked list nodes that returns one node at a time
struct ListNodeIntoIterator(Option<Box<ListNode>>);
impl Iterator for ListNodeIntoIterator {
    type Item = Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(mut node) => {
                self.0 = node.next.take();
                Some(node)
            }
        }
    }
}

use std::cmp::Ordering;
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for ListNode { 
    fn cmp(&self, other: &Self) -> Ordering { self.val.cmp(&other.val) }
}
/*
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNodeCmpWrapper(Box<ListNode>)
impl Ord for ListNodeCmpWrapper {
    fn cmp(&self, other: &Self) -> Ordering { self.val.cmp(other.val) }
}
*/

use std::collections::BinaryHeap;
use std::cmp::Reverse; // because BinaryHeap just has to be Different

// man, this is a pain
#[derive(Debug, Clone)]
struct SortedIteratorHeapElement<I : Iterator<Item: Ord>>(I::Item, I);
// do I really have to be this verbose?
impl<I : Iterator<Item: Ord>> Eq for SortedIteratorHeapElement<I> {}
impl<I : Iterator<Item: Ord>> PartialEq for SortedIteratorHeapElement<I> {
    fn eq(&self, other: &Self) -> bool { other.0.eq(&self.0) }
}
impl<I : Iterator<Item: Ord>> PartialOrd for SortedIteratorHeapElement<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl<I : Iterator<Item: Ord>> Ord for SortedIteratorHeapElement<I> {
    // NOTE: we compare other.0 to self.0 because the stupid BinaryHeap ordering is MAX-HEAP which is exactly the opposite of what
    // everything else needs
    fn cmp(&self, other: &Self) -> Ordering { other.0.cmp(&self.0) }
}

/// SortedIteratorMerger wraps a collection of iterators, each of which returns items in
/// sorted order, and returns the contents of those iterators as a sorted list.
struct SortedIteratorMerger<I : Iterator<Item : Ord>>
{
    map: BinaryHeap<SortedIteratorHeapElement<I>>,
}
impl<I : Iterator<Item : Ord>> SortedIteratorMerger<I> {
    pub fn new<I2 : Iterator<Item = I>>(sources : I2) -> Self {
        SortedIteratorMerger {
            map: sources.filter_map(|mut it| {
                        let item = it.next()?;
                        Some( SortedIteratorHeapElement(item, it) )
                        }).collect(),
        }
    }
}
impl<I : Iterator<Item: Ord>> Iterator for SortedIteratorMerger<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        // pull the next ordered item out of the heap
        let SortedIteratorHeapElement(returned_item, mut next_iter) = self.map.pop()?;
        // advance that iterator. if it returns another item, put it back into the queue
        if let Some(next_item) = next_iter.next() {
            self.map.push( SortedIteratorHeapElement(next_item, next_iter) );
        }
        Some(returned_item)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // first, convert the linked lists into iterators which will return individual nodes, detached from any list
        // then build the merger, which will return ALL of those individual nodes, in order
        let merger = SortedIteratorMerger::new(
            lists.into_iter()
                .map(|list| ListNodeIntoIterator(list) )
        );

        let mut root: Option<Box<ListNode>> = None;
        let mut tail_ptr = &mut root;

        for node in merger {
            *tail_ptr = Some(node);
            tail_ptr = &mut (tail_ptr.as_mut().unwrap().next);
        }

        root
    }
}
