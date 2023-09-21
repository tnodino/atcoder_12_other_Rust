// https://atcoder.jp/contests/practice2/tasks/practice2_a

use proconio::input;
use proconio::fastout;
use ac_library::Dsu;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
    }
    let mut UF = Dsu::new(N);
    for _ in 0..Q {
        input! {
            (t, u, v): (usize, usize, usize),
        }
        match t {
            0 => {
                UF.merge(u, v);
            },
            1 => {
                println!("{}", UF.same(u, v) as usize);
            },
            _ => {},
        }
    }
}