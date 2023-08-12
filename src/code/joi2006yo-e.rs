// https://atcoder.jp/contests/joi2006yo/tasks/joi2006yo_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        _k: usize,
        m: usize,
        r: usize,
    }
    let ma = 20000;
    let mut d = vec![0; ma];
    let mut x = 1;
    let k = n;
    for i in 0..ma {
        d[i] = x / k;
        x %= k;
        x *= 10;
    }
    if m == 1 {
        for b in 1..n {
            let mut x = 1;
            let k = b * n;
            for i in 0..ma {
                d[i] += x / k;
                x %= k;
                x *= 10;
            }
        }
    }
    for i in (1..ma).rev() {
        d[i-1] += d[i] / 10;
        d[i] %= 10;
    }
    print!("{}.", d[0]);
    for i in 1..=r {
        print!("{}", d[i]);
    }
    println!();
}