/// This is the inner recursion function for the leetcode problem 77. Combinations to combine numbers from `start` to `n` with `k` numbers. `curr` and `res` are used to execute side-effects.
fn combine_inner(start: i32, n: i32, k: i32, curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if k <= 0 {
        res.push(curr.to_vec());
        return;
    }
    let mx = n - k + 1;
    if start > mx {
        return;
    }
    for i in start..(mx+1) {
        curr.push(i);
        combine_inner(i+1, n, k-1, curr, res);
        curr.pop();
    }
}

/// This is the solution to the leetcode problem 77. Combinations.
/// 
/// Here we use the backtracking approach to generate all the combinations of k numbers out of [1, n]. The stop condition is when k==0 and we cut the recursion when the start number is greater than n-k+1. 
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut temp: Vec<i32> = Vec::new();
    let mut answer = Vec::new();
    combine_inner(1, n, k, &mut temp, &mut answer);
    answer
}

/// This is the inner recursion function for the leetcode problem 78. Subsets to generate all the subsets of a given set of numbers. 
/// In each recursion call, we have two choices: either include the current number or not. We use `current` to store the current subset and `subsets` to store all the subsets. To ensure backtracking, the side-effect is canceled by pushing and popping the current number from `current`.
fn subset_meta(subsets: &mut Vec<Vec<i32>>, nums: &Vec<i32>, ind: usize, current: &mut Vec<i32>, n_flag: usize) {
    if ind >= n_flag {
        subsets.push(current.to_vec());
        return;
    }
    current.push(nums[ind]);
    subset_meta(subsets, nums, ind+1, current, n_flag);
    current.pop();
    subset_meta(subsets, nums, ind+1, current, n_flag);
}

/// This is the solution to the leetcode problem 78. Subsets.
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let mut temp = Vec::new();
    subset_meta(&mut ans, &nums, 0, &mut temp, nums.len());
    ans
}

/// This is the solution to the leetcode problem 79. Word Search: search for a given word in a 2D board of characters. The word can be constructed from letters of sequentially adjacent cells, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.
/// 
/// Backtracking is used to implement the search.
pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let mut wv: Vec<char> = word.chars().rev().collect();
    let (m, n) = (board.len() as isize, board[0].len() as isize);
    for i in 0..m {
        for j in 0..n {
            if exist_backtracking(&mut board, &mut wv, i, j) { return true; }
        }
    }
    false
}

/// This is the inner recursion function for the leetcode problem 79. Word Search. It finds whether the word exists in the board starting at position (p, q).
fn exist_backtracking(bd: &mut Vec<Vec<char>>, wd: &mut Vec<char>, p: isize, q: isize) -> bool {
    //assert_eq!(bd.is_empty(), false);
    //print!("This is to test {p}, {q}: words: {:?}", wd);
    let (m, n) = (bd.len(), bd[0].len());
    if wd.is_empty() {
        //println!("Triggered empty, return true.");
        return true;
    }
    // This is constraint checker
    if p as usize > m-1 || p < 0 || q as usize > n-1 || q < 0 { 
        //println!("Out of bound, return false.");
        return false; 
    } else if bd[p as usize][q as usize] == '\t' { 
        //println!("Have been searched, return false.");
        return false; 
    }
    let temp = bd[p as usize][q as usize];
    let x = *wd.last().unwrap();
    if temp == x {
        wd.pop();
        bd[p as usize][q as usize] = '\t';
        if exist_backtracking(bd, wd, p-1, q) || exist_backtracking(bd, wd, p, q-1) || exist_backtracking(bd, wd, p, q+1) || exist_backtracking(bd, wd, p+1, q) {
            //println!("Passed at {}, {}", p, q);
            return true;
        }
        bd[p as usize][q as usize] = temp;
        wd.push(x);
        //println!("All directions are impossible. Backtracking... {:?}", wd);
    }
    return false;
}

