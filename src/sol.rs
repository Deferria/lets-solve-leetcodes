//! This is the solutions to some leetcode problems in Rust.

#![allow(dead_code)]
#![allow(unused_variables)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    /// Create a new ListNode with the given value.
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    pub fn from_vec(arr: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in arr.iter().rev() {
            let mut temp = ListNode::new(val);
            temp.next = head;
            head = Some(Box::new(temp))
        }
        head
    }
}

impl ListNode {
    pub fn to_vec(&self) -> Vec<i32> {
        let mut vect = Vec::new();
        let mut curr = Some(self);
        while let Some(node) = curr {
            vect.push(node.val);
            curr = self.next.as_deref()
        }
        vect
    }
}

impl ListNode {
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut curr = Some(self);
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_deref();
        }
        len
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr = Some(self);
        let mut first = true;
        while let Some(node) = curr {
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node.val)?;
            curr = node.next.as_deref();
            first = false;
        }
        Ok(())
    }
}

/// This is the leetcode solution to problem 1.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

/// This is the leetcode solution to the problem 2."Add Two Numbers".
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: None }));
    let mut tail = &mut dummy;
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry > 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        carry = sum / 10;
        let new_node = Box::new(ListNode {
            val: sum % 10,
            next: None,
        });

        if let Some(tail_node) = tail {
            tail_node.next = Some(new_node);
            tail = &mut tail_node.next;
        }
    }
    dummy.unwrap().next
}

// Hash maps are needed to facilitate the solution to the problem 3."Longest Substring Without Repeating Characters".
use std::{collections::HashMap, i32, result};

/// This is the leetcode solution to the problem 3."Longest Substring Without Repeating Characters".
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut record = HashMap::new();
    let mut left = 0;
    let mut res = 0;

    for (id, c) in s.char_indices() {
        if let Some(pos) = record.get_mut(&c) {
            if *pos >= left {
                left = *pos + 1;
                *pos = id;
            }
        }
        record.insert(c, id);
        res = if res > id - left + 1 {
            res
        } else {
            id - left + 1
        }
    }
    res as i32
}

/// This is the tool function used below (leetcode problem 5)
fn expand(chs: &[char], mut left: isize, mut right: isize) -> (isize, isize) {
    let ns = chs.len() as isize;
    while left >= 0 && right < ns && chs[left as usize] == chs[right as usize] {
        left -= 1;
        right += 1;
    }
    (left + 1, right - 1)
}

/// This is the leetcode solution to the problem 5."Longest Palindromic Substring".
pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    let mut left = 0;
    let mut maxlen = 0;

    if n <= 1 {
        return s;
    }

    for i in 0..n {
        let (l1, r1) = expand(&chars, i as isize, i as isize);
        let len1 = (r1 - l1 + 1) as usize;

        if len1 > maxlen {
            left = l1 as usize;
            maxlen = len1;
        }

        let (l2, r2) = expand(&chars, i as isize, i as isize + 1);
        let len2: usize = if l2 <= r2 { (r2 - l2 + 1) as usize } else { 0 };

        if len2 > maxlen {
            left = l2 as usize;
            maxlen = len2;
        }

        println!("After {} iteration, max len is {}", i, maxlen)
    }

    return chars[left..(left + maxlen)].into_iter().collect();
}

/// This is the leetcode solution to the problem 6."Zigzag Conversion".
pub fn zigzag_converter(s: String, num_rows: i32) -> String {
    let nr = num_rows as usize;
    let mut vect: Vec<String> = vec![String::from(""); nr];
    let n = 2 * nr - 2;

    if n == 0 {
        return s;
    }

    for (id, c) in s.char_indices() {
        let remainder = id % n;

        let pos = if remainder < nr {
            remainder
        } else {
            n - remainder
        };
        let u = &mut vect[pos];
        u.push(c);
    }
    vect.into_iter().collect()
}

