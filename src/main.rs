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
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let Some(node) = root else { return vec![] };
    let n = node.borrow();
    // leaf
    if n.left.is_none() && n.right.is_none() {
        return vec![n.val.to_string()];
    }
    let mut out = Vec::new();
    for path in binary_tree_paths(n.left.clone()) {
        println!("path: {:?}, val: {:?}", path, n.val);
        out.push(format!("{}->{}", n.val, path));
    }
    for path in binary_tree_paths(n.right.clone()) {
        println!("{:?}", path);
        out.push(format!("{}->{}", n.val, path));
    }
    out
}

fn main() {}
