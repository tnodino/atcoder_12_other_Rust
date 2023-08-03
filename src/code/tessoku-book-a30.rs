// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ad

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

pub struct Combination {
    fact: Vec<usize>,
    factinv: Vec<usize>,
}

impl Combination {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![0; n+1];
        let mut factinv = vec![0; n+1];
        fact[0] = 1;
        factinv[0] = 1;
        for i in 1..=n {
            fact[i] = (fact[i-1] * i) % MOD;
            factinv[i] = bin_power(fact[i], MOD-2);
        }
        Self {
            fact,
            factinv,
        }
    }

    pub fn ncr(&mut self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        return ((self.fact[n] * self.factinv[r]) % MOD * self.factinv[n-r]) % MOD;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        r: usize,
    }
    let mut Comb = Combination::new(n);
    println!("{}", Comb.ncr(n, r));
}