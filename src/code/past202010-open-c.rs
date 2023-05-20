// https://atcoder.jp/contests/past202010-open/tasks/past202010_c

use proconio::input;
use proconio::fastout;

const DX: &[usize] = &[!0, !0, !0, 0, 0, 0, 1, 1, 1];
const DY: &[usize] = &[!0, 0, 1, !0, 0, 1, !0, 0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut s = Vec::new();
    for _ in 0..N {
        input! {
            x: String,
        }
        let x = x.chars().collect::<Vec<char>>();
        s.push(x);
    }
    let mut ans = vec![vec![0; M]; N];
    for i in 0..N {
        for j in 0..M {
            for d in 0..9 {
                let nx = i.wrapping_add(DX[d]);
                let ny = j.wrapping_add(DY[d]);
                if N <= nx || M <= ny {
                    continue;
                }
                if s[nx][ny] == '#' {
                    ans[i][j] += 1;
                }
            }
        }
    }
    for i in 0..N {
        println!("{}", ans[i].iter().map(|&x| x.to_string()).collect::<String>());
    }
}