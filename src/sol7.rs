#![allow(deprecated, unused_variables)]
use crate::collections::{ TreeNode, ListNode };
use std::cell::RefCell;
use std::rc::Rc;

use std::collections::{ HashMap, VecDeque };

/// This is the solution to leetcode problem 94. Binary Tree Inorder Traversal. Given the root of a binary tree, return the inorder traversal of its nodes' values.
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = Vec::new();
    inorder_traversal_recursive(&root, &mut ans);
    ans
}

pub fn inorder_traversal_recursive(rt: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(t) = rt {
        let node_ref = t.borrow();
        inorder_traversal_recursive(&node_ref.left, res);
        res.push(node_ref.val);
        inorder_traversal_recursive(&node_ref.right, res);
    }
}

pub fn preorder_traversal_recursive(rt: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(t) = rt {
        let node_ref = t.borrow();
        res.push(node_ref.val);
        preorder_traversal_recursive(&node_ref.left, res);
        preorder_traversal_recursive(&node_ref.right, res);
    }
}

pub fn postorder_traversal_recursive(rt: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(t) = rt {
        let node_ref = t.borrow();
        postorder_traversal_recursive(&node_ref.left, res);
        postorder_traversal_recursive(&node_ref.right, res);
        res.push(node_ref.val);
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

/// This is the solution to the Leetcode problem 100. Same Tree. 
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_same_tree_inner(&p, &q)
}

/// The inner function for checking if two binary trees are the same. It compares the values of the current nodes and recursively checks the left and right subtrees. 
fn is_same_tree_inner(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }
    if let (Some(pnode), Some(qnode)) = (p, q) {
        if pnode.borrow().val != qnode.borrow().val {
            return false;
        }
        return is_same_tree_inner(&pnode.borrow().left, &qnode.borrow().left) && is_same_tree_inner(&pnode.borrow().right, &qnode.borrow().right);
    }
    return false;
}

/// This is the solution to the Leetcode problem 101. Symmetric Tree.
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.as_ref().is_none() { return true; }
    let rt = root.unwrap();
    let (ltree, rtree) = (&rt.borrow().left, &rt.borrow().right);
    return is_symmetric_inner(ltree, rtree);
}

/// The inner function for checking if two binary trees are mirror images of each other. It compares the values of the current nodes and recursively checks the left subtree of one tree against the right subtree of the other tree, and vice versa.
fn is_symmetric_inner(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }
    if let (Some(pnode), Some(qnode)) = (p, q) {
        if pnode.borrow().val != qnode.borrow().val {
            return false;
        }
        return is_same_tree_inner(&pnode.borrow().left, &qnode.borrow().right) && is_same_tree_inner(&pnode.borrow().right, &qnode.borrow().left);
    }
    return false;
}

/// This is the solution to the Leetcode problem 102. Binary Tree Level Order Traversal. Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut answer = Vec::new();
    if root.is_none() { return answer; }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap().clone());
    let mut n_prev = 1;
    let mut n = 0;
    let mut n_next = 0;
    let mut temp = Vec::new();
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        n += 1;
        
        let node_ref = node.borrow();
        temp.push(node_ref.val);
        if node_ref.left.as_ref().is_some() {
            queue.push_back(node_ref.left.clone().unwrap());
            n_next += 1;
        }
        if node_ref.right.as_ref().is_some() {
            queue.push_back(node_ref.right.clone().unwrap());
            n_next += 1;
        }
        if n == n_prev {
            answer.push(temp.clone());
            temp.clear();
            n_prev = n_next;
            n_next = 0;
            n = 0;
        }
    }
    assert!(n_next == 0);
    answer
}

/// This is the solution to the Leetcode problem 103. Binary Tree Zigzag Level Order Traversal. Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut answer = Vec::new();
    if root.is_none() { return answer; }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap().clone());
    let mut is_reverse = false;
    while !queue.is_empty() {
        let mut temp = Vec::new();
        let n = queue.len();
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let node_ref = node.borrow();
            temp.push(node_ref.val);
            if node_ref.left.as_ref().is_some() {
                queue.push_back(node_ref.left.clone().unwrap());
            }
            if node_ref.right.as_ref().is_some() {
                queue.push_back(node_ref.right.clone().unwrap());
            }
        }
        if is_reverse {
            temp.reverse();
        }
        answer.push(temp.to_vec());
        is_reverse = !is_reverse;
    }
    answer
}

/// This is the solution to the Leetcode problem 104. Maximum Depth of Binary Tree. Given the root of a binary tree, return its maximum depth.
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_depth_inorder_traversal_recursive(&root, 0, 0)
}

