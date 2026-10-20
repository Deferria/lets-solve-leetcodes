#![allow(deprecated)]
use crate::collections::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

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

/// This is the solution to leetcode problem 95. Unique Binary Search Trees II. Given an integer n, generate all structurally unique binary search trees that store values 1 ... n.
pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    assert!(n > 0);
    let mut memo = HashMap::new();
    generate_trees_recursive(&mut memo, 1, n)
}

fn generate_trees_recursive(
    record: &mut HashMap<(i32, i32), Vec<Option<Rc<RefCell<TreeNode>>>>>,
    start: i32,
    end: i32,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if let Some(t) = record.get(&(start, end)) {
        return t.to_vec();
    }
    let mut trees = Vec::new();
    if start > end {
        trees.push(None);
        return trees;
    }
    for rval in start..=end {
        let ltrees = generate_trees_recursive(record, start, rval - 1);
        let rtrees = generate_trees_recursive(record, rval + 1, end);
        for ltree in ltrees.iter() {
            for rtree in rtrees.iter() {
                let rt = Rc::new(RefCell::new(TreeNode {
                    val: rval,
                    left: ltree.clone(),
                    right: rtree.clone(),
                }));
                trees.push(Some(rt));
            }
        }
    }
    record.insert((start, end), trees.clone());
    trees
}

/// This is the solution to leetcode problem 96. Unique Binary Search Trees. Given n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.
///
/// In fact, this is exactly the Catalan Number problem. Given n nodes, the number of unique BSTs can be calculated using the recursive formula:
///
/// $ C_n = \\frac{2(2n-1)}{n+1} C_{n-1} $
///
/// where $C_0 = 1$.
pub fn num_trees(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let (mut ans, mut prev) = (1, 1);
    for i in 1..=(n as usize) {
        ans = (4 * i - 2) * prev / (i + 1);
        prev = ans
    }
    ans as i32
}

#[deprecated]
pub fn is_valid_bst_deprecate(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_inner_deprecate(&root)
}

#[deprecated]
fn is_valid_bst_inner_deprecate(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let rt = root.as_ref().unwrap();
    let rt_ref = rt.borrow();
    let val = rt_ref.val;
    let (rl, rr) = (&rt_ref.left, &rt_ref.right);
    //drop(rt_ref);
    let mut ans = true;
    if rl.is_some() {
        let lval = rl.as_ref().unwrap().borrow().val;
        if lval >= val {
            println!("False, since (l){lval}>=(m){val}");
            return false;
        }
        ans = ans && is_valid_bst_inner_deprecate(rl);
    }
    if rr.is_some() {
        let rval = rr.as_ref().unwrap().borrow().val;
        if rval <= val {
            println!("False, since (r){rval}<=(m){val}");
            return false;
        }
        ans = ans && is_valid_bst_inner_deprecate(rr);
    }
    ans
}

/// This is the solution to leetcode problem 98. Validate Binary Search Tree. Given the root of a binary tree, determine if it is a valid binary search tree (BST).
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_inner(&root, i64::MIN, i64::MAX)
}

/// The inner function for validating a binary search tree. It checks if the current node's value is within the valid range defined by `minimum` and `maximum`. If the current node is `None`, it returns `true`. Otherwise, it checks the left and right subtrees recursively, updating the valid range accordingly.
/// 
/// Since the values of the tree nodes are of type `i32`, we use `i64` for `minimum` and `maximum` to avoid potential overflow issues when comparing values.
fn is_valid_bst_inner(root: &Option<Rc<RefCell<TreeNode>>>, minimum: i64, maximum: i64) -> bool {
    if root.as_ref().is_none() { return true; }
    let rt = root.as_ref().unwrap();
    let rt_ref = rt.borrow();
    let val = rt_ref.val as i64;
    let (rl, rr) = (&rt_ref.left, &rt_ref.right);
    if val >= maximum || val <= minimum { return false; }
    return is_valid_bst_inner(rl, minimum, val) && is_valid_bst_inner(rr, val, maximum);
}

/// This is the (inefficient) solution to the Leetcode problem 99. Recover Binary Search Tree. Two elements of a binary search tree (BST) are swapped by mistake. Recover the tree without changing its structure (in-place).
/// 
/// This implementation requires traversing the tree twice, once to construct the inorder value list to find the two swapped values, and once to recover the tree by swapping the two values back. The time complexity is O(n) and the space complexity is O(n) due to storing the inorder traversal values in a vector.
pub fn recover_tree_1(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut v = Vec::new();
    inorder_traversal_recursive(root, &mut v);
    let (mut prev, mut succ) = (0, 0);
    let mut cont_flag = -1;
    for i in 1..v.len() {
        if v[i-1] > v[i] {
            if cont_flag == -1 {
                prev = v[i-1];
                cont_flag = i as isize;
                continue;
            } else {
                succ = v[i];
                cont_flag = -2;
                break;
            }
        }
    }
    if cont_flag != -2 {
        succ = v[cont_flag as usize]
    }
    println!("{prev}, {succ}");
    inorder_traversal_to_recover_recursive(root, prev, succ);
}