/// This is the leetcode solution to the problem 7."Reverse Integer".
pub fn reverse(x: i32) -> i32 {
    if x == 0 {
        return 0;
    } else if x == i32::MIN {
        return 0;
    }
    let mut ch = if x < 0 {
        (-x).to_string()
    } else {
        x.to_string()
    };
    ch = ch.chars().rev().collect();
    let res: u64 = ch.parse().unwrap();
    println!("res: {}", res);
    if res > i32::MAX as u64 {
        return 0;
    }
    return if x < 0 { -(res as i32) } else { res as i32 };
}

/// This is the tool function used below (leetcode problem 8)
fn parse_char_to_num(c: &char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => u8::MAX,
    }
}

/// This is the leetcode solution to the problem 8."String to Integer (my atoi)".
pub fn my_atoi(s: String) -> i32 {
    let trimmed = s.trim_start();
    let mut res = String::from("");
    let mut is_neg = false;
    'outer: for (id, c) in (*trimmed).char_indices() {
        if id == 0 {
            if c == '+' {
                is_neg = false
            } else if c == '-' {
                is_neg = true
            } else if c == ' ' {
                continue;
            } else if !c.is_ascii_digit() {
                return 0;
            } else {
                res.push(c);
            }
        } else {
            if c == ' ' {
                continue;
            } else if !c.is_ascii_digit() {
                break 'outer;
            } else {
                res.push(c);
            }
        }
    }
    if res == "" {
        return 0;
    }
    let n = res.len();
    let mut ans: u128 = 0;
    for (i, c) in res.char_indices() {
        ans += 10u128.pow(n as u32 - 1 - i as u32) * parse_char_to_num(&c) as u128;
        println!("current: {}", ans)
    }
    if is_neg {
        let max_neg_abs = (i32::MAX as u128) + 1;
        if ans >= max_neg_abs {
            return i32::MIN;
        } else {
            return -(ans as i32);
        }
    } else {
        if ans > i32::MAX as u128 {
            return i32::MAX;
        }

        return ans as i32;
    }
}

