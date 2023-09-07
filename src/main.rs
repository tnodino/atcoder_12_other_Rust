use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, T): (usize, usize),
    }
    let mut DP = vec![vec![0; N]; T+1];
    DP[0][0] = 1;
    for i in 0..T {
        for j in 0..N {
            for k in 1..=6 {
                let idx = (j + k) % N;
                DP[i+1][idx] += DP[i][j];
                DP[i+1][idx] %= MOD;
            }
        }
    }
    println!("{}", DP[T][0]);
}