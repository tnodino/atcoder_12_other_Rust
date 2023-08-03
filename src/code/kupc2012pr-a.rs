// https://atcoder.jp/contests/kupc2012pr/tasks/kupc2012pr_1

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        m: usize,
        n: usize,
    }
    let M = 3;
    let N = 60;
    let mut ans = vec![vec![0; N+1]; M+1];
    for j in 0..=N {
        ans[0][j] = j + 1;
    }
    ans[1][0] = ans[0][1];
    for j in 1..=N {
        ans[1][j] = ans[1][j-1] + 1;
    }
    ans[2][0] = ans[1][1];
    for j in 1..=N {
        ans[2][j] = ans[2][j-1] + 2;
    }
    ans[3][0] = ans[2][1];
    for j in 1..=N {
        ans[3][j] = ans[3][j-1] + pow(2, j+2);
    }
    println!("{}", ans[m][n]);
}