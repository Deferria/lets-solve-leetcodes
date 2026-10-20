use crate::collections::TreeNode;
use std::cell::{RefCell};
use std::rc::Rc;

/// This is the solution to leetcode problem 94. Binary Tree Inorder Traversal. Given the root of a binary tree, return the inorder traversal of its nodes' values.
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = Vec::new();
    inorder_traversal_recursive(&root, &mut ans);
    ans
}

fn inorder_traversal_recursive(rt: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(t) = rt {
        let node_ref = t.borrow();
        inorder_traversal_recursive(&node_ref.left, res);
        res.push(node_ref.val);
        inorder_traversal_recursive(&node_ref.right, res);
    }
}

#[cfg(test)]
mod test_module {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        let arr = [Some(1), Some(2), Some(3), None, Some(4), Some(5), Some(6), Some(7), Some(8)];
        let root = TreeNode::from_vec(&arr);
        let ans = inorder_traversal(root);
        let expected = vec![2, 7, 4, 8, 1, 5, 3, 6];
        assert_eq!(ans, expected);
    }
}