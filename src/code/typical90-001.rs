// https://atcoder.jp/contests/typical90/tasks/typical90_a

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: usize, N: usize, L: usize, K: usize, A: &Vec<usize>) -> bool {
    let mut cnt = 0;
    let mut pre = 0;
    for i in 0..N {
        if A[i] - pre >= x && L - A[i] >= x {
            cnt += 1;
            pre = A[i];
        }
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
        mut N: usize,
        L: usize,
        K: usize,
        A: [usize; N],
    }
    let mut ok = 0;
    let mut ng = L + 1;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        if f(mid, N, L, K, &A) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}