// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ec

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut pow100 = vec![1; N+1];
    for i in 0..N {
        pow100[i+1] = pow100[i] * 100;
        pow100[i+1] %= MOD;
    }
    let mut hashl = vec![0; N+1];
    for i in 0..N {
        hashl[i+1] = hashl[i] * 100 + (S[i] as usize) - ('a' as usize) + 1;
        hashl[i+1] %= MOD;
    }
    let mut hashr = vec![0; N+1];
    for i in 0..N {
        hashr[i+1] = hashr[i] * 100 + (S[N-i-1] as usize) - ('a' as usize) + 1;
        hashr[i+1] %= MOD;
    }
    for _ in 0..Q {
        input! {
            L: usize,
            R: usize,
        }
        let idx = R - L + 1;
        let hash1 = (hashl[R] + MOD - (hashl[L-1] * pow100[idx] % MOD)) % MOD;
        let hash2 = (hashr[N-L+1] + MOD - (hashr[N-R] * pow100[idx] % MOD)) % MOD;
        if hash1 == hash2 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}