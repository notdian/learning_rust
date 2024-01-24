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

        let mut head: Box<ListNode> = Box::new(ListNode::new(0));
        let mut output_pos = &mut head;

        let mut current_pos = l1.as_ref();
        let mut current_pos2 = l2.as_ref();

        while let Some(v1) = current_pos {
            current_pos = v1.next.as_ref();

            let mut sum = v1.val + carry;

            if let Some(v) = current_pos2 {
                sum += v.val;
                current_pos2 = v.next.as_ref();
            }

            if sum > 9 {
                carry = 1;
                sum = sum - 10;
            } else {
                carry = 0
            }

            output_pos.next = Some(Box::new(ListNode::new(sum)));

            output_pos = output_pos.next.as_mut().unwrap();

            if current_pos.is_none() {
                current_pos = current_pos2.take();
            }
        }
        if carry > 0 {
            output_pos.next = Some(Box::new(ListNode::new(1)));
        }

        return head.next;
    }
    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode { val: 1, next: None })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))
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

#[test]
pub fn groupAnagrams() {
    const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(
                std::collections::HashMap::<[u8; N_LETTERS], Vec<String>>::new(),
                |mut map, s| {
                    let freqs = s.bytes().map(|b| (b - b'a') as usize).fold(
                        [0; N_LETTERS],
                        |mut freqs, bin| {
                            freqs[bin] += 1;
                            freqs
                        },
                    );
                    map.entry(freqs).or_default().push(s);
                    map
                },
            )
            .into_values()
            .collect()
    }
    assert_eq!(
        group_anagrams(vec![
            "eat".to_string(),
            "ate".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ]),
        vec![
            vec!["tan".to_string(), "nat".to_string()],
            vec!["eat".to_string(), "ate".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
        ]
    )
}

#[test]
fn simplifyPath() {
    pub fn simplify_path(path: String) -> String {
        let mut output: Vec<&str> = vec![""];
        for dir in path.split("/") {
            if dir == ".." {
                if output.len() > 1 {
                    output.pop();
                }
                continue;
            }
            if dir == "." || dir == "" {
                continue;
            }
            output.push(dir);
        }

        if output.len() == 1 {
            return "/".to_string();
        }
        output.join("/")
    }
    assert_eq!(simplify_path("/home/dian".to_string()), "/home/dian".to_string());
    assert_eq!(simplify_path("/home/dian/./../shkurte".to_string()), "/home/shkurte".to_string());
    assert_eq!(simplify_path("/home/../dian".to_string()), "/dian".to_string());
    assert_eq!(simplify_path("/home//dian".to_string()), "/home/dian".to_string());
    assert_eq!(simplify_path("/../home//dian".to_string()), "/home/dian".to_string());
    assert_eq!(simplify_path("//".to_string()), "/".to_string());
    assert_eq!(simplify_path("/..".to_string()), "/".to_string());
    assert_eq!(simplify_path("/.".to_string()), "/".to_string());
}
