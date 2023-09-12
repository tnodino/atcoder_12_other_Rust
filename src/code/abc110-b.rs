// https://atcoder.jp/contests/abc110/tasks/abc110_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, X, Y): (usize, usize, isize, isize),
        x: [isize; N],
        y: [isize; M],
    }
    'outer: for z in X+1..=Y {
        for i in 0..N {
            if x[i] >= z {
                continue 'outer;
            }
        }
        for i in 0..M {
            if y[i] < z {
                continue 'outer;
            }
        }
        println!("No War");
        return;
    }
    println!("War");
}