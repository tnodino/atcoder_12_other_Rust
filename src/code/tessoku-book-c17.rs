// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fo

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut que1 = VecDeque::new();
    let mut que2 = VecDeque::new();
    for _ in 0..Q {
        input! {
            q: char,
        }
        if q == 'A' {
            input! {
                X: String,
            }
            que2.push_back(X);
            if que1.len() < que2.len() {
                que1.push_back(que2.pop_front().unwrap());
            }
        }
        else if q == 'B' {
            input! {
                X: String,
            }
            if que1.len() == que2.len() {
                que1.push_back(X);
            }
            else {
                que2.push_front(X);
            }
        }
        else if q == 'C' {
            que1.pop_front();
            if que1.len() < que2.len() {
                que1.push_back(que2.pop_front().unwrap());
            }
        }
        else {
            println!("{}", que1[0]);
        }
    }
}