fn inorder_traversal_to_recover_recursive(rt: &mut Option<Rc<RefCell<TreeNode>>>, t1: i32, t2: i32) {
    if let Some(t) = rt {
        let mut node_ref = t.borrow_mut();
        inorder_traversal_to_recover_recursive(&mut node_ref.left, t1, t2);
        if node_ref.val == t1 {
            node_ref.val = t2
        } else if node_ref.val == t2 {
            node_ref.val = t1
        }
        inorder_traversal_to_recover_recursive(&mut node_ref.right, t1, t2);
    }
}

/// This is the solution to the Leetcode problem 99. Recover Binary Search Tree. 
/// 
/// This implementation uses a single traversal of the tree to identify the two swapped nodes and then swaps their values to recover the binary search tree. The time complexity is O(n) and the space complexity is O(h), where h is the height of the tree due to the recursion stack.
pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let (mut prev, mut f, mut s) = (None, None, None);
    let rt = root.clone();
    recover_tree_inner(rt, &mut prev, &mut f, &mut s);
    if let (Some(t1), Some(t2)) = (f, s) {
        let temp = t1.borrow().val;
        t1.borrow_mut().val = t2.borrow().val;
        t2.borrow_mut().val = temp;
    }
}

/// This is the recursive inner function for recovering a binary search tree. It performs an inorder traversal of the tree, keeping track of the previous node and identifying the two nodes that are out of order. The first node is identified when the previous node's value is greater than the current node's value, and the second node is identified when this condition occurs again. The function updates the `prev`, `first`, and `second` references accordingly.
/// 
/// Note that the `Rc` library has `.clone()` method, which is just a shallow copy rather than extra memory allocation. Thus, we can use this facility to get rid of the ownership issue.
fn recover_tree_inner(node: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>, first: &mut Option<Rc<RefCell<TreeNode>>>, second: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(n) = node {
        let vl = n.borrow().val;
        let ltree = n.borrow().left.clone();
        recover_tree_inner(ltree, prev, first, second);
        if let Some(p) = prev {
            if p.borrow().val > vl {
                if first.as_ref().is_none() {
                    *first = Some(p.clone());
                }
                *second = Some(n.clone());
            }
        }
        *prev = Some(n.clone());
        let rtree = n.borrow().right.clone();
        recover_tree_inner(rtree, prev, first, second);
    }
}

#[cfg(test)]
mod test_module {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        let arr = [
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
        ];
        let root = TreeNode::root_from_vec(&arr);
        let ans = inorder_traversal(root);
        let expected = vec![2, 7, 4, 8, 1, 5, 3, 6];
        assert_eq!(ans, expected);
    }

    #[test]
    fn notest() {
        let arr = [
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            None,
            Some(7)
        ];
        let root = TreeNode::root_from_vec(&arr);
        let ans = TreeNode::root_to_vec(&root);
        println!("{ans:?}");
    }

    #[test]
    fn test_generate_unique_trees() {
        let v = generate_trees(3);
        for root in v.into_iter() {
            let temp = TreeNode::root_to_vec(&root);
            println!("{:?}", temp);
        }
    }

    #[test]
    fn test_is_valid_bst() {
        let arr = [Some(1), None, Some(1)];
        let root = TreeNode::root_from_vec(&arr);
        assert_eq!(false, is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_2() {
        // [5,4,6,null,null,3,7]
        let arr = [Some(5), Some(4), Some(6), None, None, Some(3), Some(7)];
        let root = TreeNode::root_from_vec(&arr);
        assert_eq!(false, is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_3() {
        let arr = [
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            None,
            Some(7)
        ];
        let root = TreeNode::root_from_vec(&arr);
        //assert_eq!(true, is_valid_bst(root));
        let ans = inorder_traversal(root);
        println!("{ans:?}");
    }

    #[test]
    fn test_recover_tree() {
        let arr = [Some(6), Some(2), Some(4), Some(1), Some(3), None, Some(7), None, None, None, None, None, Some(9)];
        println!("{arr:?}");
        let mut root = TreeNode::root_from_vec(&arr);
        recover_tree(&mut root);
        let after = TreeNode::root_to_vec(&root);
        println!("{after:?}");
    }

    #[test]
    fn test_recover_tree_2() {
        let arr = [Some(7), Some(2), Some(6), Some(1), Some(3), None, Some(4), None, None, None, None, None, Some(9)];
        println!("{arr:?}");
        let mut root = TreeNode::root_from_vec(&arr);
        recover_tree(&mut root);
        let after = TreeNode::root_to_vec(&root);
        println!("{after:?}");
    }
}
