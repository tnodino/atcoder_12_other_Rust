// https://atcoder.jp/contests/typical90/tasks/typical90_bi

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
            t: usize,
            x: usize,
        }
        if t == 1 {
            que.push_front(x);
        }
        else if t == 2 {
            que.push_back(x);
        }
        else {
            println!("{}", que[x-1]);
        }
    }
}