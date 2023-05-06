// https://atcoder.jp/contests/past202005-open/tasks/past202005_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        Q: usize,
    }
    let mut score = vec![N; M];
    let mut flg = vec![vec![false; M]; N];
    for _ in 0..Q {
        input! {
            s: usize,
        }
        if s == 1 {
            input! {
                n: usize,
            }
            let mut res = 0;
            for j in 0..M {
                if flg[n-1][j] {
                    res += score[j];
                }
            }
            println!("{}", res);
        }
        else {
            input! {
                n: usize,
                m: usize,
            }
            score[m-1] -= 1;
            flg[n-1][m-1] = true;
        }
    }
}