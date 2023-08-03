// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_w

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut A = Vec::new();
    for _ in 0..M {
        input! {
            a: [usize; N],
        }
        let mut res = 0;
        for i in 0..N {
            res <<= 1;
            res += a[i];
        }
        A.push(res);
    }
    let mut DP = vec![vec![100; 1<<N]; M+1];
    DP[0][0] = 0;
    for i in 0..M {
        for bit in 0..1<<N {
            DP[i+1][bit] = min(DP[i+1][bit], DP[i][bit]);
            DP[i+1][bit|A[i]] = min(DP[i+1][bit|A[i]], DP[i][bit] + 1);
        }
    }
    if DP[M][(1<<N)-1] == 100 {
        println!("-1");
    }
    else {
        println!("{}", DP[M][(1<<N)-1]);
    }
}