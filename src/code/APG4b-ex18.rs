// https://atcoder.jp/contests/APG4b/tasks/APG4b_ce

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut vec = vec![vec!['-'; N]; N];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        vec[A-1][B-1] = 'o';
        vec[B-1][A-1] = 'x';
    }
    for i in 0..N {
        println!("{}", vec[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}