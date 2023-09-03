// https://atcoder.jp/contests/dp/tasks/dp_r

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn matrix(A: &Vec<Vec<usize>>, B: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let L = A.len();
    let N = A[0].len();
    let M = B[0].len();
    let mut C = vec![vec![0; M]; L];
    for i in 0..L {
        for j in 0..N {
            for k in 0..M {
                C[i][k] += A[i][j] * B[j][k];
                C[i][k] %= MOD;
            }
        }
    }
    return C;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut a = Vec::new();
    for _ in 0..N {
        input! {
            x: [usize; N],
        }
        a.push(x);
    }
    let M = 60;
    let mut mat = vec![vec![vec![0; N]; N]; M];
    mat[0] = a;
    for i in 1..M {
        mat[i] = matrix(&mat[i-1], &mat[i-1]);
    }
    let mut DP = vec![vec![0; N]; N];
    for i in 0..N {
        DP[i][i] = 1;
    }
    for i in 0..M {
        if K & (1 << i) > 0 {
            DP = matrix(&DP, &mat[i]);
        }
    }
    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            ans += DP[i][j];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}