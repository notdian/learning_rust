#![allow(non_snake_case)]


#[test]
fn addTwoNumbers() {
    // Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let output_pos = &mut &mut head.as_mut().unwrap().next;
        let mut current_pos = &l1;
        let mut current_pos2 = &l2;
        while let Some(v1) = current_pos {
            let mut sum = v1.val + carry;

            if let Some(v) = current_pos2 {
                sum += v.val;
            }
            let mut value = sum;
            if sum > 9 {
                *(&mut carry) = 1;
                value = sum - 10;
            } else {
                carry = 0
            }
            let node = Box::new(ListNode::new(value));

            **output_pos = Some(node);

            *output_pos = &mut output_pos.as_mut().unwrap().next;

            current_pos = &current_pos.as_ref().unwrap().next;

            if current_pos2.is_some() {
                current_pos2 = &current_pos2.as_ref().unwrap().next;
            } else {
                current_pos2 = &None;
            }

            if current_pos.is_none() {
                current_pos = current_pos2;
                current_pos2 = &None;
            }
        }
        if carry > 0 {
            **output_pos = Some(Box::new(ListNode::new(1)));
        }
        drop(output_pos);
        return head.unwrap().next;
    }
    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode { val: 1, next: None })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        ),
        Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 2, next: None })) }))
    );
}


#[test]
fn kthGrammer() {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut row = vec![0];
        for i in 1..n + 1 {
            let mut new_vec = vec![];
            let mut col = 0;
            for r in row.iter() {
                if r == &0 {
                    new_vec.push(0);
                    new_vec.push(1);
                } else {
                    new_vec.push(1);
                    new_vec.push(0);
                }
                col += 2;
                if (col * u32::pow(2, (n - i) as u32)) >= k as u32 {
                    println!("{}, {} breaking", i, col);
                    break;
                }
            }

            row = new_vec;
        }

        return row[(k - 1) as usize];
    }
    assert_eq!(kth_grammar(30, 434991989), 0);
}
