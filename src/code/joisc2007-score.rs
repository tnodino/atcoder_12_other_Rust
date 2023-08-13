// https://atcoder.jp/contests/joisc2007/tasks/joisc2007_score

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push((s[i], i));
    }
    vec.sort_by(|a, b| b.0.cmp(&a.0));
    let mut ans = vec![0; n];
    ans[vec[0].1] = 1;
    for i in 1..n {
        if vec[i-1].0 != vec[i].0 {
            ans[vec[i].1] = i + 1;
        }
        else {
            ans[vec[i].1] = ans[vec[i-1].1];
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}