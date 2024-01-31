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
fn removeDuplicatesFromSortedArray() {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut current_pos = 0;

        let mut i = 1;
        while i < nums.len() {
            if nums[i] != nums[current_pos] {
                current_pos += 1;
                nums.swap(i, current_pos);
            }

            i += 1;
        }

        return (current_pos + 1) as i32;
    }

    let mut arr = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let unique_elems = remove_duplicates(&mut arr) as usize;
    assert_eq!(arr[..unique_elems], [0, 1, 2, 3, 4]);
}

#[test]
fn isPalindrome() {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut reversed = 0;
        let mut temp = x;

        while temp > 0 {
            reversed = reversed * 10 + temp % 10;
            temp /= 10;
        }

        return reversed == x;
    }
    assert_eq!(is_palindrome(1000021), false);
    assert_eq!(is_palindrome(-1), false);
    assert_eq!(is_palindrome(11), true);
    assert_eq!(is_palindrome(1), true);
    assert_eq!(is_palindrome(121), true);
}
#[test]
fn checkIfNAndItsDoubleExist() {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(arr.len());

        let mut zeros = 0;
        arr.iter().for_each(|v| {
            if *v == 0 {
                zeros += 1;
                return;
            }
            set.insert(v);
        });

        if zeros > 1 {
            return true;
        }

        return arr.iter().find(|p| set.contains(&(*p * 2))).is_some();
    }
    let now = std::time::Instant::now();

    assert_eq!(check_if_exist(vec![10, 2, 5, 3]), true);
    assert_eq!(check_if_exist(vec![3, 1, 7, 11]), false);
    assert_eq!(check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]), false);
    assert_eq!(check_if_exist(vec![0, 0]), true);
    assert_eq!(check_if_exist(vec![0, 0, 0, 0, 0, 0]), true);
    println!("Time elapsed: {:?}", now.elapsed());
}

#[test]
fn validMountainArray() {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut pos = arr[0];
        let mut dir = 0;
        for &el in arr.iter().skip(1) {
            if el == pos {
                return false;
            }
            if el > pos {
                if dir == -1 {
                    return false;
                }
                dir = 1;
            } else {
                if dir == 0 {
                    return false;
                }
                dir = -1
            }
            pos = el;
        }

        return dir == -1;
    }
    assert_eq!(valid_mountain_array(vec![2, 1]), false);
    assert_eq!(valid_mountain_array(vec![1, 3, 2]), true);
    assert_eq!(valid_mountain_array(vec![3, 5, 5]), false);
    assert_eq!(valid_mountain_array(vec![0, 3, 2, 1]), true);
    assert_eq!(
        valid_mountain_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
        false
    );
}

#[test]
fn replaceElementsWithGreatestElementOnRightSide() {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut output = std::collections::VecDeque::new();
        output.push_back(-1);
        for i in (1..arr.len()).rev() {
            let front = *output.front().unwrap();
            output.push_front(front.max(arr[i]));
        }
        return output.into();
    }

    assert_eq!(
        replace_elements(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    )
}

#[test]
fn MoveZeroes() {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut lastNonZeroFoundAt = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, lastNonZeroFoundAt);
                lastNonZeroFoundAt += 1;
            }
        }
    }
    let mut sample = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut sample);
    assert_eq!(sample, vec![1, 3, 12, 0, 0])
}

#[test]
fn sortArrayByParity() {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut out = std::collections::VecDeque::new();
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                out.push_front(nums[i]);
            } else {
                out.push_back(nums[i]);
            }
        }
        return out.into();
    }
    let mut sorted_array = sort_array_by_parity(vec![3, 1, 2, 4]);
    let (even, odd) = sorted_array.split_at_mut(2);
    even.sort();
    odd.sort();
    assert_eq!(sorted_array, [2, 4, 1, 3]);
}

#[test]
fn heightChecker() {
    // TODO: suboptimal
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sheights = heights.clone();
        sheights.sort();
        let mut mismatch = 0;
        for i in 0..heights.len() {
            if heights[i] == sheights[i] {
                continue;
            }
            mismatch += 1;
        }
        mismatch
    }
    assert_eq!(height_checker(vec![1, 2, 1, 2, 2, 3, 4]), 2);
}

#[test]
fn thirdMax() {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let first = nums.iter().max().unwrap();
        let second = nums.iter().filter(|&p| p < first).max().unwrap_or(first);
        return *nums.iter().filter(|&p| p < second).max().unwrap_or(first);
    }

    assert_eq!(third_max(vec![2, 2, 3, 1]), 1);
    assert_eq!(third_max(vec![2, 1]), 2);
}

