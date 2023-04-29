// https://atcoder.jp/contests/past202004-open/tasks/past202004_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut cnt = [0; 3];
    for s in S.chars() {
        cnt[s as usize - 97] += 1;
    }
    let ma = *cnt.iter().max().unwrap();
    for i in 0..3 {
        if ma == cnt[i] {
            println!("{}", (i as u8 + 97) as char);
        }
    }
}