use crate::collections::ListNode;

/// This is the solution to LeetCode Problem 54. "Spiral Matrix".
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    let m = matrix[0].len();
    let stopflag = m * n;
    let (mut pl, mut pr, mut offset) = (0, 0, 0);
    let mut result = Vec::new();
    loop {
        for _ in 0..(m-1-offset) {
            result.push(matrix[pl][pr]);
            pr += 1;
        }
        for _ in 0..(n-1-offset) {
            result.push(matrix[pl][pr]);
            pl += 1;
        }
        for _ in 0..(m-1-offset) {
            result.push(matrix[pl][pr]);
            pr -= 1;
        }
        for _ in 0..(n-1-offset) {
            result.push(matrix[pl][pr]);
            pl -= 1;
        }

        if result.len() >= stopflag {
            return Vec::from(&result[0..stopflag]);
        }

        if m == n && n % 2 != 0 && result.len() == stopflag - 1 {
            result.push(matrix[m/2][n/2]);
            return result;
        }
        pr += 1;
        pl += 1;
        offset += 2;
    }
}

/// This is the solution to LeetCode Problem 55. "Jump Game". This time, you need to justify whether you can reach the last index. 
/// 
/// The approach is to keep track of the farthest index that can be reached, and if at any point the current index is greater than the farthest index, return false. If the farthest index reaches or exceeds the last index, return true.
pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut farthest = 0;
    for i in 0..n {
        farthest = std::cmp::max(farthest, nums[i] + (i as i32));
        //println!("This is [{i}]={}, Update: farthest is at {farthest}", nums[i]);
        if farthest >= (n as i32)-1 {
            return true;
        }
        if i == farthest as usize {
            return false;
        }
    }
    return false
}

/// This is the solution to LeetCode Problem 56. "Merge Intervals". The approach is to sort the intervals by their start time, and then iterate through the sorted intervals, merging them if they overlap.
/// 
/// - Time complexity: O(n log n) due to sorting the intervals.
/// - Space complexity: O(n) for the output list of merged intervals.
pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut bigcup: Vec<Vec<i32>> = Vec::new();
    for i in intervals.into_iter() {
        if bigcup.is_empty() || i[0] > bigcup[bigcup.len()-1][1] {
            bigcup.push(i);
        } else {
            let n = bigcup.len()-1;
            bigcup[n][1] = std::cmp::max(bigcup[n][1], i[1]);
        }
    }
    bigcup
}

/// This is the solution to LeetCode Problem 57. "Insert Interval". 
pub fn insert_1(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut bigcup = Vec::new();
    if intervals.len() == 0 {
        bigcup.push(new_interval);
        return bigcup;
    }
    let n = intervals.len();
    if intervals[n-1][0] <= new_interval[0] {
        if intervals[n-1][1] < new_interval[0] {
            intervals.push(new_interval);
            return intervals;
        } else if intervals[n-1][1] > new_interval[1] {
            return intervals;
        } else {
            intervals[n-1][1] = new_interval[1];
            return intervals;
        }
    }

    for it in intervals.into_iter() {
        if new_interval[0] < it[0] {
            if bigcup.is_empty() || bigcup[bigcup.len()-1][1] < new_interval[0] {
                bigcup.push(new_interval.clone());
            } else {
                let n_temp = bigcup.len();
                bigcup[n_temp-1][1] = std::cmp::max(new_interval[1], bigcup[n_temp-1][1])
            }
        }
        if bigcup.is_empty() || bigcup[bigcup.len()-1][1] < it[0] {
            bigcup.push(it);
        } else {
            let n_temp = bigcup.len();
            bigcup[n_temp-1][1] = std::cmp::max(it[1], bigcup[n_temp-1][1])
        }
    }
    bigcup
}

