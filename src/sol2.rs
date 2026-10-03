#![allow(unused_imports)]

use std::collections::HashMap;

/// This is the solution to the leetcode problem 36."Valid Sudoku". Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be validated.
/// 
/// We use HashMaps to record the number of times a character appears in each row, column, and cell. If any character appears more than once in any row, column, or cell, return false. Otherwise, return true.
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut r: HashMap<char, u8> = HashMap::with_capacity(9);
    assert_eq!(board.len(), 9);
    // scanning rows
    for v in board.iter() {
        for ch in v.iter() {
            if *ch != '.' {
                let curr = r.entry(*ch).or_insert(0);
                *curr += 1;
                if *curr > 1u8 {
                    return false
                }
            }
        }
        r.clear();
    }
    //println!("1st pass");
    // scanning columns 
    for i in 0..9usize {
        for v in board.iter() {
            if v[i] != '.' {
                let curr = r.entry(v[i]).or_insert(0);
                *curr += 1;
                //println!("This is char {}, recorded {} times", v[i], curr);
                if *curr > 1u8 {
                    return false
                }
            }
        }
        r.clear();
    }
    //println!("2nd pass");
    // scanning cells
    let mut r2: HashMap<char, u8> = HashMap::new();
    let mut r3: HashMap<char, u8> = HashMap::new();
    for offsets in 0..3usize {
        let offsets = offsets * 3;
        for (ind, ((c1, c2), c3)) in board[offsets].iter().zip(board[offsets+1].iter()).zip(board[offsets+2].iter()).enumerate() {
            let temp = vec![c1, c2, c3];
            //println!("This is the char binding {}, {}, {}", *c1, *c2, *c3);
            if ind < 3 {
                for c in temp {
                    if *c != '.' {
                        let curr = r.entry(*c).or_insert(0);
                        *curr += 1;
                        if *curr > 1u8 {
                            return false
                        }
                    }
                }
            } else if ind < 6 {
                for c in temp {
                    if *c != '.' {
                        let curr = r2.entry(*c).or_insert(0);
                        *curr += 1;
                        if *curr > 1u8 {
                            return false
                        }
                    }
                }
            } else {
                for c in temp {
                    if *c != '.' {
                        let curr = r3.entry(*c).or_insert(0);
                        *curr += 1;
                        if *curr > 1u8 {
                            return false
                        }
                    }
                }
            }
        }
        r.clear();
        r2.clear();
        r3.clear();
    }
    //println!("3rd pass");
    true
}

/// This is the solution to the leetcode problem 37."Sudoku Solver". Write a program to solve a Sudoku puzzle by filling the empty cells. 
/// 
/// In fact, there are no efficient algorithms to solve it. We can only use the brute-force algorithm to solve it. 
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    fn is_safe(board: &mut Vec<Vec<char>>, row: usize, col:usize, value: char) -> bool {
        for i in 0..9usize {
            if board[row][i] == value {
                return false;
            }
            if board[i][col] == value {
                return false;
            }
        }
        let (row_cell, col_cell) = (row / 3 * 3, col / 3 * 3);
        for kouter in 0..3 {
            for kinner in 0..3 {
                if board[kouter+row_cell][kinner+col_cell] == value {
                    return false;
                }
            }
        }
        return true;
    }
    fn solve_sudoku_meta(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
        //println!("This is row {}, col {}", row, col);
        if row == 9 {
            return true;
        }
        let mut row_next = row;
        let mut col_next = col + 1;
        if col + 1 == 9 {
            col_next = 0;
            row_next += 1;
        }
        if board[row][col] != '.' {
            return solve_sudoku_meta(board, row_next, col_next);
        }
        for i in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            if is_safe(board, row, col, i) {
                board[row][col] = i;
                if solve_sudoku_meta(board, row_next, col_next) {
                    return true;
                }
                board[row][col] = '.';
            }
        }
        return false;
    }
    solve_sudoku_meta(board, 0, 0);
}

/// Another implementation of the Sudoku Solver. This implementation is less efficient than the previous one.
pub fn solve_sudoku_2(board: &mut Vec<Vec<char>>) {
    fn is_safe(board: &mut Vec<Vec<char>>, row: usize, col:usize, value: char) -> bool {
        for i in 0..9usize {
            if board[row][i] == value {
                return false;
            }
            if board[i][col] == value {
                return false;
            }
        }
        let (row_cell, col_cell) = (row / 3 * 3, col / 3 * 3);
        for kouter in 0..3 {
            for kinner in 0..3 {
                if board[kouter+row_cell][kinner+col_cell] == value {
                    return false;
                }
            }
        }
        return true;
    }
    fn solve_sudoku_meta(board: &mut Vec<Vec<char>>) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                //println!("This is row {}, col {}", r, c);
                if board[r][c] == '.' {
                    for ch in String::from("123456789").chars() {
                        if is_safe(board, r, c, ch) {
                            board[r][c] = ch;
                            if solve_sudoku_meta(board) {
                                return true;
                            }
                            board[r][c] = '.'
                        }
                    }
                    return false;
                }
            }
        }
        return true;
    }
    solve_sudoku_meta(board);
}

#[cfg(test)]

mod test_modules {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        let b = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        is_valid_sudoku(b);
    }

    #[test]
    fn test_is_valid_sudoku_2() {
        let b = vec![
            vec!['.','.','4','.','.','.','6','3','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['5','.','.','.','.','.','.','9','.'],
            vec!['.','.','.','5','6','.','.','.','.'],
            vec!['4','.','3','.','.','.','.','.','1'],
            vec!['.','.','.','7','.','.','.','.','.'],
            vec!['.','.','.','5','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.']
        ];
        is_valid_sudoku(b);
    }

    #[test]
    fn test_sudoku_solver() {
        let mut puzzle = vec![
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','9','.','.','1','.','.','3','.'],
            vec!['.','.','6','.','2','.','7','.','.'],
            vec!['.','.','.','3','.','4','.','.','.'],
            vec!['2','1','.','.','.','.','.','9','8'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','2','5','.','6','4','.','.'],
            vec!['.','8','.','.','.','.','.','1','.'],
            vec!['.','.','.','.','.','.','.','.','.']
        ];
        solve_sudoku_2(&mut puzzle);
    }
}