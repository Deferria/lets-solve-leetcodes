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
