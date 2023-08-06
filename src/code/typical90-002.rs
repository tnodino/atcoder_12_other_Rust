// https://atcoder.jp/contests/typical90/tasks/typical90_b

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    'outer: for bit in 0..1<<N {
        let mut s = Vec::new();
        for i in 0..N {
            if bit & (1 << i) == 0 {
                s.push('(');
            }
            else {
                s.push(')');
            }
        }
        let s = s.into_iter().rev().collect::<Vec<char>>();
        let mut cnt = 0;
        for i in 0..N {
            if s[i] == '(' {
                cnt += 1;
            }
            else {
                if cnt == 0 {
                    continue 'outer;
                }
                cnt -= 1;
            }
        }
        if cnt == 0 {
            println!("{}", s.iter().collect::<String>());
        }
    }
}