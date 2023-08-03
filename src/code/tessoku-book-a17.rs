// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_q

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N-1],
        B: [usize; N-2],
    }
    let mut DP = vec![0; N];
    DP[1] = A[0];
    for i in 2..N {
        DP[i] = min(DP[i-1] + A[i-1], DP[i-2] + B[i-2]);
    }
    let mut ans = vec![N];
    let mut idx = N - 1;
    while idx > 0 {
        if DP[idx-1] + A[idx-1] == DP[idx] {
            ans.push(idx);
            idx -= 1;
        }
        else {
            ans.push(idx-1);
            idx -= 2;
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().rev().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}