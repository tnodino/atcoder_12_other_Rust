// https://atcoder.jp/contests/tenka1-2012-final/tasks/tenka1_2012_final_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut n: usize,
    }
    let mut vec: Vec<usize> = vec![0; 60];
    vec[0] = 1;
    vec[1] = 1;
    for i in 2..60 {
        vec[i] = vec[i-1] + vec[i-2];
    }
    let mut ans = 0;
    for i in (0..60).rev() {
        while n >= vec[i] {
            ans += 1;
            n -= vec[i];
        }
    }
    println!("{}", ans);
}