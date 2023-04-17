// https://atcoder.jp/contests/abc298/tasks/abc298_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [[usize; N]; N],
        B: [[usize; N]; N],
    }
    for _ in 0..4 {
        let mut C = vec![vec![0; N]; N];
        for i in 0..N {
            for j in 0..N {
                C[N-j-1][i] = A[i][j];
            }
        }
        let mut flg = true;
        for i in 0..N {
            for j in 0..N {
                if A[i][j] == 1 {
                    if B[i][j] == 0 {
                        flg = false;
                    }
                }
            }
        }
        if flg {
            println!("Yes");
            return;
        }
        A = C;
    }
    println!("No");
}