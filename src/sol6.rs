use crate::collections::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: i32::MAX, next: head });
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

#[cfg(test)]
mod test_module {
    use super::*;

    #[test]
    fn test_delete_duplicates () {
        let lstnode = ListNode::from_vec(&[1,1,2,3,3,3,4]);
        let newnode = delete_duplicates(lstnode);
        println!("{}", newnode.unwrap());
    }
}