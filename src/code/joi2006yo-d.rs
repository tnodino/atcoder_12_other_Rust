// https://atcoder.jp/contests/joi2006yo/tasks/joi2006yo_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut flg = vec![[false; 3]; n];
    for i in 0..3 {
        input! {
            k: usize,
            vec: [usize; k],
        }
        for j in 0..k {
            flg[vec[j]-1][i] = true;
        }
    }
    let mut cnt = vec![0; n];
    for i in 1..n {
        cnt[i] = cnt[i-1] * 3 + 2;
    }
    let mut ans: usize = 1<<60;
    for k in 0..2 {
        let mut res = 0;
        let mut now = k << 1;
        for i in 0..n {
            if flg[i][now] {
                continue;
            }
            else if flg[i][1] {
                res += cnt[n-i-1] + 1;
                now ^= 2;
            }
            else {
                res += cnt[n-i-1] * 2 + 2;
            }
        }
        ans = min(ans, res);
    }
    if ans <= m {
        println!("{}", ans);
    }
    else {
        println!("-1");
    }
}