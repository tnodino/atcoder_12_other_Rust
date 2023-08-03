// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_az

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut que = VecDeque::new();
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: String,
            }
            que.push_back(x);
        }
        else if q == 2 {
            println!("{}", que[0]);
        }
        else {
            que.pop_front().unwrap();
        }
    }
}