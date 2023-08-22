// https://atcoder.jp/contests/past202303-open/tasks/past202303_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut ans = vec![0; N];
    for i in 0..N {
        ans[P[i]-1] = i + 1;
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}