/// This is another solution to LeetCode Problem 57. "Insert Interval". 
/// 
/// This approach improves logic and readability by separating the intervals into three parts: 
/// 
/// 1. those that come before the new interval, 
/// 2. those that overlap with it, and 
/// 3. those that come after it. 
/// 
/// It merges the overlapping intervals with the new interval and returns the combined result.
pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let n = intervals.len();
    if n == 0 {
        return vec![new_interval];
    }
    let mut bigcup = Vec::new();
    let mut i = 0usize;
    while i < n && intervals[i][1] < new_interval[0] {
        bigcup.push(intervals[i].clone());
        i += 1;
    }
    while i < n && intervals[i][0] <= new_interval[1] {
        new_interval[0] = std::cmp::min(new_interval[0], intervals[i][0]);
        new_interval[1] = std::cmp::max(new_interval[1], intervals[i][1]);
        i += 1;
    }
    bigcup.push(new_interval);
    bigcup.append(&mut Vec::from(&intervals[i..]));
    bigcup
}

/// This is the solution to LeetCode Problem 58. "Length of Last Word".
pub fn length_of_last_word(s: String) -> i32 {
    let mut cnt = 0;
    for c in s.chars().rev() {
        if cnt == 0 && c == ' ' {
            continue;
        }
        if cnt != 0 && c == ' ' {
            break;
        }
        cnt += 1;
    }
    cnt
}

/// This is the solution to LeetCode Problem 59. "Spiral Matrix II". 
/// The question requires generating an n x n matrix filled with elements from 1 to n^2 in spiral order.
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut protoans = Vec::new();
    for _ in 0..n {
        protoans.push(vec![0i32; n]);
    }
    let mut curr = 0i32;
    let mut offset = 0usize;
    loop {
        for i in offset..(n-1-offset) {
            curr += 1;
            protoans[offset][i] = curr;
        }
        for j in offset..(n-1-offset) {
            curr += 1;
            protoans[j][n-offset-1] = curr;
        }
        for i in offset..(n-1-offset) {
            curr += 1;
            protoans[n-offset-1][n-i-1] = curr;
        }
        for j in offset..(n-1-offset) {
            curr += 1;
            protoans[n-j-1][offset] = curr;
        }
        offset += 1;

        if curr == (n * n) as i32 {
            break;
        } else if curr == (n * n - 1) as i32 {
            curr += 1;
            protoans[n/2][n/2] = curr;
            break;
        }
    }
    protoans
}

/// This is the solution to LeetCode Problem 60. "Permutation Sequence". 
pub fn get_permutation(n: i32, k: i32) -> String {
    let mut n_fact = 1;
    let mut candid = Vec::with_capacity(n as usize);
    for i in 0..n {
        n_fact *= i+1;
        candid.push((i+1) as u8);
    }
    let mut ans = String::new();
    let mut ind = n;
    let mut start = k - 1;
    while ind > 0 {
        n_fact /= ind;
        let q = start / n_fact;
        let temp = candid.remove(q as usize);
        let temp_char = (temp + '0' as u8) as char;
        ans.push(temp_char);
        ind -= 1;
        start = start % n_fact;
    }
    ans
}

/// This is the solution to LeetCode Problem 62. "Unique Paths". Dynamic Programming is used.
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut dp = Vec::with_capacity(m);
    for _ in 0..m {
        if dp.is_empty() {
            dp.push(vec![1; n]);
        }
        let mut temp = vec![i32::MAX; n];
        temp[0] = 1;
        dp.push(temp.clone());
    }
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i-1][j] + dp[i][j-1]
        }
    }
    dp[m-1][n-1]
}

/// This is the solution to LeetCode Problem 63. "Unique Paths II". The difference is that some cells are marked as obstacles, and you cannot pass through them. 
/// 
///  Dynamic Programming is used. The function fills in the 2D DP table from [0][0] to [m-1][n-1]. When meeting an obstacle, the number falls back to 0 and continue.
pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    if obstacle_grid[0][0] == 1 || obstacle_grid[m-1][n-1] == 1 {
        return 0;
    }
    for it in obstacle_grid.iter_mut() {
        it.reverse();
    }
    obstacle_grid.reverse();
    let mut dp = Vec::with_capacity(m);
    for _ in 0..m {
        dp.push(vec![i32::MIN; n])
    }
    println!("{:?}", dp);
    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                dp[i][j] = 1
            } else if i > 0 && j == 0 {
                dp[i][j] = dp[i-1][j];
            } else if i == 0 && j > 0 {
                dp[i][j] = dp[i][j-1];
            } else {
                dp[i][j] = dp[i][j-1] + dp[i-1][j]
            }

            if obstacle_grid[i][j] == 1 {
                dp[i][j] = 0;
            }
        }
    }
    println!("{:?}", dp);
    dp[m-1][n-1]
}