/// This is the solution to the leetcode problem 80. Remove Duplicates from Sorted Array II. Given a sorted array nums, remove the duplicates in-place such that duplicates appeared at most twice and return the new length. 
/// 
/// Constraints: **Do not allocate extra space** for another array, do this by modifying the input array **in-place** with O(1) extra memory.
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return 1;
    }
    let mut cnt = 1;
    let (mut f, mut s) = (1usize, 1usize);
    while f < n {
        assert_ne!(cnt, 0);
        if nums[f] == nums[f-1] {
            cnt += 1;
        } else {
            cnt = 1;
        }
        if cnt == 1 || cnt == 2 {
            nums[s] = nums[f];
            s += 1;
        }
        f += 1;
    }
    assert!(s <= n);
    s as i32
}

/// This is the solution to the leetcode problem 81. Search in Rotated Sorted Array II. 
pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut m = nums.len();
    let n = m;
    if n == 1 {
        if target == nums[0] { return true; } else { return false; }
    }
    for i in 1..nums.len() {
        if nums[i] < nums[i-1] {
            m = i;
            break;
        }
    }
    let (mut p, mut q);
    if m == n {
        if target < nums[0] || target > nums[n-1] { return false; } else if target == nums[0] || target == nums[n-1] { return true; } else {
            (p, q) = (0, n-1);
        }
    } else {
        if target < nums[m] { return false; } else if target == nums[m] { return true; } else if target < nums[n-1] {
        (p, q) = (m, n-1);
        } else if target == nums[n-1] { return true; } else if target < nums[0] { return false; } else if target == nums[0] { return true; } else if target < nums[m-1] {
            (p, q) = (0, m-1);
        } else if target == nums[m-1] { return true; } else { return false; }
    }
    while p <= q-1 {
        let mid = (p+q) / 2;
        if target > nums[mid] {
            p = mid+1;
        } else if target < nums[mid] {
            q = mid-1;
            if q == 0 {
                return false;
            }
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test_module {
    use super::*;

    #[test]
    fn notest_combine() {
        let answer = combine(6, 3);
        println!("{:?}", answer);
    }

    #[test]
    fn notest_subsets() {
        let nums = vec![1,2,3];
        let answer = subsets(nums);
        println!("{:?}", answer);
    }

    #[test]
    fn test_word_search () {
        let board_origin = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']];
        let mut board = Vec::with_capacity(board_origin.len());
        for it in board_origin.into_iter() {
            board.push(it.to_vec());
        }
        assert_eq!(exist(board, "ABCCED".into()), true);
        //assert_eq!(exist(board, "ABCCEE".into()), true);
        //assert_eq!(exist(board, "ABCCFS".into()), true);
        //assert_eq!(exist(board, "ASFCG".into()), false);
        //assert_eq!(exist(board, "FCS".into()), true);
    }

    #[test]
    fn test_word_search_2 () {
        let board_origin = [['A'],['B'],['C'],['E']];
        let mut board = Vec::with_capacity(board_origin.len());
        for it in board_origin.into_iter() {
            board.push(it.to_vec());
        }
        exist(board, "ABCE".into());
        //assert_eq!(, true);
    }

    #[test]
    fn test_word_search_3 () {
        let board_origin = [['b'],['a'],['b'],['b'],['a']];
        let mut board = Vec::with_capacity(board_origin.len());
        for it in board_origin.into_iter() {
            board.push(it.to_vec());
        }
        assert_eq!(exist(board, "baa".into()), false);
    }

    #[test]
    fn test_remove_duplicate() {
        let mut v = vec![1,2,3,4,4,4,4,4,5,5];
        let r = remove_duplicates(&mut v);
        assert_eq!(r, 7);
        let _v2 = v.split_off(7);
        assert_eq!(v, vec![1,2,3,4,4,5,5]);
    }

    #[test]
    fn test_search_in_rotated() {
        let v = vec![2,5,6,0,0,1,2];
        assert_eq!(search(v, 1), true)
    }

    #[test]
    fn test_search_in_rotated_2() {
        let v = vec![2];
        assert_eq!(search(v, 2), true)
    }
}