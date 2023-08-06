// https://atcoder.jp/contests/joi2006ho/tasks/joi2006ho_c

use proconio::input;
use proconio::fastout;

fn dfs(n: usize, vec: &mut Vec<usize>) {
    if n == 0 {
        println!("{}", vec[1..].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        return;
    }
    for i in (1..=n).rev() {
        if vec[vec.len()-1] < i {
            continue;
        }
        vec.push(i);
        dfs(n-i, vec);
        vec.pop();
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut vec = vec![99];
    dfs(n, &mut vec);
}