// https://atcoder.jp/contests/practice2/tasks/practice2_c

use proconio::input;
use proconio::fastout;

pub fn floor_sum(n: isize, m: isize, mut a: isize, mut b: isize) -> isize {
    let mut ans = 0;
    if a >= m {
        ans += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }

    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;
    if y_max == 0 {
        return ans;
    }
    ans += (n - (x_max + a - 1) / a) * y_max;
    ans += floor_sum(y_max, a, m, (a - x_max % a) % a);
    return ans
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            N: isize,
            M: isize,
            A: isize,
            B: isize,
        }
        println!("{}", floor_sum(N, M, A, B));
    }
}