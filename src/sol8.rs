#![warn(deprecated)]
use std::collections::{ VecDeque, HashSet, HashMap };
use std::rc::Rc;

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

/// This is the solution to leetcode problem 120. Triangle. It finds the minimum path sum from top to bottom in a triangle.
pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    let n = triangle.len();
    for i in (0..n).rev() {
        if i == n-1 {
            continue;
        }
        for j in 0..=i {
            triangle[i][j] += triangle[i+1][j].min(triangle[i+1][j+1]);
        }
    }
    triangle[0][0]
}

/// This is the solution to leetcode problem 121. Best Time to Buy and Sell Stock.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut min_price = prices[0];
    let mut profit = 0;
    for i in 1..n {
        if prices[i] < min_price {
            min_price = prices[i]
        } else if prices[i] > min_price {
            profit = profit.max(prices[i] - min_price);
        }
    }
    profit
}

/// This is the solution to leetcode problem 122. Best Time to Buy and Sell Stock II.
pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut acc = 0;
    for i in 1..n {
        if prices[i-1] < prices[i] {
            acc += prices[i] - prices[i-1];
        }
    }
    acc
}

/// This is the solution to leetcode problem 123. Best Time to Buy and Sell Stock III.
pub fn max_profit_iii(prices: Vec<i32>) -> i32 {
    let (mut buy1, mut buy2) = (i32::MAX, i32::MAX);
    let (mut sell1, mut sell2) = (0, 0);
    for it in prices.iter() {
        buy1 = buy1.min(*it);
        sell1 = sell1.max(*it - buy1);
        buy2 = buy2.min(*it - sell1);
        sell2 = sell2.max(*it - buy2);
    }
    sell2
}

/// This is the (another) solution to leetcode problem 123. Best Time to Buy and Sell Stock III. If we replace 3 with other numbers, we can solve the problem of Best Time to Buy and Sell Stock with k transactions limited.
pub fn max_profit_iii_2(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut dp = vec![vec![0;n];3];
    for k in 1..3 {
        let mut min_price = prices[0];
        for i in 1..n {
            min_price = min_price.min(prices[i]-dp[k-1][i-1]);
            dp[k][i] = dp[k][i-1].max(prices[i]-min_price);
        }
    }
    //println!("{dp:?}");
    dp[2][n-1]
}

fn back_dfs(word: Rc<String>, target: Rc<String>, path: &mut Vec<String>, res: &mut Vec<Vec<String>>, query: &HashMap<Rc<String>, HashSet<Rc<String>>>) {
    if word == target{
        path.reverse();
        println!("received: {path:?}");
        res.push(path.to_vec());
        path.reverse();
        return;
    }
    let set = query.get(&word).unwrap();
    for it in set.iter() {
        println!("This is the prec {} of {}", it, word);
        path.push((**it).clone());
        back_dfs(it.clone(), target.clone(), path, res, query);
        path.pop();
    }
}

