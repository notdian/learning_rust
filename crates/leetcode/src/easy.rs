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

    assert_eq!(
        middle_node(Some(listhead.clone())),
        Some(listhead.next.unwrap())
    );
}

#[test]
fn evenDigits() {
    pub fn find_numbers(nums: Vec<i32>) -> u32 {
        return nums.iter().map(|num| (num.ilog10()) % 2).sum();
    }

    assert_eq!(
        find_numbers(vec![555, 901, 482, 123, 1111, 112266, 1771, 43210122]),
        4
    );
}
#[test]
fn sortedSquares() {
    use std::collections::VecDeque;

    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut deque = nums.iter().map(|a| a * a).collect::<VecDeque<i32>>();
        let mut output = VecDeque::new();
        for _ in 0..deque.len() {
            let last = deque.back().unwrap_or(&0);
            let front = deque.front().unwrap_or(&0);
            if last > front {
                output.push_front(deque.pop_back().unwrap());
            } else {
                output.push_front(deque.pop_front().unwrap())
            }
        }

        output.into()
    }
    assert_eq!(sorted_squares(vec![-3, -2, 1, 4, 7]), vec![1, 4, 9, 16, 49]);
}
#[test]
fn duplicateZeros() {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let len = arr.len();
        let mut original_length = arr.len();
        let mut zero_count = 0;

        // Count the number of zeros in the original vector
        let mut i = 0;
        while i < original_length {
            if arr[i] == 0 {
                if i == original_length - 1 {
                    arr[len - 1] = 0;
                    zero_count -= 1;
                }
                zero_count += 1;
                original_length -= 1;
            }
            i += 1;
        }

        // If there are no zeros, no modification is required.
        if zero_count == 0 {
            return;
        }

        let mut current_pos = original_length - 1;

        while current_pos > 0 {
            arr[current_pos + zero_count] = arr[current_pos];
            if arr[current_pos] == 0 {
                arr[current_pos + zero_count - 1] = 0;
                zero_count -= 1;
            }

            if current_pos == 0 {
                break;
            }

            current_pos -= 1;
        }
    }
    let mut v = vec![1, 0, 2, 3, 0, 4, 2, 1];
    duplicate_zeros(&mut v);
    assert_eq!(v, vec![1, 0, 0, 2, 3, 0, 0, 4]);

    let mut v = vec![1, 0, 1, 0];
    duplicate_zeros(&mut v);
    assert_eq!(v, vec![1, 0, 0, 1]);
}

#[test]
fn mergeSortedArray() {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }
        if m == 0 {
            std::mem::swap(nums2, nums1);
            return;
        }

        let mut m_current = m as usize;
        let mut n_current = n as usize;
        let mut i = m_current + n_current - 1;
        while n_current > 0 {
            if m_current > 0 && nums1[m_current - 1] > nums2[n_current - 1] {
                nums1[i] = nums1[m_current - 1];
                m_current -= 1;
            } else {
                nums1[i] = nums2[n_current - 1];
                n_current -= 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = 1;

    merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1]);

    let mut nums1 = vec![2, 0];
    let m = 1;
    let mut nums2 = vec![1];
    let n = 1;

    merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1, 2]);
}

#[test]
fn removeElement() {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut len = nums.len();
        let mut i = 0;
        while i < len {
            if nums[i] == val {
                nums[i] = nums[len - 1];
                len -= 1;
                continue;
            }
            i += 1;
        }
        return len as i32;
    }

    let mut elems = vec![3, 2, 2, 3];
    let removed = remove_element(&mut elems, 3) as usize;
    assert_eq!(&elems[..removed], vec![2, 2])
}

#[test]
fn twoSum() {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::<&i32, usize>::new();

        for (i, val) in nums.iter().enumerate() {
            if let Some(pos) = map.get(&(target - val)) {
                return vec![i as i32, *pos as i32];
            }

            map.insert(val, i);
        }

        panic!();
    }
    let mut result = two_sum(vec![2, 7, 11, 15], 9);
    result.sort();
    assert_eq!(result, vec![0, 1]);
}

#[test]
fn isPalindrome() {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } 
        let mut digits = vec![];
        let mut x_copy = x;
        while x_copy > 0 {
            digits.push(x_copy % 10);
            x_copy /= 10;
        }
        let len = digits.len();

        for i in 0..len {
            if i >= len {
                break;
            }
            if digits[i] != digits[len - 1 - i] {
                return false;
            }
        }

        return true;
    }
    assert_eq!(is_palindrome(1000021), false);
    assert_eq!(is_palindrome(-1), false);
    assert_eq!(is_palindrome(11), true);
    assert_eq!(is_palindrome(1), true);
    assert_eq!(is_palindrome(121), true);
}
