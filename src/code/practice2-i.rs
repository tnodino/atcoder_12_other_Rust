// https://atcoder.jp/contests/practice2/tasks/practice2_i

use proconio::input;
use proconio::fastout;
use ac_library::{suffix_array, lcp_array};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let sa = suffix_array(&S);
    let lcp = lcp_array(&S, &sa);
    println!("{}", (N + 1) * N / 2 - lcp.iter().sum::<usize>());
}