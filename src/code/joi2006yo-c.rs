// https://atcoder.jp/contests/joi2006yo/tasks/joi2006yo_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut dice = vec![1, 3, 6, 4, 2, 5];
    let op = [[(0, 1), (1, 2), (2, 3), (3, 0)],
                [(4, 1), (1, 5), (5, 3), (3, 4)],
                [(4, 0), (0, 5), (5, 2), (2, 4)],
                [(4, 3), (3, 5), (5, 1), (1, 4)],
                [(4, 2), (2, 5), (5, 0), (0, 4)],
                [(0, 3), (3, 2), (2, 1), (1, 0)]];
    let mut ans = 1;
    for _ in 0..n {
        input! {
            s: String,
        }
        let idx = match s.as_str() {
            "East" => 0,
            "Left" => 1,
            "North" => 2,
            "Right" => 3,
            "South" => 4,
            _ => 5,
        };
        let tmp = dice[op[idx][0].0];
        for i in (1..4).rev() {
            dice[op[idx][i].1] = dice[op[idx][i].0];
        }
        dice[op[idx][0].1] = tmp;
        ans += dice[0];
    }
    println!("{}", ans);
}