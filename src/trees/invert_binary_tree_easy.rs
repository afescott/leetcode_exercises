use std::cell::RefCell;
use std::collections::VecDeque;
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
//D- GRADE recursive better
struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let root_borrow = root.borrow();
            let mut rc = TreeNode::new(root.borrow().val);

            root_borrow.left.iter().for_each(|tree_node| {
                let mut temp = tree_node.borrow_mut();
                let val = temp.left.clone();
                temp.left = temp.right.clone();
                temp.right = val;
                drop(temp);
                rc.right.insert(tree_node.clone());
            });

            root_borrow.right.iter().for_each(|tree_node| {
                let mut temp = tree_node.borrow_mut();
                let val = temp.right.clone();
                temp.right = temp.left.clone();
                temp.left = val;
                drop(temp);
                rc.left.insert(tree_node.clone());
            });

            return Some(Rc::new(rc.into()));
        }

        None
    }
}
