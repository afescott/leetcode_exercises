// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

/* l1 =
[9]
l2 =
[1,9,9,9,9,9,9,9,9,9]

output should be [0,0,0,0,0,0,0,0,0,0,1]
*/

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode { val: 0, next: None };
        let mut tail = &mut dummy;

        let mut carry = 0;
        // keep going while there's still something to add
        while l1.is_some() || l2.is_some() || carry > 0 {
            // get this column's digits (0 if that list is finished)
            let a = if let Some(ref node) = l1 { node.val } else { 0 };
            let b = if let Some(ref node) = l2 { node.val } else { 0 };
            let total = a + b + carry;
            // write the ones digit, remember if we carry
            if total >= 10 {
                tail.next = Some(Box::new(ListNode {
                    val: total - 10,
                    next: None,
                }));
                carry = 1;
            } else {
                tail.next = Some(Box::new(ListNode {
                    val: total,
                    next: None,
                }));
                carry = 0;
            }
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
            // move tail to the node we just added
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next

        // effort 1

        /* pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {


            let mut curr = l1.as_ref();
            while let Some(node) = curr {
                tuple.l1.push(node.val.to_string());
                curr = node.next.as_ref();
            }

            let mut curr = l2.as_ref();
            while let Some(ref node2) = curr {
                tuple.l2.push(node2.val.to_string());
                curr = node2.next.as_ref();
            }

            tuple.l1.reverse();
            tuple.l2.reverse();

            let str: String = tuple.l1.concat();

            let value_1 = str.parse::<i32>().unwrap_or(0);

            let str2: String = tuple.l2.concat();

            let value_2 = str2.parse::<i32>().unwrap_or(0);

            let sum = value_1 + value_2;

            println!(
                "value_1 = {}, value_2 = {}, sum = {}, concatted 1 = {}, concatted 2 = {}",
                value_1, value_2, sum, str, str2
            );

            let mut value = None;
            for ele in sum.to_string().chars() {
                let digit = ele.to_digit(10).unwrap() as i32;
                value = Some(Box::new(ListNode {
                    val: digit,
                    next: value,
                }));
                println!("ele: {}", ele);
            }

            value
        } */
    }
}
