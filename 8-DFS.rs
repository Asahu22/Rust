//Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn depth(&self) -> i32 {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Some(left), Some(right)) => {
                std::cmp::max(left.borrow().depth(), right.borrow().depth()) + 1
            }
            (Some(left), None) => left.borrow().depth() + 1,
            (None, Some(right)) => right.borrow().depth() + 1,
            (None, None) => 1,
        }
    }
}

fn main() {

    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    println!("Maximum Depth of the Binary Tree: {}", root.borrow().depth()); 
}
