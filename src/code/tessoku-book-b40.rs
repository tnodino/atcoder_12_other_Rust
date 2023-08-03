// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dm

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 100;
    let mut cnt = vec![0; M];
    for i in 0..N {
        cnt[A[i] % 100] += 1;
    }
    let mut ans: usize = 0;
    for i in 0..M {
        let j = (M - i) % 100;
        if i > j {
            break;
        }
        if i == j {
            if cnt[i] >= 2 {
                ans += cnt[i] * (cnt[i] - 1) / 2;
            }
        }
        else {
            ans += cnt[i] * cnt[j];
        }
    }
    println!("{}", ans);
}