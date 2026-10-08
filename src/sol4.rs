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
}