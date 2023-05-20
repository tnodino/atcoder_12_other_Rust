// https://atcoder.jp/contests/arc001/tasks/arc001_2

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let mut cnt = vec![99; 41];
    cnt[A] = 0;
    let mut que = VecDeque::new();
    que.push_back(A);
    while !que.is_empty() {
        let x = que.pop_front().unwrap();
        if 1 <= x && cnt[x-1] == 99 {
            cnt[x-1] = cnt[x] + 1;
            que.push_back(x-1);
        }
        if 5 <= x && cnt[x-5] == 99 {
            cnt[x-5] = cnt[x] + 1;
            que.push_back(x-5);
        }
        if 10 <= x && cnt[x-10] == 99 {
            cnt[x-10] = cnt[x] + 1;
            que.push_back(x-10);
        }
        if x <= 39 && cnt[x+1] == 99 {
            cnt[x+1] = cnt[x] + 1;
            que.push_back(x+1);
        }
        if x <= 35 && cnt[x+5] == 99 {
            cnt[x+5] = cnt[x] + 1;
            que.push_back(x+5);
        }
        if x <= 30 && cnt[x+10] == 99 {
            cnt[x+10] = cnt[x] + 1;
            que.push_back(x+10);
        }
    }
    println!("{}", cnt[B]);
}