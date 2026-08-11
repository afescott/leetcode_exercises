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
    let (_, total_tilt) = Self::sum_tilt(root);
    total_tilt
}

fn sum_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if let Some(root_node) = root {
        let mut tilt_here = 0;
        let mut sum_here = 0;

        let mut tilt_total = 0;
        let node = root_node.borrow();

        let (left_sum, left_tilt) = Self::sum_tilt(node.left.clone());

        let (right_sum, right_tilt) = Self::sum_tilt(node.right.clone());

        tilt_here = (left_sum - right_sum).abs();
        sum_here = left_sum + right_sum + node.val;
        tilt_total = left_tilt + right_tilt + tilt_here;
        return (sum_here, tilt_total);
    }

    (0, 0)
}
