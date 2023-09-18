// https://atcoder.jp/contests/typical90/tasks/typical90_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
        A: [[usize; W]; H],
    }
    let mut row = vec![0; H];
    let mut column = vec![0; W];
    for i in 0..H {
        for j in 0..W {
            row[i] += A[i][j];
            column[j] += A[i][j];
        }
    }
    for i in 0..H {
        let mut ans = vec![0; W];
        for j in 0..W {
            ans[j] = row[i] + column[j] - A[i][j];
        }
        println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}