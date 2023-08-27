// https://atcoder.jp/contests/typical90/tasks/typical90_f

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut DP = vec![vec![N; 26]; N+1];
    for i in (0..N).rev() {
        for j in 0..26 {
            if (S[i] as usize) - ('a' as usize) == j {
                DP[i][j] = i;
            }
            else {
                DP[i][j] = DP[i+1][j];
            }
        }
    }
    let mut idx = 0;
    for i in 1..=K {
        for j in 0..26 {
            let nxt = N + i - (DP[idx][j] + 1);
            if nxt >= K {
                print!("{}", ((j as u8) + ('a' as u8)) as char);
                idx = DP[idx][j] + 1;
                break;
            }
        }
    }
    println!();
}