// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_be

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut V: [usize; K],
    }
    let mut ans = 0;
    for bit in 1..1<<K {
        let mut x = 1;
        let mut cnt = 0;
        for i in 0..K {
            if bit & (1 << i) > 0 {
                x = lcm(x, V[i]);
                cnt += 1;
            }
        }
        if cnt % 2 == 0 {
            ans -= N / x;
        }
        else {
            ans += N / x;
        }
    }
    println!("{}", ans);
}