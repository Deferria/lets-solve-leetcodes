use crate::collections::ListNode;

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
    for i in start..(mx + 1) {
        curr.push(i);
        combine_inner(i + 1, n, k - 1, curr, res);
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
fn subset_meta(
    subsets: &mut Vec<Vec<i32>>,
    nums: &Vec<i32>,
    ind: usize,
    current: &mut Vec<i32>,
    n_flag: usize,
) {
    if ind >= n_flag {
        subsets.push(current.to_vec());
        return;
    }
    current.push(nums[ind]);
    subset_meta(subsets, nums, ind + 1, current, n_flag);
    current.pop();
    subset_meta(subsets, nums, ind + 1, current, n_flag);
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
            if exist_backtracking(&mut board, &mut wv, i, j) {
                return true;
            }
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
    if p as usize > m - 1 || p < 0 || q as usize > n - 1 || q < 0 {
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
        if exist_backtracking(bd, wd, p - 1, q)
            || exist_backtracking(bd, wd, p, q - 1)
            || exist_backtracking(bd, wd, p, q + 1)
            || exist_backtracking(bd, wd, p + 1, q)
        {
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
        if nums[f] == nums[f - 1] {
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
        if target == nums[0] {
            return true;
        } else {
            return false;
        }
    }
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            m = i;
            break;
        }
    }
    let (mut p, mut q);
    if m == n {
        if target < nums[0] || target > nums[n - 1] {
            return false;
        } else if target == nums[0] || target == nums[n - 1] {
            return true;
        } else {
            (p, q) = (0, n - 1);
        }
    } else {
        if target < nums[m] {
            return false;
        } else if target == nums[m] {
            return true;
        } else if target < nums[n - 1] {
            (p, q) = (m, n - 1);
        } else if target == nums[n - 1] {
            return true;
        } else if target < nums[0] {
            return false;
        } else if target == nums[0] {
            return true;
        } else if target < nums[m - 1] {
            (p, q) = (0, m - 1);
        } else if target == nums[m - 1] {
            return true;
        } else {
            return false;
        }
    }
    while p <= q - 1 {
        let mid = (p + q) / 2;
        if target > nums[mid] {
            p = mid + 1;
        } else if target < nums[mid] {
            q = mid - 1;
            if q == 0 {
                return false;
            }
        } else {
            return true;
        }
    }
    false
}

/// This is the solution to the leetcode problem 82. Remove Duplicates from Sorted List II. Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.
/// 
/// In this implementation, to avoid borrowing and ownership issues, instead of modifying the original list, we read the values and directly construct a new listnode with according values.
/// 
/// - Time complexity: O(n)
/// - Space complexity: O(n)
pub fn delete_duplicates_1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(i32::MAX));
    let mut tl = &mut dummy;
    let (mut cval, mut cnt) = (i32::MAX, 0);
    let mut curr = head.as_ref();
    while let Some(node) = curr {
        let temp = node.val;
        if temp == cval {
            cnt += 1;
        } else {
            if cnt == 1 {
                tl.next = Some(Box::new(ListNode::new(cval)));
                tl = tl.next.as_mut().unwrap();
            }
            cval = temp;
            cnt = 1;
        }
        curr = node.next.as_ref();
    }
    if cnt == 1 {
        tl.next = Some(Box::new(ListNode::new(cval)));
    }
    dummy.next
}

/// This is the solution to the leetcode problem 82. Remove Duplicates from Sorted List II. Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.
/// 
/// In this implementation, we modify the original list in-place. We use a dummy node to simplify the edge cases and a pointer `tl` to track the tail of the new list. We check for duplicates by comparing the current node's value with the next node's value. 
/// If duplicates are found, we skip all nodes with that value. Otherwise, we move the tail pointer forward. In this step, `take()` method is used to take ownership of the next node, allowing us to modify the list without hatching borrowing problems.
/// 
/// - Time complexity: O(n)
/// - Space complexity: O(1)
pub fn delete_duplicates_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode {
        val: i32::MAX,
        next: head,
    }));
    let mut tl = &mut dummy;
    while let Some(node) = tl {
        let dup_flag = node
            .next
            .as_ref()
            .and_then(|n1| n1.next.as_ref().map(|n2| n1.val == n2.val))
            .unwrap_or(false);

        if dup_flag {
            let vl = node.next.as_ref().unwrap().val;
            let mut nxt = node.next.take();
            while let Some(m) = nxt.as_ref() {
                if m.val == vl {
                    nxt = nxt.as_mut().unwrap().next.take();
                } else {
                    node.next = nxt.take();
                }
            }
        } else {
            tl = &mut tl.as_mut().unwrap().next
        }
    }
    dummy.unwrap().next
}

/// This is the solution to the leetcode problem 82. Remove Duplicates from Sorted List II. 
/// 
/// Rather than using a pointer at the tail and use `take()` conditionally, we directly pick the next node off the listnode and check duplication. If not, we insert it back; otherwise, we discard amd move on. 
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: i32::MAX, next: head });
    let mut prev = &mut dummy;
    while let Some(mut curr) = prev.next.take() {
        let mut dup_flag = false;
        while let Some(nxt) = &mut curr.next && curr.val == nxt.val {
            curr.next = nxt.next.take();
            dup_flag = true;
        }
        if dup_flag {
            prev.next = curr.next
        } else {
            prev = prev.next.insert(curr);
        }
    }
    dummy.next
}

