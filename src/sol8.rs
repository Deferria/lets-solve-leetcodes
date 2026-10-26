#![warn(deprecated)]

#[deprecated]
/// This is the solution to leetcode problem 115. Distinct Subsequences. It is a recursive solution that uses backtracking to find all distinct subsequences of `s` that equal `t`. It is not efficient for large inputs and will time out.
/// 
/// This solution is too slow and will time out for large inputs.
pub fn num_distinct_tle(s: String, t: String) -> i32 {
    if s.len() < t.len() {
        return 0;
    }
    let mut v: Vec<char> = t.chars().rev().collect();
    let sv = s.chars().collect();
    num_distinct_inner(&sv, 0, &mut v, 0)
}

fn num_distinct_inner(s: &Vec<char>, i: usize, curr: &mut Vec<char>, acc: i32) -> i32 {
    if curr.is_empty() {
        return acc + 1;
    }
    if curr.len() > s.len() - i || i >= s.len() {
        return acc;
    }
    if let Some(ch) = curr.pop() {
        let mut temp = 0;
        if s[i] == ch {
            //println!("matched: {} with {}", s[i], ch);
            temp += num_distinct_inner(s, i+1, curr, acc);
            curr.push(ch);
            temp += num_distinct_inner(s, i+1, curr, acc);
            return temp;
        } else {
            curr.push(ch);
            return num_distinct_inner(s, i+1, curr, acc);
        }
    } else {
        unreachable!();
    }
}

/// This is the solution to leetcode problem 115. Distinct Subsequences. It uses dynamic programming to find all distinct subsequences of `s` that equal `t`. 
/// 
/// - Time complexity: O(m*n) where m is the length of `s` and n is the length of `t`.
/// - Space complexity: O(m*n).
pub fn num_distinct_1(s: String, t: String) -> i32 {
    if s.len() < t.len() {
        return 0;
    }
    let t_v: Vec<char> = t.chars().rev().collect();
    let s_v: Vec<char> = s.chars().rev().collect();
    let mut dp = vec![vec![0; s.len()+1]; t.len()+1];
    for i in 0..=s.len() {
        dp[0][i] = 1;
    }
    for i in 1..=t.len() {
        for j in 1..=s.len() {
            if s_v[j-1] == t_v[i-1] {
                dp[i][j] = dp[i-1][j-1] + dp[i][j-1];
            } else {
                dp[i][j] = dp[i][j-1];
            }
        }
    }
    //println!("{dp:?}");
    dp[t.len()][s.len()]
}

/// This is the solution to leetcode problem 115. Distinct Subsequences. It uses dynamic programming, however, it uses a 1D array to store the number of distinct subsequences of `s` that equal `t`.
/// 
/// - Time complexity: O(m*n) where m is the length of `s` and n is the length of `t`.
/// - Space complexity: O(n).
pub fn num_distinct(s: String, t: String) -> i32 {
    let (m, n) = (s.len(), t.len());
    if m < n {
        return 0;
    }
    let (s_b, t_b) = (s.as_bytes(), t.as_bytes());
    let mut dp = vec![0; n+1];
    dp[n] = 1;
    for i in (0..m).rev() {
        for j in 0..n {
            if s_b[i] == t_b[j] {
                dp[j] += dp[j+1];
            }
        }
        //println!("i={i}, with dp = {dp:?}");
    }
    dp[0]
}

/// This is the solution to leetcode problem 118. Pascal's Triangle. It generates the first `num_rows` of Pascal's triangle.
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    ans.push(vec![1]);
    if num_rows == 1 {
        return ans;
    } 
    ans.push(vec![1,1]);
    if num_rows == 2 {
        return ans;
    }
    for i in 2..num_rows as usize {
        let mut row = vec![1; i + 1];
        let lst = ans.last().unwrap();
        for j in 0..lst.len()-1 {
            row[j+1] = lst[j] + lst[j+1];
        }
        ans.push(row);
    }
    ans
}

/// This is the solution to leetcode problem 119. Pascal's Triangle II. It generates the `row_index`th row of Pascal's triangle.
pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut ans = vec![1; row_index as usize + 1];
    if row_index < 2 {
        return ans;
    }
    let mut temp = 1u64;
    for i in 1..=row_index as u64 {
        temp *= (row_index as u64) - i + 1;
        temp /= i;
        ans[i as usize] = temp as i32;
    }
    ans
}

#[cfg(test)]
mod test_module {
    use super::*;

    #[test]
    fn test_num_distinct() {
        let n = num_distinct("rabbbit".into(), "rabbit".into());
        assert_eq!(3, n);
    }

    #[test]
    fn test_num_distinct_2() {
        let n = num_distinct("babgbag".into(), "bag".into());
        assert_eq!(n, 5);
    }

    #[test]
    fn test_num_distinct_3() {
        assert_eq!(num_distinct("rat".into(), "rat".into()), 1);
    }

    #[test]
    fn test_num_distinct_4() {
        assert_eq!(num_distinct("in".into(), "r".into()), 0);
    }

    #[test]
    fn test_num_distinct_5() {
        let s = String::from("adbdadeecadeadeccaeaabdabdbcdabddddabcaaadbabaaedeeddeaeebcdeabcaaaeeaeeabcddcebddebeebedaecccbdcbcedbdaeaedcdebeecdaaedaacadbdccabddaddacdddc");
        let t = String::from("bcddceeeebecbc");
        println!("{}", num_distinct(s, t));
    }

    #[test]
    fn test_pascal_triangle_generate() {
        println!("{:?}", generate(3));
    }

    #[test]
    fn test_pascal_triangle_get_row() {
        println!("{:?}", get_row(30));
    }
}