// https://atcoder.jp/contests/joi2021yo2/tasks/joi2021_yo2_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    let mut cnt = S.iter().filter(|&x| *x == '#').count();
    let mut l = A;
    let mut r = A;
    let mut t = 0;
    let mut ans = 0;
    while cnt > 0 {
        let x;
        if t == 0 {
            x = r;
            for i in x..=N {
                if S[i-1] == '#' {
                    S[i-1] = '.';
                    cnt -= 1;
                    break;
                }
                r += 1;
            }
        }
        else {
            x = l;
            for i in (1..=x).rev() {
                if S[i-1] == '#' {
                    S[i-1] = '.';
                    cnt -= 1;
                    break;
                }
                l -= 1;
            }
        }
        t = (t + 1) % 2;
        ans += r - l;
    }
    println!("{}", ans);
}