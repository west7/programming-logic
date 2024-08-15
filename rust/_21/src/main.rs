// 21. Merge Sorted List
// link: https://leetcode.com/problems/merge-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150

#![allow(unused)]

use std::env::current_exe;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l = ListNode::new(0);
        let mut current = &mut l;

        let mut l1 = &list1;
        let mut l2 = &list2;

        while l1.is_some() && l2.is_some() {
            let mut l1_node =  l1.as_ref().unwrap();
            let mut l2_node = l2.as_ref().unwrap();
            
            if l1_node.val < l2_node.val {
                current.next = Some(Box::new(ListNode::new(l1_node.val)));
                l1 = &l1_node.next;    
            } else {
                current.next = Some(Box::new(ListNode::new(l2_node.val)));
                l2 = &l2_node.next;
            }
            current = current.next.as_mut().unwrap();
        }

        while let Some(l1_node) = l1 {
            current.next = Some(Box::new(ListNode::new(l1_node.val)));
            current = current.next.as_mut().unwrap();
            l1 = &l1_node.next;
        }

        while let Some(l2_node) = l2 {
            current.next = Some(Box::new(ListNode::new(l2_node.val)));
            current = current.next.as_mut().unwrap();
            l2 = &l2_node.next;
        }
        l.next
    }
}

fn main () {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })), 
    }));

    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })), 
    }));

    let res = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: None,
                        })),
                    })),
                })),
            })),
        })), 
    }));

    let l3 = Solution::merge_two_lists(l1, l2);
    assert_eq!(res, l3);
    //println!("res: {:#?}, l3: {:#?}", res, l3);
}