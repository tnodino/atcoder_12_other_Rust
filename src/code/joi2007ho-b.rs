// https://atcoder.jp/contests/joi2007ho/tasks/joi2007ho_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut flg = vec![false; n+1];
    for _ in 0..k {
        input! {
            a: usize,
        }
        flg[a] = true;
    }
    let mut now = false;
    let mut cnt = 0;
    let mut vec = Vec::new();
    for i in 1..=n {
        if now != flg[i] {
            vec.push(cnt);
            now = flg[i];
            cnt = 0;
        }
        cnt += 1;
    }
    vec.push(cnt);
    if now {
        vec.push(0);
    }
    let m = vec.len();
    let mut ans = 0;
    for i in (1..m).step_by(2) {
        ans = max(ans, vec[i]);
        if flg[0] && i < m - 2 {
            ans = max(ans, vec[i] + vec[i+2] + 1);
        }
    }
    println!("{}", ans);
}