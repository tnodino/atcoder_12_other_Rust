// https://atcoder.jp/contests/dp/tasks/dp_h

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut a = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        a.push(s);
    }
    let mut DP = vec![vec![0; W]; H];
    DP[0][0] = 1;
    for i in 0..H {
        for j in 0..W {
            if i + 1 < H && a[i+1][j] == '.' {
                DP[i+1][j] += DP[i][j];
                DP[i+1][j] %= MOD;
            }
            if j + 1 < W && a[i][j+1] == '.' {
                DP[i][j+1] += DP[i][j];
                DP[i][j+1] %= MOD;
            }
        }
    }
    println!("{}", DP[H-1][W-1]);
}