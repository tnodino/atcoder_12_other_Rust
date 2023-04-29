// https://atcoder.jp/contests/joi2006ho/tasks/joi2006ho_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut cnt = vec![0; m];
    for _ in 0..n {
        input! {
            a: [usize; m],
        }
        for i in 0..m {
            cnt[i] += a[i];
        }
    }
    let mut vec = Vec::new();
    for i in 0..m {
        vec.push((cnt[i], i + 1));
    }
    vec.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    println!("{}", vec.iter().map(|&x| x.1.to_string()).collect::<Vec<String>>().join(" "));
}