/// This is the leetcode solution to the problem 9."Palindrome Number".
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        false
    } else {
        let mut x = x;
        let mut x_rev: Vec<u8> = Vec::new();
        loop {
            if x == 0 {
                break;
            }
            let digit = x % 10;
            //println!("digit: {}", digit);
            x_rev.push(digit as u8);
            x = x / 10;
        }
        for i in 0..(x_rev.len() / 2) {
            if x_rev[i] != x_rev[x_rev.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

/*
fn match_meta(ptn_char: char, a: char) -> bool {
    if ptn_char == '.' {
        true
    } else {
        ptn_char == a
    }
}

pub fn is_match(s: String, p: String) -> bool {
    let mut now_at: usize = 0;
    for (id, pc) in s.char_indices() {
        let mut comparison: char;
        if pc == '.' {
            continue;
        } else if pc == '*' {
        } else {
        }
    }
    return true;
}
*/

/// This is the solution to the leetcode problem 11."Container With Most Water".
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut area = 0;
    let (mut left, mut right) = (0 as usize, height.len() - 1);

    loop {
        if left >= right {
            break;
        }
        let curr = (right - left) as i32 * std::cmp::min(height[left], height[right]);

        if curr > area {
            area = curr
        }

        if height[left] < height[right] {
            left += 1
        } else {
            right -= 1
        }
    }

    area
}

/// This is the meta function for the leetcode problem 12."Integer to Roman". Though it looks stupid to match digit one by one, it is the most straightforward way to implement the solution.
fn roman_meta(digit: i32, idx: usize) -> String {
    let meta: &str;
    if idx == 0 {
        meta = match digit {
            0 => "",
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            _ => "ERROR",
        }
    } else if idx == 1 {
        meta = match digit {
            0 => "",
            1 => "X",
            2 => "XX",
            3 => "XXX",
            4 => "XL",
            5 => "L",
            6 => "LX",
            7 => "LXX",
            8 => "LXXX",
            9 => "XC",
            _ => "ERROR",
        }
    } else if idx == 2 {
        meta = match digit {
            0 => "",
            1 => "C",
            2 => "CC",
            3 => "CCC",
            4 => "CD",
            5 => "D",
            6 => "DC",
            7 => "DCC",
            8 => "DCCC",
            9 => "CM",
            _ => "ERROR",
        }
    } else if idx == 3 {
        meta = match digit {
            0 => "",
            1 => "M",
            2 => "MM",
            3 => "MMM",
            _ => "ERROR",
        }
    } else {
        meta = ""
    }
    String::from(meta)
}

/// This is the leetcode solution to the problem 12."Integer to Roman".
pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut ans = String::new();
    let mut id: usize = 0;
    loop {
        if num == 0 {
            break;
        }
        let dig = num % 10;
        let meta = roman_meta(dig, id);
        ans = format!("{}{}", meta, ans);
        num = num / 10;
        id += 1;
    }
    ans
}

/// This is the leetcode solution to the problem 13."Roman to Integer".
pub fn roman_to_int(s: String) -> i32 {
    let mut val = 0;
    for (id, c) in s.char_indices() {
        if c == 'I' {
            val += 1;
        } else if c == 'V' {
            if id > 0 && s.chars().nth(id - 1).unwrap() == 'I' {
                val += 3;
            } else {
                val += 5;
            }
        } else if c == 'X' {
            if id > 0 && s.chars().nth(id - 1).unwrap() == 'I' {
                val += 8;
            } else {
                val += 10;
            }
        } else if c == 'L' {
            if id > 0 && s.chars().nth(id - 1).unwrap() == 'X' {
                val += 30;
            } else {
                val += 50;
            }
        } else if c == 'C' {
            if id > 0 && s.chars().nth(id - 1).unwrap() == 'X' {
                val += 80;
            } else {
                val += 100;
            }
        } else if c == 'D' {
            if id > 0 && s.chars().nth(id - 1).unwrap() == 'C' {
                val += 300;
            } else {
                val += 500;
            }
        } else if c == 'M' {
            if id > 0 && s.chars().nth(id - 1).unwrap() == 'C' {
                val += 800;
            } else {
                val += 1000;
            }
        }
    }
    val
}

/// This is the leetcode solution to the problem 14."Longest Common Prefix".
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut pref = strs[0].clone();

    for s in &strs[1..] {
        while !s.starts_with(&pref) {
            if s.is_empty() {
                return String::new();
            }
            pref.pop();

            if pref.is_empty() {
                return String::new();
            }
        }
    }
    pref
}

/// This is the leetcode solution to the problem 15."3Sum". (That is, find all unique triplets in the array which gives the sum of a definite number. In this case, the definite number is 0.)
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let n = nums.len();
    let mut res = Vec::new();
    for i in 0..n - 1 {
        if i > 0 && nums[i - 1] == nums[i] {
            continue;
        }
        let mut p1 = i + 1;
        let mut p2 = n - 1;

        let target = -nums[i];
        loop {
            if p1 >= p2 {
                break;
            }

            if p2 == n - 1 || p1 == i + 1 {
                if nums[p1] + nums[p2] > target {
                    p2 -= 1;
                } else if nums[p1] + nums[p2] < target {
                    p1 += 1;
                } else {
                    res.push(vec![nums[i], nums[p1], nums[p2]]);
                    p2 -= 1;
                    p1 += 1;
                }
            } else {
                if nums[p1] == nums[p1 - 1] {
                    p1 += 1;
                } else if nums[p2] == nums[p2 + 1] {
                    p2 -= 1;
                } else {
                    if nums[p1] + nums[p2] > target {
                        p2 -= 1;
                    } else if nums[p1] + nums[p2] < target {
                        p1 += 1;
                    } else {
                        res.push(vec![nums[i], nums[p1], nums[p2]]);
                        p2 -= 1;
                        p1 += 1;
                    }
                }
            }
        }
    }
    res
}

