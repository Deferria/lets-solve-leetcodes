use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    /// Create a new ListNode with the given value.
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    pub fn from_vec(arr: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in arr.iter().rev() {
            let mut temp = ListNode::new(val);
            temp.next = head;
            head = Some(Box::new(temp))
        }
        head
    }
}

impl ListNode {
    pub fn to_vec(&self) -> Vec<i32> {
        let mut vect = Vec::new();
        let mut curr = Some(self);
        while let Some(node) = curr {
            vect.push(node.val);
            curr = self.next.as_deref()
        }
        vect
    }
}

impl ListNode {
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut curr = Some(self);
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_deref();
        }
        len
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr = Some(self);
        let mut first = true;
        while let Some(node) = curr {
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node.val)?;
            curr = node.next.as_deref();
            first = false;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
            right: None
        }
    }
}

impl TreeNode {
    /// Create a binary tree from a vector of Option<i32> values, where None represents a missing node. Return the root of the tree.
    pub fn from_vec(arr: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.is_empty() || arr[0].is_none() {
            return None;
        }
        let rt_val = arr[0].unwrap();
        let rt = Rc::new(RefCell::new(TreeNode::new(rt_val)));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&rt));
        let mut i = 1;
        while !queue.is_empty() && i<arr.len() {
            let curr = queue.pop_front().unwrap();
            if i < arr.len() {
                if let Some(t) = arr[i] {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(t)));
                    curr.borrow_mut().left = Some(Rc::clone(&left_node));
                    queue.push_back(left_node);
                }
                i += 1
            }
            if i < arr.len() {
                if let Some(t) = arr[i] {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(t)));
                    curr.borrow_mut().right = Some(Rc::clone(&right_node));
                    queue.push_back(right_node);
                }
                i += 1
            }
        }
        queue.clear();
        Some(rt)
    }
}
