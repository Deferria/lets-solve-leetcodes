use std::collections::VecDeque;

/// This is the solution to the leetcode problem 38."Count and Say".
pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".into();
    }
    let res = count_and_say(n - 1);
    let mut ans = String::new();
    let mut curr = res.chars().nth(0).unwrap();
    let mut count: i32 = 0;
    for i in res.chars() {
        if i == curr {
            count += 1;
            continue;
        }
        ans.push_str(&count.to_string());
        ans.push(curr);
        count = 1;
        curr = i;
    }
    ans.push_str(&count.to_string());
    ans.push(res.chars().nth_back(0).unwrap());
    ans
}

/// The tool function to solve Problem 38. Run-Length Encoding implemetation.
pub fn push_down(s: String) -> String {
    assert!(s.len() > 0, "nooooooo");
    let mut ans = String::new();
    let mut curr = s.chars().nth(0).unwrap();
    println!("{}", curr);
    let mut count: i32 = 0;
    for i in s.chars() {
        if i == curr {
            count += 1;
            continue;
        }
        ans.push_str(&count.to_string());
        ans.push(curr);
        count = 1;
        curr = i;
    }
    ans.push_str(&count.to_string());
    ans.push(s.chars().nth_back(0).unwrap());
    ans
}

/// This is the solution to LeetCode Problem 39. "Combination Sum". Given an array of distinct integers as `candidates` and a target integer `target`, return a list of all unique combinations of candidates where the chosen numbers sum to target.
///
/// You may return the combinations in any order. The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.
pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn one_step(
        new_candidates: &[i32],
        new_target: i32,
        curr: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        mut i: usize,
    ) {
        println!(
            "This is the step case for target {}, CURRENT: {:?}",
            new_target, curr
        );
        if new_target == 0 {
            res.push(curr.to_vec());
            return;
        }
        while i < new_candidates.len() && new_candidates[i] <= new_target {
            curr.push(new_candidates[i]);
            one_step(new_candidates, new_target - new_candidates[i], curr, res, i);
            i += 1;
            curr.pop();
        }
    }

    candidates.sort_unstable();
    let mut r: Vec<i32> = Vec::new();
    let mut ans: Vec<Vec<i32>> = Vec::new();

    one_step(&candidates[0..], target, &mut r, &mut ans, 0);

    ans
}

/// This is the solution to LeetCode Problem 40. "Combination Sum II". Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn one_step(
        current_candidates: &Vec<i32>,
        new_target: i32,
        curr: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        ind: usize,
    ) {
        if new_target == 0 {
            res.push(curr.to_vec());
            return;
        }
        for i in ind..current_candidates.len() {
            if current_candidates[i] > new_target {
                return;
            }
            if i > ind && current_candidates[i] == current_candidates[i-1] {
                continue;
            }
            curr.push(current_candidates[i]);
            one_step(current_candidates, new_target - current_candidates[i], curr, res, i+1);
            curr.pop();
        }
    }

    candidates.sort_unstable();
    let mut r: Vec<i32> = Vec::new();
    let mut ans: Vec<Vec<i32>> = Vec::new();

    one_step(&candidates, target, &mut r, &mut ans, 0);

    ans
}

/// This is the solution to LeetCode Problem 41. "First Missing Positive". Given an unsorted integer array nums, return the smallest missing positive integer.
/// 
/// The algorithm works by first filtering out all non-positive numbers, then sorting the remaining numbers and removing duplicates, and finally checking for the first missing positive integer.
pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    nums.retain(|&x| x > 0);
    nums.sort_unstable();
    nums.dedup();
    let n = nums.len();
    for (id, it) in nums.into_iter().enumerate() {
        if (id+1) as i32 != it {
            return (id+1) as i32;
        }
    }
    return n as i32
}

/// This is another solution to LeetCode Problem 41. "First Missing Positive", which needn't sort the array. But it needs to traverse the array three times as well.
pub fn first_missing_positive_2(mut nums: Vec<i32>) -> i32 {
    let dummy = (nums.len() + 1) as i32;
    for it in nums.iter_mut() {
        if *it <= 0 || *it >= dummy {
            *it = dummy;
        } 
    }
    for i in 0..nums.len() {
        let temp = nums[i].abs();
        if temp < dummy {
            nums[(temp - 1) as usize] = -nums[(temp - 1) as usize].abs()
        }
    }
    for (id, it) in nums.into_iter().enumerate() {
        if it > 0 {
            return (id+1) as i32
        }
    }
    return dummy;
}

/// This is the third solution to LeetCode Problem 41. "First Missing Positive", which needn't sort the array. It only traverses the array twice.
/// 
/// During the first traversal, swap the elemets until into correct position or out of range. During the second traversal, check the first element that is not in correct position.
pub fn first_missing_positive_3(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        while 1 <= nums[i] && nums[i] <= nums.len() as i32 && nums[(nums[i]-1) as usize] != nums[i] {
            let correct = (nums[i] - 1) as usize;
            nums.swap(i, correct)
        }
    }
    for (id, it) in nums.iter().enumerate() {
        if *it != id as i32 + 1 {
            return  id as i32 + 1;
        }
    }
    return nums.len() as i32 + 1;
}

/// This is the solution to Problem **41. trapping rain water**.
pub fn trap(height: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0usize, height.len()-1);
    let (mut lm, mut rm) = (height[l], height[r]);
    let mut ans = 0i32;
    while l < r {
        if height[l] < height[r] {
            l += 1;
            lm = std::cmp::max(lm, height[l]);
            ans += height[l] - lm;
        } else {
            r -= 1;
            rm = std::cmp::max(rm, height[r]);
            ans += height[r] - rm;
        }
    }
    ans
}