/// This is the leetcode solution to the problem 16."3Sum Closest". That is, find three integers in nums such that the sum is closest to target (criteria is the absolute difference). Return the sum of the three integers.
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let nums = nums;

    let n = nums.len();
    let mut closest = nums[0] + nums[1] + nums[2];
    let mut closest_diff = closest - target;

    for i in 0..n - 2 {
        let mut p1 = i + 1;
        let mut p2 = n - 1;

        if nums[i] + nums[i + 1] + nums[i + 2] > closest && closest_diff > 0 {
            break;
        }

        if nums[i] + nums[p2] + nums[p2 - 1] < closest && closest_diff < 0 {
            continue;
        }

        while p1 < p2 {
            let sum = nums[i] + nums[p1] + nums[p2];

            if (sum - target).abs() < closest_diff.abs() {
                closest = sum;
                closest_diff = closest - target;
            }
            if sum < target {
                p1 += 1;
            } else if sum > target {
                p2 -= 1;
            } else {
                return target;
            }
        }
    }
    closest
}

/// This is the tool function used below (leetcode problem 17).
/// It takes a string and a character (2-9) and returns a vector of strings with the character appended to the original string.
fn string_push(original: &String, key: char) -> Vec<String> {
    let value = match key {
        '2' => "abc",
        '3' => "def",
        '4' => "ghi",
        '5' => "jkl",
        '6' => "mno",
        '7' => "pqrs",
        '8' => "tuv",
        '9' => "wxyz",
        _ => "",
    };
    let value_string = String::from(value);
    let mut vect: Vec<String> = Vec::new();

    for c in value_string.chars() {
        let mut temp = original.clone();
        temp.push(c);
        vect.push(temp);
    }
    vect
}

/// This is the solution to the leetcode problem 17. letter combinations of a phone number.
pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for c in digits.chars() {
        if res.is_empty() {
            let mut temp = string_push(&String::from(""), c);
            res.append(&mut temp);
        } else {
            let mut partial: Vec<String> = Vec::new();
            for it in res.iter() {
                let mut temp = string_push(it, c);
                partial.append(&mut temp);
            }
            res = partial;
        }
    }
    res
}

/// To handle overflow in the four_sum problem, we define an enum to represent the result of a safe sum operation.
enum I32WithOverflow {
    Item(i32),
    Upward,
    Downward,
}

fn safe_sum(numbers: &[i32]) -> I32WithOverflow {
    let mut sum = 0i32;
    for &num in numbers {
        match sum.checked_add(num) {
            Some(res) => sum = res,
            None => {
                if sum > 0 && num > 0 {
                    return I32WithOverflow::Upward;
                } else {
                    return I32WithOverflow::Downward;
                }
            }
        }
    }
    I32WithOverflow::Item(sum)
}

/// This is the leetcode solution to the problem 18."4Sum". That is, find all unique quadruplets in the array which gives the sum of the target.
/// Overflow might happen. Thus, we use a `safe_sum` function to handle the sum of integers (i32).
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let n = nums.len();
    let mut res = Vec::new();

    if n < 4 {
        return res;
    }

    for i in 0..n - 3 {
        match safe_sum(&nums[i..i + 4]) {
            I32WithOverflow::Upward => {
                break;
            }
            I32WithOverflow::Downward => {}
            I32WithOverflow::Item(s) => {
                if s > target {
                    break;
                }
            }
        }

        match safe_sum(&[nums[i], nums[n - 3], nums[n - 2], nums[n - 1]]) {
            I32WithOverflow::Downward => {
                continue;
            }
            I32WithOverflow::Upward => {}
            I32WithOverflow::Item(s) => {
                if s < target {
                    continue;
                }
            }
        }

        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        for j in i + 1..n - 2 {
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }

            match safe_sum(&[nums[i], nums[j], nums[j + 1], nums[j + 2]]) {
                I32WithOverflow::Upward => {
                    break;
                }
                I32WithOverflow::Downward => {}
                I32WithOverflow::Item(s) => {
                    if s > target {
                        break;
                    }
                }
            }

            match safe_sum(&[nums[i], nums[j], nums[n - 2], nums[n - 1]]) {
                I32WithOverflow::Downward => {
                    continue;
                }
                I32WithOverflow::Upward => {}
                I32WithOverflow::Item(s) => {
                    if s < target {
                        continue;
                    }
                }
            }

            let mut p1 = j + 1;
            let mut p2 = n - 1;
            let target1 = target - nums[i] - nums[j];

            while p1 < p2 {
                if p2 == n - 1 || p1 == j + 1 {
                    if nums[p1] + nums[p2] > target1 {
                        p2 -= 1;
                        //println!("When p1 {}, p2 {}, no match, p2 <- 1.", p1, p2) // verbose debugging
                    } else if nums[p1] + nums[p2] < target1 {
                        p1 += 1;
                        //println!("When p1 {}, p2 {}, no match, p1 -> 1.", p1, p2) // verbose debugging
                    } else {
                        res.push(vec![nums[i], nums[j], nums[p1], nums[p2]]);
                        p2 -= 1;
                        p1 += 1;
                    }
                } else {
                    if nums[p1] == nums[p1 - 1] {
                        p1 += 1;
                    } else if nums[p2] == nums[p2 + 1] {
                        p2 -= 1;
                    } else {
                        if nums[p1] + nums[p2] > target1 {
                            p2 -= 1;
                        } else if nums[p1] + nums[p2] < target1 {
                            p1 += 1;
                        } else {
                            res.push(vec![nums[i], nums[j], nums[p1], nums[p2]]);
                            p2 -= 1;
                            p1 += 1;
                        }
                    }
                }
            }
        }
    }
    res
}