/// This is the solution to leetcode problem 126. Word Ladder II. It finds all shortest transformation sequences from `begin_word` to `end_word`, such that only one letter can be changed at a time and each transformed word must exist in the word list.
/// 
/// To avoid ownership issues, we use `Rc` pointer to store the words in data structures. 
pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let word_list_2 = word_list.clone();
    let begin_word_rc = Rc::new(begin_word);
    let mut word_set: HashSet<Rc<String>> = HashSet::new();
    for i in word_list_2.into_iter() {
        word_set.insert(Rc::new(i));
    }
    println!("{word_set:?}");
    if !word_set.contains(&end_word) {
        //println!("1");
        return Vec::new();
    }
    let mut dists = HashMap::new();
    dists.insert(begin_word_rc.clone(), 0);
    let mut precedors: HashMap<Rc<String>, HashSet<Rc<String>>> = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back(begin_word_rc.clone());
    if word_set.contains(&begin_word_rc) {
        word_set.remove(&begin_word_rc);
    }
    let mut found_flag = false;
    while !q.is_empty() && !found_flag {
        let mut visited: HashSet<Rc<String>> = HashSet::new();
        let n = q.len();
        for _ in 0..n {
            let curr = q.pop_front().unwrap().clone();
            let curr_dist = *dists.get(&curr).unwrap();
            println!("Pred_curr: {}", *curr);
            if *curr == end_word {
                found_flag = true;
                continue;
            }
            let mut v: Vec<char> = curr.chars().collect();
            let l = v.len();
            for i in 0..l {
                let temp_char = v[i];
                for c in String::from("qwertyuiopasdfghjklzxcvbnm").chars() {
                    if c == v[i] {
                        continue;
                    }
                    v[i] = c;
                    let testing: String = v.iter().collect();
                    v[i] = temp_char;
                    let testing_rc = Rc::new(testing);
                    println!("Pred_testing: {}", *testing_rc);
                    if *testing_rc == end_word {
                        found_flag = true;
                        precedors.entry(testing_rc.clone()).or_insert(HashSet::new());
                        let set = precedors.get_mut(&testing_rc).unwrap();
                        set.insert(curr.clone());
                    }
                    if word_set.contains(&testing_rc) {
                        println!("into this branch");
                        if !dists.contains_key(&testing_rc) {
                            dists.insert(testing_rc.clone(), curr_dist+1);
                            visited.insert(testing_rc.clone());
                            q.push_back(testing_rc.clone());
                            precedors.entry(testing_rc.clone()).or_insert(HashSet::new());
                            let set = precedors.get_mut(&testing_rc).unwrap();
                            set.insert(curr.clone());
                        } else {
                            if let Some(&val) = dists.get(&testing_rc) && val == curr_dist + 1 {
                                precedors.entry(testing_rc.clone()).or_insert(HashSet::new());
                                let set = precedors.get_mut(&testing_rc).unwrap();
                                set.insert(curr.clone());
                            }
                        }
                    }
                }
            }
        }
        word_set.retain(|c| !visited.contains(c));
    }
    if !found_flag {
        return Vec::new();
    }
    let mut ans: Vec<Vec<String>> = Vec::new();
    let mut path = vec![end_word.clone()];
    back_dfs(Rc::new(end_word), begin_word_rc, &mut path, &mut ans, &precedors);
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
    fn test_pascal_triangle_get_row_2() {
        assert_eq!(vec![1,3,3,1], get_row(3));
    }

    #[test]
    fn test_pascal_triangle_get_row() {
        println!("{:?}", get_row(30));
    }

    #[test]
    fn test_minimal_total() {
        let v_triangle = vec![vec![2], vec![3,4],vec![6,5,7],vec![4,1,8,3]];
        assert_eq!(11, minimum_total(v_triangle));
    }

    #[test]
    fn test_max_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(5, max_profit(prices));
    }

    #[test]
    fn test_max_profit_ii() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(7, max_profit_ii(prices));
    }

    #[test]
    fn test_max_profit_iii() {
        let prices = vec![3,3,5,0,0,3,1,4];
        assert_eq!(max_profit_iii_2(prices), 6);
    }

    #[test]
    fn test_max_profit_iii_2() {
        let prices = vec![7,6,4,3,1];
        assert_eq!(max_profit_iii(prices), 0);
    }

    #[test]
    fn test_max_profit_iii_3() {
        let prices = vec![1,2,3,4,5];
        assert_eq!(max_profit_iii(prices), 4);
    }

    #[test]
    fn test_max_profit_iii_4() {
        let prices = vec![1,3,4,6,2,3,2,4,0];
        assert_eq!(max_profit_iii(prices), 7);
    }

    #[test]
    fn test_max_profit_iii_5() {
        let prices = vec![1,2,4,2,5,7,2,4,9,0];
        assert_eq!(max_profit_iii_2(prices), 13);
    }

    #[test]
    fn test_word_ladder() {
        let wd_list_prev: Vec<String> = vec!["hot".into(),"dot".into(),"dog".into(),"lot".into(),"log".into(),"cog".into()].into_iter().collect();
        let res = find_ladders("hit".into(), "cog".into(), wd_list_prev);
        println!("{res:?}");
    }
}