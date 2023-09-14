#![allow(non_snake_case)]

#[test]
fn dian() {
    fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut running_sums: Vec<i32> = Vec::with_capacity(nums.len());
        running_sums.push(nums[0]);
        for num in 1..nums.len() {
            println!("{}", num);
            running_sums[num] = nums[num] + running_sums[num - 1];
        }

        return running_sums;
    }
    assert_eq!(running_sum(vec![1]), [1])
}

#[test]
fn maximumWealth() {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_wealth = 0;

        for account in accounts.iter() {
            let current_wealth = account.iter().sum();
            if current_wealth > max_wealth {
                max_wealth = current_wealth;
            }
        }
        return max_wealth;
    }

    assert_eq!(6, maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]));
}

#[test]
fn fizzBuzz() {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let index = |n: i32| -> String {
            return match (n % 3 == 0, n % 5 == 0) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                (false, false) => n.to_string(),
            };
        };

        return (1..n + 1)
            .collect::<Vec<i32>>()
            .iter()
            .map(|x| index(*x))
            .collect::<Vec<String>>();
    }

    assert_eq!(vec!["1", "2", "Fizz"], fizz_buzz(3));
}
#[test]

fn middleNode() {
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

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pos_to_value = std::collections::HashMap::<u16, &Box<ListNode>>::new();
        let mut pos = 0;
        let mut current_head = &head;
        while let Some(next) = current_head {
            pos_to_value.insert(pos, next);
            pos += 1;
            current_head = &next.next;
        }
        return pos_to_value.get(&(pos / 2)).map(|p| *p).cloned();
    }

    let mut listhead = Box::new(ListNode::new(1));
    listhead.next = Some(Box::new(ListNode::new(2)));
    listhead.next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    assert_eq!(middle_node(Some(listhead.clone())),Some(listhead.next.unwrap()));
}

#[test]
fn evenDigits() {
    pub fn find_numbers(nums: Vec<i32>) -> u32 {
        return nums.iter().map(|num| (num.ilog10()) % 2).sum();
    }

    assert_eq!(find_numbers(vec![555, 901, 482, 123, 1111, 112266,1771,43210122]), 4);
}
