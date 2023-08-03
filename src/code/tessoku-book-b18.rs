// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cq

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N],
    }
    let mut DP = vec![vec![false; S+1]; N+1];
    DP[0][0] = true;
    for i in 0..N {
        for j in 0..=S {
            DP[i+1][j] |= DP[i][j];
            if j + A[i] <= S {
                DP[i+1][j+A[i]] |= DP[i][j];
            }
        }
    }
    if !DP[N][S] {
        println!("-1");
        return;
    }
    let mut ans = Vec::new();
    let mut idx = S;
    for i in (0..N).rev() {
        if A[i] <= idx && DP[i][idx-A[i]] {
            idx -= A[i];
            ans.push(i+1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}