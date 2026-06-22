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

pub struct Tuple {
    l1: Vec<String>,
    l1_expired: bool,
    l2: Vec<String>,
    l2_expired: bool,
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut tuple = Tuple {
            l1: Vec::new(),
            l2: Vec::new(),
            l1_expired: false,
            l2_expired: false,
        };

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
    }
}

fn main() {}