/// This is the solution to LeetCode Problem 64. "Minimum Path Sum". The function calculates the minimum path sum from the top-left corner to the bottom-right corner of a grid filled with non-negative numbers. You can only move either down or right at any point in time.
/// 
/// The function fills in the 2D DP table from [m-1][n-1] to [0][0]. In this problem, we directly modify the original grid to save space.
pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {}
            else if i == 0 && j > 0 {
                grid[m-1-i][n-1-j] += grid[m-1-i][n-j];
            } else if i > 0 && j == 0 {
                grid[m-1-i][n-1-j] += grid[m-i][n-j-1];
            } else {
                grid[m-1-i][n-1-j] += std::cmp::min(grid[m-i][n-1-j], grid[m-1-i][n-j]);
            }
        }
    }
    grid[0][0]
}

/// This is the older implementation of the solution to LeetCode Problem 64. "Minimum Path Sum". 
/// 
/// This solution fills in the 2D DP table from [0][0] to [m-1][n-1] (in-place). Thus, we first reverse the grid recursively.
pub fn min_path_sum_2(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    for it in grid.iter_mut() {
        it.reverse();
    }
    grid.reverse();
    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {}
            else if i == 0 && j > 0 {
                grid[i][j] += grid[i][j-1];
            } else if i > 0 && j == 0 {
                grid[i][j] += grid[i-1][j];
            } else {
                grid[i][j] += std::cmp::min(grid[i-1][j], grid[i][j-1]);
            }
        }
    }
    //println!("Grid is\n{:?}", grid);
    grid[m-1][n-1]
}

/// This is the solution to LeetCode Problem 65. "Valid Number". The function checks if a given string can be interpreted as a valid decimal number.
pub fn is_number(s: String) -> bool {
    // 1. the leading pattern 
    let mut cs = s.chars();
    if let Some(ch) = cs.next() {
        if ch.is_ascii_digit() {} // If the first char is a digit, we are good to go
        else if ch == '.' {
            if let Some(nch) = cs.next() { // If the first char is a dot, the next char must be a digit
                if !nch.is_ascii_digit() {
                    return false;
                }
            } else { // When no next char following the dot, it is not a valid number
                return false;
            }
        }
        else if ch == '+' || ch == '-' {
            if let Some(nch) = cs.next() {
                if !nch.is_ascii_digit() && nch != '.' { // Signs must be followed by a digit or a dot
                    return false;
                } else if nch == '.' { // The dot pattern as above (must be followed by a digit)
                    if let Some(nnch) = cs.next() {
                        if !nnch.is_ascii_digit() {
                            return false;
                        } else {}
                    } else {
                        return false;
                    }
                }
            } else { // Only a sign without any following char is not a valid number
                return false;
            }
        } else { // If the first char is neither a digit, nor a dot, nor a sign, it is not a valid number
            return false;
        }
    }
    // 2. special chars
    let (mut dot_c, mut e_c, mut sign_c) = (0, 0, 0);
    let (mut dot_p, mut e_p, mut sign_p) = (usize::MAX, usize::MAX, usize::MAX);
    for (id, i) in s.char_indices() {
        if i == '.' {
            dot_c += 1;
            dot_p = id;
        } else if i.to_lowercase().to_string() == "e" {
            e_c += 1;
            e_p = id;
        } else if i.is_ascii_digit() {}
        else if i == '+' || i == '-' {
            sign_c += 1;
            sign_p = id;
        } else {
            return false; // Any other char is not allowed
        }

        if dot_c > 1 || e_c > 1 || sign_c > 2 { // #dots at most 1, #exps at most 1, #signs at most 2 (leading and after exp)
            return false;
        }
    }
    if e_c == 0 && sign_c > 1 { // when no exps, #signs at most 1
        return false;
    }
    // 3. positions
    let n = s.len();
    if e_p == 0 || e_p == n-1 { // exp is not allowed to be leading or tailing
        return false;
    }
    if e_p != usize::MAX && dot_p != usize::MAX && e_p < dot_p { // dot is not allowed to be after exp
        return false;
    }
    if sign_p != usize::MAX { // signs ought to be put next back to exp or leading
        if sign_p != 0 {
            if e_p != usize::MAX && sign_p == e_p + 1 {} else {
                return false;
            }
        }
        if sign_p == n-1 { // even if signs are properly put, it cannot be the last char
            return false;
        }
    }
    return true; // If all the above checks are passed, it is a valid number
}

