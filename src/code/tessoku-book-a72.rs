// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bt

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
    }
    let mut c = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        c.push(s);
    }
    let mut ans = 0;
    for bit in 0..1<<H {
        let mut flg = 0;
        let mut s = c.clone();
        for i in 0..H {
            if bit & (1 << i) > 0 {
                flg += 1;
                for j in 0..W {
                    s[i][j] = '#';
                }
            }
        }
        if flg > K {
            continue;
        }
        let mut res = 0;
        let mut cnt = vec![0; W];
        for i in 0..H {
            for j in 0..W {
                if s[i][j] == '#' {
                    res += 1;
                }
                else {
                    cnt[j] += 1;
                }
            }
        }
        cnt.sort_by(|a, b| b.cmp(a));
        for i in 0..K-flg {
            res += cnt[i];
        }
        ans = max(ans, res);
    }
    println!("{}", ans);
}