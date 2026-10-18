use crate::collections::ListNode;

/// This is the solution to leetcode problem 83. Remove Duplicates from Sorted List.
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: i32::MAX,
        next: head,
    });
    let mut tl = &mut dummy;
    let mut curr = i32::MAX;
    while let Some(node) = tl.next.take() {
        if curr != node.val {
            curr = node.val;
            tl = tl.next.insert(node);
        } else {
            tl.next = node.next;
        }
    }
    dummy.next
}

/// This is the solution to leetcode problem 92. Reverse Linked List II.
pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    if left == right {
        return head;
    }
    let mut dummy = Box::new(ListNode { val: i32::MAX, next: head });
    let mut tl = &mut dummy;
    let mut inserted = None;
    let mut ind = 0;

    while let Some(node) = tl.next.take() {
        ind += 1;
        if ind < left {
            tl = tl.next.insert(node);
        } else if ind >= left && ind <= right {
            let vl = node.val;
            let new_node = ListNode { val: vl, next: inserted };
            inserted = Some(Box::new(new_node));
            tl.next = node.next;
        } else if ind == right + 1 {
            tl.next = Some(node);
            break;
        }
    }
    let mut carrier = Box::new(ListNode { val: i32::MAX, next: inserted.take() });
    let mut tail = &mut carrier;
    while tail.next.as_ref().is_some() {
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = tl.next.take();
    tl.next = carrier.next.take();
    dummy.next
}

#[cfg(test)]
mod test_module {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        let lstnode = ListNode::from_vec(&[1, 1, 2, 3, 3, 3, 4]);
        let newnode = delete_duplicates(lstnode);
        println!("{}", newnode.unwrap());
    }

    #[test]
    fn test_reverse_between() {
        let lstnode = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let newnode = reverse_between(lstnode, 2, 5);
        println!("{}", newnode.unwrap());
    }

    #[test]
    fn test_reverse_between_2() {
        let lstnode = ListNode::from_vec(&[5]);
        let newnode = reverse_between(lstnode, 1, 1);
        println!("{}", newnode.unwrap());
    }
}