pub fn max_depth_inorder_traversal_recursive(rt: &Option<Rc<RefCell<TreeNode>>>, max_curr: i32, curr: i32) -> i32 {
    if let Some(t) = rt {
        let node_ref = t.borrow();
        let n1 = max_depth_inorder_traversal_recursive(&node_ref.left, max_curr, curr+1);
        let n2 = max_depth_inorder_traversal_recursive(&node_ref.right, max_curr, curr+1);
        return n1.max(n2);
    } else {
        return max_curr.max(curr);
    }
}

/// This is the solution to the Leetcode problem 105. Construct Binary Tree from Preorder and Inorder Traversal. Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut memo = HashMap::new();
    for (i, it) in inorder.into_iter().enumerate() {
        memo.insert(it, i as isize);
    }
    let mut preorder_iter = preorder.iter();
    build_tree_recursive(&mut preorder_iter, &memo, 0, preorder.len() as isize -1)
}

fn build_tree_recursive(preorder: &mut std::slice::Iter<i32>, memo: &HashMap<i32, isize>, start: isize, end: isize) -> Option<Rc<RefCell<TreeNode>>> {
    if start <= end {
        if let Some(&val) = preorder.next() {
            if let Some(&i) = memo.get(&val) {
                let root = Rc::new(RefCell::new(TreeNode {
                    val: val,
                    left: build_tree_recursive(preorder, memo, start, i-1),
                    right: build_tree_recursive(preorder, memo, i+1, end)
                }));
                return Some(root);
            }
        }
    }
    None
}

/// This is the solution to the Leetcode problem 106. Construct Binary Tree from Inorder and Postorder Traversal. 
pub fn build_tree_postorder(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut memo = HashMap::new();
    for (i, it) in inorder.into_iter().enumerate() {
        memo.insert(it, i as isize);
    }
    let mut postorder_iter = postorder.iter().rev();
    build_tree_recursive_postorder(&mut postorder_iter, &memo, 0, postorder.len() as isize -1)
}

fn build_tree_recursive_postorder(postorder: &mut std::iter::Rev<std::slice::Iter<i32>>, memo: &HashMap<i32, isize>, start: isize, end: isize) -> Option<Rc<RefCell<TreeNode>>> {
    if start <= end {
        if let Some(&val) = postorder.next() {
            if let Some(&i) = memo.get(&val) {
                let root = Rc::new(RefCell::new(TreeNode {
                    val: val,
                    right: build_tree_recursive_postorder(postorder, memo, i+1, end),
                    left: build_tree_recursive_postorder(postorder, memo, start, i-1)
                }));
                return Some(root);
            }
        }
    }
    None
}

/// This is the solution to the Leetcode problem 107. Binary Tree Level Order Traversal II. Given the root of a binary tree, return the bottom-up level order traversal of its nodes' values. (i.e., from left to right, level by level from leaf to root).
/// 
/// This is a light-hearted implementation, for it reuses the `level_order` function and simply reverses the result. 
pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut answer = Vec::new();
    if root.is_none() { return answer; }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap().clone());
    while !queue.is_empty() {
        let n = queue.len();
        let mut temp = Vec::new();
        
        for _ in 0..n {
            let node = queue.pop_front().unwrap();
            let node_ref = node.borrow();
            temp.push(node_ref.val);
            if node_ref.left.as_ref().is_some() {
                queue.push_back(node_ref.left.clone().unwrap());
            }
            if node_ref.right.as_ref().is_some() {
                queue.push_back(node_ref.right.clone().unwrap());
            }
        }
        
        answer.push(temp.clone());
    }
    answer.reverse();
    answer
}

/// This is the solution to the Leetcode problem 108. Convert Sorted Array to Binary Search Tree. Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    sorted_to_bst_inner(&nums, 0, nums.len() as isize - 1)
}

fn sorted_to_bst_inner(nums: &Vec<i32>, start: isize, end: isize) -> Option<Rc<RefCell<TreeNode>>> {
    if start > end {
        return None;
    }
    let mid = (start + end) / 2;
    let new_root = Rc::new(RefCell::new(TreeNode {
        val: nums[mid as usize], 
        left: sorted_to_bst_inner(nums, start, mid-1),
        right: sorted_to_bst_inner(nums, mid+1, end)
    }));
    Some(new_root)
}

/// This is the solution to the Leetcode problem 109. Convert Sorted List to Binary Search Tree. Given the head of a singly linked list where elements are sorted in ascending order, convert it to a height-balanced binary search tree.
/// 
/// We choose to convert the linked list ot an array first and then reuse the `sorted_array_to_bst` function to construct the binary search tree. 
pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut v = Vec::new();
    linked_list_to_vec(&head, &mut v);
    let n = v.len() as isize;
    sorted_to_bst_inner(&v, 0, n-1)
}

