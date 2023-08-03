// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dy

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: String,
    }
    let mut A = A.chars().collect::<Vec<char>>();
    let mut que = VecDeque::new();
    que.push_back(X-1);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        if A[pos] == '.' {
            A[pos] = '@';
            if 0 < pos {
                que.push_back(pos-1);
            }
            if pos + 1 < N {
                que.push_back(pos+1);
            }
        }
    }
    println!("{}", A.iter().collect::<String>());
}