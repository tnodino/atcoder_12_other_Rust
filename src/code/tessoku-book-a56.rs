// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bd

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
    let mut hash = vec![0; N+1];
    for i in 0..N {
        hash[i+1] = hash[i] * 100 + (S[i] as usize) - ('A' as usize) + 1;
        hash[i+1] %= MOD;
    }
    for _ in 0..Q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        let idx = b - a + 1;
        let hash1 = (hash[b] + MOD - (hash[a-1] * pow100[idx] % MOD)) % MOD;
        let idx = d - c + 1;
        let hash2 = (hash[d] + MOD - (hash[c-1] * pow100[idx] % MOD)) % MOD;
        if hash1 == hash2 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}