fn linked_list_to_vec(node: &Option<Box<ListNode>>, res: &mut Vec<i32>) {
    if let Some(n) = node {
        res.push(n.val);
        linked_list_to_vec(&n.next, res);
    } else {
        return;
    }
}

/// This is the solution to the Leetcode problem 110. Balanced Binary Tree. Given a binary tree, determine if it is height-balanced.
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    return is_balanced_inner(&root) != -1;
}

fn is_balanced_inner(rt: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = rt {
        let n1 = is_balanced_inner(&node.borrow().left);
        let n2 = is_balanced_inner(&node.borrow().right);
        if n1 == -1 || n2 == -1 {
            return -1;
        } else if n1.abs_diff(n2) > 1 {
            return -1;
        } else {
            return 1 + n1.max(n2);
        }
    } else {
        return 0;
    }
}

/// This is the solution to the Leetcode problem 111. Minimum Depth of Binary Tree. Given a binary tree, find its minimum depth.
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    get_tree_depth_min(&root)
}

fn get_tree_depth_min(rt: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = rt {
        let node_ref = node.borrow();
        let n1 = get_tree_depth_min(&node_ref.left);
        let n2 = get_tree_depth_min(&node_ref.right);
        if n1 == 0 && n2 == 0 {
            return 1;
        } else if n1 != 0 && n2 != 0 {
            return 1+n1.min(n2);
        } else if n1 !=0 && n2 == 0 {
            return 1+n1;
        } else {
            return 1+n2;
        }
    } else {
        return 0;
    }
}

/// This is the solution to the Leetcode problem 112. Path Sum. Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals `target_sum`.
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    has_path_sum_recursive(&root, target_sum)
}

fn has_path_sum_recursive(curr: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
    if let Some(node) = curr {
        let curr_ref = node.borrow();
        let target_new = target - curr_ref.val;
        if curr_ref.left.is_none() && curr_ref.right.is_none() {
            return target_new == 0;
        } else {
            return has_path_sum_recursive(&curr_ref.left, target_new) || has_path_sum_recursive(&curr_ref.right, target_new);
        }
    } else {
        return false;
    }
}

/// This is the solution to the Leetcode problem 113. Path Sum II. Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths where each path's sum equals `target_sum`.
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let mut temp = Vec::new();
    path_sum_recursive(&root, target_sum, &mut ans, & mut temp);
    ans
}

