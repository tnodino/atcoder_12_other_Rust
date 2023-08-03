// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ed

use proconio::input;
use proconio::fastout;

fn f(num: usize) -> usize {
    let mut x = num;
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    return num - s;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut DP = vec![vec![0; N+1]; 64];
    for i in 1..=N {
        DP[0][i] = f(i);
    }
    for d in 1..64 {
        for i in 0..=N {
            DP[d][i] = DP[d-1][DP[d-1][i]];
        }
    }
    let mut ans = vec![0; N+1];
    for i in 1..=N {
        ans[i] = i;
    }
    for d in (0..64).rev() {
        if K & (1 << d) > 0 {
            for i in 1..=N {
                ans[i] = DP[d][ans[i]];
            }
        }
    }
    for i in 1..=N {
        println!("{}", ans[i]);
    }
}