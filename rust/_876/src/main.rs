// 876. Middle of the Linked List
// link: https://leetcode.com/problems/middle-of-the-linked-list/?envType=daily-question&envId=2024-07-21

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn _new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
struct Solution;

impl Solution {
    fn _middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &head;
        let mut count = 0;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }

        current = &head;
        let mid = count / 2;

        for _ in 0..mid {
            if let Some(node) = current {
                current = &node.next;
            }
        }
        
        current.clone()
    }
}
fn main(){
    let mut  head = Some(Box::new(ListNode::_new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::_new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::_new(3)));
    head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::_new(4)));
    head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::_new(5)));

    println!("{:?}", Solution::_middle_node(head));
}
