// https://atcoder.jp/contests/genocon2021/tasks/genocon2021_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        m: usize,
    }
    for _ in 0..m {
        input! {
            s: String,
        }
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut ans = Vec::new();
        for i in (0..n).rev() {
            ans.push(match s[i] {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                _ => 'C',
            });
        }
        println!("{}", ans.iter().collect::<String>());
    }
}