fn path_sum_recursive(rt: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, final_ans: &mut Vec<Vec<i32>>, curr_ans: &mut Vec<i32>) {
    if let Some(node) = rt {
        let curr = node.borrow();
        curr_ans.push(curr.val);
        let target_new = target_sum - curr.val;
        match (curr.left.as_ref(), curr.right.as_ref()) {
            (None, None) => {
                if target_new == 0 {
                    final_ans.push(curr_ans.to_vec());
                }
                curr_ans.pop();
                return;
            },
            (ltree, rtree) => {
                path_sum_recursive(&curr.left, target_new, final_ans, curr_ans);
                path_sum_recursive(&curr.right, target_new, final_ans, curr_ans);
                curr_ans.pop();
            }
        }
    } else {
        return;
    }
}
/// This is the solution to the Leetcode problem 114. Flatten Binary Tree to Linked List. Given the root of a binary tree, flatten the tree into a "linked list" (left child is none, right child is the next node in the list) in-place.
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let mut node_ref = node.borrow_mut();
        flatten(&mut node_ref.left);
        flatten(&mut node_ref.right);
        drop(node_ref);
        let mut prev = node.borrow_mut().left.clone();
        while let Some(inner) = prev {
            if inner.borrow().right.is_none() {
                inner.borrow_mut().right = node.borrow().right.clone();
                let mut node_ref = node.borrow_mut();
                node_ref.right = node_ref.left.clone();
                node_ref.left = None;
                return;
            }
            prev = inner.borrow_mut().right.clone();
        }
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

    #[test]
    fn test_is_same_tree() {
        let arr1 = [Some(7), Some(2), Some(6), Some(1), Some(3), None, Some(4), None, None, None, None, None, Some(9)];
        let arr2 = [Some(7), Some(2), Some(6), Some(1), Some(3), None, Some(4), None, None, None, None, None, Some(9)];
        let (r1, r2) = (TreeNode::root_from_vec(&arr1), TreeNode::root_from_vec(&arr2));
        assert_eq!(true, is_same_tree(r1, r2));
    }

    #[test]
    fn test_is_same_tree_2() {
        let arr1 = [None];
        let arr2 = [Some(7), Some(2), Some(6), Some(1), Some(3), None, Some(4), None, None, None, None, None, Some(9)];
        let (r1, r2) = (TreeNode::root_from_vec(&arr1), TreeNode::root_from_vec(&arr2));
        assert_eq!(false, is_same_tree(r1, r2));
    }

    #[test]
    fn test_is_same_tree_3() {
        let arr1 = [Some(7), Some(1), Some(2)];
        let arr2 = [Some(7), Some(2), Some(1)];
        let (r1, r2) = (TreeNode::root_from_vec(&arr1), TreeNode::root_from_vec(&arr2));
        assert_eq!(false, is_same_tree(r1, r2));
    }

    #[test]
    fn test_is_symmetric_tree() {
        let arr = [Some(0), Some(7), Some(7), Some(2), Some(1), Some(1), Some(2)];
        let rt = TreeNode::root_from_vec(&arr);
        assert_eq!(true, is_symmetric(rt));
    }

    #[test]
    fn test_level_order_traversal() {
        let arr = [Some(0), Some(7), Some(7), Some(2), Some(1), Some(1), None, Some(7), None, Some(4)];
        let rt = TreeNode::root_from_vec(&arr);
        let v = level_order(rt);
        println!("{:?}", v);
    }

    #[test]
    fn test_level_order_traversal_2() {
        let arr = [Some(0), Some(1), None, Some(2)];
        let rt = TreeNode::root_from_vec(&arr);
        let v = level_order(rt);
        println!("{:?}", v);
    }

    #[test]
    fn test_zigzag_level_order_traversal() {
        let arr = [Some(0), Some(7), Some(7), Some(2), Some(1), Some(1), None, Some(7), None, Some(4)];
        let rt = TreeNode::root_from_vec(&arr);
        let v = zigzag_level_order(rt);
        println!("{:?}", v);
    }

    #[test]
    fn test_max_depth() {
        let arr = [Some(0), Some(7), Some(7), Some(2), Some(1), Some(1), None, Some(7), None, Some(4)];
        let rt = TreeNode::root_from_vec(&arr);
        max_depth(rt);
    }

    #[test]
    fn test_max_depth_2() {
        let arr = [None];
        let rt = TreeNode::root_from_vec(&arr);
        assert_eq!(max_depth(rt), 0);
    }

    #[test]
    fn test_build_tree() {
        let p = vec![7, 2, 1, 0, 3, 6, 5, 9, 4];
        let i = vec![1, 0, 2, 3, 7, 5, 9, 6, 4];
        let root = build_tree(p, i);
        let v = TreeNode::root_to_vec(&root);
        assert_eq!(v, vec![Some(7), Some(2), Some(6), Some(1), Some(3), Some(5), Some(4), None, Some(0), None, None, None, Some(9)])
    }

    #[test]
    fn test_build_tree_postorder() {
        let p = vec![0, 1, 3, 2, 9, 5, 4, 6, 7];
        let i = vec![1, 0, 2, 3, 7, 5, 9, 6, 4];
        let root = build_tree_postorder(i, p);
        let v = TreeNode::root_to_vec(&root);
        assert_eq!(v, vec![Some(7), Some(2), Some(6), Some(1), Some(3), Some(5), Some(4), None, Some(0), None, None, None, Some(9)])
    }

    #[test]
    fn test_sorted_array_to_bst() {
        let v = vec![-10,-3,0,5,9];
        let root = sorted_array_to_bst(v);
        let displayed = TreeNode::root_to_vec(&root);
        println!("{displayed:?}");
    }

    #[test]
    fn test_sorted_list_to_bst() {
        let v = ListNode::from_vec(&[-10,-3,0,5,9]);
        let root = sorted_list_to_bst(v);
        let displayed = TreeNode::root_to_vec(&root);
        println!("{displayed:?}");
    }

    #[test]
    fn test_get_tree_height_min() {
        let arr = [Some(3),Some(9),Some(20),None,None,Some(15),Some(7)];
        let root = TreeNode::root_from_vec(&arr);
        assert_eq!(min_depth(root), 2);
    }

    #[test]
    fn test_get_tree_height_min_2() {
        let arr = [Some(3),None,Some(9),None,Some(20),None,Some(15),Some(7)];
        let root = TreeNode::root_from_vec(&arr);
        assert_eq!(min_depth(root), 5);
    }

    #[test]
    fn test_path_sum() {
        let arr = [Some(1), Some(3), Some(2), Some(4), Some(5), Some(6), Some(7)];
        let root = TreeNode::root_from_vec(&arr);
        let ans = path_sum(root, 9);
        println!("{ans:?}");
    }

    #[test]
    fn test_flatten() {
        let arr = [Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)];
        let mut root = TreeNode::root_from_vec(&arr);
        flatten(&mut root);
        /* let mut v1 = Vec::new();
        preorder_traversal_recursive(&root, &mut v1);
        dbg!(v1); */
        let view = TreeNode::root_to_vec(&root);
        println!("{view:?}");
    }
}
