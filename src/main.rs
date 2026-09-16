//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;

        let mut curr = head.take();

        while curr.is_some() {
            let mut curr = curr.take().unwrap();
            let next_node = curr.next.take();
            curr.next = prev.take();
            prev = Some(curr);
            curr = next_node.unwrap();
        }

        prev
    }
}

fn main() {}
