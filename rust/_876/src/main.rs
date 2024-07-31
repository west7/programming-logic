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

fn main(){
}

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