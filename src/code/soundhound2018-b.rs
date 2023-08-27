// https://atcoder.jp/contests/soundhound2018/tasks/soundhound2018_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        L: usize,
        R: usize,
        mut a: [usize; n],
    }
    for i in 0..n {
        if a[i] <= L {
            a[i] = L;
        }
        else if R <= a[i] {
            a[i] = R;
        }
    }
    println!("{}", a.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}