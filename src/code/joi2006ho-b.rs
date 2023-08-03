// https://atcoder.jp/contests/joi2006ho/tasks/joi2006ho_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut s = s.chars().collect::<Vec<char>>();
    for _ in 0..n {
        let M = s.len();
        let mut t = "".to_string();
        let mut c = s[0];
        let mut cnt = 1;
        for i in 1..M {
            if c != s[i] {
                let x = cnt * 10 + (c as usize) - ('0' as usize);
                let x = x.to_string();
                t = format!("{}{}", t, x);
                c = s[i];
                cnt = 1;
            }
            else {
                cnt += 1;
            }
        }
        let x = cnt * 10 + (c as usize) - ('0' as usize);
        let x = x.to_string();
        t = format!("{}{}", t, x);
        s = t.chars().collect::<Vec<char>>();
    }
    println!("{}", s.iter().collect::<String>());
}