#[test]
fn findDisappearedNumbers() {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            let n = (nums[i].abs() - 1) as usize;
            nums[n] = -nums[n].abs();
        }

        nums.iter()
            .enumerate()
            .filter_map(|(i, num)| match num.cmp(&0) {
                std::cmp::Ordering::Greater => Some(i as i32 + 1),
                _ => None,
            })
            .collect()
    }

    assert_eq!(
        find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        [5, 6]
    );
}

#[test]
fn containsDuplicate() {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for n in nums.iter() {
            if set.contains(n) {
                return true;
            }
            set.insert(n);
        }

        false
    }
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
}
#[test]
fn validAnagram() {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = std::collections::HashMap::<char, u32>::new();
        for char in s.chars() {
            let count = map.get_mut(&char);
            if count.is_none() {
                map.insert(char, 1);
            } else {
                *count.unwrap() += 1;
            }
        }
        for char in t.chars() {
            let count = map.get_mut(&char);
            if count.is_none() {
                break;
            }
            let count = count.unwrap();

            if count == &1 {
                map.remove(&char);
            } else {
                *count -= 1;
            }
        }

        map.is_empty()
    }

    assert_eq!(
        is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
}

#[test]
fn validPalindrome() {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let reversed: String = s.chars().rev().collect();
        s.eq(&reversed)
    }

    assert_eq!(
        is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
    assert_eq!(is_palindrome("race a car".to_string()), false);
    assert_eq!(is_palindrome(" ".to_string()), true);
}

#[test]
fn romanToInt() {
    pub fn roman_to_int(s: String) -> i32 {
        let romans = std::collections::HashMap::from([
            ("M", 1000),
            ("D", 500),
            ("C", 100),
            ("L", 50),
            ("X", 10),
            ("V", 5),
            ("I", 1),
        ]);

        let mut sum = 0;
        let mut i = 0;

        while i < s.len() {
            let (first, second) = (s.get(i..i + 1).unwrap(), s.get(i + 1..i + 2));
            let mut composed = true;
            sum += match (first, second) {
                ("I", Some(y)) if y == "X" || y == "V" => romans.get(y).unwrap() - 1,
                ("X", Some(y)) if y == "L" || y == "C" => romans.get(y).unwrap() - 10,
                ("C", Some(y)) if y == "D" || y == "M" => romans.get(y).unwrap() - 100,
                _ => {
                    *(&mut composed) = false;
                    *romans.get(first).unwrap()
                }
            };

            if composed {
                i += 2
            } else {
                i += 1;
            }
        }

        sum
    }

    assert_eq!(roman_to_int("IX".to_string()), 9);
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("LVIII".to_string()), 58);
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
}

#[test]
fn sqrt() {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 1 {
            return 1;
        }

        let (mut left, mut right) = (0, x.min(46340));

        if x >= right * right {
            return right;
        }

        loop {
            if right - left <= 1 {
                return left;
            }

            let current_mid = (left + right) / 2;
            let squared = current_mid * current_mid;

            if squared == x {
                return current_mid;
            }
            if squared > x {
                right = current_mid;
            } else {
                left = current_mid;
            }
        }
    }

    assert_eq!(my_sqrt(2147395599), 46339);
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(9), 3);
    assert_eq!(my_sqrt(12), 3);
    assert_eq!(my_sqrt(27), 5);
}

#[test]
fn validParentheses() {
    pub fn is_valid(s: String) -> bool {
        // odd lengths are always invalid
        if s.len() & 1 == 1 {
            return false;
        }

        let mut open = vec![];

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => open.push(c),
                _ => match open.pop() {
                    Some('(') if c == ')' => (),
                    Some('[') if c == ']' => (),
                    Some('{') if c == '}' => (),
                    _ => return false,
                },
            }
        }

        return open.is_empty();
    }

    assert_eq!(is_valid("()[]".to_string()), true);
    assert_eq!(is_valid("[()[]([{}])]".to_string()), true);
    assert_eq!(is_valid("(])(".to_string()), false);
}

#[test]
fn longestCommonPrefix() {
    // We can also use divide and conquer
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs.get(0..1).unwrap()[0].to_owned();
        }

        let mut i = 0;
        let first = &strs[0];

        'common: while i < first.len() {
            for str in strs.iter().skip(1) {
                if str.get(i..i + 1) != first.get(i..i + 1) {
                    break 'common;
                }
            }
            i += 1
        }
        return first.get(0..i).unwrap_or_default().to_owned();
    }

    assert_eq!(
        longest_common_prefix(vec![
            "flower".to_string(),
            "flower".to_string(),
            "flower".to_string()
        ]),
        "flower".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec![
            "flower".to_string(),
            "flower".to_string(),
            "fl".to_string()
        ]),
        "fl".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec!["f".to_string(),]),
        "f".to_string()
    )
}

