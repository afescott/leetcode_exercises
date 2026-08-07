mod trees;
use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root_node) = root {
        let mut root_borrow = root_node.borrow_mut();
        let mut left_val = 0;
        let mut right_val = 0;

        println!("root child: {:?}", root_borrow.val);

        let res_left = find_tilt(root_borrow.left);

        let res_right = find_tilt(root_borrow.right);

        let calc = (res_left - res_right).abs();
        println!(
            "Result is difference between, {:?} {:?}",
            left_val, right_val
        );

        return calc;
    }

    0
}

fn main() {}