/// This is the solution to LeetCode Problem 43. "Multiply Strings". Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
/// 
/// Built-in BigInteger libs are forbidden.
pub fn multiply(num1: String, num2: String) -> String {
    let zero = String::from("0");
    if num1 == zero || num2 == zero {
        return String::from("0")
    }
    let mut v = vec![0u8; num1.len()+num2.len()];

    for (i, it1) in num1.char_indices().rev() {
        for (j, it2) in num2.char_indices().rev() {
            let temp = (it1 as u8 - '0' as u8) * (it2 as u8 - '0' as u8);
            v[i+j+1] += temp;
            v[i+j] += v[i+j+1] / 10;
            v[i+j+1] %= 10;
        }
    }

    let mut nonzero = v.len() - 1;
    for (i, it) in v.iter().enumerate() {
        if *it != 0 {
            nonzero = i;
            break;
        }
    }

    let mut prod = String::new();
    for i in nonzero..v.len() {
        prod.push((v[i] + '0' as u8) as char);
    }
    prod
}

/// Solution to LeetCode Problem 45. "Jump Game II". 
/// This is the solution using dynamic programming:
/// - Time complexity: O(n^2)
/// - Space complexity: O(n)
pub fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![i32::MAX; n];
    for (i, it) in nums.into_iter().enumerate().rev() {
        if i == n-1 {
            dp[i] = 0;
        } else {
            let mut temp = i32::MAX-1;
            for j in 1..(it+1) {
                if j as usize + i < n {
                    temp = std::cmp::min(temp, dp[j as usize + i])
                }
            }
            dp[i] = temp + 1;
        }
    }
    dp[0]
}

/// Solution to LeetCode Problem 45. "Jump Game II".
/// This is the solution using greedy BFS: to find the farthest index that can be reached in the current jump, and then jump to that index. Repeat until reaching the end of the array.
/// - Time complexity: O(n)
/// - Space complexity: O(1)
pub fn jump_2(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut prev, mut farthest, mut ind) = (0, 0, 0);
    let mut jumps = 0;
    while (prev as usize) < n-1 {
        println!("This is ind {}, farthest: {}, prev: {}, jumps: {}", ind, farthest, prev, jumps);
        farthest = std::cmp::max(farthest, ind + nums[ind as usize]);
        if ind == prev {
            prev = farthest;
            jumps += 1;
            println!("Update: prev = farthest: {}", prev);
        }
        ind += 1;
    }
    jumps
}

/// Recursively generate all permutations of a given vector of integers. This function uses a VecDeque to efficiently pop and push elements from both ends.
fn permute_inner(mut nums: VecDeque<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    if n == 1 {
        return vec![Vec::from(nums)]
    }
    let mut ans = Vec::new();
    for _ in 0..n {
        let temp = nums.pop_front().unwrap();
        let mut perms = permute_inner(nums.clone());

        for it in perms.iter_mut() {
            it.push(temp);
        }
        ans.append(&mut perms);
        nums.push_back(temp);
    }
    ans
}

/// This is the solution to LeetCode Problem 46. "Permutations". Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let nums = VecDeque::from(nums);
    permute_inner(nums)
}

/// Reuses the next_permutation function to generate all unique permutations of a given vector of integers (in LeetCode Problem 31. "Next Permutation").
fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut i = n-1;
    while i > 0 && nums[i-1] >= nums[i] {
        i -= 1;
    }

    if i == 0 {
        nums.reverse();
    } else if i == n-1 {
        nums.swap(n-1, n-2);
    } else {
        let mut j = n-1;
        while j>= i && nums[j] <= nums[i-1] {
            j -= 1;
        }
        nums.swap(j, i-1);
        let to_reverse = &mut nums[i..];
        to_reverse.reverse();
    }
}

/// This is the solution to LeetCode Problem 47. "Permutations II" (or 46. "Permutations") Given a collection of numbers that might contain duplicates, return all possible unique permutations in any order.
pub fn permute_2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut temp = nums.clone();
    next_permutation(&mut temp);
    let mut ans = Vec::new();
    ans.push(temp.clone());
    while temp != nums {
        next_permutation(&mut temp);
        println!("Temp is {:?} after permutation", temp);
        ans.push(temp.clone());
    }
    ans
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for j in 0..(n/2) {
        for i in j..(n-1-j) {
            let temp = matrix[j][i];
            matrix[j][i] = matrix[n-i-1][j];
            matrix[n-i-1][j] = matrix[n-j-1][n-i-1];
            matrix[n-j-1][n-i-1] = matrix[i][n-j-1];
            matrix[i][n-j-1] = temp;
        }
    }
}

#[cfg(test)]
mod test_modules {
    use super::*;

    #[test]
    fn test_count_and_say() {
        assert_eq!(count_and_say(5), String::from("111221"));
    }

    #[test]
    fn test_combination_sum() {
        let input = vec![2, 3, 5];
        let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(combination_sum(input, 8), expected)
    }

    #[test]
    fn test_combination_sum2() {
        let input = vec![2, 5, 2, 1, 2];
        let expected = vec![vec![2, 1, 2], vec![5]];
        assert_eq!(combination_sum2(input, 5), expected)
    }

    #[test]
    fn test_jump_2() {
        let v = vec![2, 3, 1, 2, 4, 2];
        assert_eq!(jump_2(v), 3);
    }

    #[test]
    fn test_next_permutation() {
        let mut v = vec![1, 3, 4, 2];
        next_permutation(&mut v);
        assert_eq!(v, vec![1, 4, 2, 3]);
    }

    #[test]
    fn test_permute() {
        let v = vec![0, 1];
        let expected = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(permute(v), expected);
    }

    #[test]
    fn notest_permute_2() {
        let v = vec![1, 1, 2];
        println!("{:?}", permute(v));
    }
}