/// This is the solution to LeetCode Problem 66. "Plus One". The goal is to add one to a number represented as an array of digits. 
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let n = digits.len();
    let mut ind = 0;
    for (id, d) in digits.iter_mut().enumerate().rev() {
        if id == n-1 {
            *d += 1
        }
        *d += ind;
        if *d == 10 {
            *d = 0;
            ind = 1
        } else {
            ind = 0
        }
    }
    if ind == 1 {
        let mut temp = vec![1];
        temp.append(&mut digits);
        return temp;
    }
    digits
}

/// This is the solution to LeetCode Problem 61. "Rotate List". The function rotates a linked list to the right by k places, where k is non-negative.
/// 
/// Since **cyclic listnodes are not allowed in Rust** (due to ownership and borrowing rules), we need to carefully manipulate the list nodes to achieve the rotation without creating cycles.
pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut n = 0;
    let mut curr = head.as_ref();
    while let Some(node) = curr {
        n += 1;
        curr = node.next.as_ref();
    }
    if n == 0 {
        return head;
    }
    let r = k % n;
    if r == 0 {
        return head;
    }
    let mut dummy = Box::new(ListNode {
        val: i32::MAX,
        next: None,
    });
    let mut tail = &mut dummy;
    let mut curr2 = head.as_mut();
    for _ in 0..(n-r-1) {
        //println!("This is {i} iter");
        curr2 = curr2.unwrap().next.as_mut();
    }
    //println!("Val: {}", curr2.as_ref().unwrap().val);
    let temp = curr2.unwrap().next.take();
    tail.next = temp;
    for _ in 0..r {
        //println!("This is {i} iter resumed");
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = head;
    dummy.next
}

#[cfg(test)]
mod test_modules {
    use super::*;

    #[test]
    fn test_can_jump() {
        let vect = vec![2,3,1,1,4];
        assert_eq!(can_jump(vect), true);
    }

    #[test]
    fn notest_generate_spiral_matrix() {
        println!("{:?}", generate_matrix(3))
    }

    #[test]
    fn notest_get_permutation() {
        println!("{}", get_permutation(6, 444))
    }

    #[test]
    fn test_unique_paths() {
        assert_eq!(28, unique_paths(3, 7))
    }

    #[test]
    fn test_unique_paths_with_obstacles() {
        let obs = vec![vec![0,1],vec![0,0]];
        assert_eq!(unique_paths_with_obstacles(obs), 1);
    }

    #[test]
    fn test_minimum_path_sum() {
        let board = vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]];
        assert_eq!(min_path_sum(board), 7);
    }

    #[test]
    fn test_plus_one() {
        let d = vec![4,3,9,9];
        let expected = vec![4,4,0,0];
        assert_eq!(plus_one(d), expected);
    }

    #[test]
    fn test_rotate_right() {
        let hd = ListNode::from_vec(&[1,2,3,4,5]);
        println!("{}", hd.as_ref().unwrap());
        println!("{}", rotate_right(hd, 2).unwrap());
    }
}

#[cfg(test)]
mod test_is_valid_number_dedicated {
    use super::is_number;

    #[test]
    fn test_1() {
        assert_eq!(is_number(String::from("-8.74e-7845")), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(is_number(String::from("-8.74-7845")), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(is_number(String::from("-8.74-e7845")), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(is_number(String::from("-e-7845")), false);
    }

    #[test]
    fn test_5() {
        assert_eq!(is_number(String::from("-8.74e-7.845")), false);
    }

    #[test]
    fn test_6() {
        assert_eq!(is_number(String::from("-8.74e-")), false);
    }

    #[test]
    fn test_7() {
        assert_eq!(is_number(String::from("-8e+7")), true);
    }

    #[test]
    fn test_8() {
        assert_eq!(is_number(String::from("0x11")), false);
    }

    #[test]
    fn test_9() {
        assert_eq!(is_number(String::from("6+1")), false)
    }
}