/// This is the solution to the leetcode problem 19."Remove Nth Node From End of List".
///
/// The first implementation is a naive one, which uses a vector to store the values of the linked list, which is time-consuming. The second implementation is more efficient, which uses two pointers to find the target node.
pub fn remove_nth_from_end_1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut res = Vec::new();
    let mut curr = head;

    loop {
        let Some(node) = &curr else {
            break;
        };
        res.push(node.val);
        curr = node.next.clone();
    }

    let mut constructor = None;
    for (id, &val) in res.iter().rev().enumerate() {
        if id != n as usize - 1 {
            println!("This is the {} node, with value {}", id, val);
            let mut node = ListNode::new(val);
            node.next = constructor;
            constructor = Some(Box::new(node));
        } else {
        }
    }
    constructor
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;

    let mut len = 0;
    let mut p = head.as_ref();

    while let Some(node) = p {
        len += 1;
        p = node.next.as_ref();
    }

    let target = len - n as usize;

    let nodes = &mut head;
    let mut curr = nodes;
    for _ in 0..target {
        if let Some(node) = curr {
            curr = &mut node.next;
        } else {
            return None;
        }
    }
    if let Some(mut target) = curr.take() {
        *curr = target.next.take();
    }
    head
}

/// This is the solution to the leetcode problem 20."Valid Parenthesis".
pub fn is_valid(s: String) -> bool {
    let mut p = String::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => p.push(c),
            ')' => {
                if p.is_empty() {
                    return false;
                }

                let ch = p.chars().nth_back(0).unwrap();
                match ch {
                    '(' => {
                        p.pop();
                    }
                    '[' | '{' => {
                        return false;
                    }
                    _ => {}
                }
            }
            ']' => {
                if p.is_empty() {
                    return false;
                }

                let ch = p.chars().nth_back(0).unwrap();
                match ch {
                    '[' => {
                        p.pop();
                    }
                    '(' | '{' => {
                        return false;
                    }
                    _ => {}
                }
            }
            '}' => {
                if p.is_empty() {
                    return false;
                }

                let ch = p.chars().nth_back(0).unwrap();
                match ch {
                    '{' => {
                        p.pop();
                    }
                    '[' | '(' => {
                        return false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    if p.is_empty() {
        return true;
    } else {
        return false;
    }
}

/// This is the solution to the leetcode problem 21."Merge Two Sorted Lists".
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut list1, mut list2) = (list1, list2);
    let mut dummy = Box::new(ListNode {
        val: i32::MAX,
        next: None,
    });
    let mut tail = &mut dummy;

    while list1.is_some() && list2.is_some() {
        let v1 = list1.as_ref().unwrap().val;
        let v2 = list2.as_ref().unwrap().val;

        if v1 <= v2 {
            let mut temp = list1.take();
            list1 = temp.as_mut().unwrap().next.take();
            tail.next = temp;
        } else {
            let mut temp = list2.take();
            list2 = temp.as_mut().unwrap().next.take();
            tail.next = temp;
        }
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = if list1.is_some() { list1 } else { list2 };
    dummy.next
}

/// This is the recursive method solution to generate parenthesis according to the remaining pattern.
fn generate_parenthesis_recursive(s: String, open: i32, closed: i32, target: &mut Vec<String>) {
    if open == 0 && closed == 0 {
        target.push(s);
        return;
    } else if open == 0 {
        let mut temp = s;
        for _ in 0..closed {
            temp.push(')');
        }
        target.push(temp);
        return;
    } else if open == closed {
        let mut s1 = s;
        s1.push('(');
        return generate_parenthesis_recursive(s1, open - 1, closed, target);
    } else {
        let mut s1 = s.clone();
        let mut s2 = s;
        s1.push('(');
        s2.push(')');
        generate_parenthesis_recursive(s1, open - 1, closed, target);
        generate_parenthesis_recursive(s2, open, closed - 1, target);
    }
}

/// This is the solution to the LeetCode problem 22
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let open = n;
    let closed = n;
    let mut res = Vec::new();
    let temp = String::new();
    generate_parenthesis_recursive(temp, open, closed, &mut res);
    res
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut original = lists;
    if original.len() == 0 {
        return None;
    } else {
        while original.len() > 1 {
            let mut temp: Vec<Option<Box<ListNode>>> = Vec::new();
            let mut i = 0usize;

            while i < original.len() {
                let l1 = original[i].take();
                let l2 = if i + 1 < original.len() {
                    original[i + 1].take()
                } else {
                    None
                };
                temp.push(merge_two_lists(l1, l2));
                i += 2;
            }
            original = temp;
        }
    }
    original[0].take()
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: i32::MAX,
        next: head,
    });
    let mut curr = &mut dummy;

    loop {
        let mut origin = match curr.next.take() {
            Some(node) => node,
            None => break,
        };

        let mut target = match origin.next.take() {
            Some(node) => node,
            None => {
                curr.next = Some(origin);
                break;
            }
        };

        let successor = target.next.take();

        origin.next = successor;
        target.next = Some(origin);
        curr.next = Some(target);

        let temp = curr.next.as_mut().unwrap().next.as_mut().unwrap();
        curr = temp;
    }
    dummy.next.take()
}

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }

    let mut p = &head;

    for _ in 0..k {
        match p {
            Some(node) => p = &node.next,
            None => {
                return head;
            }
        }
    }

    let mut result = None;

    for _ in 0..k {
        let mut temp = head.take().unwrap();
        head = temp.next.take();
        temp.next = result.take();
        result = Some(temp)
    }

    let rest = reverse_k_group(head, k);

    let mut tail = &mut result;

    while let Some(node) = tail {
        if node.next.is_none() {
            node.next = rest;
            break;
        }
        tail = &mut node.next;
    }

    result
}

