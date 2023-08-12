// https://atcoder.jp/contests/joi2023yo1c/tasks/joi2023_yo1c_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut vec = Vec::new();
    for i in 0..N {
        vec.push((A[i], i));
    }
    vec.sort();
    let mut ans = vec![0; N];
    ans[vec[0].1] = 1;
    for i in 1..N {
        if vec[i-1].0 == vec[i].0 {
            ans[vec[i].1] = ans[vec[i-1].1];
        }
        else {
            ans[vec[i].1] = i + 1;
        }
    }
    for i in 0..N {
        println!("{}", ans[i]);
    }
}