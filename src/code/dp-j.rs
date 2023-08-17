// https://atcoder.jp/contests/dp/tasks/dp_j

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;
    for i in 0..N {
        match a[i] {
            1 => c1 += 1,
            2 => c2 += 1,
            _ => c3 += 1,
        }
    }
    let mut DP = vec![vec![vec![0.; N+1]; N+1]; N+1];
    for k in 0..=N {
        for j in 0..=N {
            for i in 0..=N {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                if i + j + k > N {
                    continue;
                }
                let n = N as f64;
                let x = i as f64;
                let y = j as f64;
                let z = k as f64;
                DP[i][j][k] = 1.;
                if i >= 1 {
                    DP[i][j][k] += DP[i-1][j][k] * x / n;
                }
                if j >= 1 {
                    DP[i][j][k] += DP[i+1][j-1][k] * y / n;
                }
                if k >= 1 {
                    DP[i][j][k] += DP[i][j+1][k-1] * z / n;
                }
                DP[i][j][k] *= n / (x + y + z);
            }
        }
    }
    println!("{}", DP[c1][c2][c3]);
}