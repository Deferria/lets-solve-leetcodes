#![allow(unused_imports)]
use leetcodes::sol7::{ inorder_traversal_recursive, postorder_traversal_recursive, preorder_traversal_recursive };
use std::rc::Rc;
use leetcodes::collections::TreeNode;

/// Only some important debug would use the main function for better print and lint. Otherwise, run tests in the #[cfg(test)] mod tests in `sol.rs`.
fn main() {
    let arr = [Some(7), Some(2), Some(6), Some(1), Some(3), Some(5), Some(4), None, Some(0), None, None, None, Some(9)];
    let root = TreeNode::root_from_vec(&arr);
    {
        let mut v1 = Vec::new();
        preorder_traversal_recursive(&root, &mut v1);
        println!("preorder: {v1:?}");
    }
    {
        let mut v2 = Vec::new();
        inorder_traversal_recursive(&root, &mut v2);
        println!("inorder: {v2:?}");
    }
    {
        let mut v3 = Vec::new();
        postorder_traversal_recursive(&root, &mut v3);
        println!("postorder: {v3:?}");
    }
}
