// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_t

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                for l in k+1..N {
                    for m in l+1..N {
                        if A[i] + A[j] + A[k] + A[l] + A[m] == 1_000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}