// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fi

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: f64, N: usize, K: usize, A: &Vec<f64>) -> bool {
    let mut cnt = 0;
    for i in 0..N {
        cnt += (A[i] / x) as usize;
    }
    if cnt >= K {
        return true;
    }
    return false;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [f64; N],
    }
    let mut ok = 1.;
    let mut ng = 1_000_000_000.;
    for _ in 0..100 {
        let mid = (ok + ng) / 2.;
        if f(mid, N, K, &A) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    let mut ans = Vec::new();
    for i in 0..N {
        ans.push((A[i] / ok) as usize);
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}