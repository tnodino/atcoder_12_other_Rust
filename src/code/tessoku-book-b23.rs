// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cv

use proconio::input;
use proconio::fastout;
use libm::hypot;

const INF: f64 = 1000000000.;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut X = Vec::new();
    let mut Y = Vec::new();
    for _ in 0..N {
        input! {
            x: f64,
            y: f64,
        }
        X.push(x);
        Y.push(y);
    }
    let mut DP = vec![vec![INF; N]; 1<<N];
    for i in 1..N {
        DP[1<<i][i] = hypot((X[i]-X[0]).abs(), (Y[i]-Y[0]).abs());
    }
    for bit in 0..1<<N {
        for i in 0..N {
            if bit & (1 << i) > 0 {
                continue;
            }
            let bit2 = bit | (1 << i);
            for j in 0..N {
                if bit & (1 << j) > 0 {
                    let res = DP[bit][j] + hypot((X[i]-X[j]).abs(), (Y[i]-Y[j]).abs());
                    if res < DP[bit2][i] {
                        DP[bit2][i] = res;
                    }
                }
            }
        }
    }
    println!("{}", DP[(1<<N)-1][0]);
}