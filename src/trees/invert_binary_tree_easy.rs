use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/*  Definition for a binary tree node. */
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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root_val) = root {
        let (left, right) = {
            //Useful function below in future
            /*             std::mem::swap(&mut val.left, &mut val.right); */
            let mut val = root_val.borrow_mut();
            let left = val.left.take();

            let right = val.right.take();
            // swap the links
            val.left = right;
            val.right = left;
            (val.left.clone(), val.right.clone())
        };
        invert_tree(left);
        invert_tree(right);
        return Some(root_val.clone()); // ← here
    }
    None
}
