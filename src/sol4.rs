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

#[cfg(test)]
mod test_modules {
    use super::*;

    #[test]
    fn test_can_jump() {
        let vect = vec![2,3,1,1,4];
        assert_eq!(can_jump(vect), true);
    }
}