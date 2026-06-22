// Definition for singly-linked list.
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

impl Iterator for ListNode {
    type Item = i32;

    /* fn next(&mut self) -> Option<Self::Item> {
        Some(self.next)
    } */
}

pub struct Tuple {
    l1: Vec<i32>,
    l1_expired: bool,
    l2: Vec<i32>,
    l2_expired: bool,
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        /*         println!("l1: {:?}, l2 :{:?}", l1, l2); */
        //these come in reverse order
        //
        //
        /*         let mut vec : (Vec<i32>, Vec<i32>) = Vec::new(); */
        let mut val = true;

        let mut tuple = Tuple {
            l1: Vec::new(),
            l2: Vec::new(),
            l1_expired: false,
            l2_expired: false,
        };

        while val == true {
            if let Some(ref node) = l1 {
                tuple.l1.push(node.val);
                let mut total = 0;
                //maybe you don't need this. i think we use this line for l2
                /* let next = node
                .next
                .unwrap_or(Box::new(ListNode { val: 0, next: None })); */

                //add the vertical numbers of the node together
            } else {
                tuple.l1_expired = true;
            }

            if let Some(ref node2) = l2 {
                tuple.l2.push(node2.val);
            } else {
                tuple.l2_expired = true;
            }

            if tuple.l1_expired == true && tuple.l2_expired == true {
            } else {
                val = false;
            }
        }
        tuple.l1.reverse();
        tuple.l2.reverse();


        Some(Box::new(ListNode { val: 0, next: None }))
    }
}

fn recursive_add (Option<i32>, Option<i32>, i32) -> (Option<i32>, i32) {
    //base case
    if l1 == None && l2 == None {
        return (None, carry);
    }

    let mut sum = carry;
    if let Some(val) = l1 {
        sum += val;
    }
    if let Some(val) = l2 {
        sum += val;
    }




    (Some(Box::new(new_node)), final_carry)
}

fn main() {}