/// This is the solution to the leetcode problem 82. Remove Duplicates from Sorted List II.
/// 
/// This is a valid implementation without using a dummy node.
pub fn delete_duplicates_3(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut hd, mut rt) = (head, None);
    let mut tl = &mut rt;
    let mut compr = i32::MAX;
    
    while let Some(mut node) = hd {
        hd = node.next.take();
        if node.val == compr { continue; }
        if hd.is_some() && node.val == hd.as_ref().unwrap().val {
            compr = node.val;
            continue;
        }
        tl = &mut tl.insert(node).next;
        compr = i32::MAX;
    }
    rt
}

/// This is the solution to the leetcode problem 82. Remove Duplicates from Sorted List II.
/// 
/// This is a valid implementation using unsafe Rust code.
pub fn delete_duplicates_4(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: i32::MAX, next: head });
    unsafe {
        let mut curr = &mut *dummy as *mut ListNode;
        while let Some(node) = (*curr).next.as_mut() {
            if let Some(nxt) = node.next.as_ref() {
                println!("This is val {}", node.val);
                if node.val == nxt.val {
                    let frozen = node.val;
                    let nxt_ptr = &mut (*curr).next;
                    while let Some(n) = nxt_ptr {
                        if n.val == frozen {
                            *nxt_ptr = n.next.take();
                        } else {
                            break;
                        }
                    }
                    continue;
                }
            }
            curr = (*curr).next.as_deref_mut().unwrap() as *mut ListNode;
        }
    }
    dummy.next
}

/// This is the solution to leetcode problem 84. Largest Rectangle in Histogram. 
/// 
/// Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.
pub fn largest_rectangle_area_1(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut indl: Vec<isize> = vec![-1; n];
    let mut stack: Vec<isize> = Vec::new();
    // This is to find the first (strict) left-smaller element's index
    for i in 0..n {
        while let Some(val) = stack.last() && heights[*val as usize] >= heights[i] {
            stack.pop();
        }
        indl[i] = *stack.last().unwrap_or(&-1) as isize;
        stack.push(i as isize);
    }
    stack.clear();
    // This is to find the first (strict) right-smaller element's index
    let mut indr = vec![1; n];
    for i in 0..n {
        let j = n-1-i;
        while let Some(val) = stack.last() && heights[*val as usize] >= heights[j] {
            stack.pop();
        }
        indr[j] = *stack.last().unwrap_or(&(n as isize)) as isize;
        stack.push(j as isize);
    }
    let mut max_area = 0;
    for i in 0..n {
        let temp = heights[i] * (indr[i] - indl[i] - 1) as i32;
        max_area = std::cmp::max(max_area, temp);
    }
    max_area
}

/// This is the solution to leetcode problem 84. Largest Rectangle in Histogram.
/// 
/// An optimization to the version above. Without using two vectors to record the position of the first left-smaller and right-smaller elements, we can use a single stack to keep track of the indices of the bars. 
/// When we encounter a bar that is shorter than the bar at the index stored at the top of the stack, we pop from the stack and calculate the area of the rectangle that can be formed with the popped bar as the shortest bar. 
/// We continue this process until we have processed all bars in the histogram.
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut stack = Vec::new();
    let mut area = 0i32;

    for (i, &h) in heights.iter().enumerate() {
        let mut start = i;
        while let Some(&(hindex, hnew)) = stack.last() {
            if h < hnew {
                stack.pop();
                let temp = (i - hindex) as i32 * hnew;
                area = area.max(temp);
                start = hindex;
            } else {
                break;
            }
        }
        stack.push((start, h));
    }
    while let Some((hindex, h)) = stack.pop() {
        let temp = (n-hindex) as i32 * h;
        area = area.max(temp)
    }
    area
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
        let nums = vec![1, 2, 3];
        let answer = subsets(nums);
        println!("{:?}", answer);
    }

    #[test]
    fn test_word_search() {
        let board_origin = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ];
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
    fn test_word_search_2() {
        let board_origin = [['A'], ['B'], ['C'], ['E']];
        let mut board = Vec::with_capacity(board_origin.len());
        for it in board_origin.into_iter() {
            board.push(it.to_vec());
        }
        exist(board, "ABCE".into());
        //assert_eq!(, true);
    }

    #[test]
    fn test_word_search_3() {
        let board_origin = [['b'], ['a'], ['b'], ['b'], ['a']];
        let mut board = Vec::with_capacity(board_origin.len());
        for it in board_origin.into_iter() {
            board.push(it.to_vec());
        }
        assert_eq!(exist(board, "baa".into()), false);
    }

    #[test]
    fn test_remove_duplicate() {
        let mut v = vec![1, 2, 3, 4, 4, 4, 4, 4, 5, 5];
        let r = remove_duplicates(&mut v);
        assert_eq!(r, 7);
        let _v2 = v.split_off(7);
        assert_eq!(v, vec![1, 2, 3, 4, 4, 5, 5]);
    }

    #[test]
    fn test_search_in_rotated() {
        let v = vec![2, 5, 6, 0, 0, 1, 2];
        assert_eq!(search(v, 1), true)
    }

    #[test]
    fn test_search_in_rotated_2() {
        let v = vec![2];
        assert_eq!(search(v, 2), true)
    }

    #[test]
    fn test_delete_duplicates() {
        let lstnode = ListNode::from_vec(&[1, 1, 1, 2, 2, 2, 3, 4, 5, 5, 5, 6, 6, 6, 7]);
        let newnode = delete_duplicates_4(lstnode);
        println!("{}", newnode.unwrap());
    }

    #[test]
    fn test_delete_duplicates_2() {
        let lstnode = ListNode::from_vec(&[]);
        let newnode = delete_duplicates_1(lstnode);
        println!("{}", newnode.is_none());
    }

    #[test]
    fn test_largest_rectangle_area() {
        let v = vec![2,4,2,6,4,2,1,7,6,3,2,1,2,1,3];
        largest_rectangle_area(v);
    }
}
