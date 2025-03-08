use std::collections::HashMap;


pub struct Solution {}

// Definition for singly-linked list.
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

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        let mut acc = 0;
        let mut l1 = l1;
        let mut l2 = l2;
        while l1.is_some() || l2.is_some() {
            let mut now = acc;
            if let Some(node) = l1 {
                now += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                now += node.val;
                l2 = node.next;
            }
            acc = now / 10;
            tail.next = Some(Box::new(ListNode::new(now % 10)));
            tail = tail.next.as_mut().unwrap();
        }
        if acc > 0 {
            tail.next = Some(Box::new(ListNode::new(acc)));
            tail = tail.next.as_mut().unwrap();
        }
        head.next
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let res = Solution::add_two_numbers(Vec::from([2, 7, 4, 3]), 9);
        // println!("{:?}", res);
    }
}
