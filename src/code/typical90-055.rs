// https://atcoder.jp/contests/typical90/tasks/typical90_bc

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: usize,
        Q: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    for a in 0..N {
        for b in a+1..N {
            for c in b+1..N {
                for d in c+1..N {
                    for e in d+1..N {
                        let res = A[a] * A[b] % P * A[c] % P * A[d] % P * A[e] % P;
                        if res == Q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}