#[test]
fn mergeTwoSortedLists() {
    // Non optimal. We should use splicing using mem::swap
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut r: Option<Box<ListNode>> = None;
        let mut result = &mut r;
        let mut head1 = list1;
        let mut head2 = list2;

        loop {
            match (&mut head1, &mut head2) {
                (Some(node1), Some(node2)) => {
                    if node1.val < node2.val {
                        *result = Some(Box::new(ListNode::new(node1.val)));
                        head1 = head1.unwrap().next;
                    } else {
                        *result = Some(Box::new(ListNode::new(node2.val)));
                        head2 = head2.unwrap().next;
                    }
                    let next = &mut result.as_mut().unwrap().next;
                    result = next;
                }
                (Some(_), None) => {
                    *result = head1;
                    break;
                }
                (None, Some(_)) => {
                    *result = head2;
                    break;
                }
                (None, None) => break,
            }
        }
        r
    }

    let mut array1 = Box::new(ListNode::new(1));
    array1.next = Some(Box::new(ListNode::new(2)));
    array1.next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut array2 = Box::new(ListNode::new(1));
    array2.next = Some(Box::new(ListNode::new(3)));
    array2.next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut array3 = Box::new(ListNode::new(1));
    array3.next = Some(Box::new(ListNode::new(1)));
    array3.next.as_deref_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    array3
        .next
        .as_deref_mut()
        .unwrap()
        .next
        .as_deref_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(3)));
    array3
        .next
        .as_deref_mut()
        .unwrap()
        .next
        .as_deref_mut()
        .unwrap()
        .next
        .as_deref_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(4)));
    array3
        .next
        .as_deref_mut()
        .unwrap()
        .next
        .as_deref_mut()
        .unwrap()
        .next
        .as_deref_mut()
        .unwrap()
        .next
        .as_deref_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(4)));

    assert_eq!(merge_two_lists(Some(array1), Some(array2)), Some(array3));
}

#[test]
fn strStr() {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(p) = haystack.find(&needle) {
            return p as i32;
        }
        -1
    }
    assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
}
#[test]
fn search_insert() {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = (left + right) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Equal => {
                    left = mid;
                    break;
                }
                std::cmp::Ordering::Greater => right = mid - 1,
                std::cmp::Ordering::Less => left = mid + 1,
            }
        }
        left
    }
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
}

#[test]
fn lengthOfLastWord() {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut len = 0;
        let separator = ' ' as u8;
        for b in s.as_bytes().iter().rev() {
            if b == &separator {
                if len > 0 {
                    return len;
                } else {
                    continue;
                }
            }
            len += 1;
        }

        len
    }
    assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    assert_eq!(
        length_of_last_word("   fly me   to   the moon  ".to_string()),
        4
    );
}
#[test]
fn plusOne() {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits: std::collections::VecDeque<i32> = digits.into();
        let mut carry = 1;
        let mut digit = (digits.len() as i32) - 1;
        while carry > 0 && digit >= 0 {
            {
                let digit = digit as usize;
                if digits[digit] + carry > 9 {
                    digits[digit] = 0
                } else {
                    digits[digit] += carry;
                    carry = 0;
                }
            }

            digit -= 1;
        }
        if carry > 0 {
            digits.push_front(carry);
        }
        digits.into()
    }

    assert_eq!(plus_one(vec![9]), vec![1, 0]);
    assert_eq!(plus_one(vec![1, 0]), vec![1, 1]);
    assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
}