#[cfg(test)]
mod test_modules {
    use super::*;

    #[test]
    fn test_max_area() {
        let v = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(v), 49)
    }

    #[test]
    fn test_three_sum_weird() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(three_sum(v), vec![vec![0, 0, 0]])
    }

    #[test]
    fn test_three_sum_closest() {
        let v = vec![-1, 2, 1, -4];
        assert_eq!(three_sum_closest(v, 1), 2)
    }

    #[test]
    fn test_three_sum_closest_2() {
        let v = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        assert_eq!(three_sum_closest(v, 1), 60)
    }

    #[test]
    fn test_three_sum_closest_3() {
        let v = vec![
            445, 891, 901, -869, -747, -750, 112, 411, -236, 443, -759, -480, 723, -702, 624, -556,
            -763, -806, -682, -831, 449, 62, 718, 398, -937, -353, -968, 497, 504, -94, 494, 79,
            -26, 430, -766, 439, 87, 899, -216, 18, -3, 844, -160, 351, 646, -361, -375, -689, 255,
            546, -846, 294, -964, 382, -998, -994, 545, -162, -13, -519, -833, 533, -975, 974, 920,
            -768, 877, 911, 895, 677, -625, -246, -355, 639, 499, -668, -117, 807, 662, -821, -264,
            525, 428, -675, -917, -664, -933, -620, -844, 784, -462, 228, 736, 637, -424, 395, 345,
            -977, -980, -608, -258, -64, 64, 831, 591, -780, -358, 691, 310, 866, 529, 156, 821,
            -461, 369, 832, -808, -16, 92, -734, -485, 776, 192, -206, 168, 447, -54, -544, -192,
            864, -653, -868, 97, 337, -902, -108, -752, 695, 711, -802, -823, 537, 341, -477, -861,
            -169, 77, -276, -815, -836, 849, -524, -516, 273, -273, 423, -102, 260, -213, -197,
            -932, 81, -248, 409, -694, 681, 900, -897, 978, 714, 455, 946, 559, 388, 623, -839,
            -830, -935, -391, 320, -552, -755, -458, -728, 583, 663, -131, -727, -372, 576, 476,
            947, 220, -331, -124, 39, -753, -963, 284, 76, -715, 673, 325, 342, -152, -841, -79,
            495, 5, 389, -295, -925, 181, -29, 68, 231, -574, -78, 403, 763, 981, -148, -911, -711,
            998, -338, 589, -565, -324, -673, -290, 477, -825, 472, 431, -386, 219, -247, -798,
            202, 417, -698, 164, 396, 851, -356, -527, -785, -126, -437, 919, 670, -644, -33, -811,
            602, -606, -116, -988, -837, 13, 956, 780, 674, 798, 435, 441, 264, 709, 953, 232,
            -683, -120, 318, -924, -985, -505, -232, -889, 816, -564, -428, -427, -543, -10, 625,
            454, -626, -58, 486, -92, -618, -260, -443, 577, 527, 949, 245, -569, -285, -672, -573,
            -431, 363, -733, -369, 843, 373, -572, 721, -860, 526, -680, 479, 618, 598, 887, -243,
            535, 660, 758, -71, 266, -311, -528, 283, -602, -24, 114, -934, 580, -435, -871, -993,
            -319, 25, -274, 859, -389, -96, 375, 692, -749, 381, 560, 513, 982, 463, -592, 336,
            -214, -460, 374, 850, -859, 387, -879, 205, -511, 468, 6, 432, -268, 10, 103, -507,
            -226, -228, -326, -515, -422, -582, 241, 347, -414, -143, 923, 874, -89, 297, -912,
            291, 912, -65, 161, -822, -784, -230, 456, -553, 556, -832, 421,
        ];
        assert_eq!(three_sum_closest(v, -7580), -2985)
    }

    #[test]
    fn test_four_sum() {
        let v = vec![0, 0, 0, 0];
        let res = vec![vec![0, 0, 0, 0]];
        assert_eq!(four_sum(v, 0), res)
    }

    #[test]
    fn test_four_sum_overflow() {
        let v = vec![0, 0, 0, 1000000000, 1000000000, 1000000000, 1000000000];
        let expected = vec![vec![0, 0, 0, 1000000000]];
        assert_eq!(four_sum(v, 1000000000), expected)
    }

    #[test]
    fn test_parenthesis_is_valid() {
        assert_eq!(is_valid(String::from(")[]{}")), false)
    }

    #[test]
    fn test_merge_two_lists() {
        let l1 = ListNode::from_vec(&[1, 2, 4]);
        let l2 = ListNode::from_vec(&[1, 3, 4]);
        let a = ListNode::from_vec(&[1, 1, 2, 3, 4, 4]);
        assert_eq!(merge_two_lists(l1, l2), a)
    }

    #[test]
    fn notest_generate_parenthesis() {
        let s = String::from("((");
        let mut vect = Vec::new();
        generate_parenthesis_recursive(s, 1, 3, &mut vect);
        println!("Result: {:?}", vect)
    }
}
