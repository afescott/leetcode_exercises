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

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;

        // eg 1,2,3,4,5
        while let Some(mut node) = curr {
            // node is 1

            // curr becomes 2, so the loop can keep walking forward
            curr = node.next.take();
            // 1 now points back at whatever we already reversed
            node.next = prev.take();
            // 1 is the new head of the reversed part
            prev = Some(node);
        }

        prev
    }
}

fn main() {}