#[test]
fn add_binary() {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let mut carry = 0;
        let mut result = String::new();

        loop {
            match (a.pop(), b.pop()) {
                (Some(f), Some(s)) => match (f, s) {
                    ('1', '1') => {
                        if carry > 0 {
                            result.push('1')
                        } else {
                            result.push('0');
                            carry = 1;
                        }
                    }
                    ('1', '0') | ('0', '1') => {
                        if carry > 0 {
                            result.push('0');
                            carry = 1;
                        } else {
                            result.push('1');
                        }
                    }
                    _ => {
                        result.push(std::char::from_digit(carry, 10).unwrap());
                        carry = 0;
                    }
                },

                (Some(f), None) | (None, Some(f)) => match f {
                    '1' => {
                        if carry > 0 {
                            result.push('0');
                            carry = 1;
                        } else {
                            result.push('1');
                        }
                    }
                    '0' => {
                        if carry > 0 {
                            result.push('1');
                            carry = 0;
                        } else {
                            result.push('0');
                        }
                    }
                    _ => panic!(),
                },
                _ => {
                    return {
                        if carry > 0 {
                            result.push('1');
                        }
                        result.as_bytes().iter().map(|c| *c as char).rev().collect()
                    }
                }
            }
        }
    }
    assert_eq!(
        add_binary("10100".to_string(), "11".to_string()),
        "10111".to_string()
    );

    assert_eq!(
        add_binary("11".to_string(), "1".to_string()),
        "100".to_string()
    );

    assert_eq!(
        add_binary("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    );
}

#[test]
fn toHex() {
    pub fn to_hex(num: i32) -> String {
        fn u8_to_hex(byte: u8) -> char {
            match byte {
                0..=9 => char::from(b'0' + byte),
                _ => char::from(b'a' + byte - 10),
            }
        }

        if num == 0 {
            return "0".to_string();
        }

        let mut hex = String::new();

        num.to_be_bytes().iter().for_each(|byte| {
            // right shift by 4 and take the last 4
            let left_4_bits: u8 = (byte >> 4) & 0xf;
            // take the last 4
            let right_4_bits = byte & 0xf;
            hex.push(u8_to_hex(left_4_bits));
            hex.push(u8_to_hex(right_4_bits));
        });

        hex.trim_start_matches('0').to_string()
    }

    assert_eq!(to_hex(0), "0".to_string());
    assert_eq!(to_hex(16), "10".to_string());
    assert_eq!(to_hex(26), "1a".to_string());
    assert_eq!(to_hex(-1), "ffffffff".to_string());
    assert_eq!(to_hex(-100), "ffffff9c".to_string());
}

#[test]
fn majorityElement() {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        let mut current_elem = nums.pop().unwrap();
        let mut count = 1;

        for &elem in nums.iter() {
            if current_elem == elem {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    current_elem = elem;
                    count = 1;
                }
            }
        }
        current_elem
    }

    assert_eq!(majority_element(vec![3, 2, 3]), 3);
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    assert_eq!(
        majority_element(vec![
            6, 6, 6, 3, 2, 3, 4, 4, 4, 4, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5, 4, 3, 2
        ]),
        6
    );
}

#[test]
fn countBinarySubstrings() {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut sequence = s.as_bytes().iter().map(|&bit| bit == '0' as u8);
        let mut prev = sequence.next().unwrap();
        let mut groups: Vec<i32> = vec![];
        groups.push(1);
        for curr in sequence {
            if prev == curr {
                *groups.last_mut().unwrap() += 1;
            } else {
                groups.push(1);
            }

            prev = curr;
        }
        let mut gIter = groups.iter();
        let mut mins: Vec<i32> = vec![];
        let mut prev = gIter.next().unwrap();
        for curr in gIter {
            mins.push(*prev.min(curr));
            prev = curr;
        }
        mins.iter().sum()
    }
    assert_eq!(
        count_binary_substrings("00110011010110110101".to_string()),
        16
    );
    assert_eq!(count_binary_substrings("00110011".to_string()), 6);
    assert_eq!(count_binary_substrings("10101".to_string()), 4);
}

#[test]
fn generateParenthesis() {
    pub fn generate_parenthesis(n: usize) -> Vec<String> {
        let mut result = vec![];
        fn generate_valid(result: &mut Vec<String>, n: usize, o: usize, c: usize, path: String) {
            if o == n && c == n {
                result.push(path);
                return;
            }

            if o < n {
                generate_valid(result, n, o + 1, c, format!("{}(", path));
            }
            if o > c {
                generate_valid(result, n, o, c + 1, format!("{})", path));
            }
        }
        generate_valid(&mut result, n, 0, 0, "".to_string());
        result
    }
    let mut r0 = generate_parenthesis(3);
    let mut t0 = vec![
        "((()))".to_string(),
        "(()())".to_string(),
        "(())()".to_string(),
        "()(())".to_string(),
        "()()()".to_string(),
    ];
    r0.sort();
    t0.sort();
    assert_eq!(r0, t0);

    let mut r1 = generate_parenthesis(4);
    let mut t1 = vec![
        "(((())))".to_string(),
        "((()()))".to_string(),
        "((())())".to_string(),
        "((()))()".to_string(),
        "(()(()))".to_string(),
        "(()()())".to_string(),
        "(()())()".to_string(),
        "(())(())".to_string(),
        "(())()()".to_string(),
        "()((()))".to_string(),
        "()(()())".to_string(),
        "()(())()".to_string(),
        "()()(())".to_string(),
        "()()()()".to_string(),
    ];
    r1.sort();
    t1.sort();
    assert_eq!(r1, t1);
}
