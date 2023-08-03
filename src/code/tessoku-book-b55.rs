// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_eb

use proconio::input;
use proconio::fastout;
use std::cmp::min;
use std::collections::BTreeSet;

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut set = BTreeSet::new();
    for _ in 0..Q {
        input! {
            q: usize,
            x: isize,
        }
        if q == 1 {
            set.insert(x);
        }
        else {
            let mut ans = INF;
            if let Some(&val) = set.range(x..).next() {
                ans = min(ans, (x - val).abs());
            }
            if let Some(&val) = set.range(..=x).next_back() {
                ans = min(ans, (x - val).abs());
            }
            if ans == INF {
                println!("-1");
            }
            else {
                println!("{}", ans);
            }
        }
    }
}